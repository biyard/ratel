use bdk::prelude::*;
use serde::Deserialize;

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct TimeRange(pub i64, pub i64); // (started_at, ended_at)

impl TimeRange {
    pub fn is_valid(&self) -> bool {
        self.0 < self.1
    }
}
