use crate::models::frontend_connection::FrontendConnection;
use axum::{extract::WebSocketUpgrade, response::IntoResponse};

pub async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.write_buffer_size(crate::config::WEBSOCKET_BUFFER_SIZE)
        .max_write_buffer_size(crate::config::WEBSOCKET_MAX_BUFFER_SIZE)
        .on_upgrade(|socket| async move {
            let connection = FrontendConnection::new(socket).await;
            connection.handle_connection().await;
        })
}
