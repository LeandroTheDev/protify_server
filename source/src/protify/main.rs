#![allow(dead_code)]

// Context Libs
use crate::libs::http::response::HttpMethod;
use crate::libs::http::response::HttpResponse;

// Rust Libs
use std::fmt;

use super::pages;

pub struct Protify {}
impl Protify {
    pub fn new() -> Protify {
        Self {}
    }

    pub fn receive_request(&self, response: HttpResponse) {
        // Method Selector
        match response.method {
            HttpMethod::GET => Self::get_requests(response),
            HttpMethod::POST => Self::post_requests(response),
            HttpMethod::PATCH => Self::patch_requests(response),
            _ => Self::error(response, ProtifyError::NotFound),
        }
    }

    fn get_requests(response: HttpResponse) {
        match response.url.as_str() {
            "/store_showcase" => pages::store::Instance::showcase(response),
            _ => Self::error(response, ProtifyError::NotFound),
        }
    }

    fn post_requests(response: HttpResponse) {}

    fn patch_requests(response: HttpResponse) {}

    pub fn error(response: HttpResponse, error: ProtifyError) {}
}

pub enum ProtifyError {
    NotFound,
    InvalidParameter,
    InternalError,
    DatabaseError,
}
impl fmt::Display for ProtifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ProtifyError::NotFound => write!(f, "NOT_FOUND"),
            ProtifyError::InvalidParameter => write!(f, "INVALID_PARAMETER"),
            ProtifyError::InternalError => write!(f, "INTERNAL_ERROR"),
            ProtifyError::DatabaseError => write!(f, "DATABASE_ERROR"),
        }
    }
}
