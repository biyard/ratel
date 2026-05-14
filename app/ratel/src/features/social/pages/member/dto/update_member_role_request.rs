use super::super::*;
use super::TeamRole;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct UpdateMemberRoleRequest {
    pub user_pk: String,
    pub role: TeamRole,
}
