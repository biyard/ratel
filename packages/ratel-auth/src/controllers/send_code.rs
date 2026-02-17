use crate::constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT};
use crate::models::{
    EmailVerification, EmailVerificationQueryOption, PhoneVerification,
    PhoneVerificationQueryOption,
};
use crate::utils::time::get_now_timestamp;
use crate::utils::{generate_random_code, generate_random_numeric_code};

use dioxus::prelude::*;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum SendCodeRequest {
    SMS { phone: String },
    Email { email: String },
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct SendCodeResponse {
    pub expired_at: i64,
}

#[post("/api/auth/verification/send-code")]
pub async fn send_code_handler(
    form: dioxus::fullstack::Form<SendCodeRequest>,
) -> std::result::Result<SendCodeResponse, ServerFnError> {
    let req: SendCodeRequest = form.0;

    match req {
        SendCodeRequest::Email { email } => send_email_code_handler(email).await,
        SendCodeRequest::SMS { phone } => send_phone_code_handler(phone).await,
    }
}

async fn send_phone_code_handler(
    phone: String,
) -> std::result::Result<SendCodeResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();

    let (verification_list, _) =
        PhoneVerification::find_by_phone(cli, &phone, PhoneVerification::opt_one())
            .await
            .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    let PhoneVerification {
        expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at < (get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(ServerFnError::new(
            "Exceeded maximum attempt for phone verification",
        ));
    } else {
        let code = generate_random_numeric_code();
        let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;

        if !verification_list.is_empty() {
            let v = verification_list[0].clone();
            PhoneVerification::updater(v.pk.clone(), v.sk.clone())
                .with_attempt_count(0)
                .with_value(code.clone())
                .with_expired_at(expired_at)
                .execute(cli)
                .await
                .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;
            PhoneVerification {
                value: code,
                expired_at,
                ..v
            }
        } else {
            let phone_verification = PhoneVerification::new(phone.clone(), code, expired_at);
            phone_verification
                .create(cli)
                .await
                .map_err(|e| ServerFnError::new(format!("DB create failed: {:?}", e)))?;
            phone_verification
        }
    };

    // TODO: Send SMS via SNS when AWS clients are configured

    Ok(SendCodeResponse { expired_at })
}

async fn send_email_code_handler(
    email: String,
) -> std::result::Result<SendCodeResponse, ServerFnError> {
    let c = crate::config::get();
    let cli = c.common.dynamodb();

    let (verification_list, _) = EmailVerification::find_by_email(
        cli,
        &email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await
    .map_err(|e| ServerFnError::new(format!("DB query failed: {:?}", e)))?;

    let EmailVerification {
        expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at < (get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(ServerFnError::new(
            "Exceeded maximum attempt for email verification",
        ));
    } else {
        let code = generate_random_code();
        let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;

        if !verification_list.is_empty() {
            let v = verification_list[0].clone();
            EmailVerification::updater(v.pk.clone(), v.sk.clone())
                .with_attempt_count(0)
                .with_value(code.clone())
                .with_expired_at(expired_at)
                .execute(cli)
                .await
                .map_err(|e| ServerFnError::new(format!("DB update failed: {:?}", e)))?;
            EmailVerification {
                value: code,
                expired_at,
                ..v
            }
        } else {
            let email_verification = EmailVerification::new(email.clone(), code, expired_at);
            email_verification
                .create(cli)
                .await
                .map_err(|e| ServerFnError::new(format!("DB create failed: {:?}", e)))?;
            email_verification
        }
    };

    // TODO: Send email via SES when AWS clients are configured

    Ok(SendCodeResponse { expired_at })
}
