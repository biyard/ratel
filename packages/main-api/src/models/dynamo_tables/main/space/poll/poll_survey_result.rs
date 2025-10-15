use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollSpaceSurveyResult {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub summaries: Vec<SurveySummary>,
}

impl PollSpaceSurveyResult {
    pub fn new(space_pk: Partition, summaries: Vec<SurveySummary>) -> Self {
        let created_at = get_now_timestamp_millis();

        Self {
            pk: space_pk,
            sk: EntityType::PollSpaceSurveyResult,
            created_at,
            summaries,
        }
    }
}
