use bdk::prelude::*;

#[api_model(base = "/", table = user_points)]
pub struct UserPoint {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(action = create)]
    pub amount: i64,
    #[api_model(action = create)]
    pub description: String,

    #[api_model(many_to_one = users, action = create)]
    #[serde(default)]
    pub user_id: i64,
}
