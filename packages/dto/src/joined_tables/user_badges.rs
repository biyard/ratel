use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = user_badges, action = [], action_by_id = [delete, update])]
pub struct UserBadge {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(many_to_one = badges)]
    pub badge_id: i64,
}
