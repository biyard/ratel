//! Thin wrapper around `tauri-plugin-google-auth`'s `sign_in` that injects
//! the OAuth web client_id from `google-services.json` (parsed at compile
//! time by build.rs). The web bundle calls this command with no arguments.

use tauri::Runtime;
use tauri_plugin_google_auth::{FlowType, GoogleAuthExt, SignInRequest, TokenResponse};

/// OAuth web client_id, baked at compile time from
/// `app/ratel-tauri/src-tauri/google-services.json`.
const CLIENT_ID: &str = env!("GOOGLE_OAUTH_CLIENT_ID");

#[tauri::command]
pub async fn google_sign_in<R: Runtime>(
    app: tauri::AppHandle<R>,
) -> Result<TokenResponse, String> {
    app.google_auth()
        .sign_in(SignInRequest {
            client_id: CLIENT_ID.to_string(),
            client_secret: None,
            scopes: Some(vec!["openid".into(), "email".into(), "profile".into()]),
            hosted_domain: None,
            login_hint: None,
            redirect_uri: None,
            success_html_response: None,
            flow_type: Some(FlowType::Native),
        })
        .map_err(|e| e.to_string())
}
