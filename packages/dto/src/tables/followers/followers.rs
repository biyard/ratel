use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/followers", table = followers, action = [], action_by_id = [delete])]
pub struct Follower {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, 
        many_to_many = followers_users, 
        foreign_table_name = users, 
        foreign_primary_key = user_id, 
        foreign_reference_key = follower_id, 
        aggregator = count, 
        action = [follow])]
    pub follower_id: i64,

    #[api_model(summary, 
        many_to_many = followers_users, 
        foreign_table_name = users, 
        foreign_primary_key = user_id, 
        foreign_reference_key = followed_id, 
        aggregator = count, 
        action = [follow])]
    pub followed_id: i64,
}