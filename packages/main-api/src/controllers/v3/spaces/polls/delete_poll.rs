use crate::models::space::SpaceCommon;

use crate::features::spaces::polls::*;
use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::{AppState, Error};

use bdk::prelude::*;

use by_axum::axum::extract::{Path, State};
use by_axum::axum::Json;

use crate::models::user::User;
use aide::NoApi;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct DeletePollSpaceResponse {
    pub status: String,
}

pub async fn delete_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
) -> crate::Result<Json<DeletePollSpaceResponse>> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) || !matches!(poll_sk, EntityType::SpacePoll(_)) {
        return Err(Error::NotFoundPoll);
    }

    // Check Permissions
    let (_space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    // Check if poll exists
    let poll = Poll::get(&dynamo.client, &space_pk, Some(&poll_sk))
        .await?
        .ok_or(Error::NotFoundPoll)?;

    // Delete the poll
    Poll::delete(&dynamo.client, &poll.pk, Some(&poll.sk)).await?;

    Ok(Json(DeletePollSpaceResponse {
        status: "success".to_string(),
    }))
}
