use std::{collections::HashMap, io::Write};

use serde_json::json;

// Context Libs
use crate::{
    libs::{database::main::Database, http::response::HttpResponse},
    protify::main::{Protify, ProtifyError},
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
                    Some(_) => return Protify::error(response, ProtifyError::DatabaseError),
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
            Err(_) => return Protify::error(response, ProtifyError::DatabaseError),
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
}
