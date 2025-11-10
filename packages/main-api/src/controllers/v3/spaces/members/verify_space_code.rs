use crate::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::members::{
    InvitationStatus, SpaceEmailVerification, SpaceInvitationMember,
};
use crate::models::{SpaceCommon, User};
use crate::types::{EntityType, SpaceStatus};
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
pub struct VerifySpaceCodeRequest {}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct VerifySpaceCodeResponse {
    #[schemars(description = "Indicates if the verification was successful.")]
    pub success: bool,
}

pub async fn verify_space_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    NoApi(user): NoApi<Option<User>>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(_req): Json<VerifySpaceCodeRequest>,
) -> Result<Json<VerifySpaceCodeResponse>, Error> {
    if user.is_none() {
        return Ok(Json(VerifySpaceCodeResponse { success: false }));
    }

    let user = user.unwrap_or_default();

    let space = SpaceCommon::get(
        &dynamo.client,
        space_pk.clone(),
        Some(EntityType::SpaceCommon),
    )
    .await?
    .ok_or(Error::SpaceNotFound)?;

    if space.status == Some(SpaceStatus::Started) || space.status == Some(SpaceStatus::Finished) {
        return Err(Error::FinishedSpace);
    }

    let (pk, sk) = SpaceInvitationMember::keys(&space.pk, &user.pk);
    tracing::debug!("verification pk: {:?}, sk: {:?}", pk, sk);

    let user = SpaceInvitationMember::get(&dynamo.client, pk.clone(), Some(sk.clone())).await?;

    if user.is_none() {
        return Ok(Json(VerifySpaceCodeResponse { success: true }));
    }

    let _ = SpaceInvitationMember::updater(pk, sk)
        .with_status(InvitationStatus::Accepted)
        .execute(&dynamo.client)
        .await
        .unwrap_or_default();

    // let now = get_now_timestamp();
    // let verification = SpaceEmailVerification::get(
    //     &dynamo.client,
    //     &space_pk,
    //     Some(EntityType::SpaceEmailVerification(user.email.clone())),
    // )
    // .await?;

    // if verification.is_none() {
    //     return Err(Error::NotFoundVerificationCode);
    // }

    // tracing::debug!("code {}", req.code);

    // let email_verification = verification.unwrap_or_default();

    // if email_verification.authorized {
    //     return Err(Error::ExceededAttemptEmailVerification);
    // }

    // if email_verification.attempt_count >= MAX_ATTEMPT_COUNT {
    //     return Err(Error::ExceededAttemptEmailVerification);
    // }

    // if email_verification.expired_at < now {
    //     return Err(Error::ExpiredVerification);
    // }

    // if email_verification.value != req.code {
    //     SpaceEmailVerification::updater(email_verification.pk, email_verification.sk)
    //         .increase_attempt_count(1)
    //         .execute(&dynamo.client)
    //         .await?;
    //     return Err(Error::InvalidVerificationCode);
    // } else {
    //     SpaceEmailVerification::updater(email_verification.pk, email_verification.sk)
    //         .with_authorized(true)
    //         .execute(&dynamo.client)
    //         .await?;
    // }

    Ok(Json(VerifySpaceCodeResponse { success: true }))
}
