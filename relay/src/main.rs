mod config;
mod controllers;
mod models;

use axum::response::IntoResponse;
use axum::{
    routing::{any, get},
    Router, ServiceExt,
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_target(true)
        .with_thread_names(true)
        .init();

    let app = Router::new()
        .route("/", get(controllers::index::handler))
        .route("/up", get(controllers::up::handler))
        .route("/ws", any(controllers::ws::handler))
        .fallback(get(controllers::not_found::handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new()),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind to port 3000");
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
