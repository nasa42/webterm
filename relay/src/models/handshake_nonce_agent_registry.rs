use crate::config::{HANDSHAKE_MAX_NONCES, HANDSHAKE_NONCE_EXPIRE_IN};
use crate::models::agent_connection::AgentConnection;
use crate::models::agent_registry::AgentRegistry;
use crate::models::relay_error::RelayError;
use axum::extract::ws::WebSocket;
use std::sync::{Arc, OnceLock};
use webterm_core::models::device_id::DeviceId;
use webterm_core::random::random_alphanumeric;
use webterm_core::simple_cache::SimpleCache;

pub struct HandshakeNonceAgentRegistry {
    map: SimpleCache<String, DeviceId>,
}

impl HandshakeNonceAgentRegistry {
    pub async fn singleton() -> &'static Arc<HandshakeNonceAgentRegistry> {
        static INSTANCE: OnceLock<Arc<HandshakeNonceAgentRegistry>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Arc::new(Self {
                map: SimpleCache::new(HANDSHAKE_MAX_NONCES),
            })
        })
    }

    pub async fn create_nonce(&self, device_id: DeviceId) -> Result<String, RelayError> {
        let nonce = random_alphanumeric(64);

        self.map
            .insert(nonce.clone(), device_id, HANDSHAKE_NONCE_EXPIRE_IN)
            .await?;

        Ok(nonce)
    }

    pub async fn consume_nonce(
        &self,
        nonce: String,
        socket: WebSocket,
    ) -> Result<Arc<AgentConnection>, RelayError> {
        let device_id = self.map.remove(&nonce).await?;
        let connection = Arc::new(AgentConnection::new(device_id, socket).await);
        AgentRegistry::register(connection.clone()).await?;

        Ok(connection)
    }
}
