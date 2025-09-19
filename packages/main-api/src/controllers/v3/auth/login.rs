use crate::{
    AppState, Error2, config,
    models::user::{User, UserPrincipal, UserPrincipalQueryOption},
    utils::{dynamo_extractor::get_principal_from_auth, password::verify_password},
};
use bdk::prelude::*;

use dto::{
    JsonSchema,
    aide::{self},
    by_axum::{
        auth::{Authorization, DYNAMO_USER_SESSION_KEY, DynamoUserSession, generate_jwt},
        axum::{
            Extension,
            extract::{Json, State},
            http::{HeaderMap, header::SET_COOKIE},
        },
    },
    by_types::Claims,
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct LoginRequest {
    email: Option<String>,
    password: Option<String>,

    telegram_raw: Option<String>,
}

pub type LoginResponse = (HeaderMap, ()); // Define the response type

#[tracing::instrument(skip_all, fields(email = ?req.email))]
pub async fn login_handler(
    State(AppState { dynamo, .. }): State<AppState>,
    Extension(session): Extension<Session>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<LoginRequest>,
) -> Result<(HeaderMap, ()), Error2> {
    let principal = get_principal_from_auth(auth);
    let mut user: Option<User> = None;
    if let Ok(principal) = principal {
        let (user_principal, _) = UserPrincipal::find_by_principal(
            &dynamo.client,
            principal,
            UserPrincipalQueryOption::builder(),
        )
        .await?;
        if user_principal.len() == 1 {
            let user_principal = user_principal.get(0).unwrap();
            user = Some(
                User::get(&dynamo.client, &user_principal.pk, None::<String>)
                    .await?
                    .ok_or(Error2::NotFound("User not found".into()))?,
            );
        }
    }
    match (req.email, req.password, req.telegram_raw) {
        (Some(email), Some(password), None) => {
            let (u, _bookmark) =
                User::find_by_email(&dynamo.client, &email, Default::default()).await?;
            if u.len() == 0 || u.len() > 1 {
                tracing::debug!("Found {} users for email: {}", u.len(), email);

                return Err(Error2::Unauthorized("Invalid email or password".into()));
            }
            let u = u.get(0).unwrap();
            if verify_password(&password, &u.password)
                .map_err(|e| Error2::InternalServerError(format!("Password verify error: {}", e)))?
                == true
            {
                user = Some(u.clone());
            }
        }
        (None, None, Some(_telegram_raw)) => {
            // Handle Telegram login
            //Not implemented yet
            return Err(Error2::BadRequest("Telegram login not implemented".into()));
        }
        _ => {
            return Err(Error2::BadRequest(
                "Invalid login request. Please provide either email and password or telegram_raw."
                    .into(),
            ));
        }
    }
    if let Some(user) = user {
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

        return Ok((headers, ()));
    }

    Err(Error2::Unauthorized("User not found".into())).into()
}
