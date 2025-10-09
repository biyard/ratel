use aide::NoApi;
use askama::Template;
use axum::extract::*;
use axum::response::{Html, IntoResponse};
use bdk::prelude::*;

use crate::AppState;
use crate::controllers::v3::me::get_info::GetInfoResponse;
use crate::controllers::v3::networks::get_suggestions::get_suggestions_handler;
use crate::controllers::v3::posts::list_posts::{ListPostsQueryParams, list_posts_handler};
use crate::controllers::v3::promotions;
use crate::models::user::{User, UserMetadata};
use crate::types::{BootData, IndexTmpl, InitialQuery};

pub async fn home_page_handler(
    State(app_state): State<AppState>,
    tmpl: IndexTmpl,
    user: Option<User>,
) -> Result<impl IntoResponse, crate::Error2> {
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

    let suggestions = get_suggestions_handler(State(app_state.clone()));

    let top_promotion = promotions::get_top_promotion::get_top_promotion_handler();

    // resolve all async
    let (Json(posts), Json(top_promotion), Json(suggestions)) =
        tokio::try_join!(posts, top_promotion, suggestions)?;

    let query_results = vec![
        InitialQuery::new_infinite_list(
            serde_json::json!(["feeds","list",{"status":2}]),
            &posts,
            posts.bookmark.clone(),
        )?,
        InitialQuery::new(["ratel-top-promotion"], top_promotion)?,
        InitialQuery::new(serde_json::json!(["user-get-info"]), user_info)?,
        InitialQuery::new(serde_json::json!(["get-networks"]), suggestions)?,
    ];

    let boot = BootData::new(query_results);

    let template = tmpl
        .with_boot_json(boot.to_json()?)
        .with_description("The first platform connecting South Korea’s citizens with lawmakers to drive institutional reform for the crypto industry.Are you with us ?")
        .with_image_url("https://metadata.ratel.foundation/logos/logo-symbol.png");

    Ok(Html(template.render()?))
}
