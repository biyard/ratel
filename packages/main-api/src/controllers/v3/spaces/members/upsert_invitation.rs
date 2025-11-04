use std::collections::HashSet;

use crate::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::members::{
    SpaceEmailVerification, SpaceInvitationMember, SpaceInvitationMemberQueryOption,
};
use crate::models::{SpaceCommon, User};
use crate::types::Partition;
use crate::types::SpaceStatus;
use crate::types::TeamGroupPermission;
use crate::types::{EntityType, SpacePublishState};
use crate::utils::aws::{DynamoClient, SesClient};
use crate::{
    AppState, Error,
    constants::MAX_ATTEMPT_COUNT,
    models::email::{EmailVerification, EmailVerificationQueryOption},
    utils::time::get_now_timestamp,
};
use bdk::prelude::*;
use by_axum::axum::extract::{Json, Path, State};
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct UpsertInvitationRequest {
    pub user_pks: Vec<Partition>,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct UpsertInvitationResponse {
    pub space_pk: Partition,
    pub user_pks: Vec<Partition>,
}

pub async fn upsert_invitation_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<UpsertInvitationRequest>,
) -> Result<Json<UpsertInvitationResponse>, Error> {
    //Request Validation
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    // Check Permissions
    let (space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    if space_common.status == Some(SpaceStatus::Started)
        || space_common.status == Some(SpaceStatus::Finished)
    {
        return Err(Error::FinishedSpace);
    }

    if space_common.publish_state == SpacePublishState::Published {
        published_invitations(&dynamo, &ses, &space_pk, req.user_pks.clone()).await?;
        return Ok(Json(UpsertInvitationResponse {
            space_pk,
            user_pks: req.user_pks,
        }));
    }

    let mut bookmark = None::<String>;

    loop {
        // remove existing data
        let (responses, new_bookmark) = SpaceInvitationMember::query(
            &dynamo.client,
            space_pk.clone(),
            if let Some(b) = &bookmark {
                SpaceInvitationMemberQueryOption::builder()
                    .sk("SPACE_INVITATION_MEMBER".to_string())
                    .bookmark(b.clone())
            } else {
                SpaceInvitationMemberQueryOption::builder()
                    .sk("SPACE_INVITATION_MEMBER".to_string())
            },
        )
        .await?;

        for response in responses {
            SpaceInvitationMember::delete(&dynamo.client, response.pk, Some(response.sk)).await?;
        }

        match new_bookmark {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    for user_pk in req.user_pks.clone() {
        let user = User::get(&dynamo.client, user_pk.clone(), Some(EntityType::User)).await?;

        if user.is_none() {
            continue;
        }

        let member = SpaceInvitationMember::new(space_pk.clone(), user.unwrap_or_default());
        member.create(&dynamo.client).await?;
    }

    Ok(Json(UpsertInvitationResponse {
        space_pk,
        user_pks: req.user_pks,
    }))
}

// FIXME: enhancement this logic
pub async fn published_invitations(
    dynamo: &DynamoClient,
    ses: &SesClient,
    space_pk: &Partition,
    user_pks: Vec<Partition>,
) -> Result<(), Error> {
    let members = SpaceInvitationMember::list_invitation_members(dynamo, space_pk).await?;

    let current_set: HashSet<String> = members.iter().map(|m| m.user_pk.to_string()).collect();
    let desired_set: HashSet<String> = user_pks.iter().map(|p| p.to_string()).collect();

    let delete_keys: HashSet<String> = current_set.difference(&desired_set).cloned().collect();
    let add_keys: HashSet<String> = desired_set.difference(&current_set).cloned().collect();

    for delete_key in delete_keys {
        let m = SpaceInvitationMember::get(
            &dynamo.client,
            space_pk,
            Some(EntityType::SpaceInvitationMember(delete_key.to_string())),
        )
        .await?
        .unwrap_or_default();
        SpaceInvitationMember::delete(&dynamo.client, m.pk, Some(m.sk)).await?;
        SpaceEmailVerification::delete(
            &dynamo.client,
            space_pk,
            Some(EntityType::SpaceEmailVerification(m.email.clone())),
        )
        .await?;
    }

    for add_key in add_keys {
        let user = User::get(&dynamo.client, add_key.clone(), Some(EntityType::User)).await?;
        if user.is_none() {
            continue;
        }

        let member = SpaceInvitationMember::new(space_pk.clone(), user.unwrap_or_default());
        member.create(&dynamo.client).await?;

        let _ = SpaceEmailVerification::send_email(&dynamo, &ses, member.email, space_pk.clone())
            .await?;
    }

    Ok(())
}
