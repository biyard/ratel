use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = team_members, action = [], action_by_id = [delete, update])]
pub struct TeamMember {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = users)]
    pub team_id: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,
}
