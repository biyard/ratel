use bdk::prelude::*;
use serde::{Deserialize, Serialize};

#[api_model(base = "/", table = telegram_subscribes)]
pub struct TelegramSubscribe {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = create)]
    pub chat_id: i64,

    #[api_model(action = create, nullable, version = v0.1)]
    pub lang: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, schemars::JsonSchema, aide::OperationIo)]
#[serde(rename_all = "snake_case")]
pub enum TelegramNotificationPayload {
    SprintLeague(SprintLeaguePayload),
}

#[derive(Deserialize, Debug, Serialize, schemars::JsonSchema, aide::OperationIo)]
pub struct SprintLeaguePayload {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub started_at: i64,
    pub ended_at: i64,
    pub player_names: Vec<String>,
}
