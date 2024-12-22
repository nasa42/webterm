use flatbuffers::InvalidFlatbuffer;
use std::fmt;
use std::sync::Arc;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::mpsc::error::SendError;
use webterm_core::models::reader_socket_error::ReaderSocketError;
use webterm_core::simple_cache::CacheError;

#[derive(Debug)]
pub enum RelayError {
    FBParseError(String),
    FlatbufferError(InvalidFlatbuffer),
    SocketSendError(SendError<Vec<u8>>),
    SocketRecvError(RecvError),
    SocketReadError(ReaderSocketError),
    AxumError(Arc<axum::Error>),
    AgentNotFound,
    CacheError(CacheError),
    IOError(std::io::Error),
    RuntimeError(String),
}

impl std::error::Error for RelayError {}

impl fmt::Display for RelayError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelayError::FBParseError(e) => write!(f, "Flatbuffer parse error: {}", e),
            RelayError::FlatbufferError(e) => write!(f, "Flatbuffer error: {}", e),
            RelayError::SocketSendError(e) => write!(f, "Socket send error: {}", e),
            RelayError::SocketRecvError(e) => write!(f, "Socket receive error: {}", e),
            RelayError::SocketReadError(e) => write!(f, "Socket read error: {}", e),
            RelayError::AxumError(e) => write!(f, "Axum error: {}", e),
            RelayError::AgentNotFound => write!(f, "Agent not found"),
            RelayError::CacheError(e) => write!(f, "Cache error: {}", e),
            RelayError::IOError(e) => write!(f, "IO error: {}", e),
            RelayError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
        }
    }
}

impl From<InvalidFlatbuffer> for RelayError {
    fn from(err: InvalidFlatbuffer) -> Self {
        RelayError::FlatbufferError(err)
    }
}

impl From<SendError<Vec<u8>>> for RelayError {
    fn from(err: SendError<Vec<u8>>) -> Self {
        RelayError::SocketSendError(err)
    }
}

impl From<RecvError> for RelayError {
    fn from(err: RecvError) -> Self {
        RelayError::SocketRecvError(err)
    }
}

impl From<Arc<axum::Error>> for RelayError {
    fn from(err: Arc<axum::Error>) -> Self {
        RelayError::AxumError(err)
    }
}

impl From<axum::Error> for RelayError {
    fn from(err: axum::Error) -> Self {
        RelayError::AxumError(Arc::new(err))
    }
}

impl From<std::io::Error> for RelayError {
    fn from(err: std::io::Error) -> Self {
        RelayError::IOError(err)
    }
}

impl From<CacheError> for RelayError {
    fn from(err: CacheError) -> Self {
        RelayError::CacheError(err)
    }
}

impl From<ReaderSocketError> for RelayError {
    fn from(err: ReaderSocketError) -> Self {
        RelayError::SocketReadError(err)
    }
}
