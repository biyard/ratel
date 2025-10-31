use crate::User;
use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceInvitationMember {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
    pub display_name: String,
    pub profile_url: String,
    pub username: String,
    pub email: String,
}

impl SpaceInvitationMember {
    pub fn new(
        space_pk: Partition,
        User {
            pk,
            display_name,
            profile_url,
            username,
            email,
            ..
        }: User,
    ) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceInvitationMember(pk.to_string()),

            user_pk: pk,
            display_name,
            profile_url,
            username,
            email,
        }
    }
}
