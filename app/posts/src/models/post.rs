use crate::types::*;
use crate::*;

#[cfg(feature = "server")]
use super::{PostComment, PostCommentLike, PostLike, Team};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct Post {
    pub pk: Partition,
    pub sk: EntityType,

    #[dynamo(index = "gsi1", sk)]
    pub created_at: i64,
    #[dynamo(index = "gsi2", order = 2, sk)]
    #[dynamo(index = "gsi5", order = 3, sk)]
    #[dynamo(index = "gsi6", sk)]
    pub updated_at: i64,

    pub title: String,
    pub html_contents: String,
    pub post_type: PostType,

    #[dynamo(index = "gsi5", order = 1, sk)]
    pub status: PostStatus,

    #[dynamo(index = "gsi6", name = "find_by_visibility", pk)]
    #[dynamo(index = "gsi2", order = 1, sk)]
    #[dynamo(index = "gsi5", order = 2, sk)]
    pub visibility: Option<Visibility>,

    pub shares: i64,
    pub likes: i64,
    pub comments: i64,

    #[serde(default)]
    pub reports: i64,

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
    pub author_type: ratel_auth::UserType,

    pub space_pk: Option<Partition>,
    pub space_type: Option<SpaceType>,
    pub space_visibility: Option<SpaceVisibility>,
    pub booster: Option<BoosterType>,
    pub rewards: Option<i64>,

    pub urls: Vec<String>,
}

#[cfg(feature = "server")]
impl Post {
    pub fn draft(author: Author) -> Self {
        Self::new("", "", PostType::Post, author)
    }

    pub fn new<T: Into<String>>(
        title: T,
        html_contents: T,
        post_type: PostType,
        author: Author,
    ) -> Self {
        let uid = uuid::Uuid::now_v7().to_string();
        let now = chrono::Utc::now().timestamp_millis();
        let Author {
            pk,
            display_name,
            profile_url,
            username,
            user_type,
        } = author;

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
            reports: 0,

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
        user: Option<ratel_auth::User>,
    ) -> Result<TeamGroupPermissions> {
        if user.is_none() {
            return Ok(self.get_permissions_for_guest());
        }

        let user = user.unwrap();

        if self.user_pk == user.pk {
            return Ok(TeamGroupPermissions::all());
        }

        if self.author_type == ratel_auth::UserType::Individual {
            return Ok(self.get_permissions_for_guest());
        }

        let gperm = self.get_permissions_for_guest();
        let perm = match self.user_pk.clone() {
            team_pk if matches!(team_pk, Partition::Team(_)) => {
                Team::get_permissions_by_team_pk(cli, &team_pk, &user.pk).await? | gperm
            }
            _ => gperm,
        };

        Ok(perm)
    }

    fn get_permissions_for_guest(&self) -> TeamGroupPermissions {
        if self.status == PostStatus::Published && self.visibility == Some(Visibility::Public) {
            if self.space_visibility == Some(SpaceVisibility::Public) {
                return TeamGroupPermissions::read();
            } else {
                return TeamGroupPermissions(vec![TeamGroupPermission::PostRead]);
            }
        }

        TeamGroupPermissions::empty()
    }

    pub async fn is_liked(
        &self,
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> Result<bool> {
        Ok(PostLike::find_one(cli, &self.pk, user_pk).await?.is_some())
    }

    pub async fn has_permission(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: &Partition,
        user_pk: Option<&Partition>,
        perm: TeamGroupPermission,
    ) -> Result<(Self, bool)> {
        let post = Post::get(cli, post_pk, Some(EntityType::Post))
            .await?
            .ok_or(Error::NotFound("Post not found".into()))?;

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
                    Team::has_permission(cli, &Partition::Team(pk.clone()), user_pk, perm).await?;
                Ok((post, has_perm))
            }
            Partition::User(_) => {
                let has_perm = &post.user_pk == user_pk;
                Ok((post, has_perm))
            }
            _ => Err(Error::InternalServerError("Invalid post author".into())),
        }
    }

    pub async fn like(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        user_pk: Partition,
    ) -> Result<()> {
        tracing::debug!("Liking post {} by user {}", post_pk, user_pk);
        let post_tx = Self::updater(&post_pk, EntityType::Post)
            .increase_likes(1)
            .transact_write_item();
        let pl_tx = PostLike::new(post_pk, user_pk).create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![post_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to like post: {}", e);
                Error::InternalServerError("Failed to like post".into())
            })?;

        Ok(())
    }

    pub async fn unlike(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        user_pk: Partition,
    ) -> Result<()> {
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
                Error::InternalServerError("Failed to unlike post".into())
            })?;

        Ok(())
    }

    pub async fn comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        content: String,
        user: ratel_auth::User,
    ) -> Result<PostComment> {
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
                Error::InternalServerError("Failed to add comment".into())
            })?;

        Ok(comment)
    }

    pub async fn like_comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        comment_pk: EntityType,
        user_pk: Partition,
    ) -> Result<()> {
        let comment_partition = match (&post_pk, &comment_pk) {
            (Partition::Feed(_), EntityType::PostCommentReply(_, _)) => {
                Partition::PostReply(post_pk.to_string())
            }
            _ => post_pk.clone(),
        };
        let comment_tx = PostComment::updater(&comment_partition, &comment_pk)
            .increase_likes(1)
            .transact_write_item();
        let pl_tx = PostCommentLike::new(post_pk, comment_pk, user_pk).create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to like comment: {}", e);
                Error::InternalServerError("Failed to like comment".into())
            })?;
        Ok(())
    }

    pub async fn unlike_comment(
        cli: &aws_sdk_dynamodb::Client,
        post_pk: Partition,
        comment_pk: EntityType,
        user_pk: Partition,
    ) -> Result<()> {
        let comment_partition = match (&post_pk, &comment_pk) {
            (Partition::Feed(_), EntityType::PostCommentReply(_, _)) => {
                Partition::PostReply(post_pk.to_string())
            }
            _ => post_pk.clone(),
        };
        let comment_tx = PostComment::updater(&comment_partition, &comment_pk)
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
                Error::InternalServerError("Failed to unlike comment".into())
            })?;
        Ok(())
    }
}
