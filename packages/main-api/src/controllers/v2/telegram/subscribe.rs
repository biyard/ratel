use bdk::prelude::*;
use by_axum::axum::{Extension, Json, extract::State};
use dto::{
    Error, Result, TelegramSubscribe,
    by_axum::auth::Authorization,
    sqlx::{Pool, Postgres},
};

use teloxide::{Bot as TelegramBot, prelude::*};

use crate::{config, utils::users::extract_user};

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
pub struct TelegramSubscribeRequest {
    #[schemars(description = "Telegram chat ID")]
    pub chat_id: i64,

    #[schemars(description = "Optional language preference for the user")]
    pub lang: Option<String>,
}

pub async fn telegram_subscribe_handler(
    Extension(auth): Extension<Option<Authorization>>,
    State(pool): State<Pool<Postgres>>,
    Json(req): Json<TelegramSubscribeRequest>,
) -> Result<Json<()>> {
    let user = extract_user(&pool, auth).await?;
    tracing::debug!(
        "User {} is subscribing to Telegram notifications with chat_id: {}, lang: {:?}",
        user.id,
        req.chat_id,
        req.lang
    );
    if user.telegram_id.is_none() {
        return Err(Error::Unauthorized);
    }
    let repo = TelegramSubscribe::get_repository(pool.clone());
    let bot = TelegramBot::new(config::get().telegram_token);
    let mut msg = match req.lang.as_deref() {
        Some("ko") => "텔레그램 알림 신청 성공!".to_string(),
        _ => "Telegram Notification Subscription Successful".to_string(),
    };
    if let Err(e) = repo.insert(req.chat_id, user.id, req.lang.clone()).await {
        tracing::error!("Failed to subscribe user {} to Telegram: {}", user.id, e);
        msg = match req.lang.as_deref() {
            Some("ko") => "이미 구독 중입니다.".to_string(),
            _ => "You are already subscribed.".to_string(),
        };
    }
    telegram_message_sender(&bot, req.chat_id, msg).await?;
    Ok(Json(()))
}

pub async fn telegram_message_sender(bot: &TelegramBot, chat_id: i64, text: String) -> Result<()> {
    let chat_id = ChatId(chat_id);
    bot.send_message(chat_id, text).await.map_err(|e| {
        tracing::error!("Failed to send Telegram message: {}", e);
        Error::BadRequest
    })?;
    Ok(())
}
