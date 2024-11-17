use flatbuffers::InvalidFlatbuffer;
use std::fmt;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::mpsc::error::SendError;
use tokio_tungstenite::tungstenite;
use webterm_shared::models::reader_socket_error::ReaderSocketError;

#[derive(Debug)]
pub enum AgentError {
    RuntimeError(String),
    FlatbuffersError(InvalidFlatbuffer),
    SocketError(tungstenite::Error),
    SocketClosed,
    SocketSendError(SendError<Vec<u8>>),
    SocketRecvError(RecvError),
    SocketReadError(ReaderSocketError),
    PtyProcessError(pty_process::Error),
    IOError(std::io::Error),
    URLParseError(url::ParseError),
}

impl std::error::Error for AgentError {}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
            AgentError::FlatbuffersError(e) => write!(f, "Flatbuffers error: {}", e),
            AgentError::SocketError(e) => write!(f, "Socket error: {}", e),
            AgentError::SocketClosed => write!(f, "Socket closed"),
            AgentError::SocketSendError(e) => write!(f, "Socket send error: {}", e),
            AgentError::SocketRecvError(e) => write!(f, "Socket receive error: {}", e),
            AgentError::SocketReadError(e) => write!(f, "Socket read error: {}", e),
            AgentError::PtyProcessError(e) => write!(f, "Pty process error: {}", e),
            AgentError::IOError(e) => write!(f, "IO error: {}", e),
            AgentError::URLParseError(e) => write!(f, "URL parse error: {}", e),
        }
    }
}

impl From<InvalidFlatbuffer> for AgentError {
    fn from(err: InvalidFlatbuffer) -> Self {
        AgentError::FlatbuffersError(err)
    }
}

impl From<SendError<Vec<u8>>> for AgentError {
    fn from(err: SendError<Vec<u8>>) -> Self {
        AgentError::SocketSendError(err)
    }
}

impl From<RecvError> for AgentError {
    fn from(err: RecvError) -> Self {
        AgentError::SocketRecvError(err)
    }
}

impl From<ReaderSocketError> for AgentError {
    fn from(err: ReaderSocketError) -> Self {
        AgentError::SocketReadError(err)
    }
}

impl From<tungstenite::Error> for AgentError {
    fn from(err: tungstenite::Error) -> Self {
        AgentError::SocketError(err)
    }
}

impl From<pty_process::Error> for AgentError {
    fn from(err: pty_process::Error) -> Self {
        AgentError::PtyProcessError(err)
    }
}

impl From<std::io::Error> for AgentError {
    fn from(err: std::io::Error) -> Self {
        AgentError::IOError(err)
    }
}

impl From<url::ParseError> for AgentError {
    fn from(err: url::ParseError) -> Self {
        AgentError::URLParseError(err)
    }
}
