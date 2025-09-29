use crate::{
    AppState, Error2,
    constants::MAX_ATTEMPT_COUNT,
    models::email::{EmailVerification, EmailVerificationQueryOption},
    utils::time::get_now_timestamp,
};
use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::axum::extract::{Json, State},
};
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct VerifyCodeRequest {
    #[schemars(description = "Email address used for verification.")]
    pub email: String,
    #[schemars(description = "Verification code sent to user's email.")]
    pub code: String,
}

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema,
)]
pub struct VerifyCodeResponse {
    #[schemars(description = "Indicates if the verification was successful.")]
    pub success: bool,
}

pub async fn verify_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<VerifyCodeRequest>,
) -> Result<Json<VerifyCodeResponse>, Error2> {
    let now = get_now_timestamp();
    let (verification_list, _) = EmailVerification::find_by_email(
        &dynamo.client,
        &req.email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    if verification_list.is_empty() {
        return Err(Error2::NotFoundVerificationCode);
    }

    tracing::debug!("code {}", req.code);
    // string equals

    #[cfg(feature = "bypass")]
    if req.code.eq("000000") {
        return Ok(Json(VerifyCodeResponse { success: true }));
    }

    let email_verification = verification_list[0].clone();

    if email_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(Error2::ExceededAttemptEmailVerification);
    }

    if email_verification.expired_at < now {
        return Err(Error2::ExpiredVerification);
    }

    if email_verification.value != req.code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(&dynamo.client)
            .await?;
        return Err(Error2::InvalidVerificationCode);
    }

    Ok(Json(VerifyCodeResponse { success: true }))
}
