use std::collections::HashMap;

use crate::types::*;
use bdk::prelude::*;

#[derive(
    Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
pub enum SurveySummary {
    SingleChoice {
        question: ChoiceQuestion,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    MultipleChoice {
        question: ChoiceQuestion,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    ShortAnswer {
        question: SubjectiveQuestion,
        answers: HashMap<String, i64>, // (answer, count)
    },
    Subjective {
        question: SubjectiveQuestion,
        answers: HashMap<String, i64>, // (answer, count)
    },
    Checkbox {
        question: CheckboxQuestion,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    Dropdown {
        question: DropdownQuestion,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    LinearScale {
        question: LinearScaleQuestion,
        answers: HashMap<i32, i64>, // (scale_value, count)
    },
}
impl SurveySummary {
    pub fn aggregate_answer(&mut self, answer: SurveyAnswer) {
        match self {
            SurveySummary::SingleChoice { answers, .. } => {
                if let SurveyAnswer::SingleChoice { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                    }
                }
            }
            SurveySummary::MultipleChoice { answers, .. } => {
                if let SurveyAnswer::MultipleChoice { answer } = answer {
                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                    }
                }
            }
            SurveySummary::ShortAnswer { answers, .. } => {
                if let SurveyAnswer::ShortAnswer { answer } = answer {
                    if let Some(answer) = answer {
                        *answers.entry(answer).or_insert(0) += 1;
                    }
                }
            }
            SurveySummary::Subjective { answers, .. } => {
                if let SurveyAnswer::Subjective { answer } = answer {
                    if let Some(answer) = answer {
                        *answers.entry(answer).or_insert(0) += 1;
                    }
                }
            }
            SurveySummary::Checkbox { answers, .. } => {
                if let SurveyAnswer::Checkbox { answer } = answer {
                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                    }
                }
            }
            SurveySummary::Dropdown { answers, .. } => {
                if let SurveyAnswer::Dropdown { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                    }
                }
            }
            SurveySummary::LinearScale { answers, .. } => {
                if let SurveyAnswer::LinearScale { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                    }
                }
            }
        }
    }
}
impl From<SurveyQuestion> for SurveySummary {
    fn from(question: SurveyQuestion) -> Self {
        match question {
            SurveyQuestion::SingleChoice(question) => SurveySummary::SingleChoice {
                question,
                answers: HashMap::new(),
            },
            SurveyQuestion::MultipleChoice(question) => SurveySummary::MultipleChoice {
                question,
                answers: HashMap::new(),
            },
            SurveyQuestion::ShortAnswer(question) => SurveySummary::ShortAnswer {
                question,
                answers: HashMap::new(),
            },
            SurveyQuestion::Subjective(question) => SurveySummary::Subjective {
                question,
                answers: HashMap::new(),
            },
            SurveyQuestion::Checkbox(question) => SurveySummary::Checkbox {
                question,
                answers: HashMap::new(),
            },
            SurveyQuestion::Dropdown(question) => SurveySummary::Dropdown {
                question,
                answers: HashMap::new(),
            },
            SurveyQuestion::LinearScale(question) => SurveySummary::LinearScale {
                question,
                answers: HashMap::new(),
            },
        }
    }
}
