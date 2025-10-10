use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
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
        let pk = match post_pk {
            Partition::Feed(s) if !s.is_empty() => Partition::PostLike(s),
            _ => panic!("post_pk must be Partition::Post with non-empty inner value"),
        };

        Self {
            pk,
            sk: EntityType::PostLike(user_pk.to_string()),
            created_at,
            user_pk,
        }
    }

    pub async fn find_one(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: &Partition,
        user_pk: &Partition,
    ) -> Result<Option<Self>, crate::Error2> {
        let pk = match post_pk {
            Partition::Feed(s) if !s.is_empty() => Partition::PostLike(s.clone()),
            _ => panic!("post_pk must be Partition::Post with non-empty inner value"),
        };

        PostLike::get(&cli, pk, Some(EntityType::PostLike(user_pk.to_string()))).await
    }
}
