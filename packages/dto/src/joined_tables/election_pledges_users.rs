use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = election_pledges_users)]
pub struct ElectionPledgeLike {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = election_pledges)]
    pub election_pledge_id: i64,

    #[api_model(many_to_one = users)]
    pub user_id: i64,
}
