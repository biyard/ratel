use crate::Error;
use crate::email_operation::EmailOperation;
use crate::models::email_template::email_template::EmailTemplate;
use crate::models::{SpaceCommon, UserNotification};
use crate::{
    constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT},
    types::*,
    utils::{
        aws::{DynamoClient, SesClient},
        time::get_now_timestamp,
    },
};
use bdk::prelude::axum::Json;
use bdk::prelude::*;
use rand::Rng;
use regex::Regex;
use serde_json::json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceEmailVerification {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub email: String,
    pub value: String,
    pub expired_at: i64,
    pub attempt_count: i32,

    pub authorized: bool,
}

impl SpaceEmailVerification {
    pub fn new(space_pk: Partition, email: String, value: String, expired_at: i64) -> Self {
        let pk = space_pk;
        let sk = EntityType::SpaceEmailVerification(email.clone());
        let created_at = chrono::Utc::now().timestamp_micros();

        Self {
            pk,
            sk,
            email,
            created_at,
            value,
            expired_at,
            attempt_count: 0,
            authorized: false,
        }
    }

    pub async fn expire_verifications(
        dynamo: &DynamoClient,
        space_pk: Partition,
    ) -> Result<(), Error> {
        let mut bookmark = None::<String>;
        let mut tx = vec![];

        loop {
            let (responses, new_bookmark) = SpaceEmailVerification::query(
                &dynamo.client,
                space_pk.clone(),
                if let Some(b) = &bookmark {
                    SpaceEmailVerificationQueryOption::builder()
                        .sk("SPACE_EMAIL_VERIFICATION#".into())
                        .bookmark(b.clone())
                } else {
                    SpaceEmailVerificationQueryOption::builder()
                        .sk("SPACE_EMAIL_VERIFICATION#".into())
                },
            )
            .await?;

            let expired_at = get_now_timestamp();

            for response in responses {
                let d = SpaceEmailVerification::updater(response.pk, response.sk)
                    .with_expired_at(expired_at)
                    .transact_write_item();

                tx.push(d);

                if tx.len() == 10 {
                    dynamo
                        .client
                        .transact_write_items()
                        .set_transact_items(Some(tx.clone()))
                        .send()
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to update verifications: {:?}", e);
                            Error::InternalServerError("Failed to update verifications".into())
                        })?;

                    tx.clear();
                }
            }

            match new_bookmark {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }

        if !tx.is_empty() {
            dynamo
                .client
                .transact_write_items()
                .set_transact_items(Some(tx.clone()))
                .send()
                .await
                .map_err(|e| {
                    tracing::error!("Failed to update verifications: {:?}", e);
                    Error::InternalServerError("Failed to update verifications".into())
                })?;
        }

        Ok(())
    }

    async fn upsert_verification(
        dynamo: &DynamoClient,
        space_pk: Partition,
        user_email: String,
    ) -> Result<SpaceEmailVerification, Error> {
        let existing = SpaceEmailVerification::get(
            &dynamo.client,
            &space_pk,
            Some(EntityType::SpaceEmailVerification(user_email.clone())),
        )
        .await?;

        let now = get_now_timestamp();

        let verification = match existing {
            Some(v) if v.expired_at > now && v.attempt_count < MAX_ATTEMPT_COUNT => v,
            Some(mut v) => {
                let code = Self::generate_random_code();
                let expired_at = now + EXPIRATION_TIME as i64;

                SpaceEmailVerification::updater(v.pk.clone(), v.sk.clone())
                    .with_attempt_count(0)
                    .with_value(code.clone())
                    .with_expired_at(expired_at)
                    .execute(&dynamo.client)
                    .await?;

                v.value = code;
                v.expired_at = expired_at;
                v
            }
            None => {
                let code = Self::generate_random_code();
                let expired_at = now + EXPIRATION_TIME as i64;

                let v = SpaceEmailVerification::new(space_pk.clone(), user_email, code, expired_at);
                v.create(&dynamo.client).await?;
                v
            }
        };

        Ok(verification)
    }

    // #[cfg(all(not(test), not(feature = "no-secret")))]
    // async fn ensure_invite_template_exists(
    //     dynamo: &DynamoClient,
    //     ses: &SesClient,
    //     template_name: &str,
    // ) -> Result<(), Error> {
    //     use crate::features::spaces::templates::SpaceTemplate;
    //     use crate::utils::templates::{INVITE_SPACE_TEMPLATE_HTML, INVITE_SPACE_TEMPLATE_SUBJECT};

    //     let template = SpaceTemplate::get(
    //         &dynamo.client,
    //         Partition::SpaceTemplate,
    //         Some(EntityType::SpaceTemplate(template_name.to_string())),
    //     )
    //     .await?;

    //     if template.is_none() {
    //         ses.create_template(
    //             template_name,
    //             INVITE_SPACE_TEMPLATE_SUBJECT,
    //             INVITE_SPACE_TEMPLATE_HTML,
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
    ) -> Result<Json<()>, Error> {
        let mut verifications = Vec::with_capacity(user_emails.len());

        for email in &user_emails {
            let v = Self::upsert_verification(dynamo, space.pk.clone(), email.clone()).await?;
            verifications.push((email.clone(), v.value.clone()));
        }

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

        let cta_url = format!("{}/spaces/SPACE%23{}", domain, space_id);

        let email = EmailTemplate {
            targets: user_emails.clone(),
            operation: EmailOperation::SpaceInviteVerification {
                space_title: title.clone(),
                space_desc: Self::html_excerpt_ellipsis(&space.content, 160),
                author_profile: space.author_profile_url,
                author_display_name: space.author_username.clone(),
                author_username: space.author_display_name,
                cta_url,
            },
        };

        email.send_email(&dynamo, &ses).await?;

        Ok(Json(()))
    }

    pub async fn send_notification(
        dynamo: &DynamoClient,
        recipients: Vec<Partition>,
        space: &SpaceCommon,
        title: String,
    ) -> Result<(), Error> {
        if recipients.is_empty() {
            tracing::info!(
                "SpaceEmailVerification::send_notification: no recipients, skip push (space_pk={})",
                space.pk
            );
            return Ok(());
        }

        tracing::info!(
            "SpaceEmailVerification::send_notification: start, space_pk={}, recipients={}",
            space.pk,
            recipients.len()
        );

        let notif_title = "You are invited in space.".to_string();

        let excerpt = Self::html_excerpt_ellipsis(&space.content, 80);
        let notif_body = if excerpt.is_empty() {
            format!("Participate new space: {}", title)
        } else {
            format!("Participate new space: {} — {}", title, excerpt)
        };

        for user_pk in recipients {
            tracing::debug!(
                "SpaceEmailVerification::send_notification: sending to user_pk={}",
                user_pk
            );

            UserNotification::send_to_user(
                dynamo,
                &user_pk,
                notif_title.clone(),
                notif_body.clone(),
            )
            .await?;
        }

        tracing::info!(
            "SpaceEmailVerification::send_notification: done for space_pk={}",
            space.pk
        );

        Ok(())
    }

    #[allow(dead_code)]
    fn html_excerpt_ellipsis(html: &str, max_chars: usize) -> String {
        let re = regex::Regex::new(r"(?is)<[^>]+>").unwrap();
        let no_tags = re.replace_all(html, "");
        let squashed = no_tags.split_whitespace().collect::<Vec<_>>().join(" ");
        let mut s = String::new();
        for (i, ch) in squashed.chars().enumerate() {
            if i >= max_chars {
                s.push('…');
                break;
            }
            s.push(ch);
        }
        s
    }

    fn generate_random_code() -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();
        (0..6)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }
}
