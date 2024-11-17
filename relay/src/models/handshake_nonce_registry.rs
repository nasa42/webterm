use crate::config::{HANDSHAKE_MAX_NONCES, HANDSHAKE_NONCE_EXPIRE_IN};
use crate::models::agent_connection::AgentConnection;
use crate::models::agent_registry::AgentRegistry;
use crate::models::relay_error::RelayError;
use std::sync::{Arc, OnceLock};
use webterm_shared::simple_cache::SimpleCache;

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

    pub async fn singleton_agent() -> &'static Arc<HandshakeNonceRegistry> {
        let (agent, _) = Self::singleton().await;
        agent
    }

    pub async fn singleton_frontend() -> &'static Arc<HandshakeNonceRegistry> {
        let (_, frontend) = Self::singleton().await;
        frontend
    }

    pub async fn register(&self, nonce: String, server_id: String) -> Result<(), RelayError> {
        self.map
            .insert(nonce, server_id, HANDSHAKE_NONCE_EXPIRE_IN)
            .await?;

        Ok(())
    }

    pub async fn consume_nonce(&self, nonce: &str) -> Result<Arc<AgentConnection>, RelayError> {
        let server_id = self.map.remove(&nonce.to_string()).await?;
        let agent_connection = AgentRegistry::find(&server_id).await?;
        Ok(agent_connection)
    }
}
