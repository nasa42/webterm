use crate::models::agent_registry::AgentRegistry;
use crate::models::socket_connection::SocketConnection;
use axum::extract::ws::WebSocket;
use std::sync::atomic::{AtomicU64, Ordering};
use tracing::error;
use webterm_core::models::device_id::DeviceId;
use webterm_core::types::FrontendId;

pub struct AgentConnection {
    device_id: DeviceId,
    socket_connection: SocketConnection,
    next_frontend_id: AtomicU64,
}

impl AgentConnection {
    pub async fn new(device_id: DeviceId, socket: WebSocket) -> Self {
        let conn = SocketConnection::new(socket);
        Self {
            device_id,
            socket_connection: conn,
            next_frontend_id: AtomicU64::new(1),
        }
    }

    pub fn socket(&self) -> &SocketConnection {
        &self.socket_connection
    }

    pub fn device_id(&self) -> &DeviceId {
        &self.device_id
    }

    pub async fn wait_until_closed(&self) {
        self.socket_connection.close_notifier().notified().await;
        error!("STARTING THE REMOVAL");
        if let Err(e) = AgentRegistry::remove(self.device_id.clone()).await {
            error!("Failed to remove agent from registry: {:?}", e);
        }
        let _ = self.socket_connection.writer().close().await;
    }

    pub fn next_frontend_id(&self) -> FrontendId {
        FrontendId(self.next_frontend_id.fetch_add(1, Ordering::SeqCst))
    }
}
