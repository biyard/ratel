use super::super::*;
use super::TeamRole;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateMemberRoleRequest {
    pub user_pk: String,
    pub role: TeamRole,
}
