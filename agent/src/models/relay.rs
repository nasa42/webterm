use crate::config::Config;
use crate::models::agent_error::AgentError;
use tracing::{debug, info};
use url::Url;

pub struct Relay {
    host: String,
    use_http: bool,
}

impl Relay {
    pub fn new(config: &Config, host: &str) -> Result<Self, AgentError> {
        info!("Creating relay with host: {}", host);

        let mut host_with_scheme = host.to_string();
        if !host.contains("://") {
            host_with_scheme = format!("https://{}", host);
        }

        debug!("Parsed relay URL: {}", host_with_scheme);
        let parsed_url = Url::parse(&host_with_scheme)?;
        let use_http = match parsed_url.scheme() {
            "http" => true,
            "https" => false,
            _ => {
                return Err(AgentError::RuntimeError(format!(
                    "Invalid relay URL scheme: {}",
                    parsed_url.scheme()
                )))
            }
        };

        let host = parsed_url
            .host_str()
            .ok_or(AgentError::RuntimeError(format!(
                "Couldn't extract host from relay URL: {}",
                host
            )))?
            .to_string();

        let host = match parsed_url.port() {
            Some(port) => format!("{}:{}", host, port),
            None => host,
        };

        Ok(Self { host, use_http })
    }

    pub fn websocket_url(&self) -> String {
        let scheme = if self.use_http { "ws" } else { "wss" };
        format!("{}://{}/ws/agent", scheme, self.host)
    }

    pub fn auth_url(&self) -> String {
        let scheme = if self.use_http { "http" } else { "https" };
        format!("{}://{}/auth/agent", scheme, self.host)
    }
}
