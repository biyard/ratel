use crate::{
    AppState, Error2, config,
    models::{
        migrators::user::migrate_by_email_password,
        user::{
            User, UserOAuth, UserOAuthQueryOption, UserPrincipal, UserPrincipalQueryOption,
            UserQueryOption,
        },
    },
    types::Provider,
    utils::{dynamo_extractor::get_principal_from_auth, firebase},
};
use bdk::prelude::*;

use dto::{
    JsonSchema, aide,
    by_axum::{
        auth::{Authorization, DYNAMO_USER_SESSION_KEY, DynamoUserSession, generate_jwt},
        axum::{
            Extension,
            extract::{Json, State},
            http::{HeaderMap, header::SET_COOKIE},
        },
    },
    by_types::{Claims, JsonWithHeaders},
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Debug, Clone, Deserialize, aide::OperationIo, JsonSchema)]
#[serde(untagged)]
pub enum LoginRequest {
    Email { email: String, password: String },
    OAuth { provider: Provider, token: String },
    Telegram { telegram_raw: String },
}

pub type LoginResponse = (HeaderMap, ());

pub async fn login_handler(
    State(AppState { dynamo, pool, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<LoginRequest>,
) -> Result<JsonWithHeaders<User>, Error2> {
    let user = match req {
        LoginRequest::Email { email, password } => {
            login_with_email(&dynamo.client, &pool, email, password).await?
        }
        LoginRequest::OAuth { provider, token } => match provider {
            Provider::Google => {
                let uid = firebase::oauth::verify_token(&token).await?;

                // For Migration from Principal Login to Google OAuth.
                // Remove this code after the users have logged in with Google OAuth.
                if let Ok(principal) = get_principal_from_auth(auth) {
                    let (user_principal, _) = UserPrincipal::find_by_principal(
                        &dynamo.client,
                        principal,
                        UserPrincipalQueryOption::builder(),
                    )
                    .await?;
                    let user = user_principal
                        .get(0)
                        .cloned()
                        .ok_or(Error2::Unauthorized("Invalid email or password".into()))?;
                    UserOAuth::new(user.pk, Provider::Google, uid.clone())
                        .create(&dynamo.client)
                        .await?;
                }
                let (u, _) = UserOAuth::find_by_provider_and_uid(
                    &dynamo.client,
                    uid,
                    UserOAuthQueryOption::builder().sk(token),
                )
                .await?;
                let user_oauth = u
                    .get(0)
                    .cloned()
                    .ok_or(Error2::NotFound("Invalid OAuth token".into()))?;
                User::get(&dynamo.client, user_oauth.pk, None::<String>)
                    .await?
                    .ok_or(Error2::NotFound("User not found".into()))?
            }
        },
        LoginRequest::Telegram { .. } => {
            // Handle Telegram login
            // Not implemented yet
            return Err(Error2::BadRequest("Telegram login not implemented".into()));
        }
    };

    let user_session = DynamoUserSession {
        pk: user.pk.to_string(),
        typ: user.user_type as i64,
    };
    session
        .insert(DYNAMO_USER_SESSION_KEY, user_session)
        .await?;

    let mut claims = Claims {
        sub: user.pk.to_string(),
        ..Default::default()
    };
    let token = generate_jwt(&mut claims)
        .map_err(|e| Error2::InternalServerError(format!("JWT generation error: {}", e)))?;
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        format!(
            "{}_auth_token={}; SameSite=Lax; Path=/; Max-Age=2586226; HttpOnly; Secure;",
            config::get().env,
            token,
        )
        .parse()
        .unwrap(),
    );

    Ok(JsonWithHeaders {
        body: user,
        headers,
    })
}

pub async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    pool: &sqlx::PgPool,
    email: String,
    password: String,
) -> Result<User, Error2> {
    let (u, _) = User::find_by_email_and_password(
        cli,
        &email,
        UserQueryOption::builder().sk(password.to_string()),
    )
    .await?;
    let user = u.get(0).cloned();

    // FIXME(migrate): fallback to tricky migration from postgres
    let user = if user.is_none() {
        migrate_by_email_password(cli, pool, email, password).await?
    } else {
        user.unwrap()
    };

    Ok(user)
}
