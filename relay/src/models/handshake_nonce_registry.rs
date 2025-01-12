use crate::config::{HANDSHAKE_MAX_NONCES, HANDSHAKE_NONCE_EXPIRE_IN};
use crate::models::agent_connection::AgentConnection;
use crate::models::agent_registry::AgentRegistry;
use crate::models::relay_error::RelayError;
use std::sync::{Arc, OnceLock};
use webterm_core::random::random_string;
use webterm_core::simple_cache::SimpleCache;

pub struct HandshakeNonceRegistry {
    map: SimpleCache<String, String>,
}

impl HandshakeNonceRegistry {
    // (agent, frontend)
    async fn singleton() -> &'static (Arc<HandshakeNonceRegistry>, Arc<HandshakeNonceRegistry>) {
        static INSTANCE: OnceLock<(Arc<HandshakeNonceRegistry>, Arc<HandshakeNonceRegistry>)> =
            OnceLock::new();
        INSTANCE.get_or_init(|| {
            (
                Arc::new(Self {
                    map: SimpleCache::new(HANDSHAKE_MAX_NONCES),
                }),
                Arc::new(Self {
                    map: SimpleCache::new(HANDSHAKE_MAX_NONCES),
                }),
            )
        })
    }

    #[allow(dead_code)]
    pub async fn singleton_agent() -> &'static Arc<HandshakeNonceRegistry> {
        let (agent, _) = Self::singleton().await;
        agent
    }

    pub async fn singleton_frontend() -> &'static Arc<HandshakeNonceRegistry> {
        let (_, frontend) = Self::singleton().await;
        frontend
    }

    pub async fn create_nonce(&self, device_name: String) -> Result<String, RelayError> {
        let nonce = random_string(64);

        self.map
            .insert(nonce.clone(), device_name, HANDSHAKE_NONCE_EXPIRE_IN)
            .await?;

        Ok(nonce)
    }

    pub async fn consume_nonce(&self, nonce: &str) -> Result<Arc<AgentConnection>, RelayError> {
        let device_name = self.map.remove(&nonce.to_string()).await?;
        let agent_connection = AgentRegistry::find(&device_name).await?;

        Ok(agent_connection)
    }
}
