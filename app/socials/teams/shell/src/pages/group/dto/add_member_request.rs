use super::super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddMemberRequest {
    pub user_pks: Vec<String>,
}
