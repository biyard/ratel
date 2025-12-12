use aide::OperationIo;
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::{ReportPublishState, SpacePartition};

#[derive(Debug, Deserialize, JsonSchema, OperationIo)]
pub struct CreateReportRequest {
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct CreateReportResponse {
    pub space_id: SpacePartition,
    pub title: String,
    pub publish_state: ReportPublishState,
    pub created_at: i64,
}
