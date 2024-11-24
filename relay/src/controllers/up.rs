use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response {
    status: String,
}

#[axum::debug_handler]
pub async fn index_handler() -> Json<Response> {
    Json(Response {
        status: "ok".to_string(),
    })
}
