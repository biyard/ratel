use bdk::prelude::*;

use dto::{
    FeedStatus, FeedType, Post, Result, Space, aide,
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

use crate::utils::{
    space_visibility::{ViewerCtx, check_space_permission},
    users::extract_user,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct ListPostsQueryParams {
    pub size: Option<i32>,
    pub page: Option<i32>,
    pub status: Option<FeedStatus>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct ListPostsResponse {
    pub posts: Vec<Post>,
    pub is_ended: bool,
}

pub async fn list_posts_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Query(params): Query<ListPostsQueryParams>,
) -> Result<Json<ListPostsResponse>> {
    let size = params.size.unwrap_or(10);
    let page = params.page.unwrap_or(1);

    let user = extract_user(&pool, auth.clone()).await.unwrap_or_default();
    let user_id = user.id;
    let teams = user.teams;

    let builder = Post::query_builder(user_id);
    let builder = if let Some(status) = params.status {
        builder.status_equals(status)
    } else {
        builder
    };
    let builder = if let Some(uid) = params.user_id {
        if uid != 0 {
            builder.user_id_equals(uid)
        } else {
            builder
        }
    } else {
        builder
    };

    let mut fetched: Vec<Post> = builder
        .limit(size)
        .page(page)
        .feed_type_between(FeedType::Artwork, FeedType::Post)
        .order_by_created_at_desc()
        .query()
        .map(Post::from)
        .fetch_all(&pool)
        .await?;

    let is_ended = fetched.len() < size as usize;

    //FIXME: Currently, `Post::query_builder(user_id).space_builder(Space::query_builder()) ` does not work properly.
    // So, we need to fetch Space separately.
    // this bug should be fixed in the future.
    // Same issue with `get_post_handler`
    let viewer_ctx = ViewerCtx {
        user_id,
        team_ids: teams.iter().map(|t| t.id).collect(),
    };

    let mut posts: Vec<Post> = Vec::with_capacity(fetched.len());
    for mut post in fetched.drain(..) {
        let space = Space::query_builder(user_id)
            .feed_id_equals(post.id)
            .query()
            .map(Space::from)
            .fetch_optional(&pool)
            .await?;

        match space {
            None => {
                post.space = vec![];
                posts.push(post);
            }
            Some(space) => {
                if check_space_permission(space.clone(), &viewer_ctx) {
                    post.space = vec![space];
                    posts.push(post);
                }
            }
        }
    }

    Ok(Json(ListPostsResponse { posts, is_ended }))
}
