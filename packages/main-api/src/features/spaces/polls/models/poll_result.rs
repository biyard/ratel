use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

use crate::features::spaces::polls::PollSummary;
use crate::features::spaces::polls::PollUserAnswer;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollResult {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub summaries: Vec<PollSummary>,
    pub summaries_by_gender: HashMap<String, Vec<PollSummary>>, // "male"/"female"
    pub summaries_by_age: HashMap<String, Vec<PollSummary>>,    // "0-17"/"18-29"/"30-39"/.../
    pub summaries_by_school: HashMap<String, Vec<PollSummary>>, //

    pub sample_answers: Vec<PollUserAnswer>,
    pub final_answers: Vec<PollUserAnswer>,
}

impl PollResult {
    pub fn new(
        space_pk: Partition,
        summaries: Vec<PollSummary>,
        summaries_by_gender: HashMap<String, Vec<PollSummary>>,
        summaries_by_age: HashMap<String, Vec<PollSummary>>,
        summaries_by_school: HashMap<String, Vec<PollSummary>>,

        sample_answers: Vec<PollUserAnswer>,
        final_answers: Vec<PollUserAnswer>,
    ) -> Self {
        let created_at = get_now_timestamp_millis();

        Self {
            pk: space_pk,
            sk: EntityType::SpacePollResult,
            created_at,
            summaries,
            summaries_by_gender,
            summaries_by_age,
            summaries_by_school,

            sample_answers,
            final_answers,
        }
    }
}
