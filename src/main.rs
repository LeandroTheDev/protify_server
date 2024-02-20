//Protify Dependencies
mod request;
use request::handler::RequestHandler;

//Rust Dependencies
use std::collections::HashMap;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;

//Plugins Dependencies
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

    let mut ips_timeout: HashMap<String, i32> = HashMap::new();

    // We create a TcpListener and bind it to 127.0.0.1:???
    let listener = TcpListener::bind(server_address).await?;

    println!("Server started in ports: {:?}", PORTS);

    // We start a loop to continuously accept incoming connections
    loop {
        //Server Overload
        //500 millions simultanious is the limit
        if ips_timeout.len() > 500000000 {
            continue;
        }

        // Get tcp stream
        let (stream, client_address) = listener.accept().await?;

        //DDOS Attack Protection
        let addres_string = &client_address.to_string();
        //Check client existence
        if let Some(&timeout_address) = ips_timeout.get(addres_string) {
            //Check if client is timedout, the limit is 99 requisition in 1 minute
            if timeout_address == 99 {
                println!("temporary banned: {:?}", addres_string);
                continue;
            }
            //Add one limiar for the timeout
            ips_timeout.insert(addres_string.clone(), timeout_address + 1);
        }
        //In other cases create a timeout for the ddos limit and create a line to ips_timeout
        else {
            //This will make the timeout reset after 60 seconds
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                settimeout::set_timeout(Duration::from_secs(60)).await;
                ips_timeout.remove_entry(addres_string);
            });
            //Start counting
            ips_timeout.insert(addres_string.clone(), 1);
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
    //Getting the url
    let url: &Uri = request.uri();
    //Creating the handler for url
    let handler: RequestHandler = RequestHandler::new(url.to_string());
    //Receiving the response
    let response: Result<Response<Full<Bytes>>, Infallible> = handler.handle_request().await;
    //Returning to the client
    response
}
