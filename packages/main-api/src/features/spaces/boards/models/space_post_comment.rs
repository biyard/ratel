use crate::{
    Error,
    features::spaces::boards::models::{
        space_category::SpaceCategory, space_post::SpacePost,
        space_post_comment_like::SpacePostCommentLike,
    },
    models::{PostCommentLike, team::Team, user::User},
    types::{author::Author, *},
};
use bdk::prelude::*;

#[derive(
    Debug,
    Default,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    JsonSchema,
    aide::OperationIo,
)]
pub struct SpacePostComment {
    #[dynamo(index = "gsi2", pk, name = "find_by_post_order_by_likes")]
    pub pk: Partition,

    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    #[serde(default)]
    pub created_at: i64,
    #[serde(default)]
    pub updated_at: i64,
    #[serde(default)]
    pub content: String,

    #[serde(default)]
    pub likes: u64,
    #[serde(default)]
    pub reports: i64,

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
impl SpacePostComment {
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
        let uuid = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        Self {
            pk,
            sk: EntityType::SpacePostComment(uuid.to_string()),
            created_at: now,
            updated_at: now,
            content,
            author_pk,
            author_display_name,
            author_username,
            author_profile_url,
            likes: 0,
            reports: 0,
            likes_align: format!("{:020}", 0),
            updated_at_align: format!("{:020}", now),
            parent_id_for_likes: "ROOT".to_string(),
            replies: 0,
            parent_comment_sk: None,
        }
    }

    pub async fn list_by_comment(
        cli: &aws_sdk_dynamodb::Client,
        comment_sk: EntityType,
        opt: SpacePostCommentQueryOption,
    ) -> Result<(Vec<Self>, Option<String>), crate::Error> {
        let parent_comment_id = match comment_sk {
            EntityType::SpacePostComment(id) => id,
            _ => {
                tracing::error!("Invalid parent comment sk: {:?}", comment_sk);
                return Err(crate::Error::PostReplyError);
            }
        };

        SpacePostComment::find_replies_by_likes(cli, parent_comment_id, opt).await
    }

    pub async fn reply(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        space_post_pk: Partition,
        parent_comment_sk: EntityType,
        content: String,
        user: User,
    ) -> Result<Self, crate::Error> {
        let parent_comment = Self::updater(&space_post_pk, &parent_comment_sk)
            .increase_replies(1)
            .transact_write_item();

        let parent_comment_id = match &parent_comment_sk {
            EntityType::SpacePostComment(id) => id.to_string(),
            _ => {
                tracing::error!("Invalid parent_comment_sk: {:?}", parent_comment_sk);
                return Err(crate::Error::PostReplyError);
            }
        };

        let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);

        let post = SpacePost::updater(&pk, sk)
            .increase_comments(1)
            .transact_write_item();

        let mut comment = Self::new(space_post_pk, content, user)
            .with_parent_comment_sk(parent_comment_sk.clone());

        let uuid = uuid::Uuid::new_v4().to_string();
        comment.sk = EntityType::SpacePostCommentReply(parent_comment_id.clone(), uuid);
        comment.parent_id_for_likes = parent_comment_id;

        let comment_tx = comment.create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![parent_comment, comment_tx, post]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to add comment: {}", e);
                crate::Error::PostReplyError
            })?;

        Ok(comment)
    }

    pub fn like_keys(&self, user_pk: &Partition) -> (Partition, EntityType) {
        SpacePostCommentLike::keys(self.pk.clone(), self.sk.clone(), user_pk)
    }
}
