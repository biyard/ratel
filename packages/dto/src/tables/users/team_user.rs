use crate::Group;
use crate::GroupRepositoryQueryBuilder;
pub use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(table = users)]
pub struct TeamUser {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = signup, action_by_id = edit_profile)]
    pub nickname: String,
    #[api_model(action = signup, read_action = [check_email, login, find_by_email], unique)]
    #[validate(email)]
    pub email: String,
    #[api_model(action = signup, nullable, action_by_id = edit_profile)]
    #[validate(url)]
    pub profile_url: String,

    #[api_model(action = signup, version = v0.1, indexed, unique)]
    #[serde(default)]
    pub username: String,

    #[api_model(many_to_many = group_members, foreign_table_name = groups, foreign_primary_key = group_id, foreign_reference_key = user_id, nested)]
    #[serde(default)]
    pub groups: Vec<Group>,
}
