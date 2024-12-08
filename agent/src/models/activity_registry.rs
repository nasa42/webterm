use crate::models::activity::Activity;
use crate::models::agent_error::AgentError;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::debug;
use webterm_core::types::ActivityId;

static NEXT_ACTIVITY_ID: AtomicU64 = AtomicU64::new(1);

pub struct ActivityRegistry {
    map: RwLock<HashMap<ActivityId, Arc<Activity>>>,
}

impl ActivityRegistry {
    pub(crate) async fn singleton() -> &'static Self {
        static INSTANCE: OnceLock<ActivityRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            map: RwLock::new(HashMap::new()),
        })
    }

    pub fn next_activity_id() -> ActivityId {
        ActivityId(NEXT_ACTIVITY_ID.fetch_add(1, Ordering::SeqCst))
    }

    pub async fn find(activity_id: ActivityId) -> Result<Arc<Activity>, AgentError> {
        let registry = Self::singleton().await;
        registry
            .map
            .read()
            .await
            .get(&activity_id)
            .ok_or(AgentError::ActivityNotFound(Some(activity_id)))
            .cloned()
    }

    pub async fn register(activity: Arc<Activity>) -> Result<(), AgentError> {
        let registry = Self::singleton().await;
        debug!("Registering activity {:?}", activity.activity_id());
        registry
            .map
            .write()
            .await
            .insert(activity.activity_id(), activity);

        Ok(())
    }
}
