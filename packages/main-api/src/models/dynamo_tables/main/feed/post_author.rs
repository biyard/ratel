use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PostAuthor {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub display_name: String,
    pub profile_url: String,
    pub username: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub user_pk: Partition,
}

impl PostAuthor {
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
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk: EntityType::PostAuthor,
            created_at,
            updated_at: created_at,
            display_name,
            profile_url,
            username,
            user_pk,
        }
    }
}
