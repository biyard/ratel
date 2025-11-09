use crate::{
    Error,
    features::spaces::boards::models::{
        space_category::SpaceCategory, space_post_comment::SpacePostComment,
        space_post_comment_like::SpacePostCommentLike,
    },
    models::{PostCommentLike, SpaceCommon, team::Team, user::User},
    types::{author::Author, *},
    utils::aws::{DynamoClient, SesClient},
};
use bdk::prelude::axum::Json;
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
pub struct SpacePost {
    #[dynamo(index = "gsi3", name = "find_by_space_ordered", pk)]
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    #[dynamo(index = "gsi3", sk)]
    #[dynamo(index = "gsi2", order = 2, sk)]
    #[dynamo(index = "gsi6", sk)]
    pub updated_at: i64,

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
}

impl SpacePost {
    pub fn new(
        space_pk: Partition,
        title: String,
        html_contents: String,
        category_name: String,
        urls: Vec<String>,
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

            title,
            html_contents,
            category_name,
            comments: 0,
            user_pk: pk,
            author_display_name: display_name,
            author_profile_url: profile_url,
            author_username: username,

            urls,
        }
    }

    pub fn keys(space_pk: &Partition, space_post_pk: &Partition) -> (Partition, EntityType) {
        let space_post_id = match space_post_pk {
            Partition::SpacePost(v) => v.to_string(),
            _ => "".to_string(),
        };

        (space_pk.clone(), EntityType::SpacePost(space_post_id))
    }

    #[allow(unused_variables)]
    pub async fn send_email(
        dynamo: &DynamoClient,
        ses: &SesClient,
        user_email: String,
        space: SpaceCommon,
        title: String,
        html_contents: String,
        user: User,
    ) -> Result<Json<()>, Error> {
        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            tracing::warn!("sending email will be skipped for {}", user_email,);
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            use crate::utils::html::create_space_post_html;

            let mut domain = crate::config::get().domain.to_string();
            if domain.contains("localhost") {
                domain = format!("http://{}", domain).to_string();
            } else {
                domain = format!("https://{}", domain).to_string();
            }

            let space_id = match space.pk.clone() {
                Partition::Space(v) => v.to_string(),
                _ => "".to_string(),
            };

            let profile = user.profile_url;
            let username = user.username;
            let display_name = user.display_name;

            let html = create_space_post_html(
                title.clone(),
                html_contents,
                profile,
                display_name,
                username,
                format!("{}/spaces/SPACE%23{}/boards", domain, space_id),
            );

            let text = format!(
                "You're invited to join {space}\n{user} is posting the post in the {space}.\nOpen: {url}",
                space = title.clone(),
                user = space.author_username,
                url = format!("{}/spaces/SPACE%23{}/boards", domain, space_id),
            );

            let mut i = 0;
            let subject = format!("[Ratel] Posting the post in the space");

            while let Err(e) = ses
                .send_mail_html(&user_email, &subject, &html, Some(&text))
                .await
            {
                btracing::notify!(
                    crate::config::get().slack_channel_monitor,
                    &format!("Failed to send email: {:?}", e)
                );
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                i += 1;
                if i >= 3 {
                    return Err(Error::AwsSesSendEmailException(e.to_string()));
                }
            }
        }

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
        let comment_tx = SpacePostComment::updater(&space_post_pk, &comment_pk)
            .increase_likes(1)
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
        let comment_tx = SpacePostComment::updater(&space_post_pk, &comment_pk)
            .decrease_likes(1)
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
