use crate::config::{HANDSHAKE_MAX_NONCES, HANDSHAKE_NONCE_EXPIRE_IN};
use crate::models::agent_connection::AgentConnection;
use crate::models::agent_registry::AgentRegistry;
use crate::models::relay_error::RelayError;
use std::sync::{Arc, OnceLock};
use webterm_core::models::device_id::DeviceId;
use webterm_core::random::random_alphanumeric;
use webterm_core::simple_cache::SimpleCache;

pub struct HandshakeNonceFrontendRegistry {
    // tuple of (device-name, device-subname)
    map: SimpleCache<String, (String, Option<String>)>,
}

impl HandshakeNonceFrontendRegistry {
    pub async fn singleton() -> &'static Arc<HandshakeNonceFrontendRegistry> {
        static INSTANCE: OnceLock<Arc<HandshakeNonceFrontendRegistry>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Arc::new(Self {
                map: SimpleCache::new(HANDSHAKE_MAX_NONCES),
            })
        })
    }

    pub async fn create_nonce(
        &self,
        device_name: String,
        device_subname: Option<String>,
    ) -> Result<String, RelayError> {
        let nonce = random_alphanumeric(64);

        self.map
            .insert(
                nonce.clone(),
                (device_name, device_subname),
                HANDSHAKE_NONCE_EXPIRE_IN,
            )
            .await?;

        Ok(nonce)
    }

    pub async fn consume_nonce(
        &self,
        nonce: String,
        device_subname: String,
    ) -> Result<Arc<AgentConnection>, RelayError> {
        let (device_name, nonce_device_subname) = self.map.remove(&nonce).await?;
        if nonce_device_subname.is_some() && nonce_device_subname != Some(device_subname.clone()) {
            return Err(RelayError::RuntimeError(
                "Nonce is registered for a different subname".to_string(),
            ));
        }

        let device_id = DeviceId::new(device_name, device_subname);

        let agent_connection = AgentRegistry::find(&device_id).await?;

        Ok(agent_connection)
    }
}
