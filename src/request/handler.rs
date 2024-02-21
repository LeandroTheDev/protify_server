use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::Bytes, HeaderMap, Method, Response, StatusCode};

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
    pub fn size_limit() -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the response
        let mut response = Response::new(Full::new(Bytes::from("Size Limit")));
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
    method: Method,
    header: HeaderMap,
    body: String,
}
impl RequestHandler {
    ///Creates a RequestHandler controller, needs the url send by the client
    pub fn new(
        request_url: String,
        request_method: Method,
        request_header: HeaderMap,
        request_body: String,
    ) -> Self {
        RequestHandler {
            url: request_url,
            method: request_method,
            header: request_header,
            body: request_body,
        }
    }
    ///Handles the request based in url created on new function
    pub async fn handle_request(&self) -> Result<Response<Full<Bytes>>, Infallible> {
        //Handling the url
        let url_string: &str = &self.url;
        match self.method {
            //GET
            Method::GET => match url_string {
                "/store_main_list" => {
                    super::store::games::request_main_page_game_list(self.header.clone())
                }
                "/download_game" => {
                    super::store::games::download_game(self.header.clone(), self.body.clone())
                }
                "/limit_overflow" => ErrorStruct::size_limit(),
                //Not found request
                _ => ErrorStruct::not_found(),
            },
            //POST
            Method::POST => match url_string {
                "/limit_overflow" => ErrorStruct::size_limit(),
                //Not found request
                _ => ErrorStruct::not_found(),
            },
            //DELETE
            Method::DELETE => match url_string {
                "/limit_overflow" => ErrorStruct::size_limit(),
                //Not found request
                _ => ErrorStruct::not_found(),
            },
            //METHOD NOT FOUND
            _ => ErrorStruct::not_found(),
        }
    }
}
