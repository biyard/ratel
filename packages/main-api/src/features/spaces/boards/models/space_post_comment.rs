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
        space_post_pk: Partition,
        comment_sk: EntityType,
    ) -> Result<(Vec<Self>, Option<String>), crate::Error> {
        let parent_comment_id = match comment_sk {
            EntityType::SpacePostComment(id) => id,
            _ => "".to_string(),
        };

        SpacePostComment::query(
            cli,
            space_post_pk,
            SpacePostCommentQueryOption::builder()
                .limit(10)
                .sk(
                    EntityType::SpacePostCommentReply(parent_comment_id, "".to_string())
                        .to_string(),
                ),
        )
        .await
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
        comment.sk = EntityType::SpacePostCommentReply(parent_comment_id, uuid);

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
