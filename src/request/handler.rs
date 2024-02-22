use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::Bytes, HeaderMap, Method, Response, StatusCode};
use serde_json::{json, Value};

///Default Responses for client messages
pub struct DefaultResponse {
    response: Response<Full<Bytes>>,
    status_code: StatusCode,
}
impl DefaultResponse {
    ///Create the response
    pub fn new(response_body: String, response_status_code: StatusCode) -> Self {
        DefaultResponse {
            response: Response::new(Full::new(Bytes::from(response_body))),
            status_code: response_status_code,
        }
    }
    ///Build the response and return the Response with the bytes and parameters
    pub fn build_response(&mut self) -> Response<Full<Bytes>> {
        //Getting the status code
        let status_code = self.response.status_mut();
        //Changing the status code to 401 (not found)
        *status_code = self.status_code;
        self.response.clone()
    }
}
///Struct for errors with the request
pub struct ErrorStruct {}
impl ErrorStruct {
    pub fn not_found() -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the response
        let json_body: Value = json!({
            "message": "Not Found: cannot find the specific URL"
        });
        let mut response: DefaultResponse =
            DefaultResponse::new(json_body.to_string(), StatusCode::NOT_FOUND);
        Ok(response.build_response())
    }
    pub fn size_limit() -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the response
        let json_body = json!({
            "message": "Size Limit"
        });
        let mut response: DefaultResponse =
            DefaultResponse::new(json_body.to_string(), StatusCode::NOT_ACCEPTABLE);
        Ok(response.build_response())
    }
    pub fn internal_server_error(reason: String) -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the response
        let json_body = json!({
            "message": reason
        });
        let mut response: DefaultResponse =
            DefaultResponse::new(json_body.to_string(), StatusCode::INTERNAL_SERVER_ERROR);
        Ok(response.build_response())
    }
    pub fn authentication_required() -> Result<Response<Full<Bytes>>, Infallible> {
        //Creating the response
        let json_body = json!({
            "message": "Anonymous dont have acess to this action"
        });
        let mut response: DefaultResponse = DefaultResponse::new(
            json_body.to_string(),
            StatusCode::NETWORK_AUTHENTICATION_REQUIRED,
        );
        Ok(response.build_response())
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
                "/store_main" => super::store::games::store_main(self.header.clone()),
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
