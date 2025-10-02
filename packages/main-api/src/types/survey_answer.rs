use bdk::prelude::*;

use crate::types::{
    CheckboxQuestion, ChoiceQuestion, DropdownQuestion, LinearScaleQuestion, SubjectiveQuestion,
    SurveyQuestion,
};

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
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

pub fn validate_answers(questions: Vec<SurveyQuestion>, answers: Vec<SurveyAnswer>) -> bool {
    if questions.len() != answers.len() {
        return false;
    }
    for question_answer in questions.into_iter().zip(answers.into_iter()) {
        match question_answer {
            (
                SurveyQuestion::SingleChoice(ChoiceQuestion {
                    is_required,
                    options,
                    ..
                }),
                SurveyAnswer::SingleChoice { answer },
            ) => {
                // If required, answer must be present
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
                // Check answer option index not out of bounds
                if let Some(ans) = answer {
                    if ans < 0 || ans >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                SurveyQuestion::MultipleChoice(ChoiceQuestion {
                    is_required,
                    options,
                    ..
                }),
                SurveyAnswer::MultipleChoice { answer },
            ) => {
                let answers = answer.unwrap_or_default();
                // If required, answer must be present
                if is_required.unwrap_or_default() && answers.is_empty() {
                    return false;
                }
                // Check answer option index not out of bounds
                for answer in answers {
                    if answer < 0 || answer >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                SurveyQuestion::ShortAnswer(SubjectiveQuestion { is_required, .. }),
                SurveyAnswer::ShortAnswer { answer },
            ) => {
                // If required, answer must be present
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
            }
            (
                SurveyQuestion::Subjective(SubjectiveQuestion { is_required, .. }),
                SurveyAnswer::Subjective { answer },
            ) => {
                // If required, answer must be present
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
            }
            (
                SurveyQuestion::Checkbox(CheckboxQuestion {
                    is_required,
                    options,
                    is_multi,
                    ..
                }),
                SurveyAnswer::Checkbox { answer },
            ) => {
                // If is_required is true, answer Vector must be present and not empty
                let answers = answer.unwrap_or_default();
                if is_required.unwrap_or_default() && answers.is_empty() {
                    return false;
                }
                // if is_multi is false, answer Vector cannot exceed 1 item
                if !is_multi && answers.len() > 1 {
                    return false;
                }

                // Check answer option index not out of bounds
                for answer in answers {
                    if answer < 0 || answer >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                SurveyQuestion::Dropdown(DropdownQuestion {
                    is_required,
                    options,
                    ..
                }),
                SurveyAnswer::Dropdown { answer },
            ) => {
                // If required, answer must be present
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
                // Check answer option index not out of bounds
                if let Some(ans) = answer {
                    if ans < 0 || ans >= options.len() as i32 {
                        return false;
                    }
                }
            }
            (
                SurveyQuestion::LinearScale(LinearScaleQuestion {
                    is_required,
                    min_value,
                    max_value,
                    ..
                }),
                SurveyAnswer::LinearScale { answer },
            ) => {
                // If required, answer must be present
                if is_required.unwrap_or_default() && answer.is_none() {
                    return false;
                }
                // Check answer option index not out of bounds
                if let Some(ans) = answer {
                    if (ans as i64) < min_value || (ans as i64) > max_value {
                        return false;
                    }
                }
            }
            _ => {
                //Answer should match with question type
                return false;
            }
        }
    }
    true
}

#[test]
fn test_validate_answers() {
    let questions = vec![SurveyQuestion::SingleChoice(ChoiceQuestion {
        title: "What is your favorite color?".to_string(),
        description: None,
        image_url: None,
        options: vec![
            "Red".to_string(),
            "Blue".to_string(),
            "Green".to_string(),
            "Yellow".to_string(),
        ],
        is_required: Some(false),
    })];
    let answers = vec![SurveyAnswer::SingleChoice { answer: Some(1) }];
    assert!(validate_answers(questions.clone(), answers));

    let invalid_answers = vec![SurveyAnswer::SingleChoice { answer: Some(5) }];
    assert!(!validate_answers(questions.clone(), invalid_answers));

    let required_questions = vec![SurveyQuestion::SingleChoice(ChoiceQuestion {
        title: "What is your favorite color?".to_string(),
        description: None,
        image_url: None,
        options: vec![
            "Red".to_string(),
            "Blue".to_string(),
            "Green".to_string(),
            "Yellow".to_string(),
        ],
        is_required: Some(true),
    })];
    let missing_answers = vec![SurveyAnswer::SingleChoice { answer: None }];

    assert!(!validate_answers(required_questions, missing_answers));
}
