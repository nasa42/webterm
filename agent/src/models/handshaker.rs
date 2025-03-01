use crate::agent_version::agent_version_to_flatbuffers;
use crate::config::Config;
use crate::messaging::process_r2a_handshake::process_r2a_handshake;
use crate::models::agent_error::AgentError;
use crate::models::relay::Relay;
use reqwest::Client;
use std::sync::Arc;
use tracing::info;
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::handshake_v1::R2aHandshakeRoot;
use webterm_core::serialisers::handshake_v1::a2r_handshake_builder::A2rHandshakeBuilder;

pub struct Handshaker {
    pub result_relay: Arc<Relay>,
    pub result_nonce: String,
}

impl Handshaker {
    pub async fn run(config: Arc<Config>) -> Result<Self, AgentError> {
        let relay = config.random_relay();
        let mut subname = config.original_device_subname().to_string();
        loop {
            let result = initiate(config.clone(), relay.clone(), subname).await;

            match result {
                Ok((relay, nonce)) => {
                    return Ok(Self {
                        result_nonce: nonce,
                        result_relay: relay,
                    });
                }
                Err(AgentError::RelayErrorAgentAlreadyExists) => {
                    subname = config.try_new_device_subname()
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }
}

async fn initiate(
    config: Arc<Config>,
    relay: Arc<Relay>,
    subname: String,
) -> Result<(Arc<Relay>, String), AgentError> {
    info!(
        "Starting handshake: {} with device name: {}, subname: {}",
        relay.host(),
        config.device_name(),
        subname
    );
    let payload = A2rHandshakeBuilder::new()
        .root_request_connection(config.device_name(), &subname)
        .to_flatbuffers(agent_version_to_flatbuffers());
    let url = relay.handshake_url();

    let client = Client::new();
    let response = client.post(url).body(payload.0).send().await?;

    let body = response.bytes().await?;
    let message = read_message::<R2aHandshakeRoot>(&body)?;
    let output = process_r2a_handshake(message).await?;

    Ok((relay, output.auth_nonce))
}
