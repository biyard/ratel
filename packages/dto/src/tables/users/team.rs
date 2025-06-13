use crate::GroupUser;
use crate::TeamUser;
use bdk::prelude::*;

use super::*;
// use crate::GroupUserRepositoryQueryBuilder;
use crate::TeamUserRepositoryQueryBuilder;

#[derive(validator::Validate)]
#[api_model(base = "/v1/teams", table = users, action = [], action_by_id = [delete, invite_member(email = String)], read_action = get_by_id)]
pub struct Team {
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

    pub parent_id: i64,
    #[api_model(action = create, read_action = get_by_username, action_by_id = [update_team_name])]
    pub username: String,

    #[api_model(many_to_many = team_members, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = team_id, nested)]
    #[serde(default)]
    pub members: Vec<TeamUser>,

    //FIXME: fix to query by api_model using nested keyword
    // #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id, nested)]
    #[api_model(skip)]
    #[serde(default)]
    pub groups: Vec<GroupUser>,

    #[api_model(action = create)]
    #[serde(default)]
    pub html_contents: String,
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
            nickname: user.nickname,
            html_contents: user.html_contents,

            ..Default::default()
        }
    }
}
