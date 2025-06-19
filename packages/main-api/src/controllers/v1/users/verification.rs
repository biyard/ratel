use bdk::prelude::*;
use rand::distr::Alphanumeric;

use std::time::SystemTime;

use aws_sdk_sesv2::types::Content;
use by_axum::{
    auth::Authorization,
    axum::{Extension, Json, extract::State, routing::post},
};
use dto::*;
use tracing::instrument;

use crate::utils::email::send_email;

#[derive(Clone, Debug)]
pub struct VerificationController {
    repo: VerificationRepository,
    pool: sqlx::Pool<sqlx::Postgres>,
    verification_expiration: i64,
}

impl VerificationController {
    pub fn route(pool: sqlx::Pool<sqlx::Postgres>) -> Result<by_axum::axum::Router> {
        let repo = Verification::get_repository(pool.clone());

        let ctrl = VerificationController {
            pool,
            repo,
            verification_expiration: 60 * 30, // 30 minutes
        };

        Ok(by_axum::axum::Router::new()
            .route("/", post(Self::act_verification))
            .with_state(ctrl.clone()))
    }

    pub async fn act_verification(
        State(ctrl): State<VerificationController>,
        Extension(_auth): Extension<Option<Authorization>>,
        Json(body): Json<VerificationAction>,
    ) -> Result<Json<Verification>> {
        tracing::debug!("act_verification {:?}", body);

        match body {
            VerificationAction::SendVerificationCode(params) => {
                ctrl.send_verification_email(params).await
            }
            VerificationAction::Verify(params) => ctrl.verify(params).await,
        }
    }
}

impl VerificationController {
    pub async fn verify(
        &self,
        VerificationVerifyRequest { email, value }: VerificationVerifyRequest,
    ) -> Result<Json<Verification>> {
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let code = Verification::query_builder()
            .email_equals(email)
            .value_equals(value)
            .expired_at_greater_than(now)
            .query()
            .map(Verification::from)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                tracing::error!("Verification Error: {:?}", e);
                Error::InvalidVerificationCode
            })?;

        Ok(Json(Verification {
            id: code.id,
            expired_at: code.expired_at,
            ..Verification::default()
        }))
    }

    #[instrument]
    pub async fn send_verification_email(
        &self,
        body: VerificationSendVerificationCodeRequest,
    ) -> Result<Json<Verification>> {
        use rand::{Rng, rng};

        tracing::debug!("body {:?}", body);

        let code: String = rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        send_email(
            body.email.clone(),
            Content::builder()
                .data("Please finish to sign up within 30 minutes with your verification code")
                .build()
                .unwrap(),
            Content::builder()
                .data(format!("Verification code: {:?}", code))
                .build()
                .unwrap(),
        )
        .await
        .map_err(|e| {
            tracing::error!("Email Send Error: {:?}", e);
            Error::SESServiceError(e.to_string())
        })?;

        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let result = self
            .repo
            .insert(body.email, code, now + self.verification_expiration, 0)
            .await?;

        Ok(Json(Verification {
            id: result.id,
            expired_at: result.expired_at,
            ..Verification::default()
        }))
    }
}
