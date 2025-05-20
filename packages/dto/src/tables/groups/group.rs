use bdk::prelude::*;
use validator::Validate;

#[derive(Validate)]
#[api_model(base = "/v1/groups", table = groups, action = [], action_by_id = [delete, update])]
pub struct Group {
    #[api_model(summary, primary_key)]
    pub id: i64,
    #[api_model(summary, auto = [insert])]
    pub created_at: i64,
    #[api_model(summary, auto = [insert, update])]
    pub updated_at: i64,

    pub name: String,
    #[api_model(many_to_one = users)]
    pub user_id: i64,

    #[api_model(version = v0.1)]
    pub permissions: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Default, ApiModel, Translate, Copy)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub enum GroupPermission {
    #[default]
    #[translate(en = "Read posts")]
    ReadPosts = 0,
    #[translate(en = "Write posts")]
    WritePosts = 1,
    #[translate(en = "Delete posts")]
    DeletePosts = 2,
    #[translate(en = "Write pending posts")]
    WritePendingPosts = 3,

    #[translate(en = "Read replies")]
    ReadReplies = 4,
    #[translate(en = "Write replies")]
    WriteReplies = 5,
    #[translate(en = "Delete replies")]
    DeleteReplies = 6,

    #[translate(en = "Read profile")]
    ReadProfile = 7,
    #[translate(en = "Update profile")]
    UpdateProfile = 8,

    // Admin
    #[translate(en = "[Admin] Manage promotions")]
    ManagePromotions = 62,
    #[translate(en = "[Admin] Manage news")]
    ManageNews = 63,
}

pub struct GroupPermissions(Vec<GroupPermission>);

impl Default for GroupPermissions {
    fn default() -> Self {
        Self(vec![
            GroupPermission::ReadPosts,
            GroupPermission::ReadReplies,
            GroupPermission::WriteReplies,
            GroupPermission::DeleteReplies,
            GroupPermission::ReadProfile,
            GroupPermission::UpdateProfile,
        ])
    }
}

impl AsRef<[GroupPermission]> for GroupPermissions {
    fn as_ref(&self) -> &[GroupPermission] {
        &self.0
    }
}

impl From<GroupPermissions> for i64 {
    fn from(permissions: GroupPermissions) -> Self {
        let mut result = 0;
        for permission in permissions.0 {
            result |= 1 << permission as i32;
        }
        result
    }
}

impl From<i64> for GroupPermissions {
    fn from(permissions: i64) -> Self {
        let mut vec = Vec::new();
        for i in GroupPermission::VARIANTS {
            if permissions & (1 << (*i as i32)) != 0 {
                vec.push(GroupPermission::try_from(*i as i32).unwrap());
            }
        }
        Self(vec)
    }
}
