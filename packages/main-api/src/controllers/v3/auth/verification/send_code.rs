use crate::{
    AppState, Error2, config,
    models::email::{EmailVerification, EmailVerificationQueryOption},
    utils::time::get_now_timestamp,
};
use bdk::prelude::*;
use dto::{
    aide,
    by_axum::axum::{Json, extract::State},
};
use rand::Rng;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema, Validate)]
pub struct SendCodeRequest {
    #[schemars(description = "User's email address")]
    #[validate(email)]
    pub email: String,
}

#[derive(Debug, Clone, Serialize, Default, aide::OperationIo, JsonSchema)]
pub struct SendCodeResponse {
    #[schemars(description = "Expiration time of the verification code.")]
    pub expired_at: i64,
}

const EXPIRATION_TIME: u64 = 1800; // 30 minutes
const MAX_ATTEMPT_COUNT: i32 = 5;

pub async fn send_code_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    Json(req): Json<SendCodeRequest>,
) -> Result<Json<SendCodeResponse>, Error2> {
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
    {
        return Err(Error2::ExceededAttemptEmailVerification);
    } else {
        let code = generate_random_code();
        let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;
        let email_verification = EmailVerification::new(req.email.clone(), code, expired_at);
        email_verification.create(&dynamo.client).await?;
        email_verification
    };

    let mut i = 0;
    while let Err(e) = ses
        .send_mail(
            &req.email,
            "Please finish to sign up within 30 minutes with your verification code",
            format!("Verification code: {:?}", value).as_ref(),
        )
        .await
    {
        btracing::notify!(
            config::get().slack_channel_monitor,
            &format!("Failed to send email: {:?}", e)
        );
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        i += 1;
        if i >= 3 {
            return Err(Error2::AwsSesSendEmailException(e.to_string()));
        }
    }

    Ok(Json(SendCodeResponse { expired_at }))
}

fn generate_random_code() -> String {
    let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    let code: String = (0..6)
        .map(|_| {
            let idx = rng.random_range(0..charset.len());
            charset[idx] as char
        })
        .collect();
    code
}
