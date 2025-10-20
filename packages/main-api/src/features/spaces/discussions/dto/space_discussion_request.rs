use crate::types::Partition;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, JsonSchema)]
pub struct SpaceDiscussionRequest {
    pub started_at: i64,
    pub ended_at: i64,

    pub name: String,
    pub description: String,
    pub user_ids: Vec<Partition>,
}
