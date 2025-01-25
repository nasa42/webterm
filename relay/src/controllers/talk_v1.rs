use crate::models::bridge::Bridge;
use crate::models::handshake_nonce_agent_registry::HandshakeNonceAgentRegistry;
use axum::extract::ws::WebSocket;
use axum::extract::Query;
use axum::{extract::WebSocketUpgrade, response::IntoResponse};
use tracing::{error, info};

#[derive(serde::Deserialize)]
pub struct FrontendParams {
    handshake_nonce: String,
    device_subname: String,
}

#[derive(serde::Deserialize)]
pub struct AgentParams {
    handshake_nonce: String,
}

pub async fn frontend_handler(
    ws: WebSocketUpgrade,
    params: Query<FrontendParams>,
) -> impl IntoResponse {
    ws.write_buffer_size(crate::config::WEBSOCKET_BUFFER_SIZE)
        .max_write_buffer_size(crate::config::WEBSOCKET_MAX_BUFFER_SIZE)
        .on_upgrade(move |socket| on_upgrade_frontend(socket, params))
}

pub async fn agent_handler(ws: WebSocketUpgrade, params: Query<AgentParams>) -> impl IntoResponse {
    ws.write_buffer_size(crate::config::WEBSOCKET_BUFFER_SIZE)
        .max_write_buffer_size(crate::config::WEBSOCKET_MAX_BUFFER_SIZE)
        .on_upgrade(move |socket| on_upgrade_agent(socket, params.handshake_nonce.clone()))
}

async fn on_upgrade_frontend(socket: WebSocket, params: Query<FrontendParams>) {
    info!("Starting new WebSocket connection for frontend");

    let result = Bridge::connect_and_run(
        socket,
        params.handshake_nonce.clone(),
        params.device_subname.clone(),
    )
    .await;

    if let Err(error) = result {
        error!(
            "Frontend websocket connection closed with error: {:?}",
            error
        );
    } else {
        info!("Frontend websocket connection closed");
    }
}

async fn on_upgrade_agent(socket: WebSocket, handshake_nonce: String) {
    info!("Starting new WebSocket connection for agent");

    let result = HandshakeNonceAgentRegistry::singleton()
        .await
        .consume_nonce(handshake_nonce, socket)
        .await;

    match result {
        Ok(connection) => {
            connection.wait_until_closed().await;
        }
        Err(error) => {
            error!("Agent websocket connection closed with error: {:?}", error);
        }
    }
}
