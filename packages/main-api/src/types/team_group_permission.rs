use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
    Translate,
    JsonSchema,
)]
#[repr(u8)]
pub enum TeamGroupPermission {
    //Avaliable Permission Value: 0 ~ 63
    #[default]
    // Post Permissions
    PostRead = 0,
    PostWrite = 1,
    PostEdit = 2,
    PostDelete = 3,

    // Space Permissions
    SpaceRead = 10,
    SpaceWrite = 11,
    SpaceEdit = 12,
    SpaceDelete = 13,

    //Team Permission
    TeamAdmin = 20, // Change Group Permissions + All Other Permissions
    TeamEdit = 21,  // Edit Team Info, Add/Remove Group
    GroupEdit = 22, // Edit Group Members (Invite/Kick), Change Group Info
    // TeamDelete, //  Only Team Owner can delete the team.

    // Admin
    ManagePromotions = 62,
    ManageNews = 63,
}

#[derive(Debug)]
pub struct TeamGroupPermissions(pub Vec<TeamGroupPermission>);

impl Default for TeamGroupPermissions {
    fn default() -> Self {
        Self(vec![
            TeamGroupPermission::PostRead,
            TeamGroupPermission::PostWrite,
            TeamGroupPermission::PostEdit,
            TeamGroupPermission::PostDelete,
            TeamGroupPermission::SpaceRead,
            TeamGroupPermission::SpaceWrite,
            TeamGroupPermission::SpaceEdit,
            TeamGroupPermission::SpaceDelete,
        ])
    }
}

impl AsRef<[TeamGroupPermission]> for TeamGroupPermissions {
    fn as_ref(&self) -> &[TeamGroupPermission] {
        &self.0
    }
}

impl From<TeamGroupPermissions> for i64 {
    fn from(permissions: TeamGroupPermissions) -> Self {
        let mut result = 0;
        for permission in permissions.0 {
            result |= 1 << permission as i32;
        }
        result
    }
}

impl From<i64> for TeamGroupPermissions {
    fn from(permissions: i64) -> Self {
        let mut vec = Vec::new();
        for i in TeamGroupPermission::VARIANTS {
            if permissions & (1 << (*i as i32)) != 0 {
                vec.push(*i);
            }
        }
        Self(vec)
    }
}
