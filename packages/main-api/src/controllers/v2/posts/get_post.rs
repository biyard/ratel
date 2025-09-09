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

use crate::utils::users::extract_user;

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
    let user_id = user.unwrap_or_default().id;

    let post = Post::query_builder(user_id)
        .space_builder(Space::query_builder(user_id))
        .comment_list_builder(
            Comment::query_builder(user_id).replies_builder(Reply::query_builder()),
        )
        .id_equals(params.id)
        .query()
        .map(Post::from)
        .fetch_one(&pool)
        .await?;

    Ok(Json(post))
}
