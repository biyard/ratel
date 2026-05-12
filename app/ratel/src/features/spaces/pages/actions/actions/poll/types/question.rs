use crate::features::spaces::pages::actions::actions::poll::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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

impl Question {
    pub fn title(&self) -> &str {
        match self {
            Question::SingleChoice(q) => &q.title,
            Question::MultipleChoice(q) => &q.title,
            Question::ShortAnswer(q) => &q.title,
            Question::Subjective(q) => &q.title,
            Question::Checkbox(q) => &q.title,
            Question::Dropdown(q) => &q.title,
            Question::LinearScale(q) => &q.title,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct DropdownQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct CheckboxQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
    pub is_multi: bool,
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SubjectiveQuestion {
    pub title: String,
    pub description: String,
    pub is_required: Option<bool>,
}

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, Default)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct ChoiceQuestion {
    pub title: String,
    pub description: Option<String>,
    pub image_url: Option<String>,
    pub options: Vec<String>,
    pub is_required: Option<bool>,
    pub allow_other: Option<bool>,
}
