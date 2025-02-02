use crate::handshake::process_a2r_handshake::process_a2r_handshake;
use crate::handshake::process_f2r_handshake::process_f2r_handshake;
use crate::relay_version::relay_version_to_flatbuffers;
use axum::body::Bytes;
use axum::http::{header, StatusCode};
use axum::response::{IntoResponse, Response};
use webterm_core::flatbuffers_helpers::read_message;
use webterm_core::generated::flatbuffers_schema::handshake_v1::{
    A2rHandshakeRoot, F2rHandshakeRoot,
};

#[axum::debug_handler]
pub async fn frontend_handler(body: Bytes) -> Result<Response, StatusCode> {
    let message = read_message::<F2rHandshakeRoot>(&body).map_err(|_| StatusCode::BAD_REQUEST)?;
    let builder = process_f2r_handshake(message).await;

    build_octet_response(builder.to_flatbuffers(relay_version_to_flatbuffers()).0)
}

#[axum::debug_handler]
pub async fn agent_handler(body: Bytes) -> impl IntoResponse {
    let message = read_message::<A2rHandshakeRoot>(&body).map_err(|_| StatusCode::BAD_REQUEST)?;
    let builder = process_a2r_handshake(message).await;

    build_octet_response(builder.to_flatbuffers(relay_version_to_flatbuffers()).0)
}

fn build_octet_response(bytes: Vec<u8>) -> Result<Response, StatusCode> {
    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(bytes.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
