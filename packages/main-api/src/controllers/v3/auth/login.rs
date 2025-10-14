use crate::{
    AppState, Error2,
    constants::SESSION_KEY_USER_ID,
    models::user::{User, UserQueryOption},
    types::Provider,
    utils::password::hash_password,
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
) -> Result<Json<User>, Error2> {
    let user = match req {
        LoginRequest::Email { email, password } => {
            login_with_email(&dynamo.client, email, password).await?
        }
        LoginRequest::OAuth {
            provider,
            access_token,
        } => login_with_oauth(&dynamo.client, provider, access_token).await?,
        LoginRequest::Telegram { .. } => {
            // Handle Telegram login
            // Not implemented yet
            return Err(Error2::BadRequest("Telegram login not implemented".into()));
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
) -> Result<User, Error2> {
    let email = provider.get_email(&access_token).await?;

    let user = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1))
        .await?
        .0
        .get(0)
        .cloned().ok_or(Error2::Unauthorized(
            "No user found with the given email".into(),
        ))?;

    Ok(user)
}

pub async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    email: String,
    password: String,
) -> Result<User, Error2> {
    let hashed_password = hash_password(&password);
    let (u, _) = User::find_by_email_and_password(
        cli,
        &email,
        UserQueryOption::builder().sk(hashed_password),
    )
    .await?;
    let user = u.get(0).cloned().ok_or(Error2::Unauthorized(
        "Invalid email or password".into(),
    ))?;

    // FIXME(migrate): fallback to tricky migration from postgres
    // let user = if user.is_none() {
    //     migrate_by_email_password(cli, pool, email, password)
    //         .await
    //         .map_err(|e| {
    //             tracing::error!("Failed to migrate user by email: {}", e);
    //             Error2::Unauthorized("Invalid email or password".into())
    //         })?
    // } else {
    //     user.unwrap()
    // };

    Ok(user)
}
