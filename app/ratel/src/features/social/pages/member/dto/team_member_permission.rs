use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TeamMemberPermission {
    pub team_pk: TeamPartition,
    pub permissions: i64,
    #[serde(default)]
    pub role: super::TeamRole,
}
