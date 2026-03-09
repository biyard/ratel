use crate::features::spaces::actions::poll::*;

use super::question::{
    CheckboxQuestion, ChoiceQuestion, DropdownQuestion, LinearScaleQuestion, Question,
    SubjectiveQuestion,
};

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum Answer {
    SingleChoice {
        answer: Option<i32>,
        #[serde(default)]
        other: Option<String>,
    },
    MultipleChoice {
        answer: Option<Vec<i32>>,
        #[serde(default)]
        other: Option<String>,
    },
    ShortAnswer {
        answer: Option<String>,
    },
    Subjective {
        answer: Option<String>,
    },
    Checkbox {
        answer: Option<Vec<i32>>,
    },
    Dropdown {
        answer: Option<i32>,
    },
    LinearScale {
        answer: Option<i32>,
    },
}

impl Default for Answer {
    fn default() -> Self {
        Self::SingleChoice {
            answer: None,
            other: None,
        }
    }
}

pub fn validate_answers(questions: Vec<Question>, answers: Vec<Answer>) -> bool {
    if questions.len() != answers.len() {
        return false;
    }
    for question_answer in questions.into_iter().zip(answers.into_iter()) {
        match question_answer {
            (
                Question::SingleChoice(ChoiceQuestion {
                    is_required,
                    options,
                    ..
                }),
                Answer::SingleChoice { answer, other },
            ) => {
                let _other = other;
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
                if let Some(ans) = answer {
                    if ans < 0 || ans >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                Question::MultipleChoice(ChoiceQuestion {
                    is_required,
                    options,
                    ..
                }),
                Answer::MultipleChoice { answer, other },
            ) => {
                let _other = other;
                let answers = answer.unwrap_or_default();
                if is_required.unwrap_or_default() && answers.is_empty() {
                    return false;
                }
                for answer in answers {
                    if answer < 0 || answer >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                Question::ShortAnswer(SubjectiveQuestion { is_required, .. }),
                Answer::ShortAnswer { answer },
            ) => {
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
            }
            (
                Question::Subjective(SubjectiveQuestion { is_required, .. }),
                Answer::Subjective { answer },
            ) => {
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
            }
            (
                Question::Checkbox(CheckboxQuestion {
                    is_required,
                    options,
                    is_multi,
                    ..
                }),
                Answer::Checkbox { answer },
            ) => {
                let answers = answer.unwrap_or_default();
                if is_required.unwrap_or_default() && answers.is_empty() {
                    return false;
                }
                if !is_multi && answers.len() > 1 {
                    return false;
                }
                for answer in answers {
                    if answer < 0 || answer >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                Question::Dropdown(DropdownQuestion {
                    is_required,
                    options,
                    ..
                }),
                Answer::Dropdown { answer },
            ) => {
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
                if let Some(ans) = answer {
                    if ans < 0 || ans >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                Question::LinearScale(LinearScaleQuestion {
                    is_required,
                    min_value,
                    max_value,
                    ..
                }),
                Answer::LinearScale { answer },
            ) => {
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
                if let Some(ans) = answer {
                    if (ans as i64) < min_value || (ans as i64) > max_value {
                        return false;
                    }
                }
            }
            _ => {
                return false;
            }
        }
    }
    true
}
