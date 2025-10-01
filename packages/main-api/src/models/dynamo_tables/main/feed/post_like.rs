use crate::{models::user::User, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, Default)]
pub struct PostLike {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "LIKE", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

impl PostLike {
    pub fn new(pk: Partition, User { pk: user_pk, .. }: User) -> Self {
        let created_at = chrono::Utc::now().timestamp();

        Self {
            pk,
            sk: EntityType::PostLike(user_pk.to_string()),
            created_at,
            user_pk,
        }
    }
}
