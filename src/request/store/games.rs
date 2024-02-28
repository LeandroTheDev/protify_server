//Protify Dependencies
use crate::{
    components::database::Database,
    request::handler::{DefaultResponse, ErrorStruct},
};

//Rust Dependencies
use std::{collections::HashMap, convert::Infallible};

//Plugins Dependencies
use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use serde_json::{json, Value};

pub fn store_showcase() -> Result<Response<Full<Bytes>>, Infallible> {
    // //Store Path
    // let mut store_path: PathBuf;
    // //Getting the protify path
    // {
    //     if let Ok(protify_path) = env::current_dir() {
    //         store_path = protify_path.clone();
    //     } else {
    //         return ErrorStruct::internal_server_error(String::from(
    //             "invalid_store_path_contact_the_server_owner",
    //         ));
    //     }
    // }
    // store_path.push("store");

    // //Games and Softwares
    // let mut games: Vec<String> = Vec::new();
    // let mut softwares: Vec<String> = Vec::new();
    // //Get games
    // {
    //     store_path.push("games");
    //     let directory: Result<ReadDir, std::io::Error> = store_path.read_dir();
    //     match directory {
    //         Ok(dir) => {
    //             for entry_result in dir {
    //                 match entry_result {
    //                     Ok(entry) => games.push(entry.file_name().to_string_lossy().to_string()),
    //                     Err(e) => eprintln!("[Games] Cannot read the game: {:?}", e.kind()),
    //                 }
    //             }
    //         }
    //         Err(e) => eprintln!("[Games] Cannot read the games folder: {:?}", e.kind()),
    //     }
    //     store_path.pop();
    // }
    // //Get Softwares
    // {
    //     store_path.push("softwares");
    //     let directory: Result<ReadDir, std::io::Error> = store_path.read_dir();
    //     match directory {
    //         Ok(dir) => {
    //             for entry_result in dir {
    //                 match entry_result {
    //                     Ok(entry) => {
    //                         softwares.push(entry.file_name().to_string_lossy().to_string())
    //                     }
    //                     Err(e) => eprintln!("[Games] Cannot read the softwares: {:?}", e.kind()),
    //                 }
    //             }
    //         }
    //         Err(e) => eprintln!("[Games] Cannot read the softwares folder: {:?}", e.kind()),
    //     }
    //     store_path.pop();
    // }

    //Getting the store items throught database
    let mut store_items: Vec<u32> = vec![];
    let database_result: Result<Database, std::io::Error> = Database::new();
    match database_result {
        Ok(database) => {
            //Getting database result
            let response: Vec<HashMap<String, String>> =
                database.select(vec![], "SHOWCASE", vec![], vec!["ID"]);
            //Check database error
            match database.check_errors(&response) {
                Some(err) => return ErrorStruct::internal_server_error(err),
                None => {}
            }
            //Swiping all showcases
            let mut length: usize = 0;
            for id_showcase in response {
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
        Err(err) => return ErrorStruct::internal_server_error(err.to_string()),
    }

    //Success Response
    let json_body: Value = json!({
        "message": "SUCCESS",
        "content": store_items,
    });
    let mut response: DefaultResponse = DefaultResponse::new(json_body.to_string(), StatusCode::OK);
    Ok(response.build_response())
}
pub fn download_item(query: HashMap<String, String>) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("{:?}", query);
    let json_body: Value = json!({
        "message": "SUCCESS"
    });
    let mut response: DefaultResponse = DefaultResponse::new(json_body.to_string(), StatusCode::OK);
    Ok(response.build_response())
}
pub fn get_item_info(query: HashMap<String, String>) -> Result<Response<Full<Bytes>>, Infallible> {
    let item_id: u16 = match query.get("item") {
        Some(value) => match value.parse() {
            Ok(parsed_value) => parsed_value,
            Err(_) => return ErrorStruct::invalid_parameters(),
        },
        None => return ErrorStruct::invalid_parameters(),
    };
    let item_data: HashMap<String, Value>;
    //Getting item data
    let database_result: Result<Database, std::io::Error> = Database::new();
    match database_result {
        Ok(database) => {
            let response: Vec<HashMap<String, String>> = database.select(
                vec![],
                "STORE",
                vec![format!("ID = {}", item_id).as_str()],
                vec!["ID", "NAME", "CATEGORY", "LANGUAGES", "DESCRIPTION"],
            );
            //Check database error
            match database.check_errors(&response) {
                Some(err) => return ErrorStruct::internal_server_error(err),
                None => {}
            }
            //Check if exist
            if response.len() == 0 {
                return ErrorStruct::invalid_parameters();
            }
            //Convert the data to json
            item_data = Database::convert_hash_map_to_json_value(response[0].clone());
        }
        Err(err) => return ErrorStruct::internal_server_error(err.to_string()),
    }
    let json_body: Value = json!({
        "message": "SUCCESS",
        "content": item_data
    });
    let mut response: DefaultResponse = DefaultResponse::new(json_body.to_string(), StatusCode::OK);
    println!("{:?}", item_data);
    Ok(response.build_response())
}
