use askama::Template;
use axum::extract::*;
use axum::response::{Html, IntoResponse};
use bdk::prelude::*;

pub async fn home_page_handler(
    State(_cli): State<aws_sdk_dynamodb::Client>,
) -> Result<impl IntoResponse, crate::Error2> {
    let index_js = option_env!("WEB_INDEX_JS").unwrap_or("index.js");
    let index_css = option_env!("WEB_INDEX_CSS").unwrap_or("index.css");

    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {
        title: String,
        index_js: &'static str,
        index_css: &'static str,
    }

    let template = Tmpl {
        title: "Ratel Foundation".to_string(),
        index_js,
        index_css,
    };

    Ok(Html(template.render()?))
}
