use askama::Template;
use axum::http;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

// inspired from https://github.com/rinja-rs/askama/blob/0.12.1/askama_axum/src/lib.rs
// askama_axum crate is deprecated, see https://github.com/rinja-rs/askama/issues/1035#issuecomment-2585948493
pub fn render_response<T: Template>(status_code: StatusCode, template: &T) -> Response {
    match template.render() {
        Ok(body) => {
            let headers = [(
                http::header::CONTENT_TYPE,
                http::HeaderValue::from_static(T::MIME_TYPE),
            )];

            (status_code, headers, body).into_response()
        }
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
