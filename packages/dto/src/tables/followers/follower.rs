use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/followers", table = followers, action_by_id = [update, delete])]
pub struct Follower {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub profile_image: String,
    #[api_model(summary)]
    pub title: String,
    #[api_model(summary, nullable)]
    pub description: Option<String>,
    #[api_model(summary)]
    pub followed: bool,

    #[api_model(many_to_one = users, action = [follow])]
    pub user_id: i64,
}
