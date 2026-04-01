use super::super::*;
use super::TeamRole;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddTeamMemberRequest {
    pub user_pks: Vec<String>,
    pub role: TeamRole,
}
