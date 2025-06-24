use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/surveys/:survey-id/responses", table = survey_responses)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(summary, version = v0.1)]
    pub user_id: i64,

    #[api_model(summary, action = respond_answer, type = JSONB)]
    pub answers: Vec<Answer>,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Answer {
    SingleChoice { answer: i32 },
    MultipleChoice { answer: Vec<i32> },
    ShortAnswer { answer: String },
    Subjective { answer: String },
}
