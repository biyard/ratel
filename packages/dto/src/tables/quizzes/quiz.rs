use bdk::prelude::*;
use validator::Validate;

use crate::ElectionPledge;

#[derive(Validate)]
#[api_model(base = "/v1/quizzes", table = quizzes)]
pub struct Quiz {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary)]
    pub policy: String,

    #[api_model(many_to_many = election_pledges_quizzes, foreign_table_name = election_pledges, foreign_reference_key = quiz_id, foreign_primary_key = election_pledge_id)]
    pub election_pledges: Vec<ElectionPledge>,
}
