use std::collections::HashMap;

use crate::features::spaces::pages::actions::actions::poll::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "answer_type")]
pub enum PollResultSummary {
    SingleChoice {
        total_count: i64,
        answers: HashMap<String, i64>,
        #[serde(default)]
        other_answers: HashMap<String, i64>,
    },
    MultipleChoice {
        total_count: i64,
        answers: HashMap<String, i64>,
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
        answers: HashMap<String, i64>,
    },
    Dropdown {
        total_count: i64,
        answers: HashMap<String, i64>,
    },
    LinearScale {
        total_count: i64,
        answers: HashMap<String, i64>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PollResultResponse {
    pub created_at: i64,
    pub summaries: Vec<PollResultSummary>,
    pub summaries_by_gender: HashMap<String, Vec<PollResultSummary>>,
    pub summaries_by_age: HashMap<String, Vec<PollResultSummary>>,
    pub summaries_by_school: HashMap<String, Vec<PollResultSummary>>,
    pub sample_answers: Vec<SpacePollUserAnswer>,
    pub final_answers: Vec<SpacePollUserAnswer>,
}

#[get("/api/spaces/{space_pk}/polls/{poll_sk}/results", role: SpaceUserRole)]
pub async fn get_poll_result(
    space_pk: SpacePartition,
    poll_sk: SpacePollEntityType,
) -> Result<PollResultResponse> {
    SpacePoll::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let space_pk: Partition = space_pk.into();
    let poll_sk_entity: EntityType = poll_sk.into();

    let (
        summaries,
        summaries_by_gender,
        summaries_by_age,
        summaries_by_school,
        sample_answers,
        final_answers,
    ) = SpacePollUserAnswer::summarize_responses_with_attribute(cli, &space_pk, &poll_sk_entity)
        .await?;

    Ok(PollResultResponse {
        created_at: crate::common::utils::time::get_now_timestamp_millis(),
        summaries: summaries.into_iter().map(PollResultSummary::from).collect(),
        summaries_by_age: summaries_by_age
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    value.into_iter().map(PollResultSummary::from).collect(),
                )
            })
            .collect(),
        summaries_by_gender: summaries_by_gender
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    value.into_iter().map(PollResultSummary::from).collect(),
                )
            })
            .collect(),
        summaries_by_school: summaries_by_school
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    value.into_iter().map(PollResultSummary::from).collect(),
                )
            })
            .collect(),
        sample_answers,
        final_answers,
    })
}

impl From<SpacePollSummary> for PollResultSummary {
    fn from(value: SpacePollSummary) -> Self {
        match value {
            SpacePollSummary::SingleChoice {
                total_count,
                answers,
                other_answers,
            } => Self::SingleChoice {
                total_count,
                answers: stringify_answer_map(answers),
                other_answers,
            },
            SpacePollSummary::MultipleChoice {
                total_count,
                answers,
                other_answers,
            } => Self::MultipleChoice {
                total_count,
                answers: stringify_answer_map(answers),
                other_answers,
            },
            SpacePollSummary::ShortAnswer {
                total_count,
                answers,
            } => Self::ShortAnswer {
                total_count,
                answers,
            },
            SpacePollSummary::Subjective {
                total_count,
                answers,
            } => Self::Subjective {
                total_count,
                answers,
            },
            SpacePollSummary::Checkbox {
                total_count,
                answers,
            } => Self::Checkbox {
                total_count,
                answers: stringify_answer_map(answers),
            },
            SpacePollSummary::Dropdown {
                total_count,
                answers,
            } => Self::Dropdown {
                total_count,
                answers: stringify_answer_map(answers),
            },
            SpacePollSummary::LinearScale {
                total_count,
                answers,
            } => Self::LinearScale {
                total_count,
                answers: stringify_answer_map(answers),
            },
        }
    }
}

fn stringify_answer_map(map: HashMap<i32, i64>) -> HashMap<String, i64> {
    map.into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect()
}
