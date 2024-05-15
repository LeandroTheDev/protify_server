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

    //Addreses using the services, only 1 address can use only 1 service at time
    let mut ip_services: HashMap<String, bool> = HashMap::new();

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
        println!("[Socket] {} connected", ip);

        //Services Check
        match ip_services.get(&ip.to_string()) {
            Some(_) => {
                println!(
                    "[Socket] {} trying to access more than 1 service at a time",
                    ip
                );
                continue;
            }
            None => {
                let _ = ip_services.insert(ip.to_string(), true);
            }
        }

        //Spliting the client into receiver and sender
        let (mut receiver, sender) = match client.split() {
            Ok(value) => value,
            //Cannot split the receivers and senders
            Err(_) => {
                eprintln!("[Socket] Panic spliting client: {}", ip);
                continue;
            }
        };

        let mut handler: handler::ResponseHandler =
            handler::ResponseHandler::new(sender, ip.to_string());

        //Awaiting for client messages
        for message in receiver.incoming_messages() {
            //Connection check
            let message = match message {
                Ok(message) => message,
                Err(_) => {
                    ip_services.remove(&ip.to_string());
                    println!("[Socket] {} connection terminated", ip);
                    break;
                }
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
                    //Checking if action is empty
                    if response_action.as_str() == "" {
                        eprint!("[Socket] Invalid response from: {}", ip);
                    }

                    handler.set_body_action(response_body, response_action);
                    handler.handle_response();
                }
                _ => (),
            }
        }
    }
}
