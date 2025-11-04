use bdk::prelude::*;

use crate::error::Error;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::models::SpaceCommon;
use crate::spaces::SpacePostPath;
use crate::spaces::SpacePostPathParam;
use crate::{
    AppState,
    controllers::v3::posts::PostPath,
    features::spaces::boards::models::space_post_comment::SpacePostComment,
    models::{
        feed::{Post, PostComment},
        team::Team,
        user::User,
    },
    types::{EntityType, Partition, TeamGroupPermission},
};
use aide::NoApi;
use by_axum::axum::extract::{Json, Path, State};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct SpaceAddCommentRequest {
    pub content: String,
}

#[derive(Debug, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct SpaceAddCommentResponse {
    pub comment_pk: String,
}

pub async fn add_space_comment_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
    Json(req): Json<SpaceAddCommentRequest>,
) -> Result<Json<SpacePostComment>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (_, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceRead,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);

    let space_post = SpacePost::get(&dynamo.client, pk, Some(sk)).await?;
    if space_post.is_none() {
        return Err(Error::PostNotFound)?;
    }

    let comment = SpacePost::comment(
        &dynamo.client,
        space_pk.clone(),
        space_post_pk.clone(),
        req.content,
        user,
    )
    .await?;

    Ok(Json(comment))
}
