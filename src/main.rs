//Protify Dependencies
mod request;
use request::handler::RequestHandler;
mod components;
use components::ip_hash::IpHash;

//Rust Dependencies
use std::convert::Infallible;
use std::net::SocketAddr;

//Plugins Dependencies
use http_body_util::{BodyExt, Collected, Full};
use hyper::body::{Body, Bytes};
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Method, Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const PORTS: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut ips_timeout: IpHash = IpHash::new();
    let server_address = SocketAddr::from(([127, 0, 0, 1], PORTS));

    // We create a TcpListener and bind it to 127.0.0.1:???
    let listener = TcpListener::bind(server_address).await?;

    println!("Server started in ports: {:?}", PORTS);

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
        let addres_string = client_address.to_string();
        //Check the client limit connections
        if ips_timeout.get_value(&addres_string) < ips_timeout.limit {
            ips_timeout.insert(addres_string.clone());
        }
        //If is banned just ignore
        else {
            continue;
        }

        println!("request from: {:?}", client_address);

        // Convert the TcpStream into Io Tokio Stream
        let io = TokioIo::new(stream);

        // Handle multiple connections concurrently
        tokio::task::spawn(async move {
            // Building http message
            if let Err(err) = http1::Builder::new()
                // Transforming the http message to the handler
                .serve_connection(io, service_fn(handler))
                .await
            {
                println!("Error connection: {:?}", err);
            }
        });
    }
}

///Handles the message from the client
async fn handler(
    request: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    async fn handle_request(
        url: String,
        method: Method,
        header: hyper::HeaderMap,
        body_string: String,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the handler for url
        let handler: RequestHandler =
            RequestHandler::new(url.to_string(), method, header, body_string);
        //Receiving the response
        let response: Result<Response<Full<Bytes>>, Infallible> = handler.handle_request().await;
        //Returning to the client
        response
    }
    //Getting the url
    let url: String = request.uri().clone().to_string();

    //Getting the method (GET/POST/DELETE...)
    let method: Method = request.method().clone();

    //Getting the Headers
    let headers: hyper::HeaderMap = request.headers().clone();
    //Receiving the headers size
    let headers_size: u16 = match headers.capacity().try_into() {
        Ok(value) => value,
        Err(_) => 0,
    };
    //Check if the headers size is valid
    if headers_size == 0 {
        return handle_request(
            String::from("/limit_overflow"),
            method,
            headers,
            String::from("Limit Overflow"),
        )
        .await;
    }

    //Getting the body size
    let body_size: u64 = match request.body().size_hint().upper() {
        Some(value) => value,
        None => 0,
    };
    //Checking if the body size is upper the max: 10 mb
    if body_size >= 10485760 {
        return handle_request(
            String::from("/limit_overflow"),
            method,
            headers,
            String::from("Limit Overflow"),
        )
        .await;
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

    handle_request(url, method, headers, body_string).await
}
