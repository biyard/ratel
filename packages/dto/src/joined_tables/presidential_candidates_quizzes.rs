use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/", table = presidential_candidates_quizzes, action = [], action_by_id = [delete, update])]
pub struct PresidentialCandidateQuiz {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = presidential_candidates)]
    pub presidential_candidate_id: i64,

    #[api_model(many_to_one = quizzes)]
    pub quiz_id: i64,
}
