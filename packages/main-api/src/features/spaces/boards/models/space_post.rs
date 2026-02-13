use crate::models::UserNotification;
use crate::services::fcm_notification::FCMService;
use std::collections::HashMap;
use urlencoding::encode;

// #[cfg(all(not(test), not(feature = "no-secret")))]
// use crate::features::spaces::templates::SpaceTemplate;
use crate::email_operation::EmailOperation;
use crate::features::migration::*;
use crate::models::user;
use crate::{
    Error,
    features::spaces::boards::models::{
        space_category::SpaceCategory, space_post_comment::SpacePostComment,
        space_post_comment_like::SpacePostCommentLike,
    },
    models::{
        PostCommentLike, SpaceCommon, email_template::email_template::EmailTemplate, team::Team,
        user::User,
    },
    types::{author::Author, *},
    utils::aws::{DynamoClient, SesClient},
};
use crate::{config, transact_write_all_items};
use aws_sdk_dynamodb::types::TransactWriteItem;
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

    pub title: String,
    pub html_contents: String,
    #[dynamo(index = "gsi6", name = "find_by_category", order = 2, pk)]
    pub category_name: String,
    pub comments: i64,
    #[serde(default)]
    pub reports: i64,

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
            reports: 0,
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

        email.send_email(&dynamo, &ses).await?;
        Ok(Json(()))
    }

    pub async fn send_notification(
        dynamo: &DynamoClient,
        fcm: &mut FCMService,
        space: &SpaceCommon,
        post_title: String,
        recipients: Vec<Partition>,
    ) -> Result<(), Error> {
        if recipients.is_empty() {
            tracing::info!("send_notification: no recipients, skip push");
            return Ok(());
        }

        let title = "Space members are posting new space contents.".to_string();
        let body = post_title;

        let pk_str = space.pk.to_string();
        let space_pk_encoded = encode(&pk_str);
        let deeplink = format!("ratelapp://space/{space_pk_encoded}");

        tracing::info!("send_notification: start, recipients={}", recipients.len());

        UserNotification::send_to_users(dynamo, fcm, &recipients, title, body, Some(deeplink))
            .await?;

        tracing::info!("send_notification: done");
        Ok(())
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
        let comment = SpacePostComment::new(space_pk.clone(), space_post_pk, content, user);
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

#[async_trait::async_trait]
impl TrickyMigrator for SpacePost {
    fn version() -> i32 {
        1
    }

    fn doc_type(pk: String) -> MigrationDataType {
        MigrationDataType::SpacePost(pk)
    }

    async fn migrate(cli: &aws_sdk_dynamodb::Client, pk: String) -> crate::Result<usize> {
        let opt = SpacePost::opt_all();

        let (items, _bookmark) = SpacePost::find_by_space_ordered(cli, pk, opt).await?;

        let affected = items.len();
        let txs: Vec<TransactWriteItem> = items
            .into_iter()
            .map(|post| post.upsert_transact_write_item())
            .collect();

        transact_write_all_items!(cli, txs);

        Ok(affected)
    }
}
