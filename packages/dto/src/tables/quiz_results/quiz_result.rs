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

    #[api_model(type = JSONB, action = [answer], version = v0.1)]
    pub answers: Vec<QuizAnswer>,
}

#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SupportPolicy {
    pub presidential_candidate_id: i64,
    pub candidate_name: String,
    pub support: i64,
    pub percent: f64,
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
        let candidates = self.percentage_of_each_candidate();

        candidates.get(0).map(|x| x.0).unwrap_or(0)
    }

    pub fn percentage_of_each_candidate(&self) -> Vec<(i64, String, f64)> {
        let mut percentages = vec![];

        for result in &self.results {
            percentages.push((
                result.presidential_candidate_id,
                result.candidate_name.clone(),
                result.percent,
            ));
        }

        percentages.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        percentages
    }
}
