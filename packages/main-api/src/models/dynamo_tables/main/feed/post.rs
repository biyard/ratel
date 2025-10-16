use crate::{
    Error2,
    models::{PostCommentLike, team::Team, user::User},
    types::{author::Author, *},
};
use bdk::prelude::*;

use super::{PostComment, PostLike};

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
    #[dynamo(index = "gsi2", order = 2, sk)]
    pub updated_at: i64,

    pub title: String,
    pub html_contents: String,
    pub post_type: PostType,

    #[dynamo(index = "gsi5", sk)]
    pub status: PostStatus,

    #[dynamo(index = "gsi6", name = "find_by_visibility", pk)]
    #[dynamo(index = "gsi2", order = 1, sk)]
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
    pub space_visibility: Option<SpaceVisibility>,
    pub booster: Option<BoosterType>,
    // only for reward spaces
    pub rewards: Option<i64>,

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
        let now = chrono::Utc::now().timestamp();
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
            urls: vec![],
            space_visibility: None,
        }
    }

    pub async fn get_permissions(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user: Option<User>,
    ) -> Result<TeamGroupPermissions, crate::Error2> {
        if user.is_none() {
            return Ok(self.get_permissions_for_guest());
        }

        let user = user.unwrap();

        if self.user_pk == user.pk {
            return Ok(TeamGroupPermissions::all());
        }

        if self.author_type == UserType::Individual {
            return Ok(self.get_permissions_for_guest());
        }

        match self.user_pk.clone() {
            team_pk if matches!(team_pk, Partition::Team(_)) => {
                return Team::get_permissions_by_team_pk(cli, &team_pk, &user.pk).await;
            }
            _ => {
                return Err(Error2::NotSupported(format!(
                    "Post({}) author type {:?} is not supported",
                    self.pk, self.author_type
                )));
            }
        }
    }

    fn get_permissions_for_guest(&self) -> TeamGroupPermissions {
        if self.status == PostStatus::Published && self.visibility == Some(Visibility::Public) {
            return TeamGroupPermissions::read();
        }

        TeamGroupPermissions::empty()
    }

    pub async fn is_liked(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> Result<bool, crate::Error2> {
        Ok(PostLike::find_one(cli, &self.pk, user_pk).await?.is_some())
    }

    pub async fn has_permission(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: &Partition,
        user_pk: Option<&Partition>,
        perm: TeamGroupPermission,
    ) -> Result<(Self, bool), crate::Error2> {
        let post = Post::get(cli, post_pk, Some(EntityType::Post))
            .await?
            .ok_or(Error2::PostNotFound)?;

        if post.status == PostStatus::Published && post.visibility == Some(Visibility::Public) {
            return Ok((post, true));
        }

        if user_pk.is_none() {
            return Ok((post, false));
        }

        let user_pk = user_pk.unwrap();

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
        tracing::debug!("Liking post {} by user {}", post_pk, user_pk);
        let post_tx = Self::updater(&post_pk, EntityType::Post)
            .increase_likes(1)
            .transact_write_item();
        let pl_tx = PostLike::new(post_pk, user_pk).create_transact_write_item();

        tracing::debug!("Post like transact items: {:?}, {:?}", post_tx, pl_tx);

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
        let (p, s) = PostLike::keys(&post_pk, &user_pk);
        let pl_tx = PostLike::delete_transact_write_item(p, s);

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

    pub async fn comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        content: String,
        user: User,
    ) -> Result<PostComment, crate::Error2> {
        let post = Post::updater(&post_pk, EntityType::Post)
            .increase_comments(1)
            .transact_write_item();
        let comment = PostComment::new(post_pk, content, user);
        let comment_tx = comment.create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, post]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to add comment: {}", e);
                crate::Error2::PostCommentError
            })?;

        Ok(comment)
    }

    pub async fn like_comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        comment_pk: EntityType,
        user_pk: Partition,
    ) -> Result<(), crate::Error2> {
        let comment_tx = PostComment::updater(&post_pk, &comment_pk)
            .increase_likes(1)
            .transact_write_item();
        let pl_tx = PostCommentLike::new(post_pk, comment_pk, user_pk).create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to like comment: {}", e);
                crate::Error2::PostLikeError
            })?;
        Ok(())
    }

    pub async fn unlike_comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        comment_pk: EntityType,
        user_pk: Partition,
    ) -> Result<(), crate::Error2> {
        let comment_tx = PostComment::updater(&post_pk, &comment_pk)
            .decrease_likes(1)
            .transact_write_item();
        let pcl = PostCommentLike::new(post_pk, comment_pk, user_pk);
        let pl_tx = PostCommentLike::delete_transact_write_item(&pcl.pk, &pcl.sk);

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to unlike comment: {}", e);
                crate::Error2::PostLikeError
            })?;
        Ok(())
    }
}
