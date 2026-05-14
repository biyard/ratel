//! Web-side caller for Google Sign In via `tauri-plugin-google-auth`.
//!
//! The plugin's native flow uses Android Credential Manager + Google
//! AuthorizationClient. It returns tokens directly from Google (no Firebase
//! Auth exchange). The backend's OAuth handler hits
//! `https://openidconnect.googleapis.com/v1/userinfo` with the returned
//! `access_token`, so no extra Firebase round-trip is needed.

use serde::{Deserialize, Serialize};

use crate::tauri::invoke::{InvokeError, invoke};

/// Shape returned by `tauri-plugin-google-auth`'s `signIn` command.
/// Matches the plugin's TokenResponse interface (camelCase wire format).
#[derive(Debug, Clone, Deserialize)]
pub struct TokenResponse {
    #[serde(rename = "idToken")]
    pub id_token: Option<String>,
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: Option<String>,
    #[serde(rename = "expiresAt")]
    pub expires_at: Option<i64>,
    #[serde(default)]
    pub scopes: Vec<String>,
}

#[derive(Serialize)]
struct SignInArgs<'a> {
    #[serde(rename = "clientId")]
    client_id: &'a str,
    scopes: &'static [&'static str],
    #[serde(rename = "flowType")]
    flow_type: &'static str,
}

/// Launch the Google Sign-In flow. Resolves to a `TokenResponse` containing
/// at least `access_token` (and usually `id_token` because we request the
/// `openid` scope).
pub async fn sign_in() -> Result<TokenResponse, InvokeError> {
    let client_id = option_env!("GOOGLE_OAUTH_CLIENT_ID").unwrap_or("");
    invoke(
        "plugin:google-auth|sign_in",
        SignInArgs {
            client_id,
            scopes: &["openid", "email", "profile"],
            flow_type: "native",
        },
    )
    .await
}
