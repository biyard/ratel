use crate::{features::spaces::boards::models::space_post_comment::SpacePostComment, types::*};
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default, DynamoEntity)]
pub struct SpacePostCommentLike {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(prefix = "LIKE", name = "find_by_user", index = "gsi1", pk)]
    pub user_pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
}

impl SpacePostCommentLike {
    pub fn new(space_post_pk: Partition, comment_sk: EntityType, user_pk: Partition) -> Self {
        let created_at = chrono::Utc::now().timestamp();

        let (pk, sk) = Self::keys(space_post_pk, comment_sk, &user_pk);

        Self {
            pk,
            sk,
            created_at,
            user_pk,
        }
    }

    pub fn keys(
        space_post_pk: Partition,
        comment_sk: EntityType,
        user_pk: &Partition,
    ) -> (Partition, EntityType) {
        let pk = match space_post_pk {
            Partition::SpacePost(s) if !s.is_empty() => Partition::SpacePostLike(s),
            _ => panic!("post_pk must be Partition::Post with non-empty inner value"),
        };

        let comment_id = match &comment_sk {
            EntityType::SpacePostComment(id) => id.to_string(),
            _ => panic!("comment_sk must be EntityType::PostComment"),
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
            EntityType::SpacePostComment(id) => id,
            _ => return false,
        };

        match &self.sk {
            EntityType::SpacePostCommentLike(_, comment_id) if comment_id == cid => {}
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
