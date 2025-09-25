use bdk::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum SurveyAnswer {
    SingleChoice { answer: Option<i32> },
    MultipleChoice { answer: Option<Vec<i32>> },
    ShortAnswer { answer: Option<String> },
    Subjective { answer: Option<String> },
    Checkbox { answer: Option<Vec<i32>> },
    Dropdown { answer: Option<i32> },
    LinearScale { answer: Option<i32> },
}

impl Default for SurveyAnswer {
    fn default() -> Self {
        Self::SingleChoice { answer: None }
    }
}
