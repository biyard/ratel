use crate::{models::user::User, types::*};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct SpaceDiscussionMember {
    pub pk: Partition, //discussion_pk
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType, //(discussion_pk, user_pk)

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
        let (discussion_pk, sk) = Self::keys(&discussion_pk, &pk);

        Self {
            pk: discussion_pk,
            sk,
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
        }
    }

    pub fn keys(discussion_pk: &Partition, user_pk: &Partition) -> (Partition, EntityType) {
        (
            discussion_pk.clone(),
            EntityType::SpaceDiscussionMember(discussion_pk.to_string(), user_pk.to_string()),
        )
    }

    pub async fn is_member(
        cli: &aws_sdk_dynamodb::Client,
        discussion_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<bool, crate::Error2> {
        let (pk, sk) = Self::keys(discussion_pk, user_pk);
        let member = SpaceDiscussionMember::get(&cli, pk, Some(sk)).await?;

        Ok(member.is_some())
    }
}
