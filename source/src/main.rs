mod libs;
mod protify_http;
mod protify_stream;

// Context Libs
use libs::{
    http::{main::HttpInstance, response::HttpResponse},
    logs,
    stream::{main::StreamInstance, response::StreamResponse},
};
use logs::main::LogsInstance;
use protify_http::main::ProtifyHttp;
use protify_stream::main::ProtifyStream;

// Rust Libs
use std::{
    env,
    net::{IpAddr, Ipv4Addr},
    sync::mpsc,
    thread,
};

fn main() {
    // Printing the current execution path
    match env::current_exe() {
        Ok(exe_path) => match exe_path.parent().map(|p| p.to_path_buf()) {
            Some(path) => LogsInstance::print(
                format!("Working on path: {:?}", path).as_str(),
                colored::Color::White,
            ),
            None => LogsInstance::print(
                "[ERROR] Cannot read the executable path",
                colored::Color::Red,
            ),
        },
        Err(err) => LogsInstance::print(
            format!("[ERROR] Cannot read the executable path, reason: {}", err).as_str(),
            colored::Color::Red,
        ),
    };

    // Protify Service
    {
        // Initializing http instance
        let http_instance: HttpInstance =
            HttpInstance::new(6161, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));
        // Initializing stream instance
        let stream_instance: StreamInstance =
            StreamInstance::new(6262, IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)));

        // Http listener instance
        thread::spawn(move || {
            let protify_service: ProtifyHttp = ProtifyHttp::new();
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

        // Stream listener instance
        thread::spawn(move || {
            let protify_service: ProtifyStream = ProtifyStream::new();
            // Instanciating communication channel between structs
            let (sender, receiver): (mpsc::Sender<StreamResponse>, mpsc::Receiver<StreamResponse>) =
                mpsc::channel();
            // Start listening stream responses
            thread::spawn(move || stream_instance.infinity_listen(sender));

            // Creating the infinite loop for listenng the stream instance
            loop {
                match receiver.recv() {
                    // Message received
                    Ok(response) => {
                        protify_service.receive_stream(response);
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
