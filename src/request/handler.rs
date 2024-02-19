use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};

///Struct for errors with the request
struct ErrorStruct {}
impl ErrorStruct {
    pub fn not_found() -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the response
        let mut response = Response::new(Full::new(Bytes::from("Not Found")));
        //Getting the status code
        let status_code = response.status_mut();
        //Changing the status code to 401 (not found)
        *status_code = StatusCode::UNAUTHORIZED;
        //Finish
        Ok(response)
    }
}

///Handle the request from client
pub struct RequestHandler {
    url: String,
}
impl RequestHandler {
    ///Creates a RequestHandler controller, needs the url send by the client
    pub fn new(client_url: String) -> Self {
        RequestHandler {
            url: client_url,
        }
    }
    ///Handles the request based in url created on new function
    pub async fn handle_request(&self) -> Result<Response<Full<Bytes>>, Infallible> {
        let url_string: &str = &self.url;
        match url_string {
            "/store_main_list" => super::store::games::request_main_page_game_list(),
            //Not found request
            _ => ErrorStruct::not_found(),
        }
    }
}
