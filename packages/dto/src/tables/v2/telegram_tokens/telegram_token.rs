use bdk::prelude::*;

#[api_model(table = telegram_tokens)]
pub struct TelegramToken {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(unique)]
    pub token: String,

    pub telegram_user_id: i64,
    pub username: Option<String>,
    pub profile_url: Option<String>,
    pub email: Option<String>,
}
