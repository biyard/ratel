use crate::{models::user::User, types::*};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationDiscussionMember {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    #[dynamo(
        prefix = "DISCUSSION_PK",
        name = "find_by_discussion_pk",
        index = "gsi2",
        pk
    )]
    pub discussion_pk: Partition,
}

impl DeliberationDiscussionMember {
    pub fn new(
        deliberation_pk: Partition,
        discussion_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let user_id = match &pk {
            Partition::User(user_id) => user_id.clone(),
            _ => panic!("requires a user Partition"),
        };

        let discussion_id = match &discussion_pk {
            Partition::Discussion(discussion_id) => discussion_id.clone(),
            _ => panic!("requires a discussion Partition"),
        };

        Self {
            pk: deliberation_pk,
            sk: EntityType::DeliberationDiscussionMember(discussion_id, user_id),
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            discussion_pk,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct DiscussionMemberResponse {
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl From<DeliberationDiscussionMember> for DiscussionMemberResponse {
    fn from(member: DeliberationDiscussionMember) -> Self {
        Self {
            user_pk: member.user_pk,
            author_display_name: member.author_display_name,
            author_profile_url: member.author_profile_url,
            author_username: member.author_username,
        }
    }
}
