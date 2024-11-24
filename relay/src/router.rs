use crate::controllers;
use axum::routing::post;
use axum::{routing::get, Router};
use tower_http::trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer};

pub fn app_router() -> Router {
    Router::new()
        .route("/", get(controllers::index::index_handler))
        .route("/up", get(controllers::up::index_handler))
        .route(
            "/handshake/v1/agent",
            post(controllers::handshake_v1::agent_handler),
        )
        .route(
            "/handshake/v1/frontend",
            post(controllers::handshake_v1::frontend_handler),
        )
        .route(
            // if we ever support WebTransport for talk/v1, it can go to
            // /talk/v1/wt/agent
            "/talk/v1/agent",
            get(controllers::talk_v1::agent_handler),
        )
        .route(
            "/talk/v1/frontend",
            get(controllers::talk_v1::frontend_handler),
        )
        .fallback(get(controllers::not_found::handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new())
                .on_request(DefaultOnRequest::new())
                .on_response(DefaultOnResponse::new()),
        )
}
