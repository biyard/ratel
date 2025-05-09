use super::*;
use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(base = "/v1/teams", table = users, action = [], action_by_id = [delete])]
pub struct Team {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = create, action_by_id = [update_profile_image])]
    #[validate(url)]
    pub profile_url: String,

    pub parent_id: i64,
    #[api_model(action = create, action_by_id = [update_team_name])]
    pub username: String,
}

impl From<User> for Team {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            created_at: user.created_at,
            updated_at: user.updated_at,
            profile_url: user.profile_url,
            parent_id: user.parent_id.expect("Team must have parent_id"),
            username: user.username,
        }
    }
}
