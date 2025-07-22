use dto::{TelegramSubscribe, sqlx::PgPool};
use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageEntityKind},
};

use crate::config;

// pub  async fn set_command(bot: Bot) {
//     let command_ko = vec![
//         BotCommand::new("help", "도움말"),
//         BotCommand::new("subscribe", "구독"),
//         BotCommand::new("unsubscribe", "구독 취소"),
//     ];
//     let command_en = vec![
//         BotCommand::new("help", "Help"),
//         BotCommand::new("subscribe", "Subscribe"),
//         BotCommand::new("unsubscribe", "Unsubscribe"),
//     ];

//     bot.set_my_commands(command_ko)
//         .language_code("ko")
//         .await
//         .expect("Failed to set commands in Korean");

//     bot.set_my_commands(command_en.clone())
//         .language_code("en")
//         .await
//         .expect("Failed to set commands in English");

//     bot.set_my_commands(command_en)
//         .await
//         .expect("Failed to set commands in Default");
// }

pub async fn telegram_handler(
    bot: Bot,
    msg: Message,
    pool: PgPool,
) -> std::result::Result<(), teloxide::RequestError> {
    tracing::debug!("Received message: {:?}", msg);

    let conf = config::get();

    let me = bot.get_me().await?;
    let bot_username = me.user.username.as_deref().unwrap_or_default();
    let chat_id = msg.chat.id;
    let lang = msg.from.clone().and_then(|user| user.language_code);

    if let Some(_) = msg.new_chat_members() {
        let res = TelegramSubscribe::get_repository(pool.clone())
            .insert(chat_id.0, lang.clone())
            .await;
        match res {
            Ok(_) => {
                tracing::debug!("Subscription successful for chat_id: {}", chat_id.0)
            }
            Err(e) => {
                tracing::error!("Failed to subscribe: {}", e);
            }
        }
        match lang.as_deref() {
            Some("ko") => {
                bot.send_message(chat_id, "구독 처리되었습니다.").await?;
            }
            _ => {
                bot.send_message(chat_id, "Subscription processed.").await?;
            }
        }
    }
    if let Some(_) = msg.left_chat_member() {
        let sub = TelegramSubscribe::query_builder()
            .chat_id_equals(chat_id.0)
            .query()
            .map(TelegramSubscribe::from)
            .fetch_optional(&pool)
            .await
            .ok()
            .flatten();
        if let Some(sub) = sub {
            let res = TelegramSubscribe::get_repository(pool.clone())
                .delete(sub.id)
                .await;
            if let Err(e) = res {
                tracing::error!("Failed to unsubscribe: {}", e);
            }
        }
        match lang.as_deref() {
            Some("ko") => {
                bot.send_message(chat_id, "구독 취소 되었습니다.").await?;
            }
            _ => {
                bot.send_message(chat_id, "Unsubscription processed.")
                    .await?;
            }
        }
    }
    // if let Some(text) = msg.text() {
    //     match Command::parse(text, bot_username) {
    //         Ok(command) => {
    //             match command {
    //                 Command::Subscribe => {
    //                     let res = TelegramSubscribe::get_repository(pool.clone())
    //                         .insert(chat_id.0, lang.clone())
    //                         .await;
    //                     match res {
    //                         Ok(_) => {
    //                             tracing::debug!(
    //                                 "Subscription successful for chat_id: {}",
    //                                 chat_id.0
    //                             )
    //                         }
    //                         Err(e) => {
    //                             tracing::error!("Failed to subscribe: {}", e);
    //                         }
    //                     }
    //                     match lang.as_deref() {
    //                         Some("ko") => {
    //                             bot.send_message(chat_id, "구독 처리되었습니다.").await?;
    //                         }
    //                         _ => {
    //                             bot.send_message(chat_id, "Subscription processed.").await?;
    //                         }
    //                     }
    //                 }
    //                 Command::Unsubscribe => {
    //                     let mut message = match lang.as_deref() {
    //                         Some("ko") => "구독 취소 되었습니다.",
    //                         _ => "Unsubscription processed.",
    //                     };

    //                     let sub = TelegramSubscribe::query_builder()
    //                         .chat_id_equals(chat_id.0)
    //                         .query()
    //                         .map(TelegramSubscribe::from)
    //                         .fetch_optional(&pool)
    //                         .await
    //                         .ok()
    //                         .flatten();
    //                     if sub.is_none() {
    //                         match lang.as_deref() {
    //                             Some("ko") => {
    //                                 message = "구독 중이 아닙니다.";
    //                             }
    //                             _ => {
    //                                 message = "You are not subscribed.";
    //                             }
    //                         }
    //                     } else {
    //                         let res = TelegramSubscribe::get_repository(pool.clone())
    //                             .delete(sub.unwrap().id)
    //                             .await;
    //                         if let Err(e) = res {
    //                             tracing::error!("Failed to unsubscribe: {}", e);
    //                         }
    //                     }
    //                     bot.send_message(chat_id, message).await?;
    //                 }
    //             }
    //             return Ok(());
    //         }
    //         Err(_) => {
    //             tracing::debug!("Message is not a command.");
    //         }
    //     }
    // }

    if let Some(entities) = msg.entities() {
        for entity in entities {
            if let MessageEntityKind::Mention = entity.kind {
                let text = msg.text().unwrap_or_default();
                let mention_text = text
                    .chars()
                    .skip(entity.offset)
                    .take(entity.length)
                    .collect::<String>();

                if mention_text == format!("@{}", bot_username) {
                    tracing::debug!("Bot was mentioned in the message");
                    let url = format!("{}", conf.telegram_mini_app_uri);
                    let url = url.parse().unwrap();
                    let text = match lang.as_deref() {
                        Some("ko") => ("🧩 미니앱 실행", "여기를 눌러 미니앱을 실행하세요 🧩"),
                        _ => ("🧩 Start Mini App", "Click here to run the mini app 🧩"),
                    };
                    let keyboard = InlineKeyboardMarkup::new([[InlineKeyboardButton::url(
                        text.0.to_string(),
                        url,
                    )]]);
                    bot.send_message(chat_id, text.1)
                        .reply_markup(keyboard)
                        .await?;
                    break;
                }
            }
        }
    }

    Ok(())
}
