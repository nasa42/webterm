use crate::models::agent_error::AgentError;
use crate::models::terminal::Terminal;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use webterm_shared::types::{ActivityId, SessionId};

static NEXT_ACTIVITY_ID: AtomicU64 = AtomicU64::new(1);

// in future, manage more activities like a "file browser"
pub struct ActivityRegistry {
    // sessions may come and go as relay may come and go but activities are long-lived and can
    // be reattached to a new session, so a user can reconnect to a new relay and resume on the
    // same terminal from a new session
    // in future, a single session should be able to manage multiple activities
    sessions_to_activities: RwLock<HashMap<SessionId, ActivityId>>,
    activities_to_sessions: RwLock<HashMap<ActivityId, SessionId>>,
    terminals: RwLock<HashMap<ActivityId, Arc<Terminal>>>,
}

impl ActivityRegistry {
    pub(crate) async fn singleton() -> &'static ActivityRegistry {
        static INSTANCE: OnceLock<ActivityRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| ActivityRegistry {
            sessions_to_activities: RwLock::new(HashMap::new()),
            activities_to_sessions: RwLock::new(HashMap::new()),
            terminals: RwLock::new(HashMap::new()),
        })
    }

    pub async fn register_new(&self, session_id: SessionId) -> Result<Arc<Terminal>, AgentError> {
        let activity_id = Self::next_activity_id();
        let terminal = Terminal::new(activity_id, "/bin/bash").await?;
        let terminal = Arc::new(terminal);
        self.terminals
            .write()
            .await
            .insert(activity_id, terminal.clone());
        self.sessions_to_activities
            .write()
            .await
            .insert(session_id, activity_id);
        self.activities_to_sessions
            .write()
            .await
            .insert(activity_id, session_id);
        Ok(terminal)
    }

    pub fn next_activity_id() -> ActivityId {
        NEXT_ACTIVITY_ID.fetch_add(1, Ordering::SeqCst)
    }

    pub async fn activity_for_session(&self, session_id: SessionId) -> Option<ActivityId> {
        self.sessions_to_activities
            .read()
            .await
            .get(&session_id)
            .cloned()
    }

    pub async fn session_for_activity(&self, activity_id: ActivityId) -> Option<SessionId> {
        self.activities_to_sessions
            .read()
            .await
            .get(&activity_id)
            .cloned()
    }

    pub async fn get_terminal_for_session(&self, session_id: SessionId) -> Option<Arc<Terminal>> {
        let activity_id = self.activity_for_session(session_id).await?;
        self.get_terminal(activity_id).await
    }

    pub async fn get_terminal(&self, activity_id: ActivityId) -> Option<Arc<Terminal>> {
        Some(self.terminals.read().await.get(&activity_id)?.clone())
    }

    pub async fn remove_terminal(&self, activity_id: ActivityId) {
        self.terminals.write().await.remove(&activity_id);
    }
}
