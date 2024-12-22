use crate::models::agent_error::AgentError;
use crate::models::frontend::Frontend;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, RwLock};
use tracing::debug;
use webterm_core::types::FrontendId;

// Frontend -> (has one) Session -> (has many) Activities
// Frontends are transient and are lost when the agent switches a relay.
// Sessions are persistent while the agent is running and a after switching to a new relay,
// the frontend may reconnect to its existing session using a new frontend_id.
pub struct FrontendRegistry {
    map: RwLock<HashMap<FrontendId, Arc<Mutex<Frontend>>>>,
}

impl FrontendRegistry {
    pub(crate) async fn singleton() -> &'static Self {
        static INSTANCE: OnceLock<FrontendRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            map: RwLock::new(HashMap::new()),
        })
    }

    pub async fn find(frontend_id: FrontendId) -> Result<Arc<Mutex<Frontend>>, AgentError> {
        let registry = Self::singleton().await;
        registry
            .map
            .read()
            .await
            .get(&frontend_id)
            .ok_or(AgentError::FrontendNotFound(Some(frontend_id)))
            .cloned()
    }

    pub async fn build_frontend(
        frontend_id: FrontendId,
    ) -> Result<Arc<Mutex<Frontend>>, AgentError> {
        let registry = Self::singleton().await;
        debug!("Registering frontend {:?}", frontend_id);
        let frontend = Arc::new(Mutex::new(Frontend::new(frontend_id)));
        registry
            .map
            .write()
            .await
            .insert(frontend_id, frontend.clone());

        Ok(frontend)
    }
}
