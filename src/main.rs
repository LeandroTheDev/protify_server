mod request;
use request::handler::RequestHandler;

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

const PORTS: u16 = 3000;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let server_address = SocketAddr::from(([127, 0, 0, 1], PORTS));

    // We create a TcpListener and bind it to 127.0.0.1:???
    let listener = TcpListener::bind(server_address).await?;

    println!("Server started in ports: {:?}", PORTS);

    // We start a loop to continuously accept incoming connections
    loop {
        // Get tcp stream
        let (stream, client_address) = listener.accept().await?;

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
    //Getting the url
    let url: &Uri = request.uri();
    //Creating the handler for url
    let handler: RequestHandler = RequestHandler::new(url.to_string());
    //Receiving the response
    let response: Result<Response<Full<Bytes>>, Infallible> = handler.handle_request().await;
    //Returning to the client
    response
}
