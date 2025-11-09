use crate::models::feed::{Post, PostLike, PostQueryOption};
use crate::models::user::User;
use crate::types::list_items_response::ListItemsResponse;
use crate::types::{EntityType, Partition, PostStatus, Visibility};
use crate::{AppState, Error};
use aide::NoApi;
use bdk::prelude::*;
use by_axum::axum::{
    Json,
    extract::{Path, Query, State},
};
use serde::Deserialize;
use validator::Validate;

use crate::controllers::v3::posts::post_response::PostResponse;

#[derive(Debug, Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Validate)]
pub struct ListTeamPostsQueryParams {
    pub bookmark: Option<String>,
    /// Filter posts by status (draft, published, etc.)
    pub status: Option<i64>,
}

pub async fn list_team_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(team_pk): Path<String>,
    Query(ListTeamPostsQueryParams { bookmark, status }): Query<ListTeamPostsQueryParams>,
) -> Result<Json<ListItemsResponse<PostResponse>>, Error> {
    tracing::debug!(
        "list_team_posts_handler: user = {:?}, team_pk = {:?}, bookmark = {:?}, status = {:?}",
        user,
        team_pk,
        bookmark,
        status
    );

    // Parse team_pk to ensure it's valid
    let team_partition: Partition = if team_pk.starts_with("TEAM#") {
        Partition::Team(team_pk.strip_prefix("TEAM#").unwrap().to_string())
    } else {
        return Err(Error::BadRequest(format!(
            "Invalid team_pk format: {}",
            team_pk
        )));
    };

    let mut query_options = PostQueryOption::builder().limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    // Query posts by team (author_pk)
    let (mut posts, bookmark) = if let Some(status_val) = status {
        let post_status = match status_val {
            1 => PostStatus::Draft,
            2 => PostStatus::Published,
            _ => return Err(Error::BadRequest("Invalid status value".into())),
        };

        // Query all posts for the team
        let (all_posts, bookmark) =
            Post::find_by_user_pk(&dynamo.client, &team_partition, query_options).await?;

        // Filter by status in memory
        let filtered_posts: Vec<Post> = all_posts
            .into_iter()
            .filter(|post| post.status == post_status)
            .collect();

        (filtered_posts, bookmark)
    } else {
        // No status filter - return all team posts (public only if no auth)
        Post::find_by_user_pk(&dynamo.client, &team_partition, query_options).await?
    };

    // If no status filter, only show public posts
    if status.is_none() {
        posts.retain(|post| post.visibility == Some(Visibility::Public));
    }

    tracing::debug!(
        "list_team_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    // Fetch post likes for authenticated users
    let likes = match (&user, posts.is_empty()) {
        (Some(user), false) => {
            PostLike::batch_get(
                &dynamo.client,
                posts
                    .iter()
                    .map(|post| PostLike::keys(&post.pk, &user.pk))
                    .collect(),
            )
            .await?
        }
        _ => vec![],
    };

    tracing::debug!("list_team_posts_handler: returning {} items", posts.len());
    let items: Vec<PostResponse> = posts
        .into_iter()
        .map(|post| {
            let post_like_pk = post
                .pk
                .clone()
                .to_post_like_key()
                .expect("to_post_like_key");
            let liked = likes.iter().any(|like| like.pk == post_like_pk);
            PostResponse::from(post).with_like(liked)
        })
        .collect();

    Ok(Json(ListItemsResponse { items, bookmark }))
}
