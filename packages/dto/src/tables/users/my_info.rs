use bdk::prelude::*;

use crate::Group;

use super::Team;

#[derive(validator::Validate)]
#[api_model(base = "/v1/me/info", read_action = my_info, table = users)]
pub struct MyInfo {
    pub id: i64,
    pub created_at: i64,
    pub updated_at: i64,

    pub nickname: String,
    pub principal: String,
    #[validate(email)]
    pub email: String,
    #[validate(url)]
    pub profile_url: String,

    #[api_model(many_to_many = team_members, foreign_table_name = users, foreign_primary_key = team_id, foreign_reference_key = user_id)]
    pub teams: Vec<Team>,

    #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id)]
    #[serde(default)]
    pub groups: Vec<Group>,
    #[api_model(version = v0.2)]
    #[serde(default)]
    pub html_contents: String,
}
