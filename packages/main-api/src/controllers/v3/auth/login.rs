use crate::{
    AppState, Error,
    constants::SESSION_KEY_USER_ID,
    models::{
        UserTelegram, UserTelegramQueryOption,
        user::{User, UserQueryOption},
    },
    transact_write,
    types::{EntityType, Provider},
    utils::{password::hash_password, telegram::parse_telegram_raw},
};
use bdk::prelude::*;

use by_axum::axum::{
    Extension,
    extract::{Json, State},
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum LoginRequest {
    Email {
        email: String,
        password: String,
    },
    OAuth {
        provider: Provider,
        access_token: String,
    },
    Telegram {
        telegram_raw: String,
    },
}

pub async fn login_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<User>, Error> {
    let user = match req {
        LoginRequest::Email { email, password } => {
            login_with_email(&dynamo.client, email, password).await?
        }
        LoginRequest::OAuth {
            provider,
            access_token,
        } => login_with_oauth(&dynamo.client, provider, access_token).await?,
        LoginRequest::Telegram { telegram_raw } => {
            login_with_telegram(&dynamo.client, telegram_raw).await?
        }
    };

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await?;

    Ok(Json(user))
}

pub async fn login_with_oauth(
    cli: &aws_sdk_dynamodb::Client,
    provider: Provider,
    access_token: String,
) -> Result<User, Error> {
    let email = provider.get_email(&access_token).await?;

    let user = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
        .await?
        .0
        .get(0)
        .cloned()
        .ok_or(Error::Unauthorized(
            "No user found with the given email".into(),
        ))?;

    Ok(user)
}

pub async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    email: String,
    password: String,
) -> Result<User, Error> {
    let hashed_password = hash_password(&password);
    let (u, _) = User::find_by_email_and_password(
        cli,
        &email,
        UserQueryOption::builder().sk(hashed_password),
    )
    .await?;
    let user = u
        .get(0)
        .cloned()
        .ok_or(Error::Unauthorized("Invalid email or password".into()))?;

    // FIXME(migrate): fallback to tricky migration from postgres
    // let user = if user.is_none() {
    //     migrate_by_email_password(cli, pool, email, password)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to migrate user by email: {}", e);
    //             Error::Unauthorized("Invalid email or password".into())
    //         })?
    // } else {
    //     user.unwrap()
    // };

    Ok(user)
}

pub async fn login_with_telegram(
    cli: &aws_sdk_dynamodb::Client,
    telegram_raw: String,
) -> Result<User, Error> {
    let telegram_user = parse_telegram_raw(telegram_raw.clone()).map_err(|e| {
        tracing::error!("Failed to parse telegram raw data: {}", e);
        Error::Unauthorized("Invalid telegram data".into())
    })?;
    tracing::debug!("Parsed telegram user: {:?}", telegram_user);
    let (res, _) = UserTelegram::find_by_telegram_id(
        cli,
        telegram_user.id,
        UserTelegramQueryOption::builder().limit(1),
    )
    .await?;
    let user = if res.is_empty() {
        let username = telegram_user
            .username
            .clone()
            .unwrap_or(format!("telegram{}", telegram_user.id))
            .to_lowercase();
        let display_name = format!(
            "{} {}",
            telegram_user.first_name.unwrap_or_default(),
            telegram_user.last_name.unwrap_or_default()
        );
        let email = format!("{}@telegram.placeholder", username);
        let user = User::new(
            display_name,
            email,
            telegram_user.photo_url.unwrap_or_default(),
            false,
            false,
            crate::types::UserType::Anonymous,
            username,
            None,
        );
        let user_telegram = UserTelegram::new(user.pk.clone(), telegram_user.id, telegram_raw);

        transact_write!(
            cli,
            user.create_transact_write_item(),
            user_telegram.create_transact_write_item()
        )?;
        user
    } else {
        let user_telegram = res.first().cloned().ok_or(Error::Unauthorized(
            "No user linked with the given telegram account".into(),
        ))?;
        User::get(cli, &user_telegram.pk, Some(EntityType::User))
            .await?
            .ok_or(Error::Unauthorized(
                "No user linked with the given telegram account".into(),
            ))?
    };

    Ok(user)
}
