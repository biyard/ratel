// #[cfg(all(not(test), not(feature = "no-secret")))]
// use crate::features::spaces::templates::SpaceTemplate;
use crate::{
    Error,
    features::spaces::boards::models::{
        space_category::SpaceCategory, space_post_comment::SpacePostComment,
        space_post_comment_like::SpacePostCommentLike,
    },
    models::{
        PostCommentLike, SpaceCommon,
        email_template::email_template::{EmailOperation, EmailTemplate},
        team::Team,
        user::User,
    },
    types::{author::Author, *},
    utils::aws::{DynamoClient, SesClient},
};
use bdk::prelude::axum::Json;
use bdk::prelude::*;
use serde_json::json;

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
pub struct SpacePost {
    #[dynamo(index = "gsi3", name = "find_by_space_ordered", pk)]
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    #[dynamo(index = "gsi3", sk)]
    #[dynamo(index = "gsi2", order = 2, sk)]
    #[dynamo(index = "gsi6", sk)]
    pub updated_at: i64,

    #[serde(default)]
    pub started_at: i64,
    #[serde(default)]
    pub ended_at: i64,

    pub title: String,
    pub html_contents: String,
    #[dynamo(index = "gsi6", name = "find_by_cagetory", pk)]
    #[dynamo(index = "gsi2", order = 1, sk)]
    pub category_name: String,
    pub comments: i64,

    #[dynamo(
        prefix = "USER_VISIBILITY",
        name = "find_by_user_pk_visibility",
        index = "gsi2",
        pk
    )]
    pub user_pk: Partition,
    pub author_display_name: String,
    pub author_profile_url: String,
    pub author_username: String,

    pub urls: Vec<String>,
    pub files: Option<Vec<File>>,
}

impl SpacePost {
    pub fn new(
        space_pk: Partition,
        title: String,
        html_contents: String,
        category_name: String,
        urls: Vec<String>,
        files: Option<Vec<File>>,
        started_at: i64,
        ended_at: i64,
        User {
            pk,
            display_name,
            profile_url,
            username,
            ..
        }: User,
    ) -> Self {
        let now = chrono::Utc::now().timestamp_micros();
        let uuid = uuid::Uuid::new_v4().to_string();
        Self {
            pk: space_pk,
            sk: EntityType::SpacePost(uuid),
            created_at: now,
            updated_at: now,
            started_at,
            ended_at,

            title,
            html_contents,
            category_name,
            comments: 0,
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,

            urls,
            files,
        }
    }

    pub fn keys(space_pk: &Partition, space_post_pk: &Partition) -> (Partition, EntityType) {
        let space_post_id = match space_post_pk {
            Partition::SpacePost(v) => v.to_string(),
            _ => "".to_string(),
        };

        (space_pk.clone(), EntityType::SpacePost(space_post_id))
    }

    // #[cfg(all(not(test), not(feature = "no-secret")))]
    // async fn ensure_space_post_template_exists(
    //     dynamo: &DynamoClient,
    //     ses: &SesClient,
    //     template_name: &str,
    // ) -> Result<(), Error> {
    //     use crate::utils::templates::{
    //         CREATE_SPACE_POST_TEMPLATE_HTML, CREATE_SPACE_POST_TEMPLATE_SUBJECT,
    //     };

    //     let template = SpaceTemplate::get(
    //         &dynamo.client,
    //         Partition::SpaceTemplate,
    //         Some(EntityType::SpaceTemplate(template_name.to_string())),
    //     )
    //     .await?;

    //     if template.is_none() {
    //         ses.create_template(
    //             template_name,
    //             CREATE_SPACE_POST_TEMPLATE_SUBJECT,
    //             CREATE_SPACE_POST_TEMPLATE_HTML,
    //         )
    //         .await
    //         .map_err(|e| Error::AwsSesSendEmailException(e.to_string()))?;

    //         let temp = SpaceTemplate::new(template_name.to_string());
    //         temp.create(&dynamo.client).await?;
    //     }

    //     Ok(())
    // }

    #[allow(unused_variables)]
    pub async fn send_email(
        dynamo: &DynamoClient,
        ses: &SesClient,
        user_emails: Vec<String>,
        space: SpaceCommon,
        title: String,
        html_contents: String,
        user: User,
    ) -> Result<Json<()>, Error> {
        let mut domain = crate::config::get().domain.to_string();
        if domain.contains("localhost") {
            domain = format!("http://{}", domain);
        } else {
            domain = format!("https://{}", domain);
        }

        let space_id = match space.pk.clone() {
            Partition::Space(v) => v.to_string(),
            _ => "".to_string(),
        };

        let connect_link = format!("{}/spaces/SPACE%23{}/boards", domain, space_id);

        let email = EmailTemplate {
            targets: user_emails.clone(),
            operation: EmailOperation::SpacePostNotification {
                author_profile: user.profile_url,
                author_display_name: user.display_name,
                author_username: user.username,
                post_title: title,
                post_desc: html_contents,
                connect_link,
            },
        };

        email.send_email(ses).await?;

        Ok(Json(()))
    }

    pub async fn comment(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        space_post_pk: Partition,
        content: String,
        user: User,
    ) -> Result<SpacePostComment, crate::Error> {
        let (pk, sk) = SpacePost::keys(&space_pk, &space_post_pk);
        let post = SpacePost::updater(&pk, sk)
            .increase_comments(1)
            .transact_write_item();
        let comment = SpacePostComment::new(space_post_pk, content, user);
        let comment_tx = comment.create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, post]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to add comment: {}", e);
                crate::Error::PostCommentError
            })?;

        Ok(comment)
    }

    pub async fn like_comment(
        cli: &aws_sdk_dynamodb::Client,
        space_post_pk: Partition,
        comment_pk: EntityType,
        user_pk: Partition,
    ) -> Result<(), crate::Error> {
        let comment = SpacePostComment::get(cli, space_post_pk.clone(), Some(comment_pk.clone()))
            .await?
            .ok_or(crate::Error::PostCommentError)?;

        let new_likes = comment.likes.saturating_add(1);
        let new_likes_align = format!("{:020}", new_likes);

        let comment_tx = SpacePostComment::updater(&space_post_pk, &comment_pk)
            .with_likes(new_likes)
            .with_likes_align(new_likes_align)
            .transact_write_item();

        let pl_tx = SpacePostCommentLike::new(space_post_pk, comment_pk, user_pk)
            .create_transact_write_item();

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to like comment: {}", e);
                crate::Error::PostLikeError
            })?;

        Ok(())
    }

    pub async fn unlike_comment(
        cli: &aws_sdk_dynamodb::Client,
        space_post_pk: Partition,
        comment_pk: EntityType,
        user_pk: Partition,
    ) -> Result<(), crate::Error> {
        let comment = SpacePostComment::get(cli, space_post_pk.clone(), Some(comment_pk.clone()))
            .await?
            .ok_or(crate::Error::PostCommentError)?;

        let new_likes = comment.likes.saturating_sub(1);
        let new_likes_align = format!("{:020}", new_likes);

        let comment_tx = SpacePostComment::updater(&space_post_pk, &comment_pk)
            .with_likes(new_likes)
            .with_likes_align(new_likes_align)
            .transact_write_item();

        let pcl = SpacePostCommentLike::new(space_post_pk, comment_pk, user_pk);
        let pl_tx = SpacePostCommentLike::delete_transact_write_item(&pcl.pk, &pcl.sk);

        cli.transact_write_items()
            .set_transact_items(Some(vec![comment_tx, pl_tx]))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Failed to unlike comment: {}", e);
                crate::Error::PostLikeError
            })?;

        Ok(())
    }
}
