use dto::{TelegramChannel, sqlx::PgPool};
use teloxide::{prelude::*, types::ChatMemberUpdated, utils::command::BotCommands};

use super::Command;

pub async fn message_handler(
    bot: Bot,
    msg: Message,
    _: PgPool,
) -> std::result::Result<(), teloxide::RequestError> {
    tracing::debug!("Received message: {:?}", msg);

    let me = bot.get_me().await?;
    let bot_username = me.user.username.as_deref().unwrap_or_default();
    let chat_id = msg.chat.id;
    let lang = msg.from.clone().and_then(|user| user.language_code);

    if let Some(text) = msg.text() {
        match Command::parse(text, bot_username) {
            Ok(command) => {
                match command {
                    Command::Help => {
                        let help_text = match lang.as_deref() {
                            Some("ko") => "도움말: /help - 도움말을 표시합니다.\n",
                            _ => "Help: /help - Show this help message.\n",
                        };
                        bot.send_message(chat_id, help_text).await?;
                    }
                }
                return Ok(());
            }
            Err(_) => {
                tracing::debug!("Message is not a command.");
            }
        }
    }

    Ok(())
}

pub async fn member_update_handler(
    bot: Bot,
    update: ChatMemberUpdated,
    pool: PgPool,
) -> std::result::Result<(), teloxide::RequestError> {
    let chat_id = update.chat.id.0;
    let old_status = &update.old_chat_member.status();
    let new_status = &update.new_chat_member.status();

    if update.new_chat_member.user.id == bot.get_me().await?.id {
        let repo = TelegramChannel::get_repository(pool.clone());

        if !old_status.is_administrator() && new_status.is_administrator() {
            tracing::info!("Bot added as admin to channel: {}", chat_id);
            let _ = repo.insert(chat_id, None).await.map_err(|e| {
                tracing::error!("Failed to insert channel {}: {}", chat_id, e);
                teloxide::RequestError::Api(teloxide::ApiError::BotBlocked)
            })?;
        } else if new_status.is_left() || new_status.is_banned() {
            tracing::info!("Bot left channel: {}", chat_id);
            let channel = TelegramChannel::query_builder()
                .chat_id_equals(chat_id)
                .query()
                .map(TelegramChannel::from)
                .fetch_one(&pool)
                .await
                .map_err(|e| {
                    tracing::error!("Failed to find channel {}: {}", chat_id, e);
                    teloxide::RequestError::Api(teloxide::ApiError::ChatNotFound)
                })?;
            repo.delete(channel.id).await.map_err(|e| {
                tracing::error!("Failed to delete channel {}: {}", chat_id, e);
                teloxide::RequestError::Api(teloxide::ApiError::ChatNotFound)
            })?;
        };
    }

    Ok(())
}
