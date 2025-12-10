use aide::OperationIo;
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::SpacePartition;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct PublishReportResponse {
    pub space_id: SpacePartition,
    pub published_at: i64,
    pub price_dollars: i64,
    pub x402_resource: String,
}
