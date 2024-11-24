use std::fmt;

// Errors which are ok to crash the agent
#[derive(Debug)]
pub enum PanicError {
    RelayParseError(String),
}

impl std::error::Error for PanicError {}

impl fmt::Display for PanicError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanicError::RelayParseError(e) => write!(f, "Relay parse error: {}", e),
        }
    }
}
