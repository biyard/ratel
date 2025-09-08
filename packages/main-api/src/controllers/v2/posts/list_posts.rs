use bdk::prelude::*;

use dto::{
    FeedStatus, FeedType, GroupPermission, Post, RatelResource, Result, aide,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Query, State},
        },
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::{security::check_perm, utils::users::extract_user};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct ListPostsQueryParams {
    pub size: Option<i32>,
    pub page: Option<i32>,
    pub status: Option<FeedStatus>,
    pub user_id: Option<i64>,
}

pub async fn list_posts_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(params): Query<ListPostsQueryParams>,
) -> Result<Json<Vec<Post>>> {
    let size = params.size.unwrap_or(10);
    let page = params.page.unwrap_or(1);

    let user = extract_user(&pool, auth.clone()).await;
    let user_id = user.unwrap_or_default().id;

    if let Some(query_user_id) = params.user_id {
        tracing::debug!(
            "Checking permissions for user_id: {} {}",
            query_user_id,
            user_id
        );
        if query_user_id != user_id {
            check_perm(
                &pool,
                auth,
                RatelResource::Post {
                    team_id: query_user_id,
                },
                GroupPermission::ReadPosts,
            )
            .await?;
        }
    };

    let builder = Post::query_builder(user_id);
    let builder = if let Some(status) = params.status {
        builder.status_equals(status)
    } else {
        builder
    };

    let builder = if let Some(user_id) = params.user_id {
        builder.user_id_equals(user_id)
    } else {
        builder
    };

    let posts: Vec<Post> = builder
        .limit(size)
        .page(page)
        .feed_type_between(FeedType::Artwork, FeedType::Post)
        .order_by_created_at_desc()
        .query()
        .map(Post::from)
        .fetch_all(&pool)
        .await?;

    Ok(Json(posts))
}
