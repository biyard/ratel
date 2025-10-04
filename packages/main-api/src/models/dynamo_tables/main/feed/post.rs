use crate::{
    Error2,
    models::team::Team,
    types::{author::Author, sorted_visibility::SortedVisibility, *},
};
use bdk::prelude::*;

use super::PostLike;

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
pub struct Post {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi6", sk)]
    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    pub updated_at: i64,

    pub title: String,
    pub html_contents: String,
    pub post_type: PostType,

    #[dynamo(index = "gsi5", sk)]
    pub status: PostStatus,

    #[dynamo(index = "gsi6", name = "find_by_visibility", pk)]
    pub visibility: Option<Visibility>,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    #[dynamo(prefix = "USER_PK", name = "find_by_user_pk", index = "gsi1", pk)]
    #[dynamo(
        prefix = "USER_VISIBILITY",
        name = "find_by_user_pk_visibility",
        index = "gsi2",
        pk
    )]
    #[dynamo(
        prefix = "USER_STATUS",
        index = "gsi5",
        name = "find_by_user_and_status",
        pk
    )]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,
    pub author_type: UserType,

    // #[dynamo(prefix = "SPACE_PK", name = "find_by_space_pk", index = "gsi2", pk)]
    pub space_pk: Option<Partition>,
    pub space_type: Option<SpaceType>,
    pub booster: Option<BoosterType>,
    // only for reward spaces
    pub rewards: Option<i64>,

    // Only for list posts Composed key
    #[dynamo(index = "gsi2", sk)]
    pub sorted_visibility: SortedVisibility,
    pub urls: Vec<String>,
}

impl Post {
    pub fn draft(author: Author) -> Self {
        Self::new("", "", PostType::Post, author)
    }

    pub fn new<T: Into<String>, A: Into<Author>>(
        title: T,
        html_contents: T,
        post_type: PostType,
        author: A,
    ) -> Self {
        let uid = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp_micros();
        let Author {
            pk,
            display_name,
            profile_url,
            username,
            user_type,
        } = author.into();

        Self {
            pk: Partition::Feed(uid),
            sk: EntityType::Post,
            created_at: now,
            updated_at: now,
            post_type,
            title: title.into(),
            html_contents: html_contents.into(),
            status: PostStatus::Draft,
            visibility: None,
            shares: 0,
            likes: 0,
            comments: 0,

            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,
            author_type: user_type,

            space_pk: None,
            space_type: None,
            booster: None,
            rewards: None,
            sorted_visibility: SortedVisibility::Draft(now.to_string()),
            urls: vec![],
        }
    }

    pub async fn has_permission(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: &Partition,
        user_pk: Option<&Partition>,
        perm: TeamGroupPermission,
    ) -> Result<(Self, bool), crate::Error2> {
        let post = Post::get(cli, post_pk, Some(EntityType::Post))
            .await?
            .ok_or(Error2::NotFound("Post not found".to_string()))?;

        let user_pk = if let Some(user_pk) = user_pk {
            user_pk
        } else {
            if post.visibility.is_some()
                && post.visibility.as_ref().unwrap() == &Visibility::Public
                && perm == TeamGroupPermission::PostRead
                && post.status == PostStatus::Published
            {
                return Ok((post, true));
            } else {
                return Ok((post, false));
            }
        };

        match post.user_pk.clone() {
            Partition::Team(pk) => {
                let has_perm =
                    Team::has_permission(cli, &Partition::Team(pk.clone()), &user_pk, perm).await?;
                Ok((post, has_perm))
            }
            Partition::User(_) => {
                let has_perm = &post.user_pk == user_pk;
                Ok((post, has_perm))
            }
            _ => Err(Error2::InternalServerError("Invalid post author".into())),
        }
    }

    pub async fn like(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        user_pk: Partition,
    ) -> Result<(), crate::Error2> {
        tracing::info!("Liking post {} by user {}", post_pk, user_pk);
        let post_tx = Self::updater(&post_pk, EntityType::Post)
            .increase_likes(1)
            .transact_write_item();
        let pl_tx = PostLike::new(post_pk, user_pk).create_transact_write_item();

        tracing::info!("Post like transact items: {:?}, {:?}", post_tx, pl_tx);

        cli.transact_write_items()
            .set_transact_items(Some(vec![post_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to like post: {}", e);
                crate::Error2::PostLikeError
            })?;

        Ok(())
    }

    pub async fn unlike(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        user_pk: Partition,
    ) -> Result<(), crate::Error2> {
        let post_tx = Self::updater(&post_pk, EntityType::Post)
            .decrease_likes(1)
            .transact_write_item();
        let pl_tx = PostLike::delete_transact_write_item(
            &post_pk,
            EntityType::PostLike(user_pk.to_string()).to_string(),
        );

        cli.transact_write_items()
            .set_transact_items(Some(vec![post_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to unlike post: {}", e);
                crate::Error2::PostLikeError
            })?;

        Ok(())
    }
}
