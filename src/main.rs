use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
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
        //Get stream
        let (stream, client_address) = listener.accept().await?;
        
        println!("New connection: {:?}", client_address);

        // Use an adapter to create a stream for connection
        let io = TokioIo::new(stream);

        // Spawn a tokio task to serve multiple connections concurrently
        tokio::task::spawn(async move {
            // Finally, we bind the incoming connection to our `hello` service
            if let Err(err) = http1::Builder::new()
                // `service_fn` converts our function in a `Service`
                .serve_connection(io, service_fn(handler))
                .await
            {
                println!("Error connection: {:?}", err);
            }
        });
    }
}
async fn handler(request: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    let url = request.uri();
    let mut response = Response::new(Full::new(Bytes::from("Hello, World!")));
    let headers = response.headers_mut();
    headers.insert("Connection", "close".parse().unwrap());
    Ok(response)
}