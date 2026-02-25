use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamRewardPermission {
    pub team_pk: TeamPartition,
    pub permissions: i64,
    pub team_name: String,
}
