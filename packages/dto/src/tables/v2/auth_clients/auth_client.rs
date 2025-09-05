use bdk::prelude::*;

#[api_model(table = auth_clients)]
pub struct AuthClient {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    pub client_id: String,

    pub client_secret: String,

    #[api_model(type = JSONB)]
    pub redirect_uris: Vec<String>,

    #[api_model(type = JSONB)]
    pub scopes: Vec<String>,
}
