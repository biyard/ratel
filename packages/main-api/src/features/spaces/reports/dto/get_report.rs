use aide::OperationIo;
use bdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::types::{ReportPublishState, SpacePartition};

use super::set_pricing::RevenueSplitInfo;

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, OperationIo)]
pub struct GetReportResponse {
    pub space_id: SpacePartition,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub price_dollars: Option<i64>,
    pub recipient_address: Option<String>,
    pub publish_state: ReportPublishState,
    pub published_at: Option<i64>,
    pub revenue_split: Option<RevenueSplitInfo>,
    pub author_display_name: String,
    pub author_username: String,
    pub created_at: i64,
    pub updated_at: i64,
}
