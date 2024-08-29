#![allow(dead_code)]

use std::{
    collections::HashMap,
    io::{self, ErrorKind},
};

use mysql::{prelude::Queryable, *};
use serde_json::{from_str, json};

///Connect to differents tables with this struct
///
///Considerations:
///
///To connect to the database simply use the Database::new(),
///if everthing is ok then you will receive the Ok()
///otherwise the Err will be returned indicating the connection failed,
///with the Database Struct you can use the select function to call a select query to receive
///the datas from database
pub struct Database {
    connected_database: Pool,
}
impl Database {
    pub const DATABASE_NAME: &'static str = "protify_server";
    pub const DATABASE_USERNAME: &'static str = "admin";
    pub const DATABASE_PASSWORD: &'static str = "secretpassword";
    pub const DATABASE_IP: &'static str = "127.0.0.1";
    pub const DATABASE_PORTS: u16 = 3306;

    ///Create a new instance of database, will return Err if cannot connect to database
    pub fn new() -> Result<Database, io::Error> {
        //Try to instanciate
        let database_helper: Option<Pool> =
            Self::instanciate_database(String::from(Self::DATABASE_NAME));

        let database: Pool;
        //Check if database connected successfully
        match database_helper {
            Some(value) => database = value,
            None => {
                let reason = io::Error::new(io::ErrorKind::ConnectionRefused, "Connection Failed");
                Self::log_error(&reason, "None");
                return Result::Err(reason);
            }
        }
        //Automatically create table if not exist
        match database.get_conn() {
            Ok(mut connection) => {
                //Users
                {
                    connection
                        .query_drop(
                            r"CREATE TABLE IF NOT EXISTS USERS
                (ID MEDIUMINT, USERNAME VARCHAR(255), PASSWORD VARCHAR(50), TOKEN VARCHAR(255))",
                        )
                        .unwrap();
                }
                //Store Register
                {
                    connection
                        .query_drop(
                            r"CREATE TABLE IF NOT EXISTS STORE
                (ID MEDIUMINT, NAME VARCHAR(255), CATEGORY TINYTEXT, LANGUAGES TINYTEXT, DESCRIPTION LONGTEXT)",
                        )
                        .unwrap();
                }
                //Showcase
                {
                    connection
                        .query_drop(
                            r"CREATE TABLE IF NOT EXISTS SHOWCASE
                (ID MEDIUMINT)",
                        )
                        .unwrap();
                }
            }
            _ => {}
        };

        Result::Ok(Database {
            connected_database: database,
        })
    }

    ///Query the first result from the table,
    ///params explanation:
    ///
    ///select_params: keys to receive from the query, if empty will use * to select all
    ///
    ///from_param: a single string indicating the table to query
    ///
    ///where_params: values necessary to query
    ///
    ///waiting_params: the parameters keys name to get in keys from hashmap,
    ///must be the same as you want in the selection, otherwise it will be incosistent.
    ///
    ///In case of errors will return any HashMap with the value "error_message"
    pub fn select(
        &self,
        select_params: Vec<&str>,
        from_param: &str,
        where_params: Vec<&str>,
        waiting_params: Vec<&str>,
    ) -> Vec<HashMap<String, String>> {
        let mut hash_response: Vec<HashMap<String, String>> = vec![];
        //Get a connection to the database
        let mut connection: PooledConn = match self.connected_database.get_conn() {
            Ok(conn) => conn,
            Err(_) => {
                Self::log_error(
                    &io::Error::new(io::ErrorKind::ConnectionRefused, "Connection Refused"),
                    "None",
                );
                let mut hash: HashMap<String, String> = HashMap::new();
                hash.insert(
                    String::from("error_message"),
                    String::from("Cannot connect to the Database, connection refused"),
                );
                hash_response.insert(0, hash);
                return hash_response;
            }
        };

        //Query uild
        let mut query_text: String = String::from("SELECT ");
        //Select build
        {
            //Select All
            if select_params.len() == 0 {
                query_text += "*";
            }
            //Swipe Selects
            else {
                let mut lenght: i32 = 0;
                for select in &select_params {
                    if lenght == 0 {
                        query_text += select;
                    } else {
                        query_text += ", ";
                        query_text += select;
                    }
                    lenght += 1;
                }
            }
        }
        //From build
        query_text += format!(" FROM {}", from_param).as_str();
        //Where build
        {
            let mut lenght: i32 = 0;
            //Unfortunaly we need the undescore because in rust theres a type where :/
            for _where in &where_params {
                if where_params.len() == 0 {
                    break;
                }
                if lenght == 0 {
                    query_text += " WHERE ";
                    query_text += _where;
                } else {
                    query_text += " AND ";
                    query_text += _where;
                }
                lenght += 1;
            }
        }

        //Query to database
        let query: Vec<Vec<String>> = match connection
            .query_map::<_, _, _, _>(query_text.clone(), |row| Self::row_to_strings(row))
        {
            Ok(query_result) => query_result,
            Err(_) => {
                Self::log_error(
                    &io::Error::new(
                        ErrorKind::ConnectionAborted,
                        "The Server lost connection to the Database",
                    ),
                    query_text.clone().as_str(),
                );
                let mut hash: HashMap<String, String> = HashMap::new();
                hash.insert(
                    String::from("error_message"),
                    String::from("Connection aborted during the query"),
                );
                hash_response.push(hash);
                return hash_response;
            }
        };
        //Swiping the query values
        let mut response_length: usize = 0;
        for result in query {
            let mut hash: HashMap<String, String> = HashMap::new();
            //Get the values and add to the hash
            let mut wait_length: usize = 0;
            for line in result {
                //Getting the waiting parameter based in query values, in case of wait_length overflow just break it
                let waiting_param: String = match waiting_params.get(wait_length) {
                    Some(value) => value.to_string(),
                    None => break,
                };
                wait_length += 1;
                //Add it to the hash_response
                hash.insert(waiting_param, line);
            }
            hash_response.insert(response_length, hash);
            response_length += 1;
        }

        //Returning the hash with the values
        hash_response
    }

    ///Convert a HashMap<String, String> to a compatible
    ///version to json with the correct primite values
    pub fn convert_hash_map_to_json_value(
        hashmap: HashMap<String, String>,
    ) -> HashMap<String, serde_json::Value> {
        let mut converted_hash: HashMap<String, serde_json::Value> = HashMap::new();
        //Swipe the the response and get key and value
        for (key, value) in hashmap.clone().iter_mut() {
            //Default
            converted_hash.insert(key.to_string(), json!(value));

            //Try to convert to int
            match value.as_str().parse::<i32>() {
                //Success Converting
                Ok(value_parsed) => {
                    converted_hash.insert(
                        key.to_string(),
                        serde_json::Value::Number(serde_json::Number::from(value_parsed)),
                    );
                    continue;
                }
                Err(_) => {}
            };
            //Try to convert to bool
            match value.as_str().parse::<bool>() {
                //Success Converting
                Ok(value_parsed) => {
                    converted_hash.insert(key.to_string(), json!(value_parsed));
                    continue;
                }
                Err(_) => {}
            };
            //Try to convert to List
            match from_str::<serde_json::Value>(value) {
                //Success transforming in json string
                Ok(value_parsed) => {
                    //Success converting in json string to list
                    if let serde_json::Value::Array(vec) = value_parsed {
                        converted_hash.insert(key.to_string(), json!(vec));
                    }
                }
                Err(_) => {}
            }
        }
        converted_hash
    }

    ///Create a pool to the database returns None if cannot handshake to database
    fn instanciate_database(database: String) -> Option<Pool> {
        // Database Connection
        let address: String = format!(
            "mysql://{}:{}@{}:{}/{}",
            Self::DATABASE_USERNAME,
            Self::DATABASE_PASSWORD,
            Self::DATABASE_IP,
            Self::DATABASE_PORTS,
            database,
        );

        // Connect to the database
        match Pool::new(address.as_str()) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }

    ///Returns the error if exist, if not returns a empty string
    pub fn check_errors(&self, response: &[HashMap<String, String>]) -> Option<String> {
        match response
            .iter()
            .find(|map: &&HashMap<String, String>| map.contains_key("error_message"))
        {
            Some(map) => match map.get("error_message") {
                Some(err) => Some(String::from(err)),
                None => Some(String::from("error_message_is_empty")),
            },
            None => None,
        }
    }

    ///Log Errors
    fn log_error(reason: &io::Error, query: &str) {
        eprintln!(
            r"-------------------
[Database] Panic:
query: {:?}
type: {:?}
error: {:?}
-------------------",
            query,
            reason.kind(),
            reason.to_string()
        );
    }

    ///Convert the row bytes to string
    fn row_to_strings(row: mysql::Row) -> Vec<String> {
        fn value_to_string(value: &Value) -> String {
            // Converte o valor binário (Bytes) em uma string UTF-8
            match value {
                Value::Bytes(bytes) => String::from_utf8_lossy(bytes).to_string(),
                _ => String::new(), // Trate outros tipos conforme necessário
            }
        }
        row.unwrap()
            .iter()
            .map(|value| value_to_string(value))
            .collect()
    }
}
