use crate::features::spaces::actions::poll::*;
use crate::common::macros::DynamoEntity;
use crate::common::utils::time::get_now_timestamp_millis;
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]

pub struct SpacePollResult {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub summaries: Vec<SpacePollSummary>,
    pub summaries_by_gender: HashMap<String, Vec<SpacePollSummary>>, // "male"/"female"
    pub summaries_by_age: HashMap<String, Vec<SpacePollSummary>>,    // "0-17"/"18-29"/"30-39"/.../
    pub summaries_by_school: HashMap<String, Vec<SpacePollSummary>>, //

    pub sample_answers: Vec<SpacePollUserAnswer>,
    pub final_answers: Vec<SpacePollUserAnswer>,
}

impl SpacePollResult {
    pub fn new(
        space_pk: Partition,
        summaries: Vec<SpacePollSummary>,
        summaries_by_gender: HashMap<String, Vec<SpacePollSummary>>,
        summaries_by_age: HashMap<String, Vec<SpacePollSummary>>,
        summaries_by_school: HashMap<String, Vec<SpacePollSummary>>,

        sample_answers: Vec<SpacePollUserAnswer>,
        final_answers: Vec<SpacePollUserAnswer>,
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
