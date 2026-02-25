use crate::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamDraftPermission {
    pub team_pk: TeamPartition,
    pub permissions: i64,
}
