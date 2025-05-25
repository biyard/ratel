use bdk::prelude::*;
use validator::Validate;

use crate::{ElectionPledge, Party};

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

    #[api_model(summary, version = v0.1)]
    pub like_option: String,

    #[api_model(summary, version = v0.1)]
    pub dislike_option: String,

    #[api_model(version = v0.1, type = INTEGER)]
    pub like_party: Party,

    #[api_model(many_to_many = election_pledges_quizzes_likes, foreign_table_name = election_pledges, foreign_reference_key = quiz_id, foreign_primary_key = election_pledge_id)]
    pub like_election_pledges: Vec<ElectionPledge>,

    #[api_model(many_to_many = election_pledges_quizzes_dislikes, foreign_table_name = election_pledges, foreign_reference_key = quiz_id, foreign_primary_key = election_pledge_id)]
    pub dislike_election_pledges: Vec<ElectionPledge>,
}
