use crate::constants::MAX_ATTEMPT_COUNT;
use crate::models::{
    EmailVerification, EmailVerificationQueryOption, PhoneVerification,
    PhoneVerificationQueryOption,
};
use crate::utils::time::get_now_timestamp;

use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
pub enum VerifyCodeRequest {
    Email { email: String, code: String },
    Phone { phone: String, code: String },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct VerifyCodeResponse {
    pub success: bool,
}

#[post("/api/auth/verification/verify-code")]
pub async fn verify_code_handler(
    form: dioxus::fullstack::Form<VerifyCodeRequest>,
) -> std::result::Result<VerifyCodeResponse, ServerFnError> {
    let req: VerifyCodeRequest = form.0;

    match req {
        VerifyCodeRequest::Email { email, code } => {
            verify_email_code_handler(email, code).await
        }
        VerifyCodeRequest::Phone { phone, code } => {
            verify_phone_code_handler(phone, code).await
        }
    }
}

async fn verify_email_code_handler(
    email: String,
    code: String,
) -> std::result::Result<VerifyCodeResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();

    let now = get_now_timestamp();
    let (verification_list, _) = EmailVerification::find_by_email(
        cli,
        &email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    if verification_list.is_empty() {
        return Err(ServerFnError::new("Verification code not found"));
    }

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(VerifyCodeResponse { success: true });
    }

    let email_verification = verification_list[0].clone();

    if email_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(ServerFnError::new(
            "Exceeded maximum attempt for email verification",
        ));
    }

    if email_verification.expired_at < now {
        return Err(ServerFnError::new("Verification code expired"));
    }

    if email_verification.value != code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(cli)
            .await
            .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;
        return Err(ServerFnError::new("Invalid verification code"));
    }

    Ok(VerifyCodeResponse { success: true })
}

async fn verify_phone_code_handler(
    phone: String,
    code: String,
) -> std::result::Result<VerifyCodeResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();

    let now = get_now_timestamp();
    let (verification_list, _) = PhoneVerification::find_by_phone(
        cli,
        &phone,
        PhoneVerificationQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    if verification_list.is_empty() {
        return Err(ServerFnError::new("Verification code not found"));
    }

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(VerifyCodeResponse { success: true });
    }

    let phone_verification = verification_list[0].clone();

    if phone_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(ServerFnError::new(
            "Exceeded maximum attempt for phone verification",
        ));
    }

    if phone_verification.expired_at < now {
        return Err(ServerFnError::new("Verification code expired"));
    }

    if phone_verification.value != code {
        PhoneVerification::updater(phone_verification.pk, phone_verification.sk)
            .increase_attempt_count(1)
            .execute(cli)
            .await
            .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;
        return Err(ServerFnError::new("Invalid verification code"));
    }

    Ok(VerifyCodeResponse { success: true })
}
