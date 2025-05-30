use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = space_members)]
pub struct SpaceMember {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update], version = v0.1)]
    pub updated_at: i64,

    #[api_model(many_to_one = users, action = create)]
    #[serde(default)]
    pub user_id: i64,
    #[api_model(many_to_one = spaces)]
    pub space_id: i64,
}
