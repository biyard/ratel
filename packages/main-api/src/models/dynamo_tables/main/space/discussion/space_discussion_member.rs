use crate::{models::user::User, types::*};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceDiscussionMember {
    pub pk: Partition, //discussion_pk
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType, //user id

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl SpaceDiscussionMember {
    pub fn new(
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

        Self {
            pk: discussion_pk,
            sk: EntityType::SpaceDiscussionMember(user_id),
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
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
