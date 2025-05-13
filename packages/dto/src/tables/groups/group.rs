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
    ReadPosts = 1,
    #[translate(en = "Read replies")]
    ReadReplies = 1 << 1,
    #[translate(en = "Write posts")]
    WritePosts = 1 << 2,
    #[translate(en = "Write replies")]
    WriteReplies = 1 << 3,
    #[translate(en = "Write comments")]
    WritePendingPosts = 1 << 4,
}
