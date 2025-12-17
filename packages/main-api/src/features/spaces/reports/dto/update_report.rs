use aide::OperationIo;
use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema, OperationIo)]
pub struct UpdateReportRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub summary: Option<String>,
}
