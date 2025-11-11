use crate::models::feed::{Post, PostLike, PostQueryOption};
use crate::models::user::User;
use crate::types::list_items_response::ListItemsResponse;
use crate::types::{EntityType, Visibility};
use crate::*;

use super::post_response::PostResponse;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Validate)]
pub struct ListPostsQueryParams {
    pub bookmark: Option<String>,
}

pub async fn list_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Query(ListPostsQueryParams { bookmark }): Query<ListPostsQueryParams>,
) -> Result<Json<ListItemsResponse<PostResponse>>> {
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
            let likes = PostLike::batch_get(
                &dynamo.client,
                posts
                    .iter()
                    .map(|post| PostLike::keys(&post.pk, &user.pk))
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
            let post_like_pk = post
                .pk
                .clone()
                .to_post_like_key()
                .expect("to_post_like_key");
            let liked = likes.iter().any(|like| like.pk == post_like_pk);
            PostResponse::from((user.clone(), post)).with_like(liked)
        })
        .collect();

    Ok(Json(ListItemsResponse { items, bookmark }))
}
