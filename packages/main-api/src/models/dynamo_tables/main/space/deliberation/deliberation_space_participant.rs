use crate::{models::user::User, types::*};

use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct DeliberationSpaceParticipant {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    #[dynamo(index = "gsi2", sk)]
    pub sk: EntityType,

    pub participant_id: String,

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

impl DeliberationSpaceParticipant {
    pub fn new(
        deliberation_pk: Partition,
        discussion_pk: Partition,
        participant_id: String,
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
            sk: EntityType::DeliberationSpaceParticipant(uid),
            participant_id,
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            discussion_pk,
        }
    }
}
