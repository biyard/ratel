use crate::features::spaces::{SpaceRequirement, SpaceRequirementType, polls::*};
use crate::types::{EntityType, Partition, TeamGroupPermission};
use crate::{AppState, Error, Permissions, transact_write_items};

use aws_sdk_dynamodb::operation::transact_write_items;
use bdk::prelude::*;

use by_axum::axum::Json;
use by_axum::axum::extract::{Path, State};

use crate::models::user::User;
use aide::NoApi;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema, Default)]
pub struct DeletePollSpaceResponse {
    pub status: String,
}

pub async fn delete_poll_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(_user): NoApi<User>,
    NoApi(permissions): NoApi<Permissions>,
    Path(PollPathParam { space_pk, poll_sk }): PollPath,
) -> crate::Result<Json<DeletePollSpaceResponse>> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) || !matches!(poll_sk, EntityType::SpacePoll(_)) {
        return Err(Error::NotFoundPoll);
    }

    // Check Permissions
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    // Check if poll exists
    let poll = Poll::get(&dynamo.client, &space_pk, Some(&poll_sk))
        .await?
        .ok_or(Error::NotFoundPoll)?;

    // Delete the poll
    let mut txs = vec![];
    txs.push(Poll::delete_transact_write_item(&poll.pk, &poll.sk));

    if poll.is_default_poll() {
        let (pk, sk) = SpaceRequirement::keys(&space_pk, Some(SpaceRequirementType::PrePoll));
        txs.push(SpaceRequirement::delete_transact_write_item(&pk, &sk));
    }
    let cli = &dynamo.client;
    transact_write_items!(cli, txs);

    Ok(Json(DeletePollSpaceResponse {
        status: "success".to_string(),
    }))
}
