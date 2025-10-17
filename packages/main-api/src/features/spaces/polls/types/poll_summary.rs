use std::collections::HashMap;

use crate::types::{Answer, Question};
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
pub enum PollSummary {
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

impl PollSummary {
    pub fn aggregate_answer(&mut self, answer: Answer) {
        match self {
            PollSummary::SingleChoice {
                answers,
                total_count,
            } => {
                if let Answer::SingleChoice { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            PollSummary::MultipleChoice {
                answers,
                total_count,
            } => {
                if let Answer::MultipleChoice { answer } = answer {
                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                        *total_count += 1;
                    }
                }
            }
            PollSummary::ShortAnswer {
                answers,
                total_count,
            } => {
                if let Answer::ShortAnswer { answer } = answer {
                    if let Some(answer) = answer {
                        *answers.entry(answer).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            PollSummary::Subjective {
                answers,
                total_count,
            } => {
                if let Answer::Subjective { answer } = answer {
                    if let Some(answer) = answer {
                        *answers.entry(answer).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            PollSummary::Checkbox {
                answers,
                total_count,
            } => {
                if let Answer::Checkbox { answer } = answer {
                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                        *total_count += 1;
                    }
                }
            }
            PollSummary::Dropdown {
                answers,
                total_count,
            } => {
                if let Answer::Dropdown { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
            PollSummary::LinearScale {
                answers,
                total_count,
            } => {
                if let Answer::LinearScale { answer } = answer {
                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        *total_count += 1;
                    }
                }
            }
        }
    }
}
impl From<Question> for PollSummary {
    fn from(question: Question) -> Self {
        match question {
            Question::SingleChoice(_) => PollSummary::SingleChoice {
                answers: HashMap::new(),
                total_count: 0,
            },
            Question::MultipleChoice(_) => PollSummary::MultipleChoice {
                answers: HashMap::new(),
                total_count: 0,
            },
            Question::ShortAnswer(_) => PollSummary::ShortAnswer {
                answers: HashMap::new(),
                total_count: 0,
            },
            Question::Subjective(_) => PollSummary::Subjective {
                total_count: 0,
                answers: HashMap::new(),
            },
            Question::Checkbox(_) => PollSummary::Checkbox {
                total_count: 0,
                answers: HashMap::new(),
            },
            Question::Dropdown(_) => PollSummary::Dropdown {
                total_count: 0,
                answers: HashMap::new(),
            },
            Question::LinearScale(_) => PollSummary::LinearScale {
                total_count: 0,
                answers: HashMap::new(),
            },
        }
    }
}
