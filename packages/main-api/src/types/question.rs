use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
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

impl Default for Question {
    fn default() -> Self {
        Self::SingleChoice(ChoiceQuestion::default())
    }
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct LinearScaleQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub min_value: i64,
    pub max_value: i64,
    pub min_label: String,
    pub max_label: String,
    pub is_required: Option<bool>,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct DropdownQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
    pub is_required: Option<bool>,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct CheckboxQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
    pub is_multi: bool,
    pub is_required: Option<bool>,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SubjectiveQuestion {
    pub title: String,
    pub description: String,
    pub is_required: Option<bool>,
}

#[derive(
    Debug,
    Clone,
    Eq,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    Default,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
    pub is_required: Option<bool>,
    pub allow_other: Option<bool>,
}
