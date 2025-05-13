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
    #[translate(en = "Read replies")]
    ReadReplies = 1,
    #[translate(en = "Write posts")]
    WritePosts = 2,
    #[translate(en = "Write replies")]
    WriteReplies = 3,
    #[translate(en = "Write comments")]
    WritePendingPosts = 4,
}

pub struct GroupPermissions(Vec<GroupPermission>);

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
        for i in 0..64 {
            if permissions & (1 << i) != 0 {
                vec.push(GroupPermission::try_from(i as i32).unwrap());
            }
        }
        Self(vec)
    }
}
