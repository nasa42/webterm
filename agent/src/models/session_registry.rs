use crate::models::agent_error::AgentError;
use crate::models::frontend::Frontend;
use crate::models::session::Session;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, OnceLock};
use tokio::sync::{Mutex, RwLock};
use tracing::debug;
use tracing_subscriber::registry;
use webterm_core::types::{FrontendId, SessionId};

// Start from 1. Frontend may send "0" session ID to request a new session
static NEXT_SESSION_ID: AtomicU64 = AtomicU64::new(1);

pub struct SessionRegistry {
    map: RwLock<HashMap<SessionId, Arc<Mutex<Session>>>>,
}

impl SessionRegistry {
    pub(crate) async fn singleton() -> &'static Self {
        static INSTANCE: OnceLock<SessionRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| Self {
            map: RwLock::new(HashMap::new()),
        })
    }

    pub async fn find(session_id: SessionId) -> Result<Arc<Mutex<Session>>, AgentError> {
        let registry = Self::singleton().await;
        registry
            .map
            .read()
            .await
            .get(&session_id)
            .ok_or(AgentError::SessionNotFound(Some(session_id)))
            .cloned()
    }

    pub async fn build_session() -> Result<Arc<Mutex<Session>>, AgentError> {
        let registry = Self::singleton().await;
        let session = Arc::new(Mutex::new(Session::new()));
        let session_ = session.lock().await;
        debug!("Registered session {:?}", session_.session_id());
        registry
            .map
            .write()
            .await
            .insert(session_.session_id(), session.clone());

        Ok(session.clone())
    }

    pub fn next_session_id() -> SessionId {
        SessionId(NEXT_SESSION_ID.fetch_add(1, Ordering::SeqCst))
    }
}
