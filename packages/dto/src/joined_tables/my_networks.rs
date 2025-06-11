use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/my-network", table = my_networks, action = [follow, unfollow])]
pub struct Mynetwork {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = users)]
    pub follower_id: i64,
    #[api_model(summary, many_to_one = users)]
    pub following_id: i64,
}