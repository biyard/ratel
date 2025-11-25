use crate::*;
use crate::{
    AppState, Error,
    constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT},
    models::{
        email::{EmailVerification, EmailVerificationQueryOption},
        email_template::email_template::EmailTemplate,
        phone::{PhoneVerification, PhoneVerificationQueryOption},
    },
    types::email_operation::EmailOperation,
    utils::{
        aws::{DynamoClient, SesClient, SnsClient},
        generate_random_code,
        time::get_now_timestamp,
    },
};
use by_axum::axum::{Json, extract::State};
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum SendCodeRequest {
    SMS { phone: String },
    Email { email: String },
}

#[derive(Debug, Clone, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct SendCodeResponse {
    #[schemars(description = "Expiration time of the verification code.")]
    pub expired_at: i64,
}
pub async fn send_code_handler(
    State(app_state): State<AppState>,
    Json(req): Json<SendCodeRequest>,
) -> Result<Json<SendCodeResponse>> {
    match req {
        SendCodeRequest::Email { email } => send_email_code_handler(app_state, email).await,
        SendCodeRequest::SMS { phone } => send_phone_code_handler(app_state, phone).await,
    }
}

pub async fn send_phone_code_handler(
    AppState { dynamo, sns, .. }: AppState,
    phone: String,
) -> Result<Json<SendCodeResponse>> {
    let (verification_list, _) = PhoneVerification::find_by_phone(
        &dynamo.client,
        &phone,
        PhoneVerificationQueryOption::builder().limit(1),
    )
    .await?;

    let PhoneVerification {
        value, expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at < (get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(Error::ExceededAttemptPhoneVerification);
    } else {
        let code = generate_random_code();
        let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;

        if verification_list.len() > 0 {
            let mut v = verification_list[0].clone();
            PhoneVerification::updater(v.pk.clone(), v.sk.clone())
                .with_attempt_count(0)
                .with_value(code.clone())
                .with_expired_at(expired_at)
                .execute(&dynamo.client)
                .await?;
            v.value = code;
            v.expired_at = expired_at;
            v
        } else {
            let phone_verification = PhoneVerification::new(phone.clone(), code, expired_at);
            phone_verification.create(&dynamo.client).await?;
            phone_verification
        }
    };

    // Send SMS with verification code
    let message = format!("Your verification code is: {}", value);
    sns.send_sms(&phone, &message).await?;

    Ok(Json(SendCodeResponse { expired_at }))
}

pub async fn send_email_code_handler(
    AppState { dynamo, ses, .. }: AppState,
    email: String,
) -> Result<Json<SendCodeResponse>> {
    // let _ses = ses.clone();
    let (verification_list, _) = EmailVerification::find_by_email(
        &dynamo.client,
        &email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    let EmailVerification {
        value, expired_at, ..
    } = if !verification_list.is_empty()
        && verification_list[0].expired_at > get_now_timestamp()
        && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
    {
        verification_list[0].clone()
    } else if !verification_list.is_empty()
        && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
        && verification_list[0].expired_at < (get_now_timestamp() - ATTEMPT_BLOCK_TIME)
    {
        return Err(Error::ExceededAttemptEmailVerification);
    } else {
        let code = generate_random_code();
        let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;

        if verification_list.len() > 0 {
            let mut v = verification_list[0].clone();
            EmailVerification::updater(v.pk.clone(), v.sk.clone())
                .with_attempt_count(0)
                .with_value(code.clone())
                .with_expired_at(expired_at)
                .execute(&dynamo.client)
                .await?;
            v.value = code;
            v.expired_at = expired_at;
            v
        } else {
            let email_verification = EmailVerification::new(email.clone(), code, expired_at);
            email_verification.create(&dynamo.client).await?;
            email_verification
        }
    };

    let user_email = email.clone();
    let display_name = user_email.clone();

    let mut chars = value.chars();
    let code_1 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_2 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_3 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_4 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_5 = chars.next().map(|c| c.to_string()).unwrap_or_default();
    let code_6 = chars.next().map(|c| c.to_string()).unwrap_or_default();

    let email = EmailTemplate {
        targets: vec![user_email.clone()],
        operation: EmailOperation::SignupSecurityCode {
            display_name,
            code_1,
            code_2,
            code_3,
            code_4,
            code_5,
            code_6,
        },
    };

    email.send_email(&dynamo, &ses).await?;

    Ok(Json(SendCodeResponse { expired_at }))
}
