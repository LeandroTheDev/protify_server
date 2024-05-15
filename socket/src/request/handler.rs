use std::{collections::HashMap, env, fs::ReadDir, net::TcpStream, path::PathBuf};



use serde_json::{json, Value};
use websocket::{sync::Writer, OwnedMessage};

pub struct ResponseHandler {
    ip: String,
    sender: Writer<TcpStream>,
    body: HashMap<String, String>,
    action: String,
    service_type: String,
}
impl ResponseHandler {
    /// Creates a new instance to handle the socket
    pub fn new(response_sender: Writer<TcpStream>, response_ip: String) -> ResponseHandler {
        ResponseHandler {
            ip: response_ip,
            sender: response_sender,
            body: HashMap::new(),
            action: String::from("EMPTY"),
            service_type: String::from("EMPTY"),
        }
    }

    /// Handle the response based in the action
    pub fn handle_response(&mut self) {
        match self.action.as_str() {
            "AUTHENTICATE" => self.authenticate_user(),
            "GET_ITEM_INFO" => self.get_item_info(),
            _ => {}
        }
    }

    /// Updates body and action from the actual object
    pub fn set_body_action(
        &mut self,
        response_body: HashMap<String, String>,
        response_action: String,
    ) {
        self.body = response_body;
        self.action = response_action;
    }

    /// Handles the Authentication
    fn authenticate_user(&mut self) {
        
        let mut response: HashMap<String, String> = HashMap::new();
        response.insert(String::from("MESSAGE"), String::from("AUTHENTICATED"));
        let result = self
            .sender
            .send_message(&OwnedMessage::Text(serde_json::json!(response).to_string()));
        match result {
            Ok(_) => {
                println!("[Download] {} started a downloading service", self.ip);
                self.service_type = String::from("DOWNLOAD");
            }
            Err(_) => {}
        }
    }

    fn get_item_info(&mut self) {
        //Store Path
        let mut item_path: PathBuf = match env::current_dir() {
            Ok(value) => value,
            Err(_) => {
                let _ = self.sender.shutdown_all();
                return;
            }
        };
        item_path.pop();
        item_path.push("data");
        item_path.push("store");

        //Get game id
        let game_id = match self.body.get("ID") {
            Some(value) => value,
            None => {
                eprintln!("[Item] Cannot get item info from any empty ID");
                let _ = self.sender.shutdown_all();
                return;
            }
        };
        //Check if id is valid
        match game_id.parse::<u32>() {
            Ok(_) => {}
            Err(_) => {
                eprintln!("[Item] Cannot get item info from any invalid number ID");
                let _ = self.sender.shutdown_all();
                return;
            }
        }

        item_path.push(game_id);

        let mut item_quantity: u32 = 0;
        //Get item quantity
        {
            let directory: Result<ReadDir, std::io::Error> = item_path.read_dir();
            match directory {
                Ok(dir) => {
                    for entry_result in dir {
                        match entry_result {
                            Ok(_) => {
                                item_quantity += 1;
                            }
                            Err(e) => {
                                eprintln!("[Item] Cannot read the item: {:?}", e.kind());
                                let _ = self.sender.shutdown_all();
                                return;
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("[Item] Cannot read the item folder: {:?}", e.kind());
                    let _ = self.sender.shutdown_all();
                    return;
                }
            }
            item_path.pop();
        }


        //Sending Response
        let mut response: HashMap<String, Value> = HashMap::new();
        response.insert(String::from("MESSAGE"), json!("GAME_INFO"));
        response.insert(String::from("CONTENT"), json!(item_quantity));
        println!("{:?}", response);
        let _ = self
            .sender
            .send_message(&OwnedMessage::Text(serde_json::json!(response).to_string()));
    }
}
