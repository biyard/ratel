use bdk::prelude::*;

use crate::features::spaces::discussions::models::space_discussion_member::SpaceDiscussionMember;
use crate::features::spaces::invitations::SpaceInvitationMember;
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
pub struct SpaceInvitationMemberResponse {
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,

    pub authorized: bool,
}

impl From<SpaceInvitationMember> for SpaceInvitationMemberResponse {
    fn from(member: SpaceInvitationMember) -> Self {
        Self {
            user_pk: member.user_pk,
            display_name: member.display_name,
            profile_url: member.profile_url,
            username: member.username,
            email: member.email,

            authorized: false,
        }
    }
}
