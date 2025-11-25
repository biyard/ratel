use crate::{
    AppState, Error,
    constants::MAX_ATTEMPT_COUNT,
    models::email::{EmailVerification, EmailVerificationQueryOption},
    models::phone::{PhoneVerification, PhoneVerificationQueryOption},
    utils::time::get_now_timestamp,
};
use bdk::prelude::*;
use by_axum::axum::extract::{Json, State};
use serde::Deserialize;

#[derive(Debug, Clone, serde::Serialize, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum VerifyCodeRequest {
    Email {
        #[schemars(description = "Email address used for verification.")]
        email: String,
        #[schemars(description = "Verification code sent to user's email.")]
        code: String,
    },
    Phone {
        #[schemars(description = "Phone number used for verification.")]
        phone: String,
        #[schemars(description = "Verification code sent to user's phone.")]
        code: String,
    },
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
) -> Result<Json<VerifyCodeResponse>, Error> {
    match req {
        VerifyCodeRequest::Email { email, code } => {
            verify_email_code_handler(dynamo, email, code).await
        }
        VerifyCodeRequest::Phone { phone, code } => {
            verify_phone_code_handler(dynamo, phone, code).await
        }
    }
}

async fn verify_email_code_handler(
    dynamo: crate::utils::aws::DynamoClient,
    email: String,
    code: String,
) -> Result<Json<VerifyCodeResponse>, Error> {
    let now = get_now_timestamp();
    let (verification_list, _) = EmailVerification::find_by_email(
        &dynamo.client,
        &email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    if verification_list.is_empty() {
        return Err(Error::NotFoundVerificationCode);
    }

    tracing::debug!("code {}", code);

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(Json(VerifyCodeResponse { success: true }));
    }

    let email_verification = verification_list[0].clone();

    if email_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(Error::ExceededAttemptEmailVerification);
    }

    if email_verification.expired_at < now {
        return Err(Error::ExpiredVerification);
    }

    if email_verification.value != code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(&dynamo.client)
            .await?;
        return Err(Error::InvalidVerificationCode);
    }

    Ok(Json(VerifyCodeResponse { success: true }))
}

async fn verify_phone_code_handler(
    dynamo: crate::utils::aws::DynamoClient,
    phone: String,
    code: String,
) -> Result<Json<VerifyCodeResponse>, Error> {
    let now = get_now_timestamp();
    let (verification_list, _) = PhoneVerification::find_by_phone(
        &dynamo.client,
        &phone,
        PhoneVerificationQueryOption::builder().limit(1),
    )
    .await?;

    if verification_list.is_empty() {
        return Err(Error::NotFoundVerificationCode);
    }

    tracing::debug!("code {}", code);

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(Json(VerifyCodeResponse { success: true }));
    }

    let phone_verification = verification_list[0].clone();

    if phone_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(Error::ExceededAttemptPhoneVerification);
    }

    if phone_verification.expired_at < now {
        return Err(Error::ExpiredVerification);
    }

    if phone_verification.value != code {
        PhoneVerification::updater(phone_verification.pk, phone_verification.sk)
            .increase_attempt_count(1)
            .execute(&dynamo.client)
            .await?;
        return Err(Error::InvalidVerificationCode);
    }

    Ok(Json(VerifyCodeResponse { success: true }))
}
