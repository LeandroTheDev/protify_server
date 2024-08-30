#![allow(dead_code)]
// Context Libs
use crate::libs::logs::main::LogsInstance;
use crate::libs::stream::response::StreamResponse;
use crate::libs::stream::status::StreamConnectionStatus;

use std::collections::HashMap;
use std::net::Ipv4Addr;
// Rust Libs
use std::{
    io::{Read, Write},
    net::{IpAddr, TcpListener, TcpStream},
    sync::mpsc,
};

pub struct StreamInstance {
    port: u16,
    address: IpAddr,
    status: StreamConnectionStatus,
    listener: Option<TcpListener>,
}

impl StreamInstance {
    /// Instanciate a new stream connection
    pub fn new(_port: u16, _address: IpAddr) -> StreamInstance {
        let _listener: Option<TcpListener>;
        let _status: StreamConnectionStatus;

        // Try listen to address
        match TcpListener::bind(format!("{}:{}", _address, _port)) {
            // Handling success
            Ok(listener_result) => {
                LogsInstance::print(
                    format!("Stream instance will listen to {}:{}", _address, _port).as_str(),
                    colored::Color::White,
                );

                // Instanciate to the structure
                _listener = Some(listener_result);
                _status = StreamConnectionStatus::Success;
            }
            // Handling errors
            Err(err) => {
                LogsInstance::print(
                    format!("Cannot listen to: {}:{} \nReason: {}", _address, _port, err).as_str(),
                    colored::Color::Red,
                );
                _status = StreamConnectionStatus::Failed;
                _listener = None;
            }
        }

        Self {
            port: _port,
            address: _address,
            status: _status,
            listener: _listener,
        }
    }

    /// Start listening to the address and port set calling the function handle_stream
    pub fn infinity_listen(self, channel: mpsc::Sender<StreamResponse>) {
        // Check if listener exist
        match self.listener {
            Some(listener) => {
                LogsInstance::print(
                    format!(
                        "Stream instance started listening in {}:{}",
                        self.address, self.port
                    )
                    .as_str(),
                    colored::Color::Green,
                );

                // Try to listen the stream
                for stream in listener.incoming() {
                    match stream {
                        // Handling the stream stream
                        Ok(stream) => {
                            handle_stream(self.address, self.port, stream, channel.clone())
                        }
                        Err(_) => return,
                    };
                }
            }
            None => {
                LogsInstance::print(
                    "Error, cannot listen a bind not set, stream requisitions will not be received",
                    colored::Color::Red,
                );
            }
        }
    }

    /// Receive the actual connection status of the instance
    pub fn get_status(self) -> StreamConnectionStatus {
        self.status
    }

    /// Receive the address of the instance
    pub fn get_address(self) -> IpAddr {
        self.address
    }

    /// Receives the port of the instance
    pub fn get_port(self) -> u16 {
        self.port
    }
}

fn handle_stream(
    true_address: IpAddr,
    true_port: u16,
    mut stream: TcpStream,
    channel: mpsc::Sender<StreamResponse>,
) {
    // 512 bytes limit for the buffer, anything more than that will be corrupted
    let mut buffer: [u8; 512] = [0; 512];

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            // Read received response as utf8
            let request = match std::str::from_utf8(&buffer[..bytes_read]) {
                Ok(character) => character,
                Err(_) => "",
            };

            // Invalid request just ignore
            if request.is_empty() {
                LogsInstance::print(
                    format!("Invalid request send by: {:?}", stream.peer_addr()).as_str(),
                    colored::Color::Yellow,
                );
                return;
            }

            // Sending the message to the channel
            let _ = channel.send(request_treatment(true_address, true_port, request, stream));
        }
        Err(_) => {
            let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nOverflow";
            _ = stream.write(error_response.as_bytes());
            _ = stream.flush();
        }
    }
}

fn request_treatment(
    true_address: IpAddr,
    true_port: u16,
    request: &str,
    mut stream: TcpStream,
) -> StreamResponse {
    println!("{}", request);
    let mut status: StreamConnectionStatus = StreamConnectionStatus::Success;

    // Get all lines from the request
    let mut lines: Vec<&str> = request.lines().collect();

    if lines.len() < 3 {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Headers";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }

    // Line 0 should receive the: Type, URL, Version
    let request_methods: Vec<&str> = lines[0].split_whitespace().collect();
    if request_methods.len() < 3 {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Headers";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }
    let method: &str = request_methods[0];
    let mut request_url: &str = request_methods[1];
    let version: &str = request_methods[2];

    // Formatting the quries into hashmap
    let mut query: HashMap<String, String> = HashMap::new();
    for pair in request_url.split('&') {
        let mut split_pair = pair.split('=');
        if let (Some(key), Some(value)) = (split_pair.next(), split_pair.next()) {
            // Converting everthing into string
            query.insert(key.to_string(), value.to_string());
        }
    }
    // Removing the queries from url
    request_url = match request_url.split_once('&') {
        Some((before_ampersand, _)) => before_ampersand, // Url without queries
        None => request_url, // If cannot find the queries return the url
    };

    // Line 1 should receive the: User Agent
    let request_agent: Vec<&str> = lines[1].split_whitespace().collect();
    if request_agent.len() < 1 {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Headers";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }
    let agent: &str = request_agent[1];

    // Line 2 should receive the: Enconding
    let request_enconding: Vec<&str> = lines[2].split_whitespace().collect();
    if request_enconding.len() < 1 {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Headers";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }
    let enconding: &str = request_enconding[1];

    // Line 3 should have the client address
    let request_address: Vec<&str> = lines[3].split_whitespace().collect();
    if request_address.len() < 1 {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Headers";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }
    let full_addreess = request_address[1];
    let divided_address: Vec<&str> = full_addreess.split(":").collect();
    if divided_address.len() < 2 {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Headers";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }
    let address: IpAddr = divided_address[0]
        .parse::<IpAddr>()
        .unwrap_or_else(|_| IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)));
    let port: u16 = divided_address[1].parse().unwrap_or(0);

    // Checking if the true address is the same as the headers address
    if true_address != address && true_port != true_port {
        let error_response: &str = "HTTP/1.1 400 Bad Request\r\n\r\nInvalid Address";
        _ = stream.write(error_response.as_bytes());
        _ = stream.flush();
        status = StreamConnectionStatus::Failed;
    }

    // Remove header lines
    lines.drain(0..4);

    // Reconverting to string again
    let body: String = lines.join("\n");

    // Returning the stream response
    StreamResponse::new(
        address,
        port,
        method,
        request_url,
        version,
        agent,
        enconding,
        body,
        stream,
        status,
        query,
    )
}
