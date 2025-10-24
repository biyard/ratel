use teloxide::{prelude::*, types::BotCommand, utils::command::BotCommands};

use crate::features::telegrams::TelegramChannel;

#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "Commands for Telegram Bot")]
pub enum Command {
    #[command(description = "Show help information.")]
    Help,
}

pub async fn set_command(bot: Bot) {
    let command_ko = vec![BotCommand::new("help", "도움말")];
    let command_en = vec![BotCommand::new("help", "Help")];

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

pub async fn message_handler(bot: Bot, msg: Message) -> crate::Result<()> {
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

pub async fn chat_member_update_handler(
    cli: aws_sdk_dynamodb::Client,
    bot: Bot,
    update: ChatMemberUpdated,
) -> crate::Result<()> {
    let chat_id = update.chat.id.0;
    let old_status = &update.old_chat_member.status();
    let new_status = &update.new_chat_member.status();

    if update.new_chat_member.user.id == bot.get_me().await?.id {
        if !old_status.is_administrator() && new_status.is_administrator() {
            tracing::info!("Bot added as admin to channel: {}", chat_id);
            TelegramChannel::add_channel(&cli, chat_id, None).await?;
        } else if new_status.is_left() || new_status.is_banned() {
            tracing::info!("Bot left channel: {}", chat_id);
            TelegramChannel::remove_channel(&cli, chat_id).await?;
        };
    }

    Ok(())
}
