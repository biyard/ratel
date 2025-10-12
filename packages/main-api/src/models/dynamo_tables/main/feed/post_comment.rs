use crate::{
    models::{feed::Post, user::User},
    types::*,
};
use bdk::prelude::*;

use super::PostCommentLike;

#[derive(
    Debug,
    Clone,
    serde::Serialize,
    serde::Deserialize,
    DynamoEntity,
    Default,
    JsonSchema,
    aide::OperationIo,
)]
pub struct PostComment {
    pub pk: Partition,
    #[dynamo(index = "gsi1", sk)]
    pub sk: EntityType,

    pub updated_at: i64,

    pub content: String,

    #[serde(default)]
    pub likes: u64,
    #[serde(default)]
    pub replies: u64,

    pub parent_comment_sk: Option<EntityType>,

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
        let uuid = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        Self {
            pk,
            sk: EntityType::PostComment(uuid.to_string()),
            updated_at: now,
            content,
            author_pk,
            author_display_name,
            author_username,
            author_profile_url,
            likes: 0,
            replies: 0,
            parent_comment_sk: None,
        }
    }

    pub async fn list_by_comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        comment_sk: EntityType,
    ) -> Result<(Vec<Self>, Option<String>), crate::Error2> {
        let parent_comment_id = match comment_sk {
            EntityType::PostComment(id) => id,
            _ => {
                return Err(crate::Error2::InvalidPartitionKey(
                    "comment_sk must be a PostComment".into(),
                ));
            }
        };

        PostComment::query(
            cli,
            Partition::PostReply(post_pk.to_string()),
            PostCommentQueryOption::builder()
                .limit(10)
                .sk(EntityType::PostCommentReply(parent_comment_id, "".to_string()).to_string()),
        )
        .await
    }

    pub fn with_parent_comment_sk(mut self, parent_comment_sk: EntityType) -> Self {
        self.parent_comment_sk = Some(parent_comment_sk);
        self
    }

    pub async fn reply(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        parent_comment_sk: EntityType,
        content: String,
        user: User,
    ) -> Result<Self, crate::Error2> {
        let parent_comment = Self::updater(&post_pk, &parent_comment_sk)
            .increase_replies(1)
            .transact_write_item();

        let parent_comment_id = match &parent_comment_sk {
            EntityType::PostComment(id) => id.to_string(),
            _ => {
                tracing::error!("Invalid parent_comment_sk: {:?}", parent_comment_sk);
                return Err(crate::Error2::PostReplyError);
            }
        };

        let post = Post::updater(&post_pk, EntityType::Post)
            .increase_comments(1)
            .transact_write_item();

        let mut comment = Self::new(Partition::PostReply(post_pk.to_string()), content, user)
            .with_parent_comment_sk(parent_comment_sk.clone());

        let uuid = uuid::Uuid::new_v4().to_string();
        comment.sk = EntityType::PostCommentReply(parent_comment_id, uuid);

        let comment_tx = comment.create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![parent_comment, comment_tx, post]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to add comment: {}", e);
                crate::Error2::PostReplyError
            })?;

        Ok(comment)
    }

    pub fn like_keys(&self, user_pk: &Partition) -> (Partition, EntityType) {
        PostCommentLike::keys(self.pk.clone(), self.sk.clone(), user_pk)
    }
}
