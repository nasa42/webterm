use crate::relay_version::{relay_version, RELAY_GIT_VERSION};
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: String,
    version: String,
    commit: String,
}

#[axum::debug_handler]
pub async fn index_handler() -> Json<Response> {
    Json(Response {
        status: "ok".to_string(),
        version: relay_version().to_string(),
        commit: RELAY_GIT_VERSION.to_string(),
    })
}
