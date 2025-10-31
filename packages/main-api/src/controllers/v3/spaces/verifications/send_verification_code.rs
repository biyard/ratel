use crate::constants::{ATTEMPT_BLOCK_TIME, EXPIRATION_TIME, MAX_ATTEMPT_COUNT};
use crate::controllers::v3::spaces::SpacePathParam;
use crate::features::spaces::polls::Poll;
use crate::models::feed::Post;
use crate::models::space::SpaceCommon;
use crate::models::user::User;
use crate::models::{EmailVerification, EmailVerificationQueryOption};
use crate::spaces::SpacePath;
use crate::types::{EntityType, Partition, SpaceType, TeamGroupPermission};
use crate::utils::time::get_now_timestamp;
use crate::{AppState, Error, transact_write_items};
use aide::NoApi;
use axum::extract::{Json, Path, State};
use bdk::prelude::*;
use rand::Rng;

use serde::{Deserialize, Serialize};

// #[derive(Debug, Deserialize, aide::OperationIo, JsonSchema)]
// pub struct CreateSpacePathParams {
//     post_pk: Partition,
// }

#[derive(Debug, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct SendVerificationCodeRequest {
    pub user_pks: Vec<Partition>,
}

#[derive(Debug, Serialize, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct SendVerificationCodeResponse {
    pub space_pk: Partition,
}

pub async fn send_verification_code_handler(
    State(AppState { dynamo, ses, .. }): State<AppState>,
    NoApi(user): NoApi<User>,
    Path(SpacePathParam { space_pk }): SpacePath,
    Json(SendVerificationCodeRequest { user_pks }): Json<SendVerificationCodeRequest>,
) -> Result<Json<SendVerificationCodeResponse>, Error> {
    if !matches!(space_pk, Partition::Space(_)) {
        return Err(Error::NotFoundSpace);
    }

    let (_space_common, has_perm) = SpaceCommon::has_permission(
        &dynamo.client,
        &space_pk,
        Some(&user.pk),
        TeamGroupPermission::SpaceEdit,
    )
    .await?;
    if !has_perm {
        return Err(Error::NoPermission);
    }

    for user_pk in user_pks {
        let user = User::get(&dynamo.client, user_pk.clone(), Some(EntityType::User))
            .await?
            .ok_or(Error::NotFound("User not found".into()))?;

        let (verification_list, _) = EmailVerification::find_by_email(
            &dynamo.client,
            &user.email,
            EmailVerificationQueryOption::builder().limit(1),
        )
        .await?;

        let EmailVerification { value, .. } = if !verification_list.is_empty()
            && verification_list[0].expired_at > get_now_timestamp()
            && verification_list[0].attempt_count < MAX_ATTEMPT_COUNT
        {
            verification_list[0].clone()
        } else if !verification_list.is_empty()
            && verification_list[0].attempt_count >= MAX_ATTEMPT_COUNT
            && verification_list[0].expired_at < (get_now_timestamp() - ATTEMPT_BLOCK_TIME)
        {
            return Err(Error::ExceededAttemptEmailVerification);
        } else {
            let code = generate_random_code();
            let expired_at = get_now_timestamp() + EXPIRATION_TIME as i64;

            if verification_list.len() > 0 {
                let mut v = verification_list[0].clone();
                EmailVerification::updater(v.pk.clone(), v.sk.clone())
                    .with_attempt_count(0)
                    .with_value(code.clone())
                    .with_expired_at(expired_at)
                    .execute(&dynamo.client)
                    .await?;
                v.value = code;
                v.expired_at = expired_at;
                v
            } else {
                let email_verification =
                    EmailVerification::new(user.email.clone(), code, expired_at);
                email_verification.create(&dynamo.client).await?;
                email_verification
            }
        };
        #[cfg(any(test, feature = "no-secret"))]
        {
            let _ = ses;
            tracing::warn!(
                "sending email will be skipped for {}: {}",
                user.email,
                value
            );
        }

        #[cfg(all(not(test), not(feature = "no-secret")))]
        {
            let mut i = 0;
            while let Err(e) = ses
                .send_mail(
                    &user.email,
                    "Please finish to sign up within 30 minutes with your verification code",
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
    }

    Ok(Json(SendVerificationCodeResponse { space_pk }))
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
