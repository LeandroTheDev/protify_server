use std::{
    collections::HashMap,
    io::{self},
};

use super::database::Database;

use hyper::HeaderMap;

pub struct Permissions {}
impl Permissions {
    pub const ACTION_LOGIN: u16 = 0;
    pub const ACTION_STORE_MAIN: u16 = 1;
    pub const ACTION_CHANGE_USERNAME: u16 = 2;
    pub const ACTION_CHANGE_PASSWORD: u16 = 3;
    pub const ACTION_GET_GAME_INFO: u16 = 4;
    pub const ACTION_DOWNLOAD_GAME: u16 = 5;

    pub const PERMISSION_GRANTED: &'static str = "Permission Granted";
    pub const NO_PERMISSION_MESSAGE: &'static str = "No Permission";
    pub const INVALID_TOKEN: &'static str = "Invalid Token";

    ///Check permissions for anonymous user
    pub fn check_anoynmous_permission(action: u16) -> &'static str {
        match action {
            Self::ACTION_LOGIN => Self::PERMISSION_GRANTED,
            Self::ACTION_STORE_MAIN => Self::PERMISSION_GRANTED,
            Self::ACTION_GET_GAME_INFO => Self::PERMISSION_GRANTED,
            //UNKNOWN
            _ => Self::NO_PERMISSION_MESSAGE,
        }
    }
    ///Check if a logged user has the permission for the selected action
    pub fn check_user_permission(action: u16, _user_auth: &Authentication) -> &str {
        match action {
            Self::ACTION_STORE_MAIN => Self::PERMISSION_GRANTED,
            Self::ACTION_CHANGE_USERNAME => Self::PERMISSION_GRANTED,
            Self::ACTION_CHANGE_PASSWORD => Self::PERMISSION_GRANTED,
            Self::ACTION_GET_GAME_INFO => Self::PERMISSION_GRANTED,
            Self::ACTION_DOWNLOAD_GAME => Self::PERMISSION_GRANTED,
            //UNKNOWN
            _ => Self::NO_PERMISSION_MESSAGE,
        }
    }
}

///Struct to handle the authentication for users, automatically makes a handshake to the database.
pub struct Authentication {
    username: String,
    token: String,
    id: u32,
    database_connection: Database,
}
impl Authentication {
    pub fn new(header: HeaderMap) -> Result<Self, io::Error> {
        //Getting Username
        let header_username: String = header
            .get("username")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("anonymous")
            .to_string();
        //Getting token
        let header_token: String = header
            .get("token")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_string();

        let database: Database;
        match Database::new() {
            Ok(value) => database = value,
            Err(err) => {
                return Result::Err(io::Error::new(err.kind(), "connection_to_database_failed"))
            }
        }

        Result::Ok(Authentication {
            username: header_username,
            token: header_token,
            id: 1,
            database_connection: database,
        })
    }

    ///Will check if the user is authenticated,
    ///anonymous users doesnt require token so the token part is ignored,
    ///logged users will ALWAYS check the token, if not the same will return a Permissions::INVALID_TOKEN
    pub fn authenticate(&self, action: u16) -> &str {
        //Check anonymous username
        if let "anonymous" = self.username.as_str() {
            //Check if a anonymous has permission for this action
            return Permissions::check_anoynmous_permission(action);
        }

        //Otherwise check if token is valid
        if !self.token_is_valid() {
            return Permissions::INVALID_TOKEN;
        }

        //Check if user has permission for the action
        Permissions::check_user_permission(action, self)
    }

    ///Returns true if token is valid
    fn token_is_valid(&self) -> bool {
        //Check if token is validated
        let result: Vec<HashMap<String, String>> = self.database_connection.select(
            vec![],
            "USERS",
            vec![
                format!("ID = {}", self.id).as_str(),
                format!("USERNAME = '{}'", self.username).as_str(),
            ],
            vec!["ID", "USERNAME", "PASSWORD", "TOKEN"],
        );
        //Check database error
        match self.database_connection.check_errors(&result) {
            Some(_) => return false,
            None => {}
        }
        //Check size
        if result.len() == 0 {
            return false;
        }
        
        //Get token
        let result_token: &str = match result[0].get("TOKEN") {
            Some(value) => value.as_str(),
            None => "Invalid Token",
        };
        //If is invalid
        if result_token == "Invalid Token" {
            false
        }
        //Is the same from the headers
        else if result_token == self.token.as_str() {
            true
        }
        //Not the same
        else {
            false
        }
    }
}
