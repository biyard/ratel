use crate::Error;
use crate::controllers::v3::spaces::{SpacePostCommentPath, SpacePostCommentPathParam};
use crate::features::spaces::boards::models::space_post_comment::SpacePostComment;
use crate::models::SpaceCommon;
use crate::models::user::User;
use crate::types::{Partition, SpaceStatus, TeamGroupPermission};
use crate::{AppState, Permissions, models::feed::PostComment};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ReplySpaceCommentRequest {
    pub content: String,
}

pub async fn reply_space_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<ReplySpaceCommentRequest>,
) -> Result<Json<SpacePostComment>, crate::Error> {
    tracing::debug!("Handling request: {:?}", req);
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(crate::Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(crate::Error::NoPermission);
    }

    if space.status == Some(SpaceStatus::Finished) {
        return Err(Error::FinishedSpace);
    }

    let comment = SpacePostComment::reply(
        &dynamo.client,
        space_pk,
        space_post_pk,
        space_post_comment_sk,
        req.content,
        user,
    )
    .await?;

    Ok(Json(comment))
}
