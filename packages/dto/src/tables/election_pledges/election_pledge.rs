use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/election-pledges", table = election_pledges, action_by_id = like)]
pub struct ElectionPledge {
    #[api_model(summary, primary_key, action = update)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, many_to_one = presidential_candidates)]
    pub presidential_candidate_id: i64,

    #[api_model(summary, action = update)]
    pub promise: String,

    #[api_model(summary, many_to_many = election_pledges_users, foreign_table_name = users, foreign_primary_key = user_id, foreign_reference_key = election_pledge_id, aggregator = count)]
    #[serde(default)]
    pub likes: i64,
}
