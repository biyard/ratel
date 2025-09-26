use bdk::prelude::*;

#[api_model(table = telegram_channels)]
pub struct TelegramChannel {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique)]
    pub chat_id: i64,

    pub lang: Option<String>,
}
