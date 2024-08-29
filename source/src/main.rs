mod libs;
mod protify;

// Context Libs
use libs::{
    http::{main::HttpInstance, response::HttpResponse},
    logs,
};
use logs::main::LogsInstance;
use protify::main::Protify;

// Rust Libs
use std::{
    net::{IpAddr, Ipv4Addr},
    sync::mpsc,
    thread,
};

fn main() {
    // Protify Service
    {
        let protify_service: Protify = Protify::new();
        // Initializing http instance
        let http_instance: HttpInstance =
            HttpInstance::new(6161, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        // Http listener instance
        thread::spawn(move || {
            // Instanciating communication channel between structs
            let (sender, receiver): (mpsc::Sender<HttpResponse>, mpsc::Receiver<HttpResponse>) =
                mpsc::channel();
            // Start listening http responses
            thread::spawn(move || http_instance.infinity_listen(sender));

            // Creating the infinite loop for listenng the http instance
            loop {
                match receiver.recv() {
                    // Message received
                    Ok(response) => {
                        protify_service.receive_request(response);
                    }
                    // No messages
                    Err(_) => {
                        break;
                    }
                }
            }
        });
    }

    // Main Thread
    LogsInstance::print("Starting main thread", colored::Color::White);
    loop {}
}
