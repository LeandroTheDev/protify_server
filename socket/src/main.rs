mod request;

use std::collections::HashMap;

use serde_json::Value;
//Plugins Dependencies
use websocket::sync::Server;
use websocket::OwnedMessage;

use crate::request::handler;

///Server Listining Ports
const PORTS: u16 = 6262;

fn main() {
    let server_address = format!("{}:{}", "127.0.0.1", PORTS);
    //Listing the address
    let server = Server::bind(server_address).unwrap();
    println!("[Socket] Listining in ports: {:?}", PORTS);

    for request in server.filter_map(Result::ok) {
        //Try to accept connection
        let client = match request.accept() {
            //Success accepting the connection
            Ok(client) => client,
            //Invalid connection request
            Err(_) => continue,
        };

        //Get client address
        let ip = match client.peer_addr() {
            //Success getting the address
            Ok(value) => value.ip(),
            //Invalid address
            Err(_) => continue,
        };
        println!("{} connected", ip);

        //Spliting the client into receiver and sender
        let (mut receiver, mut sender) = match client.split() {
            Ok(value) => value,
            //Cannot split the receivers and senders
            Err(_) => {
                eprintln!("[Server] Panic spliting client: {}", ip);
                continue;
            }
        };

        //Awaiting for client messages
        for message in receiver.incoming_messages() {
            //Message check
            let message = match message {
                Ok(message) => message,
                Err(_) => break,
            };

            //Process the received message
            match message {
                OwnedMessage::Text(text) => {
                    //Transforming the received text into Json Value
                    let v: Value = serde_json::from_str(&text).unwrap();

                    let mut response_action: String = String::from("");
                    let mut response_body: HashMap<String, String> = HashMap::new();
                    //Checking if the value is a object
                    if let Value::Object(map) = v {
                        //Converting
                        let mut hashmap: HashMap<String, String> = HashMap::new();
                        for (key, value) in map {
                            if let Value::String(s) = value {
                                hashmap.insert(key, s);
                            }
                        }

                        //Receiving the action
                        if let Some(action) = hashmap.get("ACTION") {
                            response_action = action.to_string();
                        }
                        //Receiving the id
                        if let Some(id) = hashmap.get("ID") {
                            response_body.insert("ID".to_owned(), id.to_string());
                        }
                    }
                    if response_action.as_str() == "" {
                        eprint!("[Server] Invalid response from: {}", ip);
                    }

                    handler::ResponseHandler::handle_response();
                    sender
                        .send_message(&OwnedMessage::Text(String::from("ok")))
                        .unwrap();
                }
                _ => (),
            }
        }
    }
}
