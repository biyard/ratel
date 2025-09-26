use bdk::prelude::*;
use by_axum::axum::{Json, extract::State};
use dto::{
    Error, Result, TelegramToken,
    by_axum::axum::extract::Query,
    sqlx::{Pool, Postgres},
};

#[derive(
    Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, aide::OperationIo, JsonSchema,
)]
pub struct GetTelegramInfoQuery {
    pub token: String,
}

pub async fn get_telegram_info_handler(
    State(pool): State<Pool<Postgres>>,
    Query(GetTelegramInfoQuery { token }): Query<GetTelegramInfoQuery>,
) -> Result<Json<TelegramToken>> {
    let telegram_user = TelegramToken::query_builder()
        .token_equals(token)
        .query()
        .map(TelegramToken::from)
        .fetch_optional(&pool)
        .await?;
    if telegram_user.is_none() {
        return Err(Error::InvalidTelegramData);
    }

    Ok(Json(telegram_user.unwrap()))
}
