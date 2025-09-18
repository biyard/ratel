use crate::{
    models::user::{User, UserPrincipal, UserPrincipalQueryOption},
    utils::{aws::DynamoClient, users::get_principal},
};
use bdk::prelude::*;
use dto::{
    Error, JsonSchema, Result, aide,
    by_axum::{
        auth::{Authorization, DYNAMO_USER_SESSION_KEY, DynamoUserSession},
        axum::{
            Extension,
            extract::{Json, State},
        },
    },
};
use serde::Deserialize;
use tower_sessions::Session;

#[derive(Debug, Clone, Deserialize, Default, aide::OperationIo, JsonSchema)]
pub struct LoginRequest {
    email: Option<String>,
    password: Option<String>,

    telegram_raw: Option<String>,
}

/// Login handler
#[tracing::instrument(skip_all, fields(email = ?req.email))]
pub async fn login_handler(
    State(cli): State<DynamoClient>,
    Extension(session): Extension<Session>,
    Extension(auth): Extension<Option<Authorization>>,
    Json(req): Json<LoginRequest>,
) -> Result<()> {
    let principal = get_principal(auth);
    let mut user: Option<User> = None;

    if let Ok(principal) = principal {
        let (user_principal, _) = UserPrincipal::find_by_principal(
            &cli.client,
            principal,
            UserPrincipalQueryOption::builder(),
        )
        .await?;
        if user_principal.len() == 1 {
            let user_principal = user_principal.get(0).unwrap();
            user = Some(
                User::get(&cli.client, &user_principal.pk, None::<String>)
                    .await?
                    .ok_or(Error::NotFound)?,
            );
        }
    }

    match (req.email, req.password, req.telegram_raw) {
        (Some(email), Some(password), None) => {
            let (user, _bookmark) =
                User::find_by_email(&cli.client, &email, Default::default()).await?;
            if user.len() == 0 || user.len() > 1 {
                tracing::debug!("Found {} users for email: {}", user.len(), email);

                return Err(Error::Unauthorized);
            }
            let user = user.get(0).unwrap();
            if user.password != password {
                return Err(Error::Unauthorized);
            }
        }
        (None, None, Some(_telegram_raw)) => {
            // Handle Telegram login
            //Not implemented yet
            return Err(Error::BadRequest);
        }
        _ => {
            return Err(Error::BadRequest);
        }
    }
    if let Some(user) = user {
        let user_session = DynamoUserSession {
            user_pk: user.pk.to_string(),
            user_type: user.user_type as i64,
        };
        session
            .insert(DYNAMO_USER_SESSION_KEY, user_session)
            .await
            .map_err(|e| Error::DatabaseException(e.to_string()))?;
        return Ok(());
    }

    Err(Error::Unauthorized)
}
