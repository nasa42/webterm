use crate::config::MAX_AGENTS;
use crate::models::agent_connection::AgentConnection;
use crate::models::relay_error::RelayError;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::{debug, error};
use webterm_core::models::device_id::DeviceId;

pub struct AgentRegistry {
    agents: Arc<RwLock<HashMap<DeviceId, Arc<AgentConnection>>>>,
    // Map of all device subname for a given device name
    devices: Arc<RwLock<HashMap<String, HashSet<String>>>>,
}

impl AgentRegistry {
    async fn singleton() -> &'static Arc<AgentRegistry> {
        static INSTANCE: OnceLock<Arc<AgentRegistry>> = OnceLock::new();
        INSTANCE.get_or_init(|| {
            Arc::new(AgentRegistry {
                agents: Arc::new(RwLock::new(HashMap::new())),
                devices: Arc::new(RwLock::new(HashMap::new())),
            })
        })
    }

    pub async fn exists(device_id: &DeviceId) -> bool {
        let registry = Self::singleton().await;
        registry.agents.read().await.contains_key(device_id)
    }

    pub async fn find(device_id: &DeviceId) -> Result<Arc<AgentConnection>, RelayError> {
        debug!("finding agent for {:?}", device_id);
        let registry = Self::singleton().await;
        debug!("registry acquired");
        Ok(registry
            .agents
            .read()
            .await
            .get(device_id)
            .ok_or(RelayError::AgentNotFound)?
            .clone())
    }

    pub async fn subnames(device_name: &str) -> Vec<String> {
        let registry = Self::singleton().await;
        registry
            .devices
            .read()
            .await
            .get(device_name)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_default()
    }

    pub async fn register(agent: Arc<AgentConnection>) -> Result<(), RelayError> {
        let registry = Self::singleton().await;
        if registry.agents.read().await.len() >= MAX_AGENTS {
            return Err(RelayError::RuntimeError(
                "Agent registry is full".to_string(),
            ));
        }

        let device_id = agent.device_id();
        debug!("Registering agent {:?}", device_id);

        let mut devices = registry.devices.write().await;

        let subname_set = devices
            .entry(device_id.name().to_string())
            .or_insert_with(HashSet::new);

        if !subname_set.insert(device_id.subname().to_string()) {
            return Err(RelayError::RuntimeError(
                "Agent already registered".to_string(),
            ));
        }

        registry
            .agents
            .write()
            .await
            .insert(device_id.clone(), agent);

        Ok(())
    }

    pub async fn remove(device_id: DeviceId) -> Result<Arc<AgentConnection>, RelayError> {
        let registry = Self::singleton().await;
        let mut agents = registry.agents.write().await;
        let mut devices = registry.devices.write().await;

        let result = agents.remove(&device_id).ok_or(RelayError::AgentNotFound);

        if let Some(subnames) = devices.get_mut(device_id.name()) {
            subnames.remove(device_id.subname());

            if subnames.is_empty() {
                devices.remove(device_id.name());
            }
        }

        result
    }
}
