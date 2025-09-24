use crate::{AppState, Error2, models::email::EmailVerification, utils::time::get_now_timestamp};
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
pub async fn send_code_handler(
    State(AppState { dynamo, ses }): State<AppState>,
    Json(req): Json<SendCodeRequest>,
) -> Result<Json<SendCodeResponse>, Error2> {
    let (verification_list, _) =
        EmailVerification::find_by_email(&dynamo.client, &req.email, Default::default()).await?;

    if verification_list.is_empty() {
        return Err(Error2::NotFound(format!(
            "No verification found for email: {}",
            req.email
        )));
    }

    let email_verification = verification_list[0].clone();
    if email_verification.expired_at > get_now_timestamp() {
        return Err(Error2::BadRequest(
            "A verification code has already been sent. Please check your email.".to_string(),
        ));
    }

    let code = generate_random_code();
    let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;
    ses.send_mail(
        &req.email,
        "Please finish to sign up within 30 minutes with your verification code",
        format!("Verification code: {:?}", code).as_ref(),
    )
    .await?;

    let email_verification = EmailVerification::new(req.email.clone(), code, expired_at);
    email_verification.create(&dynamo.client).await?;
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
