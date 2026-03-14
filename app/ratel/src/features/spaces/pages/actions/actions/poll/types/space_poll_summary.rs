use crate::features::spaces::pages::actions::actions::poll::*;

use super::Question;

use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum SpacePollSummary {
    SingleChoice {
        total_count: i64,
        answers: HashMap<i32, i64>,
        #[serde(default)]
        other_answers: HashMap<String, i64>,
    },
    MultipleChoice {
        total_count: i64,

        answers: HashMap<i32, i64>,
        #[serde(default)]
        other_answers: HashMap<String, i64>,
    },
    ShortAnswer {
        total_count: i64,
        answers: HashMap<String, i64>,
    },
    Subjective {
        total_count: i64,
        answers: HashMap<String, i64>,
    },
    Checkbox {
        total_count: i64,
        answers: HashMap<i32, i64>,
    },
    Dropdown {
        total_count: i64,
        answers: HashMap<i32, i64>,
    },
    LinearScale {
        total_count: i64,
        answers: HashMap<i32, i64>,
    },
}

impl SpacePollSummary {
    pub fn aggregate_answer(&mut self, answer: Answer) {
        match self {
            SpacePollSummary::SingleChoice {
                answers,
                other_answers,
                total_count,
            } => {
                if let Answer::SingleChoice { answer, other } = answer {
                    let mut has_any = false;

                    if let Some(idx) = answer {
                        *answers.entry(idx).or_insert(0) += 1;
                        has_any = true;
                    }

                    if let Some(other_text) = other {
                        if !other_text.is_empty() {
                            *other_answers.entry(other_text).or_insert(0) += 1;
                            has_any = true;
                        }
                    }

                    if has_any {
                        *total_count += 1;
                    }
                }
            }
            SpacePollSummary::MultipleChoice {
                answers,
                other_answers,
                total_count,
            } => {
                if let Answer::MultipleChoice { answer, other } = answer {
                    let mut has_any = false;

                    if let Some(idxs) = answer {
                        for idx in idxs {
                            *answers.entry(idx).or_insert(0) += 1;
                        }
                        has_any = true;
                    }

                    if let Some(other_text) = other {
                        if !other_text.is_empty() {
                            *other_answers.entry(other_text).or_insert(0) += 1;
                            has_any = true;
                        }
                    }

                    if has_any {
                        *total_count += 1;
                    }
                }
            }
            SpacePollSummary::ShortAnswer {
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
            SpacePollSummary::Subjective {
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
            SpacePollSummary::Checkbox {
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
            SpacePollSummary::Dropdown {
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
            SpacePollSummary::LinearScale {
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

impl From<Question> for SpacePollSummary {
    fn from(question: Question) -> Self {
        match question {
            Question::SingleChoice(_) => SpacePollSummary::SingleChoice {
                answers: HashMap::new(),
                other_answers: HashMap::new(),
                total_count: 0,
            },
            Question::MultipleChoice(_) => SpacePollSummary::MultipleChoice {
                answers: HashMap::new(),
                other_answers: HashMap::new(),
                total_count: 0,
            },
            Question::ShortAnswer(_) => SpacePollSummary::ShortAnswer {
                answers: HashMap::new(),
                total_count: 0,
            },
            Question::Subjective(_) => SpacePollSummary::Subjective {
                total_count: 0,
                answers: HashMap::new(),
            },
            Question::Checkbox(_) => SpacePollSummary::Checkbox {
                total_count: 0,
                answers: HashMap::new(),
            },
            Question::Dropdown(_) => SpacePollSummary::Dropdown {
                total_count: 0,
                answers: HashMap::new(),
            },
            Question::LinearScale(_) => SpacePollSummary::LinearScale {
                total_count: 0,
                answers: HashMap::new(),
            },
        }
    }
}
