use crate::features::posts::*;

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    serde_repr::Serialize_repr,
    serde_repr::Deserialize_repr,
    Default,
)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema_repr))]
#[repr(u8)]
pub enum TeamGroupPermission {
    #[default]
    PostRead = 0,
    PostWrite = 1,
    PostEdit = 2,
    PostDelete = 3,

    SpaceRead = 10,
    SpaceWrite = 11,
    SpaceEdit = 12,
    SpaceDelete = 13,

    TeamAdmin = 20,
    TeamEdit = 21,
    GroupEdit = 22,

    ManagePromotions = 62,
    ManageNews = 63,
}

#[derive(Debug, Clone)]
pub struct TeamGroupPermissions(pub Vec<TeamGroupPermission>);

impl TeamGroupPermissions {
    pub fn all() -> Self {
        Self(vec![
            TeamGroupPermission::PostRead,
            TeamGroupPermission::PostWrite,
            TeamGroupPermission::PostEdit,
            TeamGroupPermission::PostDelete,
            TeamGroupPermission::SpaceRead,
            TeamGroupPermission::SpaceWrite,
            TeamGroupPermission::SpaceEdit,
            TeamGroupPermission::SpaceDelete,
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::TeamEdit,
            TeamGroupPermission::GroupEdit,
            TeamGroupPermission::ManagePromotions,
            TeamGroupPermission::ManageNews,
        ])
    }

    pub fn contains(&self, permission: TeamGroupPermission) -> bool {
        self.0.contains(&permission)
    }
}

impl TeamGroupPermissions {
    pub fn empty() -> Self {
        Self(vec![])
    }

    pub fn read() -> Self {
        Self(vec![
            TeamGroupPermission::PostRead,
            TeamGroupPermission::SpaceRead,
        ])
    }

    pub fn member() -> Self {
        Self::read()
    }
}

impl std::ops::BitOr for TeamGroupPermissions {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        let mut combined = self.0;
        for perm in rhs.0 {
            if !combined.contains(&perm) {
                combined.push(perm);
            }
        }
        Self(combined)
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
        let variants = [
            TeamGroupPermission::PostRead,
            TeamGroupPermission::PostWrite,
            TeamGroupPermission::PostEdit,
            TeamGroupPermission::PostDelete,
            TeamGroupPermission::SpaceRead,
            TeamGroupPermission::SpaceWrite,
            TeamGroupPermission::SpaceEdit,
            TeamGroupPermission::SpaceDelete,
            TeamGroupPermission::TeamAdmin,
            TeamGroupPermission::TeamEdit,
            TeamGroupPermission::GroupEdit,
            TeamGroupPermission::ManagePromotions,
            TeamGroupPermission::ManageNews,
        ];
        for i in variants {
            if permissions & (1 << (i as i32)) != 0 {
                vec.push(i);
            }
        }
        Self(vec)
    }
}
