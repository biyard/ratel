use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/promotions", table = promotions, action = [], action_by_id = [delete])]
pub struct Promotion {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, nullable, action_by_id = [update], action = [write_promotion])]
    pub title: String,

    #[api_model(summary, action = [], action_by_id = [update], action = [write_promotion])]
    pub html_contents: String,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
    
    // considering using float later on
    #[api_model(summary, nullable, action = [write_promotion])]
    pub price_payed: i64,

    #[api_model(summary, default = false, action = [write_promotion])]
    pub accepted: bool,
}