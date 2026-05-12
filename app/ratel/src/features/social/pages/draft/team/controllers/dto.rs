use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TeamDraftPermission {
    pub team_pk: TeamPartition,
    pub permissions: i64,
    #[serde(default)]
    pub role: crate::features::social::pages::member::dto::TeamRole,
}
