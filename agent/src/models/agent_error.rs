use flatbuffers::InvalidFlatbuffer;
use std::fmt;
use tokio::sync::broadcast::error::RecvError;
use tokio::sync::mpsc::error::SendError;
use tokio_tungstenite::tungstenite;
use tokio_tungstenite::tungstenite::Bytes;
use webterm_core::models::reader_socket_error::ReaderSocketError;
use webterm_core::models::webterm_error::WebtermError;
use webterm_core::types::{ActivityId, FrontendId, SessionId};

#[derive(Debug)]
pub enum AgentError {
    RuntimeError(String),
    FlatbuffersError(InvalidFlatbuffer),
    FBParseError(String),
    SocketError(tungstenite::Error),
    SocketClosed,
    SocketSendError(SendError<Bytes>),
    SocketRecvError(RecvError),
    SocketReadError(ReaderSocketError),
    PtyProcessError(pty_process::Error),
    IOError(std::io::Error),
    URLParseError(url::ParseError),
    CoreError(WebtermError),
    FrontendNotFound(Option<FrontendId>),
    SessionNotFound(Option<SessionId>),
    ActivityNotFound(Option<ActivityId>),
    ReqwestError(reqwest::Error),
    RelayErrorAgentAlreadyExists,
}

impl std::error::Error for AgentError {}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
            AgentError::FlatbuffersError(e) => write!(f, "Flatbuffers error: {}", e),
            AgentError::FBParseError(e) => write!(f, "Flatbuffers parse error: {}", e),
            AgentError::SocketError(e) => write!(f, "Socket error: {}", e),
            AgentError::SocketClosed => write!(f, "Socket closed"),
            AgentError::SocketSendError(e) => write!(f, "Socket send error: {}", e),
            AgentError::SocketRecvError(e) => write!(f, "Socket receive error: {}", e),
            AgentError::SocketReadError(e) => write!(f, "Socket read error: {}", e),
            AgentError::PtyProcessError(e) => write!(f, "Pty process error: {}", e),
            AgentError::IOError(e) => write!(f, "IO error: {}", e),
            AgentError::URLParseError(e) => write!(f, "URL parse error: {}", e),
            AgentError::CoreError(e) => write!(f, "CoreError: {}", e),
            AgentError::FrontendNotFound(e) => write!(f, "Frontend not found: {:?}", e),
            AgentError::SessionNotFound(e) => write!(f, "Session not found: {:?}", e),
            AgentError::ActivityNotFound(e) => write!(f, "Activity not found: {:?}", e),
            AgentError::ReqwestError(e) => write!(f, "Reqwest error: {}", e),
            AgentError::RelayErrorAgentAlreadyExists => write!(f, "Agent already exists"),
        }
    }
}

impl From<InvalidFlatbuffer> for AgentError {
    fn from(err: InvalidFlatbuffer) -> Self {
        AgentError::FlatbuffersError(err)
    }
}

impl From<SendError<Bytes>> for AgentError {
    fn from(err: SendError<Bytes>) -> Self {
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

impl From<WebtermError> for AgentError {
    fn from(err: WebtermError) -> Self {
        AgentError::CoreError(err)
    }
}

impl From<reqwest::Error> for AgentError {
    fn from(err: reqwest::Error) -> Self {
        AgentError::ReqwestError(err)
    }
}
