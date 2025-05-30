use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/spaces", table = space_likes)]
pub struct SpaceLike {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = users)]
    pub user_id: i64,
    #[api_model(summary, many_to_one = spaces)]
    pub space_id: i64,
}
