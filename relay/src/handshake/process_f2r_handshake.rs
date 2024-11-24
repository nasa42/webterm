use crate::models::agent_registry::AgentRegistry;
use crate::models::handshake_nonce_registry::HandshakeNonceRegistry;
use futures::future::err;
use tracing::error;
use tracing::{debug, info};
use webterm_shared::generated::flatbuffers_schema::handshake_v1::F2rHandshake;
use webterm_shared::handshake_v1_helpers::create_r2f_handshake;
use webterm_shared::random::random_string;

pub async fn process_f2r_handshake(message: F2rHandshake<'_>) -> Vec<u8> {
    let req_server_id = message.server_id();

    match req_server_id {
        None => {
            error!("No server_id in F2rHandshake");
            r2f_success_false_message()
        }
        Some(server_id) => {
            debug!("Processing F2rHandshake for server_id: {}", server_id);
            let agent = AgentRegistry::find(&server_id).await;
            debug!("finished finding agent");
            match agent {
                Err(_) => {
                    error!("Failed to find agent");
                    r2f_success_false_message()
                }
                Ok(agent) => {
                    info!("Found agent: {}", agent.server_id);
                    let auth_nonce = HandshakeNonceRegistry::singleton_frontend()
                        .await
                        .create_nonce(agent.server_id.clone())
                        .await;

                    match auth_nonce {
                        Err(_) => {
                            error!("Failed to create auth_nonce");
                            r2f_success_false_message()
                        }
                        Ok(auth_nonce) => {
                            debug!("Created auth_nonce: {}", auth_nonce);
                            create_r2f_handshake(true, Some(&auth_nonce))
                        }
                    }
                }
            }
        }
    }
}

fn r2f_success_false_message() -> Vec<u8> {
    create_r2f_handshake(false, None)
}
