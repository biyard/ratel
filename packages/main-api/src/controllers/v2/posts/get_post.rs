use bdk::prelude::*;

use dto::{
    Comment, Post, Reply, Result, Space,
    by_axum::{
        auth::Authorization,
        axum::{
            Extension, Json,
            extract::{Path, State},
        },
    },
    sqlx::PgPool,
};
use serde::{Deserialize, Serialize};

use crate::utils::{
    space_visibility::{ViewerCtx, scope_space_opt_to_vec},
    users::extract_user,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, aide::OperationIo, JsonSchema)]
pub struct GetPostQueryParams {
    pub id: i64,
}

pub async fn get_post_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<PgPool>,
    Path(params): Path<GetPostQueryParams>,
) -> Result<Json<Post>> {
    let user = extract_user(&pool, auth.clone()).await;
    //FIXME: Add Permission Check
    let user = user.unwrap_or_default();
    let user_id = user.id;
    let teams = user.teams;

    let mut post = Post::query_builder(user_id)
        .id_equals(params.id)
        .comment_list_builder(
            Comment::query_builder(user_id).replies_builder(Reply::query_builder()),
        )
        .query()
        .map(Post::from)
        .fetch_one(&pool)
        .await?;
    //FIXME: Currently, `Post::query_builder(user_id).space_builder(Space::query_builder()) ` does not work properly.
    // So, we need to fetch Space separately.
    // this bug should be fixed in the future.
    let space = Space::query_builder(user_id)
        .feed_id_equals(post.id)
        .query()
        .map(Space::from)
        .fetch_optional(&pool)
        .await?;

    let ctx = ViewerCtx {
        user_id,
        team_ids: teams.iter().map(|t| t.id).collect(),
    };
    post.space = scope_space_opt_to_vec(space, &ctx);

    Ok(Json(post))
}
