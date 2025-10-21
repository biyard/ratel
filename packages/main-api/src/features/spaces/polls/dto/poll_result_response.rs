use crate::features::spaces::polls::{PollResult, PollSummary};

use bdk::prelude::*;
#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
    Default,
)]
pub struct PollResultResponse {
    pub created_at: i64,
    pub summaries: Vec<PollSummary>,
}

impl From<PollResult> for PollResultResponse {
    fn from(poll_result: PollResult) -> Self {
        Self {
            created_at: poll_result.created_at,
            summaries: poll_result.summaries,
        }
    }
}
