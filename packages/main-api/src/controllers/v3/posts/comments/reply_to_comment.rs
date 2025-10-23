use crate::models::user::User;
use crate::{AppState, models::feed::PostComment};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

use super::dto::{PostCommentPath, PostCommentPathParam};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ReplyToCommentRequest {
    pub content: String,
}

pub async fn reply_to_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PostCommentPathParam {
        post_pk,
        comment_sk: comment_pk,
    }): PostCommentPath,
    Json(req): Json<ReplyToCommentRequest>,
) -> Result<Json<PostComment>, crate::Error> {
    tracing::debug!("Handling request: {:?}", req);

    let comment =
        PostComment::reply(&dynamo.client, post_pk, comment_pk, req.content, user).await?;

    Ok(Json(comment))
}
