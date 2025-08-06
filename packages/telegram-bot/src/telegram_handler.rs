use dto::{TelegramSubscribe, sqlx::PgPool};
use teloxide::{
    prelude::*,
    types::{BotCommand, InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

use crate::generate_link;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Commands for Telegram Bot")]
enum Command {
    #[command(description = "Show help information.")]
    Help,
    #[command(description = "Subscribe notifications.")]
    Subscribe,
    #[command(description = "Unsubscribe from notifications.")]
    Unsubscribe,
}

pub async fn set_command(bot: Bot) {
    let command_ko = vec![
        BotCommand::new("help", "도움말"),
        BotCommand::new("subscribe", "구독"),
        BotCommand::new("unsubscribe", "구독 취소"),
    ];
    let command_en = vec![
        BotCommand::new("help", "Help"),
        BotCommand::new("subscribe", "Subscribe"),
        BotCommand::new("unsubscribe", "Unsubscribe"),
    ];

    bot.set_my_commands(command_ko)
        .language_code("ko")
        .await
        .expect("Failed to set commands in Korean");

    bot.set_my_commands(command_en.clone())
        .language_code("en")
        .await
        .expect("Failed to set commands in English");

    bot.set_my_commands(command_en)
        .await
        .expect("Failed to set commands in Default");
}

pub async fn telegram_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
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
                            Some("ko") => {
                                "도움말: /help - 도움말을 표시합니다.\n/subcribe - 알림을 구독합니다.\n/unsubscribe - 알림 구독을 취소합니다."
                            }
                            _ => {
                                "Help: /help - Show this help message.\n/subscribe - Subscribe to updates.\n/unsubscribe - Unsubscribe from updates."
                            }
                        };
                        bot.send_message(chat_id, help_text).await?;
                    }
                    Command::Subscribe => {
                        let link = generate_link(crate::TgWebCommand::Subscribe {
                            chat_id: chat_id.0,
                            lang: lang.clone(),
                        });

                        match lang.as_deref() {
                            Some("ko") => {
                                let keyboard = InlineKeyboardMarkup::new(vec![vec![
                                    InlineKeyboardButton::url(
                                        "🔗 알림 신청하기!".to_string(),
                                        link.parse().unwrap(),
                                    ),
                                ]]);
                                bot.send_message(
                                    chat_id,
                                    "알림 신청을 위해 아래 링크를 클릭하세요:",
                                )
                                .reply_markup(keyboard)
                                .await?;
                            }
                            _ => {
                                let keyboard = InlineKeyboardMarkup::new(vec![vec![
                                    InlineKeyboardButton::url(
                                        "🔗 Subscribe Now!".to_string(),
                                        link.parse().unwrap(),
                                    ),
                                ]]);
                                bot.send_message(chat_id, "Click the link below to subscribe:")
                                    .reply_markup(keyboard)
                                    .await?;
                            }
                        }
                    }
                    Command::Unsubscribe => {
                        let mut message = match lang.as_deref() {
                            Some("ko") => "알림 신청이 취소 되었습니다.",
                            _ => "Unsubscription processed.",
                        };

                        let sub = TelegramSubscribe::query_builder()
                            .chat_id_equals(chat_id.0)
                            .query()
                            .map(TelegramSubscribe::from)
                            .fetch_optional(&pool)
                            .await
                            .ok()
                            .flatten();
                        if sub.is_none() {
                            match lang.as_deref() {
                                Some("ko") => {
                                    message = "구독 중이 아닙니다.";
                                }
                                _ => {
                                    message = "You are not subscribed.";
                                }
                            }
                        } else {
                            let res = TelegramSubscribe::get_repository(pool.clone())
                                .delete(sub.unwrap().id)
                                .await;
                            if let Err(e) = res {
                                tracing::error!("Failed to unsubscribe: {}", e);
                            }
                        }
                        bot.send_message(chat_id, message).await?;
                    }
                }
                return Ok(());
            }
            Err(_) => {
                tracing::debug!("Message is not a command.");
            }
        }
    }

    // if let Some(entities) = msg.entities() {
    //     for entity in entities {
    //         if let MessageEntityKind::Mention = entity.kind {
    //             let text = msg.text().unwrap_or_default();
    //             let mention_text = text
    //                 .chars()
    //                 .skip(entity.offset)
    //                 .take(entity.length)
    //                 .collect::<String>();

    //             if mention_text == format!("@{}", bot_username) {
    //                 tracing::debug!("Bot was mentioned in the message");
    //                 let url = format!("{}", conf.telegram_mini_app_uri);
    //                 let url = url.parse().unwrap();
    //                 let text = match lang.as_deref() {
    //                     Some("ko") => ("🧩 미니앱 실행", "여기를 눌러 미니앱을 실행하세요 🧩"),
    //                     _ => ("🧩 Start Mini App", "Click here to run the mini app 🧩"),
    //                 };
    //                 let keyboard = InlineKeyboardMarkup::new([[InlineKeyboardButton::url(
    //                     text.0.to_string(),
    //                     url,
    //                 )]]);
    //                 bot.send_message(chat_id, text.1)
    //                     .reply_markup(keyboard)
    //                     .await?;
    //                 break;
    //             }
    //         }
    //     }
    // }

    Ok(())
}
