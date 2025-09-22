use crate::{AppState, Error2, models::email::EmailVerification, utils::time::get_now_timestamp};
use bdk::prelude::*;
use dto::{
    JsonSchema, aide,
    by_axum::axum::extract::{Json, State},
};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct VerifyCodeRequest {
    #[schemars(description = "Email address used for verification.")]
    pub email: String,
    #[schemars(description = "Verification code sent to user's email.")]
    pub code: String,
}

const MAX_ATTEMPTS: i32 = 3;
pub async fn verify_code_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Json(req): Json<VerifyCodeRequest>,
) -> Result<(), Error2> {
    let now = get_now_timestamp();
    let (verification_list, _) =
        EmailVerification::find_by_email(&dynamo.client, &req.email, Default::default()).await?;

    if verification_list.is_empty() {
        return Err(Error2::NotFound(format!(
            "No verification found for email: {}",
            req.email
        )));
    }

    let email_verification = verification_list[0].clone();

    if email_verification.expired_at < now {
        EmailVerification::delete(
            &dynamo.client,
            email_verification.pk,
            Some(email_verification.sk),
        )
        .await?;
        return Err(Error2::BadRequest(
            "Verification code has expired".to_string(),
        ));
    }
    if email_verification.attempt_count >= MAX_ATTEMPTS {
        return Err(Error2::BadRequest(
            "Maximum verification attempts exceeded".to_string(),
        ));
    }
    if email_verification.value != req.code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(&dynamo.client)
            .await?;
        return Err(Error2::BadRequest("Code mismatch".to_string()));
    }

    Ok(())
}
