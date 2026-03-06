use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum QuizCorrectAnswer {
    Single {
        answer: Option<i32>,
    },
    Multiple {
        #[serde(default)]
        answers: Vec<i32>,
    },
}

impl Default for QuizCorrectAnswer {
    fn default() -> Self {
        Self::Single { answer: None }
    }
}

impl QuizCorrectAnswer {
    pub fn for_question(question: &Question) -> Self {
        match question {
            Question::MultipleChoice(_) => QuizCorrectAnswer::Multiple { answers: vec![] },
            _ => QuizCorrectAnswer::Single { answer: None },
        }
    }
}
