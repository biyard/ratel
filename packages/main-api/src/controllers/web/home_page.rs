use askama::Template;
use axum::extract::*;
use axum::response::{Html, IntoResponse};
use bdk::prelude::*;

pub async fn home_page_handler(
    State(_cli): State<aws_sdk_dynamodb::Client>,
) -> Result<impl IntoResponse, crate::Error2> {
    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {
        title: String,
    }

    let template = Tmpl {
        title: "Ratel Foundation",
    };

    Ok(Html(template.render()?))
}
