use crate::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, aide::OperationIo)]
pub struct RewardsResponse {
    // Project Info
    pub project_name: String,
    pub token_symbol: String,

    pub month: String,
    pub total_points: i64,

    pub points: i64,
    pub monthly_token_supply: i64,
}
