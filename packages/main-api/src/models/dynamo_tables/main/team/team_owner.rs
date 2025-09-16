use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct TeamOwner {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
}

impl TeamOwner {
    pub fn new(
        pk: Partition,
        User {
            pk: user_pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        Self {
            pk,
            sk: EntityType::TeamOwner,
            display_name,
            profile_url,
            username,
            user_pk,
        }
    }
}
