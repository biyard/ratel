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
        } else if !verification.clone().is_none()
            && verification.clone().unwrap().attempt_count < MAX_ATTEMPT_COUNT
            && verification.clone().unwrap().expired_at < (get_now_timestamp() - ATTEMPT_BLOCK_TIME)
        {
            return Err(Error::ExceededAttemptEmailVerification);
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

            while let Err(e) = ses
                .send_mail(
                    &user_email,
                    format!("Please Enter this verification code within 30 minutes with your verification code in space.\nspace link: {}/spaces/SPACE%23{}", domain, space_id).as_ref(),
                    format!("Verification code: {:?}", value).as_ref(),
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
