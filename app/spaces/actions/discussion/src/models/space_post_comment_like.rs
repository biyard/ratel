use crate::*;

use crate::macros::DynamoEntity;
use crate::models::SpacePostComment;

#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePostCommentLike {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "LIKE", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl SpacePostCommentLike {
    pub fn new(
        space_post_pk: SpacePostPartition,
        comment_sk: EntityType,
        user_pk: UserPartition,
    ) -> Self {
        let now = common::utils::time::get_now_timestamp_millis();

        let (pk, sk) = Self::keys(space_post_pk, comment_sk, &user_pk);

        Self {
            pk,
            sk,
            created_at: now,
            user_pk: user_pk.into(),
        }
    }

    pub fn keys(
        space_post_pk: SpacePostPartition,
        comment_sk: EntityType,
        user_pk: &UserPartition,
    ) -> (Partition, EntityType) {
        let pk = Partition::SpacePostLike(space_post_pk.to_string());

        let comment_id = match &comment_sk {
            EntityType::SpacePostComment(id) => id.clone(),
            EntityType::SpacePostCommentReply(parent_id, reply_id) => {
                format!("{}#{}", parent_id, reply_id)
            }
            _ => panic!("comment_sk must be SpacePostComment or SpacePostCommentReply"),
        };

        let user_id = user_pk.to_string();

        let sk = EntityType::SpacePostCommentLike(user_id, comment_id);
        (pk, sk)
    }

    /// Helper for cases where we have raw Partition values (e.g., from loaded entities)
    pub fn keys_from_partition(
        space_post_pk: Partition,
        comment_sk: EntityType,
        user_pk: &Partition,
    ) -> (Partition, EntityType) {
        let pk = match space_post_pk {
            Partition::SpacePost(s) if !s.is_empty() => Partition::SpacePostLike(s),
            _ => panic!("post_pk must be Partition::SpacePost with non-empty inner value"),
        };

        let comment_id = match &comment_sk {
            EntityType::SpacePostComment(id) => id.clone(),
            EntityType::SpacePostCommentReply(parent_id, reply_id) => {
                format!("{}#{}", parent_id, reply_id)
            }
            _ => panic!("comment_sk must be SpacePostComment or SpacePostCommentReply"),
        };

        let user_id = match user_pk {
            Partition::User(id) => id.clone(),
            _ => panic!("user_pk must be Partition::User"),
        };

        let sk = EntityType::SpacePostCommentLike(user_id, comment_id);
        (pk, sk)
    }
}

impl PartialEq<SpacePostComment> for SpacePostCommentLike {
    fn eq(&self, post: &SpacePostComment) -> bool {
        let cid = match &post.sk {
            EntityType::SpacePostComment(id) => id.clone(),
            EntityType::SpacePostCommentReply(parent_id, reply_id) => {
                format!("{}#{}", parent_id, reply_id)
            }
            _ => return false,
        };

        match &self.sk {
            EntityType::SpacePostCommentLike(_, comment_id) if comment_id == &cid => {}
            _ => return false,
        }

        let post_id = match &self.pk {
            Partition::SpacePostLike(id) => id,
            _ => return false,
        };
        let op_post_id = match &post.pk {
            Partition::SpacePost(id) => id,
            _ => return false,
        };

        post_id == op_post_id
    }
}

impl PartialEq<SpacePostComment> for &SpacePostCommentLike {
    fn eq(&self, post: &SpacePostComment) -> bool {
        (**self).eq(post)
    }
}
