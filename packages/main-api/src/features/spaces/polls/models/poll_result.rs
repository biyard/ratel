use crate::{types::*, utils::time::get_now_timestamp_millis};
use bdk::prelude::*;

use super::super::PollSummary;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PollResult {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub summaries: Vec<PollSummary>,
}

impl PollResult {
    pub fn new(space_pk: Partition, summaries: Vec<PollSummary>) -> Self {
        let created_at = get_now_timestamp_millis();

        Self {
            pk: space_pk,
            sk: EntityType::SpacePollResult,
            created_at,
            summaries,
        }
    }
}
