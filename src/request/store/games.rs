use std::convert::Infallible;

use http_body_util::Full;
use hyper::{body::Bytes, HeaderMap, Response};

pub fn request_main_page_game_list(header: HeaderMap) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("{:?}", header);
    Ok(Response::new(Full::new(Bytes::from("Success"))))
}
pub fn download_game(header: HeaderMap, body: String) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("{:?}", header);
    println!("{:?}", body);
    Ok(Response::new(Full::new(Bytes::from("Success"))))
}
