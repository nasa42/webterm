use crate::config::{TEST_HANDSHAKE_NONCE, TEST_SERVER_ID};
use crate::models::agent_connection::AgentConnection;
use crate::models::agent_registry::AgentRegistry;
use crate::models::frontend_connection::FrontendConnection;
use crate::models::handshake_nonce_registry::HandshakeNonceRegistry;
use crate::models::session::Session;
use axum::extract::ws::WebSocket;
use axum::extract::Query;
use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

#[derive(serde::Deserialize)]
pub struct Params {
    handshake_nonce: String,
}

pub async fn frontend_handler(ws: WebSocketUpgrade, params: Query<Params>) -> impl IntoResponse {
    let handshake_nonce = params.handshake_nonce.clone();

    ws.write_buffer_size(crate::config::WEBSOCKET_BUFFER_SIZE)
        .max_write_buffer_size(crate::config::WEBSOCKET_MAX_BUFFER_SIZE)
        .on_upgrade(|socket| on_upgrade_frontend(socket, handshake_nonce))
}

pub async fn agent_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.write_buffer_size(crate::config::WEBSOCKET_BUFFER_SIZE)
        .max_write_buffer_size(crate::config::WEBSOCKET_MAX_BUFFER_SIZE)
        .on_upgrade(|socket| on_upgrade_agent(socket))
}

async fn on_upgrade_frontend(socket: WebSocket, handshake_nonce: String) {
    info!("Starting new WebSocket connection for frontend");

    HandshakeNonceRegistry::singleton_frontend()
        .await
        .register(TEST_HANDSHAKE_NONCE.to_string(), TEST_SERVER_ID.to_string())
        .await
        .expect("Failed to register nonce");

    let frontend_connection = FrontendConnection::new(socket).await;
    let session = Session::new();
    let result = session.run_loop(frontend_connection, handshake_nonce).await;

    if let Err(error) = result {
        error!(
            "Frontend websocket connection closed with error: {:?}",
            error
        );
    } else {
        info!("Frontend websocket connection closed");
    }
}

async fn on_upgrade_agent(socket: WebSocket) {
    info!("Starting new WebSocket connection for agent");
    let connection = Arc::new(AgentConnection::new(socket).await);
    AgentRegistry::register(TEST_SERVER_ID.to_string(), connection.clone())
        .await
        .expect("Failed to register agent");

    connection.wait_until_closed().await;
}
