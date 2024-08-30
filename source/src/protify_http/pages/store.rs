use std::{collections::HashMap, env, io::Write, path::PathBuf};

use serde_json::{json, Value};

// Context Libs
use crate::{
    libs::{database::main::Database, http::response::HttpResponse},
    protify_http::main::{ProtifyError, ProtifyHttp},
};

pub struct Instance {}
impl Instance {
    pub fn showcase(mut response: HttpResponse) {
        let mut store_items: Vec<u32> = vec![];
        let database_result: Result<Database, std::io::Error> = Database::new();
        match database_result {
            Ok(database) => {
                //Getting database result
                let database_response: Vec<HashMap<String, String>> =
                    database.select(vec![], "SHOWCASE", vec![], vec!["ID"]);
                //Check database error
                match database.check_errors(&database_response) {
                    Some(_) => return ProtifyHttp::error(response, ProtifyError::DatabaseError),
                    None => {}
                }
                //Swiping all showcases
                let mut length: usize = 0;
                for id_showcase in database_response {
                    //Inserting in the items vector the item id
                    store_items.insert(
                        length,
                        match id_showcase.get("ID") {
                            Some(value) => {
                                let value_u32: u32 = match value.parse() {
                                    Ok(value) => value,
                                    Err(_) => 0,
                                };
                                value_u32
                            }
                            None => 0,
                        },
                    );
                    length += 1;
                }
            }
            Err(_) => return ProtifyHttp::error(response, ProtifyError::DatabaseError),
        }

        // Generating success message
        response.status_code = 200;
        response.status_message = String::from("SUCCESS");
        let json_body: String = json!(store_items).to_string();

        // Sending to the client
        let _ = response
            .stream
            .write_all(response.generate_response(json_body).as_bytes());
    }

    pub fn get_item(mut response: HttpResponse) {
        // Getting the item id from query
        let item_id: u32 = match response.query.get("item") {
            Some(id_str) => match id_str.parse::<u32>() {
                Ok(id) => id,
                Err(_) => 0,
            },
            None => 0,
        };
        // Checking if the id is valid
        if item_id == 0 {
            return ProtifyHttp::error(response, ProtifyError::InvalidParameter);
        }

        let item_data: HashMap<String, Value>;
        //Getting item data
        let database_result: Result<Database, std::io::Error> = Database::new();
        match database_result {
            Ok(database) => {
                let database_response: Vec<HashMap<String, String>> = database.select(
                    vec![],
                    "STORE",
                    vec![format!("ID = {}", item_id).as_str()],
                    vec!["ID", "NAME", "CATEGORY", "LANGUAGES", "DESCRIPTION"],
                );
                //Check database error
                match database.check_errors(&database_response) {
                    Some(_) => return ProtifyHttp::error(response, ProtifyError::InvalidParameter),
                    None => {}
                }
                //Check if exist
                if database_response.len() == 0 {
                    return ProtifyHttp::error(response, ProtifyError::InvalidParameter);
                }
                //Convert the data to json
                item_data = Database::convert_hash_map_to_json_value(database_response[0].clone());
            }
            Err(_) => return ProtifyHttp::error(response, ProtifyError::InternalError),
        }

        // Generating success message
        response.status_code = 200;
        response.status_message = String::from("SUCCESS");
        let json_body: String = json!(item_data).to_string();

        // Sending to the client
        let _ = response
            .stream
            .write_all(response.generate_response(json_body).as_bytes());
    }

    pub fn start_download(mut response: HttpResponse) {
        // Getting the PathBuf for the executable
        let directory: PathBuf = match env::current_exe() {
            Ok(exe_path) => match exe_path.parent().map(|p| p.to_path_buf()) {
                Some(path) => path,
                None => return ProtifyHttp::error(response, ProtifyError::InternalError),
            },
            Err(_) => return ProtifyHttp::error(response, ProtifyError::InternalError),
        };
        println!("{:?}", directory);
    }
}
