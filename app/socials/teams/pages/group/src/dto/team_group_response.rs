use crate::*;

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
