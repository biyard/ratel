use super::super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddTeamMemberResponse {
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}
