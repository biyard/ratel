use base64::{Engine, engine::general_purpose};
use serde::Serialize;

#[derive(Serialize)]
struct WebParams {
    pub command: TelegramCommand,
}
#[derive(Serialize)]
pub enum TelegramCommand {
    OpenSpacePage {
        space_pk: String,
        feature: Option<String>,
    },
}

pub fn generate_link(bot_name: &str, command: TelegramCommand) -> String {
    let base_url = format!("https://t.me/{}", bot_name);
    let params = WebParams { command };
    let encoded_params = general_purpose::STANDARD.encode(serde_json::to_string(&params).unwrap());
    format!("{}/app?startapp={}", base_url, encoded_params)
}
