use std::fmt;

#[derive(Debug, Clone)]
pub enum ReaderSocketError {
    SocketClosed,
}

impl std::error::Error for ReaderSocketError {}

impl fmt::Display for ReaderSocketError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ReaderSocketError::SocketClosed => write!(f, "Socket closed"),
        }
    }
}
