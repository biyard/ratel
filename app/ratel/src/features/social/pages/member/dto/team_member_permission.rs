use super::super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamMemberPermission {
    pub team_pk: TeamPartition,
    pub permissions: i64,
}
