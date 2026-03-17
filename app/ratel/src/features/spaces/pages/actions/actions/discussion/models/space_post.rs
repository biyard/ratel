use crate::features::spaces::pages::actions::actions::discussion::*;

use crate::features::spaces::pages::actions::actions::discussion::macros::DynamoEntity;
use crate::features::spaces::pages::actions::actions::discussion::models::{
    SpacePostComment, SpacePostCommentLike,
};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct SpacePost {
    #[dynamo(index = "gsi3", name = "find_by_space_ordered", pk)]
    #[dynamo(index = "gsi6", name = "find_by_category", order = 1, pk)]
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    #[dynamo(index = "gsi3", sk)]
    #[dynamo(index = "gsi6", sk)]
    pub updated_at: i64,

    #[serde(default)]
    pub started_at: i64,
    #[serde(default)]
    pub ended_at: i64,

    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub html_contents: String,
    #[dynamo(index = "gsi6", name = "find_by_category", order = 2, pk)]
    #[serde(default)]
    pub category_name: String,
    pub comments: i64,

    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
}

#[cfg(feature = "server")]
impl SpacePost {
    pub fn new(
        space_pk: SpacePartition,
        title: String,
        html_contents: String,
        category_name: String,
        author: &crate::common::models::space::SpaceAuthor,
        started_at: Option<i64>,
        ended_at: Option<i64>,
    ) -> Self {
        let pk: Partition = space_pk.into();
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let uuid = uuid::Uuid::now_v7().to_string();
        let started_at = started_at.unwrap_or(now);
        // End At is 7 days after start date
        let ended_at = ended_at.unwrap_or(now + 7 * 24 * 60 * 60 * 1000);
        Self {
            pk,
            sk: EntityType::SpacePost(uuid),
            created_at: now,
            updated_at: now,
            started_at,
            ended_at,
            title,
            html_contents,
            category_name,
            comments: 0,
            user_pk: author.pk.clone(),
            author_display_name: author.display_name.clone(),
            author_profile_url: author.profile_url.clone(),
            author_username: author.username.clone(),
        }
    }

    pub fn keys(
        space_pk: &SpacePartition,
        space_post_pk: &SpacePostPartition,
    ) -> (Partition, EntityType) {
        let space_post_id = space_post_pk.to_string();
        (
            space_pk.clone().into(),
            EntityType::SpacePost(space_post_id),
        )
    }

    pub async fn comment(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: SpacePartition,
        space_post_pk: SpacePostPartition,
        content: String,
        author: &crate::common::models::space::SpaceAuthor,
    ) -> crate::features::spaces::pages::actions::actions::discussion::Result<SpacePostComment>
    {
        let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
        let post = SpacePost::updater(&pk, sk)
            .increase_comments(1)
            .transact_write_item();
        let comment = SpacePostComment::new(space_pk, space_post_pk, content, author);
        let comment_tx = comment.create_transact_write_item();

        crate::transact_write_items!(cli, vec![comment_tx, post]).map_err(|e| {
            tracing::error!("Failed to add comment: {}", e);
            crate::features::spaces::pages::actions::actions::discussion::Error::Unknown(format!(
                "Failed to add comment: {}",
                e
            ))
        })?;

        Ok(comment)
    }

    pub async fn like_comment(
        cli: &aws_sdk_dynamodb::Client,
        space_post_pk: SpacePostPartition,
        comment_pk: EntityType,
        user_pk: UserPartition,
    ) -> crate::features::spaces::pages::actions::actions::discussion::Result<()> {
        let space_post_pk_p: Partition = space_post_pk.clone().into();

        // Use atomic increase for likes; likes_align is best-effort for GSI sorting.
        let comment = SpacePostComment::get(cli, &space_post_pk_p, Some(comment_pk.clone()))
            .await?
            .ok_or(
                crate::features::spaces::pages::actions::actions::discussion::Error::NotFound(
                    "Comment not found".into(),
                ),
            )?;
        let approx_likes_align = format!("{:020}", comment.likes.saturating_add(1));

        let comment_tx = SpacePostComment::updater(&space_post_pk_p, &comment_pk)
            .increase_likes(1)
            .with_likes_align(approx_likes_align)
            .transact_write_item();

        let pl_tx = SpacePostCommentLike::new(space_post_pk, comment_pk, user_pk)
            .create_transact_write_item();

        crate::transact_write_items!(cli, vec![comment_tx, pl_tx]).map_err(|e| {
            tracing::error!("Failed to like comment: {}", e);
            crate::features::spaces::pages::actions::actions::discussion::Error::Unknown(format!(
                "Failed to like comment: {}",
                e
            ))
        })?;

        Ok(())
    }

    pub async fn unlike_comment(
        cli: &aws_sdk_dynamodb::Client,
        space_post_pk: SpacePostPartition,
        comment_pk: EntityType,
        user_pk: UserPartition,
    ) -> crate::features::spaces::pages::actions::actions::discussion::Result<()> {
        let space_post_pk_p: Partition = space_post_pk.clone().into();

        let comment = SpacePostComment::get(cli, &space_post_pk_p, Some(comment_pk.clone()))
            .await?
            .ok_or(
                crate::features::spaces::pages::actions::actions::discussion::Error::NotFound(
                    "Comment not found".into(),
                ),
            )?;
        let approx_likes_align = format!("{:020}", comment.likes.saturating_sub(1));

        let comment_tx = SpacePostComment::updater(&space_post_pk_p, &comment_pk)
            .decrease_likes(1)
            .with_likes_align(approx_likes_align)
            .transact_write_item();

        let pcl = SpacePostCommentLike::new(space_post_pk, comment_pk, user_pk);
        let pl_tx = SpacePostCommentLike::delete_transact_write_item(&pcl.pk, &pcl.sk);

        crate::transact_write_items!(cli, vec![comment_tx, pl_tx]).map_err(|e| {
            tracing::error!("Failed to unlike comment: {}", e);
            crate::features::spaces::pages::actions::actions::discussion::Error::Unknown(format!(
                "Failed to unlike comment: {}",
                e
            ))
        })?;

        Ok(())
    }
}

#[cfg(feature = "server")]
impl From<(SpacePost, SpaceUserRole)>
    for crate::features::spaces::pages::actions::types::SpaceActionSummary
{
    fn from((post, role): (SpacePost, SpaceUserRole)) -> Self {
        use crate::features::spaces::pages::actions::types::SpaceActionType;
        let action_id = post.sk.to_string();
        Self {
            user_participated: post.can_participate(&role).is_ok(),
            action_id,
            action_type: SpaceActionType::TopicDiscussion,
            title: post.title,
            description: String::new(),
            created_at: post.created_at,
            updated_at: post.updated_at,
            total_score: None,
            total_point: None,
            quiz_score: None,
            quiz_total_score: None,
            quiz_passed: None,
            started_at: Some(post.started_at),
            ended_at: Some(post.ended_at),
            credits: 0,
            prerequisite: false,
        }
    }
}

impl SpacePost {
    pub fn status(&self) -> DiscussionStatus {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let started_at = self.started_at;
        let ended_at = self.ended_at;
        if now < started_at {
            DiscussionStatus::NotStarted
        } else if now <= ended_at {
            DiscussionStatus::InProgress
        } else {
            DiscussionStatus::Finish
        }
    }

    pub fn can_view(
        _user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::discussion::Result<()> {
        Ok(())
    }

    pub fn can_edit(
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::discussion::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            _ => Err(
                crate::features::spaces::pages::actions::actions::discussion::Error::NoPermission,
            ),
        }
    }

    pub fn can_participate(
        &self,
        user_role: &SpaceUserRole,
    ) -> crate::features::spaces::pages::actions::actions::discussion::Result<()> {
        match user_role {
            SpaceUserRole::Creator => Ok(()),
            SpaceUserRole::Participant => {
                if self.status() == DiscussionStatus::InProgress {
                    Ok(())
                } else {
                    Err(crate::features::spaces::pages::actions::actions::discussion::Error::SpacePostNotStarted)
                }
            }
            _ => Err(
                crate::features::spaces::pages::actions::actions::discussion::Error::NoPermission,
            ),
        }
    }
}
