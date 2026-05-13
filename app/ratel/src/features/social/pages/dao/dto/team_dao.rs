use super::EligibleAdminResponse;
use super::super::*;
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TeamDaoTeamResponse {
    pub team_pk: TeamPartition,
    pub username: String,
    pub nickname: String,
    pub dao_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct TeamDao {
    pub team: TeamDaoTeamResponse,
    pub permissions: i64,
    pub eligible_admins: Vec<EligibleAdminResponse>,
    #[serde(default)]
    pub role: crate::features::social::pages::member::dto::TeamRole,
}
