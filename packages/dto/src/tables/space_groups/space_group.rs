use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/groups", table = space_groups, action = [], action_by_id = [delete, update])]
pub struct SpaceGroup {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,

    pub name: String,
}
