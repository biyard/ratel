use bdk::prelude::*;
use validator::Validate;

use crate::PresidentialCandidate;

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

    #[api_model(many_to_many = presidential_candidates_quizzes, foreign_table_name = presidential_candidates, foreign_reference_key = quiz_id, foreign_primary_key = presidential_candidate_id)]
    pub presidential_candidates: Vec<PresidentialCandidate>,
}
