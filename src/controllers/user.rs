use askama::Template;
use axum::{extract, response::IntoResponse};

use crate::templating::HtmlTemplate;

#[derive(Template)]
#[template(path = "user/index.html")]
struct IndexTemplate {}
pub(crate) async fn index() -> impl IntoResponse {
    HtmlTemplate(IndexTemplate {})
}

#[derive(Template)]
#[template(path = "user/show.html")]
struct ShowTemplate {
    name: String,
}
pub(crate) async fn show(extract::Path(name): extract::Path<String>) -> impl IntoResponse {
    HtmlTemplate(ShowTemplate { name })
}
