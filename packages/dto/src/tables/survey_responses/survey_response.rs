use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v2/surveys/:survey-id/responses", action_by_id = remove_respond_answer, table = survey_responses)]
pub struct SurveyResponse {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,
    #[api_model(summary, many_to_one = users, version = v0.1)]
    pub user_id: i64,

    #[api_model(summary, action = respond_answer, action_by_id = update_respond_answer, type = JSONB)]
    pub answers: Vec<Answer>,

    #[api_model(many_to_one = surveys)]
    pub survey_id: i64,

    #[api_model(summary, action = [respond_answer], type = INTEGER)]
    pub survey_type: SurveyType,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum SurveyType {
    #[default]
    #[translate(ko = "표본 조사", en = "Sample Survey")]
    Sample = 1,
    #[translate(ko = "최종 설문", en = "Final Survey")]
    Survey = 2,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Answer {
    SingleChoice { answer: Option<i32> },
    MultipleChoice { answer: Option<Vec<i32>> },
    ShortAnswer { answer: Option<String> },
    Subjective { answer: Option<String> },
    Checkbox { answer: Option<Vec<i32>> },
    Dropdown { answer: Option<i32> },
    LinearScale { answer: Option<i32> },
}
