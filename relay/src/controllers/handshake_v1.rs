use crate::handshake::process_f2r_handshake::process_f2r_handshake;
use askama_axum::IntoResponse;
use axum::body::Bytes;
use axum::http::{header, StatusCode};
use axum::response::Response;
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::handshake_v1::F2rHandshake;

#[axum::debug_handler]
pub async fn frontend_handler(body: Bytes) -> Result<Response, StatusCode> {
    let message = read_message::<F2rHandshake>(&body).map_err(|_| StatusCode::BAD_REQUEST)?;
    let response_bytes = process_f2r_handshake(message).await;

    build_octet_response(response_bytes)
}

#[axum::debug_handler]
pub async fn agent_handler(_body: Bytes) -> impl IntoResponse {
    // purpose
    // ask relay if a server_id exists
    // ask frontend to perform proof of work
    // create an "auth_nonce" which can be used to establish a websocket connection
    ""
}

fn build_octet_response(bytes: Vec<u8>) -> Result<Response, StatusCode> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(bytes.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
