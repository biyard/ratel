use crate::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MemberGroup {
    pub group_id: String,
    pub group_name: String,
    pub description: String,
}
