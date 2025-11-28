use bdk::prelude::*;
use by_axum::axum::Extension;

use crate::error::Error;
use crate::features::spaces::boards::models::space_post::SpacePost;
use crate::models::SpaceCommon;
use crate::spaces::SpacePostPath;
use crate::spaces::SpacePostPathParam;
use crate::types::SpaceStatus;
use crate::{
    AppState, Permissions,
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
    NoApi(permissions): NoApi<Permissions>,
    NoApi(user): NoApi<User>,
    Path(SpacePostPathParam {
        space_pk,
        space_post_pk,
    }): SpacePostPath,
    Extension(space): Extension<SpaceCommon>,
    Json(req): Json<SpaceAddCommentRequest>,
) -> Result<Json<SpacePostComment>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    if !permissions.contains(TeamGroupPermission::SpaceRead) {
        return Err(Error::NoPermission);
    }

    if space.status == Some(SpaceStatus::Finished) {
        return Err(Error::FinishedSpace);
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
