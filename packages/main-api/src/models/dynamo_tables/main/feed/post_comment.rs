use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default, JsonSchema)]
pub struct PostComment {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    pub updated_at: i64,

    pub content: String,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,
}

impl PostComment {
    pub fn new(
        pk: Partition,
        content: String,
        User {
            pk: author_pk,
            display_name: author_display_name,
            username: author_username,
            profile_url: author_profile_url,
            ..
        }: User,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk: EntityType::PostComment(now.to_string()),
            updated_at: now,
            content,
            author_pk,
            author_display_name,
            author_username,
            author_profile_url,
        }
    }
}
