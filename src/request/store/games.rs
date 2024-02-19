use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::Bytes, Response};

pub fn request_main_page_game_list() -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
