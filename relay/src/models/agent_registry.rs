use crate::config::{AGENT_EXPIRE_IN, MAX_AGENTS};
use crate::models::agent_connection::AgentConnection;
use crate::models::relay_error::RelayError;
use std::sync::{Arc, OnceLock};
use tracing::debug;
use webterm_shared::simple_cache::SimpleCache;

pub struct AgentRegistry {
    agents: SimpleCache<String, Arc<AgentConnection>>,
}

impl AgentRegistry {
    async fn singleton() -> &'static Arc<AgentRegistry> {
        static INSTANCE: OnceLock<Arc<AgentRegistry>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Arc::new(AgentRegistry {
                agents: SimpleCache::new(MAX_AGENTS),
            })
        })
    }

    pub async fn find(server_id: &str) -> Result<Arc<AgentConnection>, RelayError> {
        debug!("finding agent {}", server_id);
        let registry = Self::singleton().await;
        debug!("registry acquired");
        match registry.agents.get(&server_id.to_string()).await {
            Err(_) => Err(RelayError::AgentNotFound),
            Ok(agent) => Ok(agent.clone()),
        }
    }

    pub async fn register(agent: Arc<AgentConnection>) -> Result<(), RelayError> {
        let registry = Self::singleton().await;
        registry
            .agents
            .insert(agent.server_id.clone(), agent, AGENT_EXPIRE_IN)
            .await?;

        Ok(())
    }
}
