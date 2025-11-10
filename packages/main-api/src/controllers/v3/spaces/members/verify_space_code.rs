use crate::NoApi;
use crate::controllers::v3::spaces::{SpacePath, SpacePathParam};
use crate::features::spaces::members::SpaceEmailVerification;
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
pub struct VerifySpaceCodeRequest {
    #[schemars(description = "Verification code sent to user's email.")]
    pub code: String,
}

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

    let _user = user.unwrap_or_default();

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
