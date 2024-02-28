//Protify Dependencies
use crate::request::handler::DefaultResponse;

//Rust Dependencies
use std::convert::Infallible;

//Plugins Dependencies
use http_body_util::Full;
use hyper::{body::Bytes, Response, StatusCode};
use serde_json::{json, Value};

pub struct User {}
impl User {
    pub fn change_username(
        body: String,
    ) -> Result<Response<Full<Bytes>>, Infallible> {
        println!("{:?}", body);
        let json_body: Value = json!({
            "message": "SUCCESS"
        });
        let mut response: DefaultResponse =
            DefaultResponse::new(json_body.to_string(), StatusCode::OK);
        Ok(response.build_response())
    }
}
