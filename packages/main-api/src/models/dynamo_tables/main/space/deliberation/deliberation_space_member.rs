use crate::{models::user::User, types::*};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceMember {
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

impl DeliberationSpaceMember {
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
        let uid = uuid::Uuid::new_v4().to_string();

        Self {
            pk: deliberation_pk,
            sk: EntityType::DeliberationSpaceMember(uid),
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            discussion_pk,
        }
    }
}

#[derive(Debug, Clone, Default, serde::Serialize, schemars::JsonSchema)]
pub struct DiscussionMemberResponse {
    pub user_pk: String,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

impl From<DeliberationSpaceMember> for DiscussionMemberResponse {
    fn from(member: DeliberationSpaceMember) -> Self {
        let user_pk = match member.user_pk {
            Partition::User(v) => v,
            Partition::Team(v) => v,
            _ => "".to_string(),
        };
        Self {
            user_pk,
            author_display_name: member.author_display_name,
            author_profile_url: member.author_profile_url,
            author_username: member.author_username,
        }
    }
}
