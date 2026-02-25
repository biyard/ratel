use crate::*;

pub type ListItemsResponse<T> = common::ListResponse<T>;

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamGroupResponse {
    pub id: String,
    pub name: String,
    pub description: String,
    pub members: i64,
    pub permissions: i64,
}

#[cfg(feature = "server")]
impl From<ratel_post::models::TeamGroup> for TeamGroupResponse {
    fn from(group: ratel_post::models::TeamGroup) -> Self {
        let group_id = match group.sk {
            EntityType::TeamGroup(uuid) => uuid,
            _ => group.sk.to_string(),
        };

        Self {
            id: group_id,
            name: group.name,
            description: group.description,
            members: group.members,
            permissions: group.permissions,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateGroupRequest {
    pub name: String,
    pub description: String,
    pub image_url: String,
    pub permissions: Vec<ratel_post::types::TeamGroupPermission>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CreateGroupResponse {
    pub group_pk: Partition,
    pub group_sk: EntityType,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub permissions: Option<Vec<ratel_post::types::TeamGroupPermission>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddMemberRequest {
    pub user_pks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct AddMemberResponse {
    pub total_added: i64,
    pub failed_pks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RemoveMemberRequest {
    pub user_pks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RemoveMemberResponse {
    pub total_removed: i64,
    pub failed_pks: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct DeleteGroupResponse {
    pub message: String,
    pub removed_members: usize,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TeamGroupPermissionContext {
    pub team_pk: TeamPartition,
    pub permissions: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct FoundUserResponse {
    pub pk: String,
    pub nickname: String,
    pub username: String,
    pub profile_url: String,
}
