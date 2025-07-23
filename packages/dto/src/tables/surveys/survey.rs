use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/spaces/:space-id/surveys", table = surveys)]
pub struct Survey {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    #[api_model(many_to_one = spaces)]
    pub space_id: i64,
    #[api_model(summary, type = INTEGER, action_by_id = update)]
    #[serde(default)]
    pub status: ProjectStatus,

    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub started_at: i64,
    #[api_model(summary, action = create, action_by_id = update)]
    #[serde(default)]
    pub ended_at: i64,

    #[api_model(summary, action = create, type = JSONB, version = v0.1, action_by_id = update)]
    #[serde(default)]
    pub questions: Vec<Question>,

    #[api_model(summary, one_to_many = survey_responses, foreign_key = survey_id, aggregator = count)]
    #[serde(default)]
    pub response_count: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Question {
    SingleChoice(ChoiceQuestion),
    MultipleChoice(ChoiceQuestion),
    ShortAnswer(SubjectiveQuestion),
    Subjective(SubjectiveQuestion),
    Checkbox(CheckboxQuestion),
    Dropdown(DropdownQuestion),
    LinearScale(LinearScaleQuestion),
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct LinearScaleQuestion {
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub min_value: i64,
    pub max_value: i64,
    pub min_label: String,
    pub max_label: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DropdownQuestion {
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CheckboxQuestion {
    pub title: String,
    pub description: String,
    pub image_url: Option<String>,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SubjectiveQuestion {
    pub title: String,
    pub description: String,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum ProjectStatus {
    #[default]
    #[translate(ko = "준비")]
    Ready = 1,
    #[translate(ko = "진행")]
    InProgress = 2,
    #[translate(ko = "마감")]
    Finish = 3,
}
