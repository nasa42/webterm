use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{extract, routing::post, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: String,
}

#[axum::debug_handler]
pub async fn handler() -> Json<Response> {
    Json(Response {
        status: "ok".to_string(),
    })
}
