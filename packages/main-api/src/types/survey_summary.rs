use std::collections::HashMap;

use crate::types::*;
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
pub enum SurveySummary {
    SingleChoice {
        total_count: i64,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    MultipleChoice {
        total_count: i64,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    ShortAnswer {
        total_count: i64,
        answers: HashMap<String, i64>, // (answer, count)
    },
    Subjective {
        total_count: i64,
        answers: HashMap<String, i64>, // (answer, count)
    },
    Checkbox {
        total_count: i64,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    Dropdown {
        total_count: i64,
        answers: HashMap<i32, i64>, // (option_idx, count)
    },
    LinearScale {
        total_count: i64,
        answers: HashMap<i32, i64>, // (scale_value, count)
    },
}
impl SurveySummary {
    pub fn aggregate_answer(&mut self, answer: SurveyAnswer) {
        match self {
            SurveySummary::SingleChoice {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::SingleChoice { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            SurveySummary::MultipleChoice {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::MultipleChoice { answer } = answer {
                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                        *total_count += 1;
                    }
                }
            }
            SurveySummary::ShortAnswer {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::ShortAnswer { answer } = answer {
                    if let Some(answer) = answer {
                        *answers.entry(answer).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            SurveySummary::Subjective {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::Subjective { answer } = answer {
                    if let Some(answer) = answer {
                        *answers.entry(answer).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            SurveySummary::Checkbox {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::Checkbox { answer } = answer {
                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                        *total_count += 1;
                    }
                }
            }
            SurveySummary::Dropdown {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::Dropdown { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            SurveySummary::LinearScale {
                answers,
                total_count,
            } => {
                if let SurveyAnswer::LinearScale { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
        }
    }
}
impl From<SurveyQuestion> for SurveySummary {
    fn from(question: SurveyQuestion) -> Self {
        match question {
            SurveyQuestion::SingleChoice(_) => SurveySummary::SingleChoice {
                answers: HashMap::new(),
                total_count: 0,
            },
            SurveyQuestion::MultipleChoice(_) => SurveySummary::MultipleChoice {
                answers: HashMap::new(),
                total_count: 0,
            },
            SurveyQuestion::ShortAnswer(_) => SurveySummary::ShortAnswer {
                answers: HashMap::new(),
                total_count: 0,
            },
            SurveyQuestion::Subjective(_) => SurveySummary::Subjective {
                total_count: 0,
                answers: HashMap::new(),
            },
            SurveyQuestion::Checkbox(_) => SurveySummary::Checkbox {
                total_count: 0,
                answers: HashMap::new(),
            },
            SurveyQuestion::Dropdown(_) => SurveySummary::Dropdown {
                total_count: 0,
                answers: HashMap::new(),
            },
            SurveyQuestion::LinearScale(_) => SurveySummary::LinearScale {
                total_count: 0,
                answers: HashMap::new(),
            },
        }
    }
}
