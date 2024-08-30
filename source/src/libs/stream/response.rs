use std::{
    collections::HashMap,
    fmt,
    net::{IpAddr, TcpStream},
};

use super::status::StreamConnectionStatus;

pub struct StreamResponse {
    pub address: IpAddr,
    pub port: u16,
    pub method: StreamMethod,
    pub url: String,
    pub version: String,
    pub agent: String,
    pub enconding: String,
    pub body: String,
    pub stream: TcpStream,
    pub status: StreamConnectionStatus,
    pub query: HashMap<String, String>,

    pub status_code: u16,
    pub status_message: String,
}

impl StreamResponse {
    pub fn new(
        _address: IpAddr,
        _port: u16,
        _method: &str,
        _url: &str,
        _version: &str,
        _agent: &str,
        _enconding: &str,
        _body: String,
        _stream: TcpStream,
        _status: StreamConnectionStatus,
        _query: HashMap<String, String>,
    ) -> StreamResponse {
        Self {
            address: _address,
            port: _port,
            method: Self::parse_stream_method(_method),
            url: _url.to_string(),
            version: _version.to_string(),
            agent: _agent.to_string(),
            enconding: _enconding.to_string(),
            body: _body,
            stream: _stream,
            status: _status,
            query: _query,

            // Default values
            status_code: 200,
            status_message: String::from("OK"),
        }
    }

    /// Parsing the string to the StreamMethod
    pub fn parse_stream_method(method: &str) -> StreamMethod {
        match method {
            "GET" => StreamMethod::GET,
            "POST" => StreamMethod::POST,
            "DELETE" => StreamMethod::DELETE,
            "PATCH" => StreamMethod::PATCH,
            "UPDATE" => StreamMethod::UPDATE,
            _ => StreamMethod::UNKOWN,
        }
    }

    /// Generates the response body to send to the client
    pub fn generate_response_with_file(&self, body: String) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_message,
            body.len(),
            body
        )
    }
}

pub enum StreamMethod {
    GET,
    POST,
    DELETE,
    PATCH,
    UPDATE,
    UNKOWN,
}
impl fmt::Display for StreamMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StreamMethod::GET => write!(f, "GET"),
            StreamMethod::POST => write!(f, "POST"),
            StreamMethod::DELETE => write!(f, "DELETE"),
            StreamMethod::PATCH => write!(f, "PATCH"),
            StreamMethod::UPDATE => write!(f, "UPDATE"),
            StreamMethod::UNKOWN => write!(f, "UNKOWN"),
        }
    }
}
