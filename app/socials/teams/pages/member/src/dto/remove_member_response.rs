use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RemoveMemberResponse {
    pub total_removed: i64,
    pub failed_pks: Vec<String>,
}
