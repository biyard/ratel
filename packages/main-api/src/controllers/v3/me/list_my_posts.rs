use crate::types::*;
use crate::{
    AppState,
    controllers::v3::posts::post_response::PostResponse,
    models::{
        feed::{Post, PostQueryOption},
        user::User,
    },
    types::{PostStatus, list_items_query::ListItemsQuery, list_items_response::ListItemsResponse},
};
use aide::NoApi;
use axum::extract::Query;
use axum::extract::State;
use axum::*;
use bdk::prelude::*;
use futures::future::try_join_all;

pub async fn list_my_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Query(Pagination { bookmark }): ListItemsQuery,
) -> Result<Json<ListItemsResponse<PostResponse>>, crate::Error> {
    tracing::debug!("Handling request: {:?}", bookmark);

    let mut opt = PostQueryOption::builder().sk(PostStatus::Published.to_string());

    if let Some(bookmark) = bookmark {
        opt = opt.bookmark(bookmark);
    }

    let (posts, next_bookmark) =
        Post::find_by_user_and_status(&dynamo.client, &user.pk, opt).await?;

    let cli = dynamo.client.clone();
    let current_user = user.clone();

    let response_items: Vec<PostResponse> = try_join_all(posts.into_iter().map(|post| {
        let cli = cli.clone();
        let current_user = current_user.clone();
        async move {
            let liked = post.is_liked(&cli, &current_user.pk).await?;
            let resp = PostResponse::from((Some(current_user), post)).with_like(liked);
            Ok::<PostResponse, crate::Error>(resp)
        }
    }))
    .await?;

    Ok(Json(ListItemsResponse {
        items: response_items,
        bookmark: next_bookmark,
    }))
}
