use crate::common::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default, PartialEq, Eq, DynamoEnum, Translate)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub enum EventStatus {
    #[default]
    #[translate(en = "Requested", ko = "요청완료")]
    Requested,
    #[translate(en = "Failed", ko = "실패")]
    Failed,
    #[translate(en = "Completed", ko = "처리완료")]
    Completed,
}
