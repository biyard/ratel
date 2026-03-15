use super::super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddMemberResponse {
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}
