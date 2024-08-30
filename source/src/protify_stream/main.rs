#![allow(dead_code)]

// Context Libs
use crate::libs::stream::response::StreamResponse;
use super::download::ItemDownload;

// Rust Libs
use std::{collections::HashMap, fmt};

pub static mut CLIENTS_DOWNLOADS: Option<HashMap<String, ItemDownload>> = None;

pub struct ProtifyStream {}
impl ProtifyStream {
    pub fn new() -> ProtifyStream {
        Self {}
    }

    pub fn error(response: StreamResponse, error: ProtifyError) {}

    pub fn receive_stream(&self, response: StreamResponse) {
        
    }
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
