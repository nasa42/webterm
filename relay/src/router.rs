use crate::controllers;
use axum::routing::post;
use axum::{routing::get, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(controllers::index::handler))
        .route("/up", get(controllers::up::handler))
        .route("/ws/agent", get(controllers::ws::agent_handler))
        .route("/ws/frontend", get(controllers::ws::frontend_handler))
        .route(
            "/handshake/agent",
            post(controllers::handshake::agent_handler),
        )
        .route(
            "/handshake/frontend",
            post(controllers::handshake::frontend_handler),
        )
        .fallback(get(controllers::not_found::handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new()),
        )
}
