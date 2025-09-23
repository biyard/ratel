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
    #[default]
    ReadPosts = 0,
    WritePosts = 1,
    DeletePosts = 2,
    EditPosts = 13,
    WritePendingPosts = 3,
    ReadPostDrafts = 12,

    ReadReplies = 4,
    WriteReplies = 5,
    DeleteReplies = 6,

    ReadProfile = 7,
    UpdateProfile = 8,

    InviteMember = 9,
    ManageGroup = 10,
    DeleteGroup = 11,

    // Space permission
    ManageSpace = 20,

    // Admin
    ManagePromotions = 62,
    ManageNews = 63,
}

pub struct TeamGroupPermissions(pub Vec<TeamGroupPermission>);

impl Default for TeamGroupPermissions {
    fn default() -> Self {
        Self(vec![
            TeamGroupPermission::ReadPosts,
            TeamGroupPermission::WritePosts,
            TeamGroupPermission::EditPosts,
            TeamGroupPermission::DeletePosts,
            TeamGroupPermission::WritePendingPosts,
            TeamGroupPermission::ReadPostDrafts,
            TeamGroupPermission::ReadReplies,
            TeamGroupPermission::WriteReplies,
            TeamGroupPermission::DeleteReplies,
            TeamGroupPermission::ReadProfile,
            TeamGroupPermission::UpdateProfile,
            TeamGroupPermission::InviteMember,
            TeamGroupPermission::ManageGroup,
            TeamGroupPermission::DeleteGroup,
            TeamGroupPermission::ManageSpace,
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
