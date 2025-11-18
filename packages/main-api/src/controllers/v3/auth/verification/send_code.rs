use crate::{
    AppState, Error,
    constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT},
    models::{
        email::{EmailVerification, EmailVerificationQueryOption},
        email_template::email_template::EmailTemplate,
    },
    types::email_operation::EmailOperation,
    utils::{
        aws::{DynamoClient, SesClient},
        generate_random_code,
        time::get_now_timestamp,
    },
};
use bdk::prelude::*;
use by_axum::axum::{Json, extract::State};
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct SendCodeRequest {
    #[schemars(description = "User's email address")]
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, serde::Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct SendCodeResponse {
    #[schemars(description = "Expiration time of the verification code.")]
    pub expired_at: i64,
}

pub async fn send_code_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    Json(req): Json<SendCodeRequest>,
) -> Result<Json<SendCodeResponse>, Error> {
    // let _ses = ses.clone();
    let (verification_list, _) = EmailVerification::find_by_email(
        &dynamo.client,
        &req.email,
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
            let email_verification = EmailVerification::new(req.email.clone(), code, expired_at);
            email_verification.create(&dynamo.client).await?;
            email_verification
        }
    };

    let user_email = req.email.clone();
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
