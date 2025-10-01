use crate::types::*;
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
    pub fn new(post_pk: Partition, user_pk: Partition) -> Self {
        let created_at = chrono::Utc::now().timestamp();

        Self {
            pk: post_pk,
            sk: EntityType::PostLike(user_pk.to_string()),
            created_at,
            user_pk,
        }
    }

    pub async fn like(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        user_pk: Partition,
    ) -> Result<(), crate::Error2> {
        PostLike::new(post_pk, user_pk).create(cli).await
    }

    pub async fn unlike(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        user_pk: Partition,
    ) -> Result<(), crate::Error2> {
        PostLike::delete(cli, post_pk, Some(user_pk)).await
    }
}
