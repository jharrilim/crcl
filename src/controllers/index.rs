use askama::Template;

use axum::response::IntoResponse;

use crate::templating::HtmlTemplate;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {}
pub(crate) async fn index() -> impl IntoResponse {
    let template = IndexTemplate {};
    HtmlTemplate(template)
}
