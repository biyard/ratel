use std::time::SystemTime;

use crate::Error;
use bdk::prelude::*;

use by_axum::axum::{
    Extension,
    extract::{Json, State},
};
use dto::{
    Result, UserV2, UserV2RepositoryUpdateRequest, Verification, by_axum::auth::Authorization,
    sqlx::PgPool,
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct ResetPasswordRequest {
    pub email: String,
    pub password: String,
    pub code: String,
}

pub async fn reset_password_handler(
    State(pool): State<PgPool>,
    Extension(_auth): Extension<Option<Authorization>>,
    Json(req): Json<ResetPasswordRequest>,
) -> Result<Json<UserV2>> {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let email = req.email;
    let code = req.code;
    let password = req.password;

    let _ = Verification::query_builder()
        .email_equals(email.clone())
        .value_equals(code)
        .expired_at_greater_than(now)
        .query()
        .map(Verification::from)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Verification Error: {:?}", e);
            Error::InvalidVerificationCode
        })?;

    let user = UserV2::query_builder()
        .email_equals(email.clone())
        .query()
        .map(UserV2::from)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to find user by email: {:?}", e);
            Error::NotFound
        })?;

    let user = UserV2::get_repository(pool.clone())
        .update(
            user.id,
            UserV2RepositoryUpdateRequest::new().with_password(password.clone()),
        )
        .await?;

    Ok(Json(user))
}
