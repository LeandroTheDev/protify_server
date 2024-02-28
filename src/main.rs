//Protify Dependencies
mod request;
use hyper::header::HeaderValue;
use request::handler::RequestHandler;
mod components;
use components::ip_hash::IpHash;

use std::collections::HashMap;
//Rust Dependencies
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;

//Plugins Dependencies
use http_body_util::{BodyExt, Collected, Full};
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{HeaderMap, Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

///Server Listining Ports
const PORTS: u16 = 6161;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut ips_timeout: IpHash = IpHash::new();
    let server_address = SocketAddr::from(([127, 0, 0, 1], PORTS));

    // We create a TcpListener and bind it to 127.0.0.1:???
    let listener = TcpListener::bind(server_address).await?;

    println!("[Server] Listining in ports: {:?}", PORTS);

    // We start a loop to continuously accept incoming connections
    loop {
        //Server Overload
        //1 thousand simultanious is the limit
        if ips_timeout.length() >= 1000 {
            continue;
        }

        // Get tcp stream
        let (stream, client_address) = listener.accept().await?;

        //DDOS Attack Protection
        let address_string: String = client_address.ip().to_string();
        //Check the client limit connections
        let timeout_value: u8 = ips_timeout.get_value(&address_string);
        if timeout_value < ips_timeout.limit {
            if timeout_value + 1 >= ips_timeout.limit {
                println!("[Server] Banned: {:?}", address_string);
                ips_timeout.insert(address_string.clone());
            } else {
                if timeout_value >= 80 {
                    println!(
                        "[Server] Suspicious request: {:?}, limit: {:?}",
                        address_string, timeout_value
                    );
                }
                ips_timeout.insert(address_string.clone());
            }
        }
        //If is banned just ignore
        else {
            continue;
        }

        // Convert the TcpStream into Io Tokio Stream
        let io = TokioIo::new(stream);

        // Handle multiple connections concurrently
        tokio::task::spawn(async move {
            let service_handler = Arc::new(ServiceHandler::new(address_string));

            let cloned_handler = Arc::clone(&service_handler);

            let service_function = service_fn(move |req| {
                let cloned_handler = Arc::clone(&cloned_handler);
                async move { cloned_handler.handle_service(req).await }
            });
            // Building http message
            if let Err(err) = http1::Builder::new()
                // Transforming the http message to the handler
                .serve_connection(io, service_function)
                .await
            {
                println!("[Server] User failed to connect: {:?}", err);
            }
        });
    }
}

struct ServiceHandler {
    ip: String,
}
impl ServiceHandler {
    pub fn new(client_ip: String) -> Self {
        ServiceHandler { ip: client_ip }
    }
    pub async fn handle_service(
        &self,
        request: Request<hyper::body::Incoming>,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        //Function Helper to handle request
        fn handle_request(
            url: String,
            method: Method,
            header: hyper::HeaderMap,
            query: HashMap<String, String>,
            body_string: String,
        ) -> Result<Response<Full<Bytes>>, Infallible> {
            //Creating the handler for url
            let handler: RequestHandler =
                RequestHandler::new(url.to_string(), method, header, query, body_string);
            //Receiving the response
            let response: Result<Response<Full<Bytes>>, Infallible> = handler.handle_request();
            //Returning to the client
            response
        }

        //Getting the url
        let mut url: String = request.uri().path().to_string();
        //Checking url limit size
        if url.len() > 1000 {
            return handle_request(
                String::from("/limit_overflow"),
                Method::GET,
                HeaderMap::new(),
                HashMap::new(),
                String::from("Limit Overflow"),
            );
        }
        
        //Getting the query
        let mut query: HashMap<String, String> = HashMap::new();
        //Spliting the string to receive only the "&..." part
        for query_param in url.split('&') {
            //Spliting the key and value based in "="
            if let Some((key, value)) = query_param.split_once('=') {
                //Inserting into query the key and value
                query.insert(key.to_string(), value.to_string());
            }
        }
        //Removing the "&" values after the url
        if let Some((path, _)) = url.split_once('&') {
            url = path.to_string()
        }

        //Getting the method (GET/POST/DELETE...)
        let method: Method = request.method().clone();

        //Getting the Headers
        let mut headers: hyper::HeaderMap = request.headers().clone();
        //Receiving the headers size
        let headers_size: u8 = match headers.capacity().try_into() {
            Ok(value) => value,
            Err(_) => 255,
        };
        //Check the headers size limit
        if headers_size == 255 {
            return handle_request(
                String::from("/limit_overflow"),
                Method::GET,
                headers,
                query,
                String::from("Limit Overflow"),
            );
        }
        //Inserting the full client address into headers
        headers.insert(
            "host_full",
            HeaderValue::from_str(self.ip.as_str()).unwrap(),
        );
        //Getting the body size
        let body_size: u8 = match request.body().size_hint().upper() {
            Some(value) => match value.try_into() {
                Ok(value) => value,
                Err(_) => 255,
            },
            None => 255,
        };
        //Check body size limit in kilobytes
        if body_size == 255 {
            return handle_request(
                String::from("/limit_overflow"),
                Method::GET,
                headers,
                query,
                String::from("Limit Overflow"),
            );
        }
        //Receiving the body data
        let body: Collected<Bytes> = match request.into_body().collect().await {
            Ok(collected) => collected,
            Err(_) => http_body_util::Collected::default(),
        };
        //Transforming to bytes
        let body_bytes: Bytes = body.to_bytes();
        //Full body string
        let body_string: String = String::from_utf8_lossy(&body_bytes.to_vec()).into_owned();

        //Return the response for the client
        handle_request(url, method, headers, query, body_string)
    }
}
