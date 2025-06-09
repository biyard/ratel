use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = redeem_codes, action = [], action_by_id = [delete, update])]
pub struct RedeemCode {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,
    #[api_model(indexed)]
    pub meta_id: i64, // space_id, team_id or etc.

    #[api_model(type = JSONB)]
    pub codes: Vec<String>,

    #[api_model(type = JSONB)]
    pub used: Vec<i32>,
}
