use super::*;
use crate::models::Post;
use crate::models::user::User;
use crate::{AppState, Error2};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct LikeCommentRequest {
    pub like: bool,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct LikeCommentResponse {
    pub liked: bool,
}

pub async fn like_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PostCommentPathParam {
        post_pk,
        comment_sk,
    }): PostCommentPath,
    Json(req): Json<LikeCommentRequest>,
) -> Result<Json<LikeCommentResponse>, Error2> {
    tracing::debug!("Handling request: {:?}", req);
    let cli = &dynamo.client;

    if req.like {
        Post::like_comment(cli, post_pk, comment_sk, user.pk).await?;
    } else {
        Post::unlike_comment(cli, post_pk, comment_sk, user.pk).await?;
    }

    Ok(Json(LikeCommentResponse { liked: req.like }))
}
