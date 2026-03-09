use super::super::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub permissions: Vec<crate::features::posts::types::TeamGroupPermission>,
}
