use super::*;
use crate::controllers::v3::spaces::{
    SpacePostCommentPath, SpacePostCommentPathParam, SpacePostPath, SpacePostPathParam,
};
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::models::user::User;
use crate::models::{Post, SpaceCommon};
use crate::types::{Partition, SpaceStatus, TeamGroupPermission};
use crate::{AppState, Error};
use aide::NoApi;
use axum::extract::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct SpaceLikeCommentRequest {
    pub like: bool,
}

#[derive(
    Debug, Default, Clone, serde::Serialize, serde::Deserialize, JsonSchema, aide::OperationIo,
)]
pub struct SpaceLikeSpaceCommentResponse {
    pub liked: bool,
}

pub async fn like_space_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePostCommentPathParam {
        space_pk,
        space_post_pk,
        space_post_comment_sk,
    }): SpacePostCommentPath,
    Json(req): Json<SpaceLikeCommentRequest>,
) -> Result<Json<SpaceLikeSpaceCommentResponse>, Error> {
    tracing::debug!("Handling request: {:?}", req);
    let cli = &dynamo.client;

    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (_space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    // if space_common.status == Some(SpaceStatus::Started)
    //     || space_common.status == Some(SpaceStatus::Finished)
    // {
    //     return Err(Error::FinishedSpace);
    // }

    if req.like {
        SpacePost::like_comment(cli, space_post_pk, space_post_comment_sk, user.pk).await?;
    } else {
        SpacePost::unlike_comment(cli, space_post_pk, space_post_comment_sk, user.pk).await?;
    }

    Ok(Json(SpaceLikeSpaceCommentResponse { liked: req.like }))
}
