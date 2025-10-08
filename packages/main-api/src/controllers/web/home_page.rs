use aide::NoApi;
use askama::Template;
use axum::extract::*;
use axum::response::{Html, IntoResponse};
use bdk::prelude::*;

use crate::AppState;
use crate::controllers::v3::me::get_info::GetInfoResponse;
use crate::controllers::v3::posts::list_posts::{ListPostsQueryParams, list_posts_handler};
use crate::controllers::v3::promotions;
use crate::models::user::{User, UserMetadata};
use crate::types::{BootData, InitialQuery};

pub async fn home_page_handler(
    State(app_state): State<AppState>,
    user: Option<User>,
) -> Result<impl IntoResponse, crate::Error2> {
    let index_js = option_env!("WEB_INDEX_JS").unwrap_or("index.js");
    let index_css = option_env!("WEB_INDEX_CSS").unwrap_or("index.css");

    let user_info = if let Some(ref user) = user {
        let user = UserMetadata::query(&app_state.dynamo.client, &user.pk).await?;
        let user_info: GetInfoResponse = user.into();
        Some(user_info)
    } else {
        None
    };

    let posts = list_posts_handler(
        State(app_state.clone()),
        NoApi(user),
        Query(ListPostsQueryParams { bookmark: None }),
    );

    let top_promotion = promotions::get_top_promotion::get_top_promotion_handler();

    // resolve all async
    let (Json(posts), Json(top_promotion)) = tokio::try_join!(posts, top_promotion)?;

    let query_results = vec![
        InitialQuery::new_infinite_list(
            serde_json::json!(["feeds","list",{"status":2}]),
            &posts,
            posts.bookmark.clone(),
        )?,
        InitialQuery::new(["ratel-top-promotion"], top_promotion)?,
        InitialQuery::new(serde_json::json!(["user-get-info"]), user_info)?,
    ];

    let boot = BootData::new(query_results);

    #[derive(Debug, Template)]
    #[template(path = "index.html")]
    struct Tmpl {
        title: String,
        index_js: &'static str,
        index_css: &'static str,
        boot_json: String,
    }

    let template = Tmpl {
        title: "Ratel Foundation".to_string(),
        index_js,
        index_css,
        boot_json: boot.to_json()?,
    };

    Ok(Html(template.render()?))
}
