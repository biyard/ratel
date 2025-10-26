use crate::AppState;
use crate::models::feed::PostComment;
use crate::models::user::User;
use crate::types::{ListItemsQuery, ListItemsResponse};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

use super::dto::{PostCommentPath, PostCommentPathParam};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ListCommentsResponse {
    #[schemars(description = "Status of the operation")]
    pub status: String,
}

pub async fn list_comments_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<Option<User>>,
    Path(PostCommentPathParam {
        comment_sk,
        post_pk,
    }): PostCommentPath,
    Query(_query): ListItemsQuery,
) -> Result<Json<ListItemsResponse<PostComment>>, crate::Error> {
    let comments = PostComment::list_by_comment(&dynamo.client, post_pk, comment_sk).await?;

    // TODO: compose with comment like

    Ok(Json(comments.into()))
}
