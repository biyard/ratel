use base64::{Engine, engine::general_purpose};
use crate::Result;
use serde::Serialize;
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};

#[derive(Clone)]
pub struct TelegramBot {
    pub bot: Bot,
    pub bot_name: String,
    pub bot_id: u64,
}

#[derive(Serialize)]
struct WebParams {
    pub command: TelegramWebCommand,
}
#[derive(Serialize)]
pub enum TelegramWebCommand {
    OpenSpacePage { space_id: i64 },
}

#[derive(Serialize)]
pub struct TelegramButton {
    pub text: String,
    pub command: TelegramWebCommand,
}

impl TelegramBot {
    pub async fn new(token: &str) -> Result<Self> {
        let bot = Bot::new(token);
        let me = bot.get_me().await?;

        Ok(TelegramBot {
            bot,
            bot_name: me.user.username.unwrap_or_default(),
            bot_id: me.user.id.0,
        })
    }

    pub async fn send_message(
        &self,
        chat_ids: Vec<i64>,
        content: &str,
        button: Option<TelegramButton>,
    ) -> Result<()> {
        let keyboard: Option<_> = if let Some(button) = button {
            let url = self.generate_link(button.command);
            Some(InlineKeyboardMarkup::new(vec![vec![
                InlineKeyboardButton::url(button.text, url.parse().unwrap()),
            ]]))
        } else {
            None
        };

        let mut succeeded = Vec::new();
        let mut failed = Vec::new();
        for chat_id in chat_ids {
            let chat_id = ChatId(chat_id);
            let msg = self.bot.send_message(chat_id, content);
            let msg = if let Some(ref kb) = keyboard {
                msg.reply_markup(kb.clone())
            } else {
                msg
            };
            match msg.parse_mode(ParseMode::Html).await {
                Ok(_) => {
                    tracing::debug!("Successfully sent message to {}", chat_id);
                    succeeded.push(chat_id);
                }
                Err(e) => {
                    tracing::error!("Failed to send message to {}: {}", chat_id, e);
                    failed.push(chat_id);
                }
            }
        }
        tracing::info!(
            "Message sent. Succeeded: {}, Failed: {}",
            succeeded.len(),
            failed.len()
        );
        Ok(())
    }

    fn generate_link(&self, command: TelegramWebCommand) -> String {
        let base_url = format!("https://t.me/{}", self.bot_name);
        let params = WebParams { command };
        let encoded_params =
            general_purpose::STANDARD.encode(serde_json::to_string(&params).unwrap());
        format!("{}/app?startapp={}", base_url, encoded_params)
    }
}
