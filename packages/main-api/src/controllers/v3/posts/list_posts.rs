use crate::models::feed::{Post, PostLike, PostQueryOption};
use crate::models::user::User;
use crate::types::list_items_response::ListItemsResponse;
use crate::types::{EntityType, Visibility};
use crate::{AppState, Error};
use aide::NoApi;
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Query, State},
};
use serde::Deserialize;
use validator::Validate;

use super::post_response::PostResponse;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Validate)]
pub struct ListPostsQueryParams {
    pub bookmark: Option<String>,
}

pub async fn list_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Query(ListPostsQueryParams { bookmark }): Query<ListPostsQueryParams>,
) -> Result<Json<ListItemsResponse<PostResponse>>, Error> {
    tracing::debug!(
        "list_posts_handler: user = {:?} bookmark = {:?}",
        user,
        bookmark
    );

    let mut query_options = PostQueryOption::builder().limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }
    let (posts, bookmark) =
        Post::find_by_visibility(&dynamo.client, Visibility::Public, query_options).await?;
    tracing::debug!(
        "list_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            let sk = EntityType::PostLike(user.pk.to_string());
            let likes = PostLike::batch_get(
                &dynamo.client,
                posts
                    .iter()
                    .map(|post| (post.pk.clone(), sk.clone()))
                    .collect(),
            )
            .await?;

            likes
        }
        _ => vec![],
    };

    tracing::debug!("list_posts_handler: returning {} items", posts.len());
    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| {
            let liked = likes.iter().any(|like| like.pk == post.pk);
            PostResponse::from((user.clone(), post)).with_like(liked)
        })
        .collect();

    Ok(Json(ListItemsResponse { items, bookmark }))
}
