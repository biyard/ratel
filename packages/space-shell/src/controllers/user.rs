use crate::*;
use common::models::*;
use dioxus::fullstack::Form;
use serde::de::DeserializeOwned;

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
#[post("/api/login", session : Extension<TowerSession>)]
pub async fn login(form: Form<LoginRequest>) -> std::result::Result<(), ServerFnError> {
    use common::{models::user::SESSION_KEY_USER_ID, EntityType};
    let c = crate::config::get();
    let cli = &c.common.dynamodb();
    debug!("Login request: {}", form.email);
    let (users, _) = User::find_by_email(cli, &form.email, User::opt_one())
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

#[derive(Serialize, Deserialize)]
pub struct OAuthLoginRequest {
    pub access_token: String,
}

#[cfg(feature = "server")]
#[derive(Debug, Deserialize)]
struct GoogleUserInfo {
    email: String,
}

#[post("/api/oauth-login", session: Extension<TowerSession>)]
pub async fn oauth_login(
    form: Form<OAuthLoginRequest>,
) -> std::result::Result<(), ServerFnError> {
    use common::models::user::SESSION_KEY_USER_ID;

    let c = crate::config::get();
    let cli = &c.common.dynamodb();

    // Validate access token with Google OIDC userinfo endpoint
    let userinfo: GoogleUserInfo = reqwest::Client::new()
        .get("https://openidconnect.googleapis.com/v1/userinfo")
        .bearer_auth(&form.access_token)
        .send()
        .await
        .map_err(|e| {
            error!("Failed to call Google userinfo: {:?}", e);
            ServerFnError::new(format!("Google userinfo request failed: {:?}", e))
        })?
        .error_for_status()
        .map_err(|e| {
            error!("Google userinfo returned error status: {:?}", e);
            ServerFnError::new("Invalid access token".to_string())
        })?
        .json()
        .await
        .map_err(|e| {
            error!("Failed to parse Google userinfo response: {:?}", e);
            ServerFnError::new(format!("Failed to parse userinfo: {:?}", e))
        })?;

    debug!("OAuth login for email: {}", userinfo.email);

    let (users, _) = User::find_by_email(cli, &userinfo.email, User::opt_one())
        .await
        .map_err(|e| {
            error!(
                "Failed to query user by email '{}': {:?}",
                userinfo.email, e
            );
            ServerFnError::new(format!("DB query failed: {:?}", e))
        })?;
    let user = users.into_iter().next().ok_or_else(|| {
        error!("No user found for email '{}'", userinfo.email);
        ServerFnError::new(format!("User not found: {}", userinfo.email))
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
