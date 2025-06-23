use by_types::QueryResponse;

use bdk::prelude::*;

use super::User;

#[derive(validator::Validate)]
#[api_model(base = "/v1/suggested-users", read_action = user_info, table = users, iter_type=QueryResponse)]
pub struct SuggestedUser {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = [signup, email_signup], action_by_id = edit_profile)]
    pub nickname: String,

    #[api_model(action = [signup, email_signup], nullable, action_by_id = edit_profile)]
    #[validate(url)]
    pub profile_url: String,

    #[api_model(action = [signup, email_signup], version = v0.1, indexed, unique)]
    #[serde(default)]
    pub username: String,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = follower_id, foreign_reference_key = following_id, aggregator = count)]
    #[serde(default)]
    pub followers_count: i64,

    #[api_model(many_to_many = my_networks, foreign_table_name = users, foreign_primary_key = following_id, foreign_reference_key = follower_id, aggregator = count)]
    #[serde(default)]
    pub followings_count: i64,

    // profile contents
    #[api_model(version = v0.2, action_by_id = edit_profile)]
    #[serde(default)]
    pub html_contents: String,
}

impl From<User> for SuggestedUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            created_at: user.created_at,
            updated_at: user.updated_at,

            nickname: user.nickname,
            profile_url: user.profile_url,
            username: user.username,
            followers_count: user.followers_count,
            followings_count: user.followings_count,
            html_contents: user.html_contents,

            ..Default::default()
        }
    }
}
