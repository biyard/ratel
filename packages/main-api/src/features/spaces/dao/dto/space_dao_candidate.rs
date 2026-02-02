use bdk::prelude::*;

#[derive(Debug, serde::Deserialize, serde::Serialize, aide::OperationIo, JsonSchema)]
pub struct SpaceDaoCandidate {
    pub user_pk: String,
    pub username: String,
    pub display_name: String,
    pub profile_url: String,
    pub evm_address: String,
}
