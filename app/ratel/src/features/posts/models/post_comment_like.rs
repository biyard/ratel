use crate::features::posts::types::*;
use crate::features::posts::*;

use super::PostComment;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
pub struct PostCommentLike {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "LIKE", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl PostCommentLike {
    pub fn normalize_feed_id(raw: &str) -> String {
        raw.strip_prefix("FEED#").unwrap_or(raw).to_string()
    }
    pub fn new(post_pk: Partition, comment_sk: EntityType, user_pk: Partition) -> Self {
        let created_at = chrono::Utc::now().timestamp();

        let (pk, sk) = Self::keys(post_pk, comment_sk, &user_pk);

        Self {
            pk,
            sk,
            created_at,
            user_pk,
        }
    }

    pub fn keys(
        post_pk: Partition,
        comment_sk: EntityType,
        user_pk: &Partition,
    ) -> (Partition, EntityType) {
        let pk = match post_pk {
            Partition::Feed(s) if !s.is_empty() => Partition::PostLike(s),
            Partition::PostReply(s) if !s.is_empty() => {
                Partition::PostLike(Self::normalize_feed_id(&s))
            }
            _ => panic!(
                "post_pk must be Partition::Feed or Partition::PostReply with non-empty inner value"
            ),
        };

        let comment_id = match &comment_sk {
            EntityType::PostComment(id) => id.to_string(),
            EntityType::PostCommentReply(_, id) => id.to_string(),
            _ => panic!("comment_sk must be EntityType::PostComment or PostCommentReply"),
        };

        let user_id = match user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("user_pk must be Partition::User"),
        };

        let sk = EntityType::PostCommentLike(user_id, comment_id);

        (pk, sk)
    }
}

impl PartialEq<PostComment> for PostCommentLike {
    fn eq(&self, post: &PostComment) -> bool {
        let cid = match &post.sk {
            EntityType::PostComment(id) => id,
            EntityType::PostCommentReply(_, id) => id,
            _ => return false,
        };

        match &self.sk {
            EntityType::PostCommentLike(_, comment_id) if comment_id == cid => {}
            _ => return false,
        }
        let feed_id = match &self.pk {
            Partition::PostLike(id) => id.clone(),
            _ => return false,
        };

        let op_feed_id = match &post.pk {
            Partition::Feed(id) => id.clone(),
            Partition::PostReply(id) => id.strip_prefix("FEED#").unwrap_or(id).to_string(),
            _ => return false,
        };

        feed_id == op_feed_id
    }
}

impl PartialEq<PostComment> for &PostCommentLike {
    fn eq(&self, post: &PostComment) -> bool {
        (**self).eq(post)
    }
}
