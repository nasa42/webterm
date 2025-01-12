use crate::config::MAX_AGENTS;
use crate::models::agent_connection::AgentConnection;
use crate::models::relay_error::RelayError;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::debug;

pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<String, Arc<AgentConnection>>>>,
}

impl AgentRegistry {
    async fn singleton() -> &'static Arc<AgentRegistry> {
        static INSTANCE: OnceLock<Arc<AgentRegistry>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Arc::new(AgentRegistry {
                agents: Arc::new(RwLock::new(HashMap::new())),
            })
        })
    }

    pub async fn find(device_name: &str) -> Result<Arc<AgentConnection>, RelayError> {
        debug!("finding agent {}", device_name);
        let registry = Self::singleton().await;
        debug!("registry acquired");
        Ok(registry
            .agents
            .read()
            .await
            .get(&device_name.to_string())
            .ok_or(RelayError::AgentNotFound)?
            .clone())
    }

    pub async fn register(agent: Arc<AgentConnection>) -> Result<(), RelayError> {
        let registry = Self::singleton().await;
        if registry.agents.read().await.len() >= MAX_AGENTS {
            return Err(RelayError::RuntimeError(
                "Agent registry is full".to_string(),
            ));
        }
        debug!("Registering agent {}", agent.device_name);
        registry
            .agents
            .write()
            .await
            .insert(agent.device_name.clone(), agent);

        Ok(())
    }

    #[allow(dead_code)]
    pub async fn remove(device_name: &str) -> Result<Arc<AgentConnection>, RelayError> {
        let registry = Self::singleton().await;
        registry
            .agents
            .write()
            .await
            .remove(&device_name.to_string())
            .ok_or(RelayError::AgentNotFound)
    }
}
