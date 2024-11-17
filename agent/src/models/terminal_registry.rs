use crate::models::agent_error::AgentError;
use crate::models::terminal::Terminal;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tracing::debug;

pub struct TerminalRegistry {
    terminals: RwLock<HashMap<u64, Arc<Terminal>>>,
}

impl TerminalRegistry {
    pub(crate) async fn singleton() -> &'static TerminalRegistry {
        static INSTANCE: OnceLock<TerminalRegistry> = OnceLock::new();
        INSTANCE.get_or_init(|| TerminalRegistry {
            terminals: RwLock::new(HashMap::new()),
        })
    }

    pub async fn register_new(&self, frontend_id: u64) -> Result<Arc<Terminal>, AgentError> {
        let terminal = Terminal::new(frontend_id, "/bin/bash").await?;
        let terminal = Arc::new(terminal);
        self.terminals
            .write()
            .await
            .insert(frontend_id, terminal.clone());
        Ok(terminal)
    }

    pub async fn get_terminal(&self, id: u64) -> Option<Arc<Terminal>> {
        Some(self.terminals.read().await.get(&id)?.clone())
    }

    pub async fn remove_terminal(&self, id: u64) {
        self.terminals.write().await.remove(&id);
    }

    pub async fn write_to_pty(&self, frontend_id: u64, data: &[u8]) -> Result<(), AgentError> {
        if let Some(terminal) = self.get_terminal(frontend_id).await {
            terminal.write(data).await?;
        } else {
            debug!("terminal not found");
        }

        Ok(())
    }
}
