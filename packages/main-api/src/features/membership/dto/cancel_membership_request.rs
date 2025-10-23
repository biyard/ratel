use crate::aide::OperationIo;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, JsonSchema, OperationIo)]
pub struct CancelMembershipRequest {
    pub reason: Option<String>,
}
