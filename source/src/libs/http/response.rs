use std::{
    collections::HashMap,
    fmt,
    net::{IpAddr, TcpStream},
};

use super::status::HttpConnectionStatus;

pub struct HttpResponse {
    pub address: IpAddr,
    pub port: u16,
    pub method: HttpMethod,
    pub url: String,
    pub version: String,
    pub agent: String,
    pub enconding: String,
    pub body: String,
    pub stream: TcpStream,
    pub status: HttpConnectionStatus,
    pub query: HashMap<String, String>,

    pub status_code: u16,
    pub status_message: String,
}

impl HttpResponse {
    pub fn new(
        _address: IpAddr,
        _port: u16,
        _method: &str,
        _url: &str,
        _version: &str,
        _agent: &str,
        _enconding: &str,
        _body: String,
        _stream: TcpStream,
        _status: HttpConnectionStatus,
        _query: HashMap<String, String>,
    ) -> HttpResponse {
        Self {
            address: _address,
            port: _port,
            method: Self::parse_http_method(_method),
            url: _url.to_string(),
            version: _version.to_string(),
            agent: _agent.to_string(),
            enconding: _enconding.to_string(),
            body: _body,
            stream: _stream,
            status: _status,
            query: _query,

            // Default values
            status_code: 200,
            status_message: String::from("OK"),
        }
    }

    /// Parsing the string to the HttpMethod
    pub fn parse_http_method(method: &str) -> HttpMethod {
        match method {
            "GET" => HttpMethod::GET,
            "POST" => HttpMethod::POST,
            "DELETE" => HttpMethod::DELETE,
            "PATCH" => HttpMethod::PATCH,
            "UPDATE" => HttpMethod::UPDATE,
            _ => HttpMethod::UNKOWN,
        }
    }

    /// Generates the response body to send to the client
    pub fn generate_response(&self, body: String) -> String {
        format!(
            "HTTP/1.1 {} {}\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code,
            self.status_message,
            body.len(),
            body
        )
    }
}

pub enum HttpMethod {
    GET,
    POST,
    DELETE,
    PATCH,
    UPDATE,
    UNKOWN,
}
impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HttpMethod::GET => write!(f, "GET"),
            HttpMethod::POST => write!(f, "POST"),
            HttpMethod::DELETE => write!(f, "DELETE"),
            HttpMethod::PATCH => write!(f, "PATCH"),
            HttpMethod::UPDATE => write!(f, "UPDATE"),
            HttpMethod::UNKOWN => write!(f, "UNKOWN"),
        }
    }
}
