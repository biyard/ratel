use bdk::prelude::*;
use by_axum::axum::{Json, extract::State};
use dto::{
    Error, Result, TelegramToken, User,
    sqlx::{Pool, Postgres},
};
use uuid::Uuid;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerifyTelegramRawRequest {
    #[schemars(description = "Telegram Raw for verifying Telegram ownership")]
    pub telegram_raw: String,
}

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Default,
    aide::OperationIo,
    JsonSchema,
)]
pub struct VerifyTelegramRawResponse {
    #[schemars(description = "Token for verifying Telegram ownership")]
    pub token: String,
}

use crate::utils::telegram::parse_telegram_raw;

pub async fn verify_telegram_raw_handler(
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<VerifyTelegramRawRequest>,
) -> Result<Json<VerifyTelegramRawResponse>> {
    let telegram_user = parse_telegram_raw(req.telegram_raw)?;

    let res = User::query_builder()
        .telegram_id_equals(telegram_user.id)
        .query()
        .map(User::from)
        .fetch_optional(&pool)
        .await?;
    if res.is_some() {
        return Err(Error::DuplicatedTelegramUser);
    }

    let token = Uuid::new_v4().to_string();
    TelegramToken::get_repository(pool)
        .insert(
            token.clone(),
            telegram_user.id,
            telegram_user.username,
            None,
            None,
        )
        .await?;

    Ok(Json(VerifyTelegramRawResponse { token }))
}
