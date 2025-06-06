use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/followers", table = followers, action_by_id = [follow, unfollow])]
pub struct Follower {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = users)]
    pub follower_id: i64,  // The user who follows

    #[api_model(summary)]
    pub follower_nickname: String,
    #[api_model(summary, nullable)]
    pub follower_profile_url: String,
    #[api_model(summary, nullable)]
    pub follower_profile_image: Option<String>,
    #[api_model(summary, nullable)]
    pub follower_description: Option<String>,


    #[api_model(summary, many_to_one = users)]
    pub following_id: i64,  // The user being followed (alternative to followee_id)

    #[api_model(summary)]
    pub following_nickname: String,
    #[api_model(summary, nullable)]
    pub following_profile_url: String,
    #[api_model(summary, nullable)]
    pub following_profile_image: Option<String>,
    #[api_model(summary, nullable)]
    pub following_description: Option<String>,
}