use teloxide::{
    Bot, payloads::SetMyCommandsSetters, prelude::Requester, types::BotCommand,
    utils::command::BotCommands,
};

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
