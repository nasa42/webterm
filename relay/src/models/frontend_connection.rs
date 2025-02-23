use crate::models::socket_connection::SocketConnection;
use axum::extract::ws::WebSocket;

pub struct FrontendConnection {
    socket_connection: SocketConnection,
}

impl FrontendConnection {
    pub async fn new(socket: WebSocket) -> Self {
        let conn = SocketConnection::new(socket);
        Self {
            socket_connection: conn,
        }
    }

    pub fn socket(&self) -> &SocketConnection {
        &self.socket_connection
    }

    pub async fn close(&self) {
        let _ = self.socket().writer().close().await;
    }
}
