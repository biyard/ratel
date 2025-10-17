use crate::models::feed::{Post, PostLike, PostQueryOption};
use crate::models::user::User;
use crate::types::list_items_response::ListItemsResponse;
use crate::types::{EntityType, Partition, PostStatus, Visibility};
use crate::{AppState, Error2};
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
    /// Filter posts by author (user_pk or team_pk)
    pub author_pk: Option<String>,
    /// Filter posts by status (draft, published, etc.)
    pub status: Option<i64>,
}

pub async fn list_posts_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Query(ListPostsQueryParams {
        bookmark,
        author_pk,
        status,
    }): Query<ListPostsQueryParams>,
) -> Result<Json<ListItemsResponse<PostResponse>>, Error2> {
    tracing::debug!(
        "list_posts_handler: user = {:?}, bookmark = {:?}, author_pk = {:?}, status = {:?}",
        user,
        bookmark,
        author_pk,
        status
    );

    let mut query_options = PostQueryOption::builder().limit(10);

    if let Some(bookmark) = bookmark {
        query_options = query_options.bookmark(bookmark);
    }

    // Determine which query method to use based on filters
    let (mut posts, bookmark) = if let Some(ref author_pk_str) = author_pk {
        let author_partition: Partition = if author_pk_str.starts_with("USER#") {
            Partition::User(author_pk_str.strip_prefix("USER#").unwrap().to_string())
        } else if author_pk_str.starts_with("TEAM#") {
            Partition::Team(author_pk_str.strip_prefix("TEAM#").unwrap().to_string())
        } else {
            return Err(Error2::BadRequest(format!(
                "Invalid author_pk format: {}",
                author_pk_str
            )));
        };

        if let Some(status_val) = status {
            // Filter by author AND status (e.g., team's drafts)
            // PostStatus is stored as numeric value: Draft=1, Published=2
            let post_status = match status_val {
                1 => PostStatus::Draft,
                2 => PostStatus::Published,
                _ => return Err(Error2::BadRequest("Invalid status value".into())),
            };

            tracing::debug!(
                "Querying find_by_user_and_status with author_partition: {:?}, post_status: {:?}",
                author_partition,
                post_status
            );

            // Query all posts by user_pk and filter by status in memory
            // GSI5 uses user_pk as PK and status as SK, but we can't do exact match on SK with begins_with
            let (all_posts, bookmark) =
                Post::find_by_user_pk(&dynamo.client, &author_partition, query_options).await?;

            // Filter by status
            let filtered_posts: Vec<Post> = all_posts
                .into_iter()
                .filter(|post| post.status == post_status)
                .collect();

            (filtered_posts, bookmark)
        } else {
            // Filter by author only - use find_by_user_pk to get all author's posts
            Post::find_by_user_pk(&dynamo.client, &author_partition, query_options).await?
        }
    } else {
        // No author filter - show all public posts
        Post::find_by_visibility(&dynamo.client, Visibility::Public, query_options).await?
    };

    // If filtering by author without status, only show public posts
    if author_pk.is_some() && status.is_none() {
        posts.retain(|post| post.visibility == Some(Visibility::Public));
    }

    tracing::debug!(
        "list_posts_handler: found {} posts, next bookmark = {:?}",
        posts.len(),
        bookmark
    );

    let likes = match (user.clone(), posts.is_empty()) {
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
