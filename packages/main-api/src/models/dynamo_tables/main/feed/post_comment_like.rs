use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
pub struct PostCommentLike {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "LIKE", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

impl PostCommentLike {
    pub fn new(post_pk: Partition, comment_sk: EntityType, user_pk: Partition) -> Self {
        let created_at = chrono::Utc::now().timestamp();

        let pk = match post_pk {
            Partition::Feed(s) if !s.is_empty() => Partition::PostLike(s),
            _ => panic!("post_pk must be Partition::Post with non-empty inner value"),
        };

        let comment_id = match &comment_sk {
            EntityType::PostComment(id) => id.to_string(),
            _ => panic!("comment_sk must be EntityType::PostComment"),
        };

        let user_id = match &user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("user_pk must be Partition::User"),
        };

        Self {
            pk,
            sk: EntityType::PostCommentLike(user_id, comment_id),
            created_at,
            user_pk,
        }
    }
}
