use std::fmt;

pub enum StreamConnectionStatus {
    Failed,
    Success,
}
impl fmt::Display for StreamConnectionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StreamConnectionStatus::Failed => write!(f, "Failed"),
            StreamConnectionStatus::Success => write!(f, "Success"),
        }
    }
}