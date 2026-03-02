use super::EligibleAdminResponse;
use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamDaoTeamResponse {
    pub team_pk: TeamPartition,
    pub username: String,
    pub nickname: String,
    pub dao_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamDao {
    pub team: TeamDaoTeamResponse,
    pub permissions: i64,
    pub eligible_admins: Vec<EligibleAdminResponse>,
}
