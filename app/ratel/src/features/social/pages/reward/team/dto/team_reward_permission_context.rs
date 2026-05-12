use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TeamRewardPermission {
    pub team_pk: TeamPartition,
    pub permissions: i64,
    pub team_name: String,
    #[serde(default)]
    pub role: crate::features::social::pages::member::dto::TeamRole,
}
