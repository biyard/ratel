use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/quizzes/results", table = quiz_results)]
pub struct QuizResult {
    #[api_model(primary_key)]
    pub id: i64,
    #[api_model(auto = [insert])]
    pub created_at: i64,
    #[api_model(auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(read_action = [get_result], unique)]
    pub principal: String,

    #[api_model(type = JSONB)]
    pub results: Vec<SupportPolicy>,

    #[api_model(type = JSONB, action = [answer])]
    pub answers: Vec<QuizAnswer>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SupportPolicy {
    pub presidential_candidate_id: i64,
    pub candidate_name: String,
    pub support: i64,
    pub against: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct QuizAnswer {
    pub quiz_id: i64,
    pub answer: QuizOptions,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum QuizOptions {
    #[default]
    Like = 1,
    Dislike = 2,
}

impl QuizResult {
    pub fn most_supportive_candidate(&self) -> i64 {
        let mut max = 0;
        let mut candidate_id = 0;

        for result in &self.results {
            if result.support > max {
                max = result.support;
                candidate_id = result.presidential_candidate_id;
            }
        }

        candidate_id
    }
}
