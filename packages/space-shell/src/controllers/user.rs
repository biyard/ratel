#[cfg(feature = "server")]
use common::axum::extract::{Extension, FromRef, FromRequest, Request, State};
#[cfg(feature = "server")]
use common::{middlewares::client_state::ClientState, models::session::TowerSession};
use common::{
    models::user::{OptionalUser, User},
    Partition,
};
use dioxus::fullstack::Form;
use dioxus::prelude::*;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use crate::AppState;

#[get("/api/user", user: OptionalUser) ]
pub async fn get_user() -> std::result::Result<Option<User>, ServerFnError> {
    Ok(user.into())
}

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/*
FIXME: Use password and support Google Login
 */
#[post("/api/login", state: State<ClientState>, app_state: State<AppState>, session : Extension<TowerSession>)]
pub async fn login(form: Form<LoginRequest>) -> std::result::Result<(), ServerFnError> {
    use common::{models::user::SESSION_KEY_USER_ID, EntityType};
    info!("Login request: {}", form.email);
    let (users, _) = User::find_by_email(&state.dynamo, &form.email, User::opt_one())
        .await
        .map_err(|e| {
            error!("Failed to query user by email '{}': {:?}", form.email, e);
            ServerFnError::new(format!("DB query failed: {:?}", e))
        })?;
    let user = users.into_iter().next().ok_or_else(|| {
        error!("No user found for email '{}'", form.email);
        ServerFnError::new(format!("User not found: {}", form.email))
    })?;

    session
        .insert(SESSION_KEY_USER_ID, user.pk.to_string())
        .await
        .map_err(|e| {
            error!("Failed to insert session for user '{}': {:?}", user.pk, e);
            ServerFnError::new(format!("Session insert failed: {:?}", e))
        })?;

    Ok(())
}
