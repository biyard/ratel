//! Web-side caller for Google Sign In.
//!
//! The native shell wraps `tauri-plugin-google-auth`'s `signIn` command and
//! injects the OAuth client_id from `google-services.json` at compile time
//! (see `app/ratel-tauri/src-tauri/build.rs` +
//! `commands/google_sign_in.rs`). The web bundle has no knowledge of any
//! OAuth client_id — it just asks the shell to sign in.

use serde::Deserialize;

use crate::tauri::invoke::{InvokeError, invoke};

/// Mirrors `tauri_plugin_google_auth::TokenResponse` (camelCase wire format).
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

/// Launch the Google Sign-In flow on Android (Credential Manager + Google
/// AuthorizationClient). Resolves to a `TokenResponse` with `access_token`
/// (and usually `id_token` because the native command requests the `openid`
/// scope).
pub async fn sign_in() -> Result<TokenResponse, InvokeError> {
    invoke("google_sign_in", ()).await
}
