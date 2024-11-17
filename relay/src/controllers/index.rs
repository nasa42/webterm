use askama_axum::Template;
use axum::response::IntoResponse;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate<'a> {
    browser_title: &'a str,
    project_url: &'a str,
}

pub async fn handler() -> impl IntoResponse {
    IndexTemplate {
        browser_title: "Web Terminal Relay",
        project_url: "https://github.com/nasa42/webterm",
    }
}
