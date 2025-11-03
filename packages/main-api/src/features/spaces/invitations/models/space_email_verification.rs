use crate::Error;
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

    pub async fn send_email(
        dynamo: &DynamoClient,
        ses: &SesClient,
        user_email: String,
        space_pk: Partition,
    ) -> Result<Json<()>, Error> {
        let verification = SpaceEmailVerification::get(
            &dynamo.client,
            &space_pk,
            Some(EntityType::SpaceEmailVerification(user_email.clone())),
        )
        .await?;

        let SpaceEmailVerification { value, .. } = if !verification.is_none()
            && verification.clone().unwrap().expired_at > get_now_timestamp()
            && verification.clone().unwrap().attempt_count < MAX_ATTEMPT_COUNT
        {
            verification.clone().unwrap_or_default()
        } else {
            let code = Self::generate_random_code();
            let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;

            if verification.is_some() {
                let mut v = verification.unwrap_or_default();
                SpaceEmailVerification::updater(v.pk.clone(), v.sk.clone())
                    .with_attempt_count(0)
                    .with_value(code.clone())
                    .with_expired_at(expired_at)
                    .execute(&dynamo.client)
                    .await?;
                v.value = code;
                v.expired_at = expired_at;
                v
            } else {
                let email_verification = SpaceEmailVerification::new(
                    space_pk.clone(),
                    user_email.clone(),
                    code,
                    expired_at,
                );
                email_verification.create(&dynamo.client).await?;
                email_verification
            }
        };

        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            tracing::warn!(
                "sending email will be skipped for {}: {}",
                user_email,
                value
            );
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            let mut domain = crate::config::get().domain.to_string();
            if domain.contains("localhost") {
                domain = format!("http://{}", domain).to_string();
            } else {
                domain = format!("https://{}", domain).to_string();
            }

            let mut i = 0;
            let space_id = match space_pk.clone() {
                Partition::Space(v) => v.to_string(),
                _ => "".to_string(),
            };

            tracing::debug!("space id: {:?}", space_id);

            while let Err(e) = ses
                .send_mail(
                    &user_email,
                    format!("Join your space within 30 minutes with your verification code").as_ref(),
                    format!("Please enter this verification code within 30 minutes to complete your invitation.\nInvite account: {}\nSpace link: {}/spaces/SPACE%23{}/members?code={}", user_email, domain, space_id, value).as_ref(),
                )
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

    fn generate_random_code() -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let mut rng = rand::rng();
        let code: String = (0..6)
            .map(|_| {
                let idx = rng.random_range(0..charset.len());
                charset[idx] as char
            })
            .collect();
        code
    }
}
