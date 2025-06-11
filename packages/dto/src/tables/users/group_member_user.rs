use bdk::prelude::*;

#[derive(validator::Validate)]
#[api_model(base = "/v1/users", table = users)]
pub struct GroupMemberUser {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = insert)]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(action = create)]
    pub nickname: String,
    #[api_model(action = signup, version = v0.1, indexed, unique)]
    pub username: String,
    #[api_model(action = signup, nullable, action_by_id = edit_profile)]
    #[validate(url)]
    pub profile_url: String,
}
