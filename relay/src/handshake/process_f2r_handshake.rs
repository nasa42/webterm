use crate::models::agent_registry::AgentRegistry;
use crate::models::handshake_nonce_registry::HandshakeNonceRegistry;
use tracing::error;
use tracing::{debug, info};
use webterm_core::generated::flatbuffers_schema::handshake_v1::F2rHandshake;
use webterm_core::handshake_v1_helpers::create_r2f_handshake;

pub async fn process_f2r_handshake(message: F2rHandshake<'_>) -> Vec<u8> {
    let req_device_name = message.device_name();

    match req_device_name {
        None => {
            error!("No device_name in F2rHandshake");
            r2f_success_false_message()
        }
        Some(device_name) => {
            debug!("Processing F2rHandshake for device_name: {}", device_name);
            let agent = AgentRegistry::find(device_name).await;
            debug!("finished finding agent");
            match agent {
                Err(_) => {
                    error!("Failed to find agent");
                    r2f_success_false_message()
                }
                Ok(agent) => {
                    info!("Found agent: {}", agent.device_name);
                    let auth_nonce = HandshakeNonceRegistry::singleton_frontend()
                        .await
                        .create_nonce(agent.device_name.clone())
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
