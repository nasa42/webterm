use crate::models::activity::Activity;
use crate::models::agent_error::AgentError;
use crate::models::frontend::Frontend;
use crate::models::session_registry::SessionRegistry;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use webterm_core::types::{ActivityId, SessionId};

pub struct Session {
    session_id: SessionId,
    activities: RwLock<HashMap<ActivityId, Arc<Activity>>>,
    current_frontend: Option<Arc<Mutex<Frontend>>>,
}

impl Session {
    pub fn new() -> Self {
        let session_id = SessionRegistry::next_session_id();

        Self {
            session_id,
            activities: RwLock::new(HashMap::new()),
            current_frontend: None,
        }
    }

    pub fn session_id(&self) -> SessionId {
        self.session_id
    }

    pub fn set_current_frontend(&mut self, frontend: Arc<Mutex<Frontend>>) {
        self.current_frontend = Some(frontend);
    }

    pub fn current_frontend(&self) -> Result<Arc<Mutex<Frontend>>, AgentError> {
        Ok(self
            .current_frontend
            .as_ref()
            .ok_or(AgentError::RuntimeError(
                "Current frontend is not set".to_string(),
            ))?
            .clone())
    }

    pub async fn get_activity(
        &self,
        activity_id: &ActivityId,
    ) -> Result<Arc<Activity>, AgentError> {
        let activity = self
            .activities
            .read()
            .await
            .get(activity_id)
            .ok_or(AgentError::RuntimeError(format!(
                "Activity {:?} not found",
                activity_id
            )))?
            .clone();

        Ok(activity)
    }

    pub async fn create_terminal_activity(&self) -> Result<Arc<Activity>, AgentError> {
        let activity = Activity::create_pty(self.session_id).await?;
        self.add_activity(activity.clone()).await;
        Ok(activity)
    }

    pub async fn add_activity(&self, activity: Arc<Activity>) {
        self.activities
            .write()
            .await
            .insert(activity.activity_id(), activity.clone());
    }
}
