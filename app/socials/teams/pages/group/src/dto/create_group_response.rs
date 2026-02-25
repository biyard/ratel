use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateGroupResponse {
    pub group_pk: Partition,
    pub group_sk: EntityType,
}
