mod config;

use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, MessageEntityKind, MessageKind,
        MessageNewChatMembers,
    },
};
use tracing_subscriber::EnvFilter;

// TODO: REMOVE THIS COMMENT

#[tokio::main]
async fn main() {
    let conf = config::get();
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from(conf.log_level))
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .try_init();

    tracing::debug!("Configuration: {:?}", conf.env);
    tracing::debug!("Starting throw dice bot...");

    let bot = Bot::new(conf.telegram_token);

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        tracing::debug!("Received message: {:?}", msg);
        let me = bot.get_me().await?;
        let bot_username = me.user.username.unwrap();

        let chat_id = msg.chat.id;

        match &msg.kind {
            MessageKind::NewChatMembers(MessageNewChatMembers { new_chat_members }) => {
                for user in new_chat_members {
                    if !user.is_bot {
                        bot.send_message(chat_id, format!("Welcome, {}!", user.first_name))
                            .await?;
                    }
                }
            }

            _ => {
                tracing::debug!("Received non-text message");
            }
        }

        if let Some(entities) = msg.entities() {
            for entity in entities {
                if let MessageEntityKind::Mention = entity.kind {
                    let mention_text =
                        &msg.text().unwrap()[entity.offset..(entity.offset + entity.length)];
                    if mention_text == format!("@{}", bot_username) {
                        tracing::debug!("Bot was mentioned in the message");
                        let url = "https://t.me/crypto_ratel_bot/spaces?startapp"
                            .parse()
                            .unwrap();
                        tracing::debug!("Sending web app link to the user {:?}", url);

                        let keyboard = InlineKeyboardMarkup::new([[InlineKeyboardButton::url(
                            "🧩 미니앱 실행".to_string(),
                            url,
                        )]]);

                        bot.send_message(chat_id, "여기를 눌러 미니앱을 실행하세요 🧩")
                            .reply_markup(keyboard)
                            .await?;

                        break;
                    }
                }
            }
        }

        Ok(())
    })
    .await;
}
