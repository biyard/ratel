use std::collections::HashMap;

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
    pub summaries_by_gender: HashMap<String, Vec<PollSummary>>, // "male"/"female"
    pub summaries_by_age: HashMap<String, Vec<PollSummary>>,    // "0-17"/"18-29"/"30-39"/.../
    pub summaries_by_school: HashMap<String, Vec<PollSummary>>, //
}

impl From<PollResult> for PollResultResponse {
    fn from(poll_result: PollResult) -> Self {
        Self {
            created_at: poll_result.created_at,
            summaries: poll_result.summaries,
            summaries_by_gender: poll_result.summaries_by_gender,
            summaries_by_age: poll_result.summaries_by_age,
            summaries_by_school: poll_result.summaries_by_school,
        }
    }
}
