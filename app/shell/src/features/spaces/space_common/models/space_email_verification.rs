use crate::features::spaces::space_common::*;
#[cfg(feature = "server")]
use common::models::space::SpaceCommon;
#[cfg(feature = "server")]
use common::utils::aws::SesClient;
#[cfg(feature = "server")]
use common::utils::time::get_now_timestamp;

const EXPIRATION_TIME: u64 = 1800; // 30 minutes
const MAX_ATTEMPT_COUNT: i32 = 5;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default)]
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
}

#[cfg(feature = "server")]
impl SpaceEmailVerification {
    pub async fn expire_verifications(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
    ) -> Result<()> {
        let mut bookmark = None::<String>;
        let mut tx = vec![];

        loop {
            let (responses, new_bookmark) = SpaceEmailVerification::query(
                cli,
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
                    cli.transact_write_items()
                        .set_transact_items(Some(tx.clone()))
                        .send()
                        .await
                        .map_err(|e| {
                            tracing::error!("Failed to update verifications: {:?}", e);
                            Error::InternalServerError(
                                "Failed to update verifications".into(),
                            )
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
            cli.transact_write_items()
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

    pub async fn upsert_verification(
        cli: &aws_sdk_dynamodb::Client,
        space_pk: Partition,
        user_email: String,
    ) -> Result<SpaceEmailVerification> {
        let existing = SpaceEmailVerification::get(
            cli,
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
                    .execute(cli)
                    .await?;

                v.value = code;
                v.expired_at = expired_at;
                v
            }
            None => {
                let code = Self::generate_random_code();
                let expired_at = now + EXPIRATION_TIME as i64;

                let v =
                    SpaceEmailVerification::new(space_pk.clone(), user_email, code, expired_at);
                v.create(cli).await?;
                v
            }
        };

        Ok(verification)
    }

    pub fn html_excerpt_ellipsis(html: &str, max_chars: usize) -> String {
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

    pub async fn send_invitation_emails(
        cli: &aws_sdk_dynamodb::Client,
        ses: &SesClient,
        user_emails: Vec<String>,
        space: &SpaceCommon,
        title: String,
    ) -> Result<()> {
        let mut verifications = Vec::with_capacity(user_emails.len());

        for email in &user_emails {
            let v = Self::upsert_verification(cli, space.pk.clone(), email.clone()).await?;
            verifications.push((email.clone(), v.value.clone()));
        }

        let space_id = match space.pk.clone() {
            Partition::Space(v) => v.to_string(),
            _ => String::new(),
        };

        let cta_url = format!("https://ratel.foundation/spaces/SPACE%23{}", space_id);

        let recipients: Vec<(String, Option<serde_json::Value>)> = user_emails
            .into_iter()
            .map(|email| {
                let data = serde_json::json!({
                    "space_title": title,
                    "space_desc": Self::html_excerpt_ellipsis(&space.content, 160),
                    "author_profile": space.author_profile_url,
                    "author_display_name": space.author_username,
                    "author_username": space.author_display_name,
                    "cta_url": cta_url,
                });
                (email, Some(data))
            })
            .collect();

        if let Err(e) = ses
            .send_bulk_with_template("email_verification", &recipients)
            .await
        {
            tracing::error!("Failed to send invitation emails: {:?}", e);
        }

        Ok(())
    }

    fn generate_random_code() -> String {
        use rand::Rng;
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
