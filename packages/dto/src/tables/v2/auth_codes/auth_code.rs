use bdk::prelude::*;

#[api_model(table = auth_codes)]
pub struct AuthCode {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    pub code: String,
    #[api_model(many_to_one = auth_clients)]
    pub client_id: String,
    #[api_model(many_to_one = users)]
    pub user_id: i64,
    pub expires_at: i64,
}
