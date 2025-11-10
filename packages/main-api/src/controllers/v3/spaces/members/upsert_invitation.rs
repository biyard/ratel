use std::collections::HashSet;

use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::SpaceParticipant;
use crate::features::spaces::members::{
    InvitationStatus, SpaceEmailVerification, SpaceInvitationMember,
    SpaceInvitationMemberQueryOption,
};
use crate::models::{Post, SpaceCommon, User};
use crate::types::Partition;
use crate::types::SpaceStatus;
use crate::types::TeamGroupPermission;
use crate::types::{EntityType, SpacePublishState};
use crate::utils::aws::{DynamoClient, SesClient};
use crate::{
    AppState, Error, Permissions,
    constants::MAX_ATTEMPT_COUNT,
    models::email::{EmailVerification, EmailVerificationQueryOption},
    utils::time::get_now_timestamp,
};
use crate::{NoApi, transact_write_items};
use aws_sdk_dynamodb::types::TransactWriteItem;
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};
use futures::future::try_join_all;
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpsertInvitationRequest {
    pub new_user_pks: Vec<Partition>,
    pub removed_user_pks: Vec<Partition>,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpsertInvitationResponse {
    pub space_pk: Partition,
    pub new_user_pks: Vec<Partition>,
    pub removed_user_pks: Vec<Partition>,
}

pub async fn upsert_invitation_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(permissions): NoApi<Permissions>,
    NoApi(_user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpsertInvitationRequest>,
) -> Result<Json<UpsertInvitationResponse>, Error> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check Permissions
    if !permissions.contains(TeamGroupPermission::SpaceEdit) {
        return Err(Error::NoPermission);
    }

    let space_common = SpaceCommon::get(&dynamo.client, &space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::NotFoundSpace)?;

    if space_common.status == Some(SpaceStatus::Started)
        || space_common.status == Some(SpaceStatus::Finished)
    {
        return Err(Error::FinishedSpace);
    }

    // Remove users
    let mut txs: Vec<TransactWriteItem> = req
        .removed_user_pks
        .iter()
        .map(|user_pk| {
            let (pk, sk) = SpaceInvitationMember::keys(&space_pk, user_pk);
            SpaceInvitationMember::delete_transact_write_item(pk, sk)
        })
        .collect();

    let get_keys: Vec<(Partition, EntityType)> = req
        .new_user_pks
        .iter()
        .map(|user_pk| (user_pk.clone(), EntityType::User))
        .collect();

    let users = User::batch_get(&dynamo.client, get_keys).await?;

    let mut create_txs = users
        .clone()
        .into_iter()
        .map(|user| {
            let mut inv = SpaceInvitationMember::new(space_pk.clone(), user);
            if space_common.publish_state == SpacePublishState::Published {
                inv.status = InvitationStatus::Invited;
            }

            inv.create_transact_write_item()
        })
        .collect();

    txs.append(&mut create_txs);

    if txs.is_empty() {
        return Err(crate::error::Error::NoInvitationFound);
    }

    transact_write_items!(&dynamo.client, txs)?;

    if space_common.publish_state == SpacePublishState::Published {
        // NOTE: Sending email immediately.
        let post_pk = space_pk.clone().to_post_key()?;
        let post = Post::get(&dynamo.client, &post_pk, Some(&EntityType::Post))
            .await?
            .unwrap_or_default();

        let futs = users.into_iter().map(|member| {
            SpaceEmailVerification::send_email(
                &dynamo,
                &ses,
                member.email,
                space_common.clone(),
                post.title.clone(),
            )
        });

        try_join_all(futs).await?;
    }

    return Ok(Json(UpsertInvitationResponse {
        space_pk,
        new_user_pks: req.new_user_pks,
        removed_user_pks: req.removed_user_pks,
    }));
}
