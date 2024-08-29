use std::fmt;

pub enum HttpConnectionStatus {
    Failed,
    Success,
}
impl fmt::Display for HttpConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HttpConnectionStatus::Failed => write!(f, "Failed"),
            HttpConnectionStatus::Success => write!(f, "Success"),
        }
    }
}