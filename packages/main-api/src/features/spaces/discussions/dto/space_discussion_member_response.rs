use bdk::prelude::*;

use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::types::Partition;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpaceDiscussionMemberResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl From<SpaceDiscussionMember> for SpaceDiscussionMemberResponse {
    fn from(member: SpaceDiscussionMember) -> Self {
        Self {
            user_pk: member.user_pk,
            author_display_name: member.author_display_name,
            author_profile_url: member.author_profile_url,
            author_username: member.author_username,
        }
    }
}
