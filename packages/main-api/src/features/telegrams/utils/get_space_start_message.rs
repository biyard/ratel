use base64::{Engine, engine::general_purpose};
use serde::Serialize;

use crate::{
    types::{Partition, SpaceType},
    utils::telegram::TelegramButton,
};

#[derive(Serialize)]
struct WebParams {
    pub command: TelegramCommand,
}
#[derive(Serialize)]
pub enum TelegramCommand {
    OpenSpacePage { space_pk: String, r#type: SpaceType },
}

pub fn generate_link(bot_name: &str, command: TelegramCommand) -> String {
    let base_url = format!("https://t.me/{}", bot_name);
    let params = WebParams { command };
    let encoded_params = general_purpose::STANDARD.encode(serde_json::to_string(&params).unwrap());
    format!("{}/app?startapp={}", base_url, encoded_params)
}

pub fn get_space_created_message(
    bot_name: &str,
    space_pk: &Partition,
    r#type: SpaceType,
    title: &str,
) -> (String, TelegramButton) {
    let link = generate_link(
        bot_name,
        TelegramCommand::OpenSpacePage {
            space_pk: space_pk.to_string(),
            r#type,
        },
    );
    let content = format!(
        "A new space \"{}\" has been created! Click the button below to view it.",
        title
    );
    let button = TelegramButton {
        text: "View Space".into(),
        link,
    };
    (content, button)
}
