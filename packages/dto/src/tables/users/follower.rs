use crate::UserType;
use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(base = "/v1/followers", table = users)]
pub struct Follower {
    #[api_model(primary_key, read_action = get_by_id)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    pub user_type: UserType,
    #[api_model(action = create)]
    pub nickname: String,
    #[api_model(action = create, action_by_id = [update_profile_image])]
    #[validate(url)]
    pub profile_url: String,
    #[api_model(read_action = [check_email, login, find_by_email], unique)]
    pub email: String,
    #[api_model(action = create, read_action = get_by_username, action_by_id = [update_team_name])]
    pub username: String,
    #[api_model(action = create)]
    #[serde(default)]
    pub html_contents: String,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = follower_id, foreign_reference_key = following_id, aggregator = count)]
    #[serde(default)]
    pub followers_count: i64,
    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = following_id, foreign_reference_key = follower_id, aggregator = count)]
    #[serde(default)]
    pub followings_count: i64,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = follower_id, foreign_reference_key = following_id)]
    #[serde(default)]
    pub followers: Vec<Follower>,
    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = following_id, foreign_reference_key = follower_id)]
    #[serde(default)]
    pub followings: Vec<Follower>,
}
