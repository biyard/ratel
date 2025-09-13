use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Error, Result, TelegramToken, User, UserRepositoryUpdateRequest,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

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
pub struct ConnectTelegramRequest {
    #[schemars(description = "Received Telegram Token")]
    pub token: String,
}

use crate::utils::users::extract_user;

pub async fn connect_telegram_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<ConnectTelegramRequest>,
) -> Result<()> {
    let user = extract_user(&pool, auth).await?;
    let telegram_info = TelegramToken::query_builder()
        .token_equals(req.token)
        .query()
        .map(TelegramToken::from)
        .fetch_optional(&pool)
        .await?;
    if telegram_info.is_none() {
        return Err(Error::InvalidTelegramData);
    }
    let telegram_info = telegram_info.unwrap();
    let telegram_id = telegram_info.telegram_user_id;
    let res = User::query_builder()
        .telegram_id_equals(telegram_id)
        .query()
        .map(User::from)
        .fetch_optional(&pool)
        .await?;
    if res.is_some() {
        return Err(Error::DuplicatedTelegramUser);
    }

    let _ = User::get_repository(pool)
        .update(
            user.id,
            UserRepositoryUpdateRequest::new().with_telegram_id(telegram_id),
        )
        .await?;

    Ok(())
}
