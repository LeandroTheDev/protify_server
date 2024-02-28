//Protify Dependencies
use crate::components::authentication::{Authentication, Permissions};
use crate::request::handler::{DefaultResponse, ErrorStruct};

//Rust Dependencies
use std::{convert::Infallible, env, fs::ReadDir, path::PathBuf};

//Plugins Dependencies
use http_body_util::Full;
use hyper::{body::Bytes, HeaderMap, Response, StatusCode};
use serde_json::{json, Value};

pub async fn store_main(header: HeaderMap) -> Result<Response<Full<Bytes>>, Infallible> {
    match Authentication::new(header) {
        Ok(authentication) => {
            if authentication
                .authenticate(Permissions::ACTION_STORE_MAIN)
                .await
                != Permissions::PERMISSION_GRANTED
            {
                return ErrorStruct::authentication_required();
            }
        }
        Err(err) => return ErrorStruct::internal_server_error(err.to_string()),
    };
    //Store Path
    let mut store_path: PathBuf;
    //Getting the protify path
    {
        if let Ok(protify_path) = env::current_dir() {
            store_path = protify_path.clone();
        } else {
            return ErrorStruct::internal_server_error(String::from(
                "The server cannot get the path, please contact the server owner",
            ));
        }
    }
    store_path.push("store");

    //Games and Softwares
    let mut games: Vec<String> = Vec::new();
    let mut softwares: Vec<String> = Vec::new();
    //Get games
    {
        store_path.push("games");
        let directory: Result<ReadDir, std::io::Error> = store_path.read_dir();
        match directory {
            Ok(dir) => {
                for entry_result in dir {
                    match entry_result {
                        Ok(entry) => games.push(entry.file_name().to_string_lossy().to_string()),
                        Err(e) => eprintln!("Cannot read the game: {:?}", e.kind()),
                    }
                }
            }
            Err(e) => eprintln!("Cannot read the games folder: {:?}", e.kind()),
        }
        store_path.pop();
    }
    //Get Softwares
    {
        store_path.push("softwares");
        let directory: Result<ReadDir, std::io::Error> = store_path.read_dir();
        match directory {
            Ok(dir) => {
                for entry_result in dir {
                    match entry_result {
                        Ok(entry) => {
                            softwares.push(entry.file_name().to_string_lossy().to_string())
                        }
                        Err(e) => eprintln!("Cannot read the software: {:?}", e.kind()),
                    }
                }
            }
            Err(e) => eprintln!("Cannot read the software folder: {:?}", e.kind()),
        }
        store_path.pop();
    }

    //Success Response
    let json_body: Value = json!({
        "message": "Success",
        "content": json!({
            "games": games,
            "softwares": softwares,
        }),
    });
    let mut response: DefaultResponse = DefaultResponse::new(json_body.to_string(), StatusCode::OK);
    Ok(response.build_response())
}
pub fn download_game(header: HeaderMap, body: String) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("{:?}", header);
    println!("{:?}", body);
    let json_body: Value = json!({
        "message": "Success"
    });
    let mut response: DefaultResponse = DefaultResponse::new(json_body.to_string(), StatusCode::OK);
    Ok(response.build_response())
}
