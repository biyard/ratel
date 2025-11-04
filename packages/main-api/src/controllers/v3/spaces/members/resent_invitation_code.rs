use crate::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::members::{
    SpaceEmailVerification, SpaceInvitationMember, SpaceInvitationMemberQueryOption,
};
use crate::models::{Post, SpaceCommon, User};
use crate::types::TeamGroupPermission;
use crate::types::{EntityType, SpacePublishState};
use crate::types::{Partition, SpaceStatus};
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
pub struct ResentInvitationCodeRequest {
    pub email: String,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct ResentInvitationCodeResponse {
    pub is_success: bool,
    pub email: String,
}

pub async fn resent_invitation_code_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(req): Json<ResentInvitationCodeRequest>,
) -> Result<Json<ResentInvitationCodeResponse>, Error> {
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

    let post_pk = space_pk.clone().to_post_key()?;
    let post = Post::get(&dynamo.client, &post_pk, Some(&EntityType::Post))
        .await?
        .unwrap_or_default();

    if space_common.publish_state != SpacePublishState::Published {
        return Err(Error::NotPublishedSpace);
    }

    if space_common.status == Some(SpaceStatus::Started)
        || space_common.status == Some(SpaceStatus::Finished)
    {
        return Err(Error::FinishedSpace);
    }

    let user_email = req.email;
    let _ = SpaceEmailVerification::send_email(
        &dynamo,
        &ses,
        user_email.clone(),
        space_common.clone(),
        post.title,
    )
    .await?;

    Ok(Json(ResentInvitationCodeResponse {
        is_success: true,
        email: user_email.clone(),
    }))
}
