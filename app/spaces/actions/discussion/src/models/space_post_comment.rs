use crate::*;

use crate::macros::DynamoEntity;
use crate::models::SpacePostCommentLike;

/// Value for `parent_id_for_likes` indicating a top-level comment (not a reply).
pub const ROOT_PARENT: &str = "ROOT";

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePostComment {
    #[dynamo(index = "gsi2", pk, name = "find_by_post_order_by_likes")]
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub space_pk: Option<Partition>,

    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub content: String,

    #[serde(default)]
    pub likes: u64,

    #[serde(default)]
    #[dynamo(index = "gsi2", sk, order = 0)]
    #[dynamo(index = "gsi3", sk, order = 0)]
    pub likes_align: String,

    #[serde(default)]
    #[dynamo(index = "gsi2", sk, order = 1)]
    #[dynamo(index = "gsi3", sk, order = 1)]
    pub updated_at_align: String,

    #[serde(default)]
    #[dynamo(index = "gsi3", pk, name = "find_replies_by_likes")]
    pub parent_id_for_likes: String,

    #[serde(default)]
    pub replies: u64,

    pub parent_comment_sk: Option<EntityType>,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    pub author_pk: Partition,
    pub author_display_name: String,
    pub author_username: String,
    pub author_profile_url: String,
}

#[cfg(feature = "server")]
impl SpacePostComment {
    pub fn new(
        space_pk: SpacePartition,
        space_post_sk: SpacePostPartition,
        content: String,
        user: &ratel_auth::User,
    ) -> Self {
        use common::utils::time::get_now_timestamp;

        let uuid = uuid::Uuid::now_v7().to_string();
        let now = get_now_timestamp();

        let pk: Partition = space_post_sk.into();

        Self {
            pk,
            sk: EntityType::SpacePostComment(uuid),
            space_pk: Some(space_pk.into()),
            created_at: now,
            updated_at: now,
            content,
            author_pk: user.pk.clone(),
            author_display_name: user.display_name.clone(),
            author_username: user.username.clone(),
            author_profile_url: user.profile_url.clone(),
            likes: 0,
            likes_align: format!("{:020}", 0),
            updated_at_align: format!("{:020}", now),
            parent_id_for_likes: ROOT_PARENT.to_string(),
            replies: 0,
            parent_comment_sk: None,
        }
    }

    pub async fn list_by_comment(
        cli: &aws_sdk_dynamodb::Client,
        comment_sk: EntityType,
        opt: SpacePostCommentQueryOption,
    ) -> crate::Result<(Vec<Self>, Option<String>)> {
        let parent_comment_id = match comment_sk {
            EntityType::SpacePostComment(id) => id,
            _ => {
                tracing::error!("Invalid parent comment sk: {:?}", comment_sk);
                return Err(crate::Error::Unknown(
                    "Invalid parent comment sk".to_string(),
                ));
            }
        };

        SpacePostComment::find_replies_by_likes(cli, parent_comment_id, opt).await
    }

    pub async fn reply(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        space_post_pk: SpacePostPartition,
        parent_comment_sk: EntityType,
        content: String,
        user: &ratel_auth::User,
    ) -> crate::Result<Self> {
        let space_post_pk_p: Partition = space_post_pk.clone().into();

        let parent_comment = Self::updater(&space_post_pk_p, &parent_comment_sk)
            .increase_replies(1)
            .transact_write_item();

        let parent_comment_id = match &parent_comment_sk {
            EntityType::SpacePostComment(id) => id.to_string(),
            _ => {
                tracing::error!("Invalid parent_comment_sk: {:?}", parent_comment_sk);
                return Err(crate::Error::Unknown(
                    "Invalid parent comment sk".to_string(),
                ));
            }
        };

        let (pk, sk) = super::SpacePost::keys(&space_pk, &space_post_pk);

        let post = super::SpacePost::updater(&pk, sk)
            .increase_comments(1)
            .transact_write_item();

        let mut comment = Self::new(space_pk, space_post_pk, content, user)
            .with_parent_comment_sk(parent_comment_sk.clone());

        let uuid = uuid::Uuid::new_v4().to_string();
        comment.sk = EntityType::SpacePostCommentReply(parent_comment_id.clone(), uuid);
        comment.parent_id_for_likes = parent_comment_id;

        let comment_tx = comment.create_transact_write_item();

        transact_write_items!(cli, vec![parent_comment, comment_tx, post]).map_err(|e| {
            tracing::error!("Failed to reply comment: {}", e);
            crate::Error::Unknown(format!("Failed to reply comment: {}", e))
        })?;

        Ok(comment)
    }

    pub fn like_keys(&self, user_pk: &Partition) -> (Partition, EntityType) {
        SpacePostCommentLike::keys_from_partition(self.pk.clone(), self.sk.clone(), user_pk)
    }
}
