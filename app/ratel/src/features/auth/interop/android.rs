//! Native Google sign-in for Android, routed through **Firebase Auth**.
//!
//! Flow:
//! 1. Embed `app/ratel/assets/google-services.json` at compile time and pull
//!    `api_key`, `mobilesdk_app_id`, `project_id`, and the web OAuth client ID
//!    out of it. Same file Firebase Console gives you for an Android app.
//! 2. Pass those strings into the Kotlin `GoogleAuthPlugin` via `manganis`.
//! 3. Kotlin initializes a named `FirebaseApp`, runs the Credential Manager
//!    to obtain a Google ID token, exchanges it for a Firebase credential,
//!    and returns the Firebase user's ID token — byte-compatible with the
//!    token the web flow already posts.
//!
//! The Kotlin side lives in `./android/src/main/kotlin/co/biyard/ratel/auth/GoogleAuthPlugin.kt`.

// Manganis FFI signatures must mirror the Kotlin method names verbatim
// (camelCase). `-D warnings` makes non_snake_case a hard error otherwise.
#![allow(non_snake_case)]

use crate::features::auth::interop::UserInfo;
use crate::features::auth::types::AuthError;

#[manganis::ffi("src/features/auth/interop/android")]
extern "Kotlin" {
    /// The Kotlin `GoogleAuthPlugin` class.
    pub type GoogleAuthPlugin;

    /// Kotlin signature:
    ///   `fun signInJson(
    ///        serverClientId: String,
    ///        firebaseApiKey: String,
    ///        firebaseAppId: String,
    ///        firebaseProjectId: String,
    ///    ): String`
    ///
    /// Success JSON:
    ///   `{ "id_token": "...", "access_token": "...", "email": "...",
    ///      "display_name": "...", "photo_url": "..." }`
    /// Failure JSON:
    ///   `{ "error": "<message>" }`
    pub fn signInJson(
        this: &GoogleAuthPlugin,
        serverClientId: String,
        firebaseApiKey: String,
        firebaseAppId: String,
        firebaseProjectId: String,
    ) -> String;
}

pub async fn sign_in_native() -> crate::common::Result<UserInfo> {
    let cfg = match firebase_config::get() {
        Ok(c) => c,
        Err(e) => {
            crate::error!("failed to parse google-services.json: {e}");
            return Err(AuthError::SignInUnsupportedOnPlatform.into());
        }
    };

    let plugin = GoogleAuthPlugin::new().map_err(|e| {
        crate::error!("failed to create GoogleAuthPlugin: {e:?}");
        AuthError::UserInfoParseFailed
    })?;

    let json = signInJson(
        &plugin,
        cfg.server_client_id.clone(),
        cfg.api_key.clone(),
        cfg.mobilesdk_app_id.clone(),
        cfg.project_id.clone(),
    )?;

    let value: serde_json::Value = serde_json::from_str(&json).map_err(|e| {
        crate::error!("failed to parse GoogleAuthPlugin response: {e}");
        AuthError::UserInfoParseFailed
    })?;

    if let Some(err) = value.get("error").and_then(|v| v.as_str()) {
        crate::error!("firebase native sign-in error: {err}");
        return Err(AuthError::UserInfoParseFailed.into());
    }

    serde_json::from_value(value).map_err(|e| {
        crate::error!("failed to map sign-in response to UserInfo: {e}");
        AuthError::UserInfoParseFailed.into()
    })
}

// ─────────────────────────────────────────────────────────────────────────────
// google-services.json parsing
// ─────────────────────────────────────────────────────────────────────────────

mod firebase_config {
    use serde::Deserialize;
    use std::sync::OnceLock;

    /// Embedded at compile time. The file is committed to the repo; Firebase's
    /// API key for Android is not a secret (it's visible to every installed
    /// client), so this is by design.
    const GOOGLE_SERVICES_JSON: &str = include_str!("../../../../assets/google-services.json");

    pub struct FirebaseConfig {
        pub api_key: String,
        pub mobilesdk_app_id: String,
        pub project_id: String,
        pub server_client_id: String,
    }

    #[derive(Deserialize)]
    struct Root {
        project_info: ProjectInfo,
        client: Vec<ClientConfig>,
    }

    #[derive(Deserialize)]
    struct ProjectInfo {
        project_id: String,
    }

    #[derive(Deserialize)]
    struct ClientConfig {
        client_info: ClientInfo,
        api_key: Vec<ApiKey>,
        #[serde(default)]
        oauth_client: Vec<OAuthClient>,
        #[serde(default)]
        services: Option<Services>,
    }

    #[derive(Deserialize)]
    struct ClientInfo {
        mobilesdk_app_id: String,
    }

    #[derive(Deserialize)]
    struct ApiKey {
        current_key: String,
    }

    #[derive(Deserialize)]
    struct OAuthClient {
        client_id: String,
        client_type: u32,
    }

    #[derive(Deserialize)]
    struct Services {
        #[serde(default)]
        appinvite_service: Option<AppinviteService>,
    }

    #[derive(Deserialize)]
    struct AppinviteService {
        #[serde(default)]
        other_platform_oauth_client: Vec<OAuthClient>,
    }

    static CACHED: OnceLock<Result<FirebaseConfig, String>> = OnceLock::new();

    pub fn get() -> Result<&'static FirebaseConfig, &'static str> {
        CACHED
            .get_or_init(|| parse().map_err(|e| e.to_string()))
            .as_ref()
            .map_err(|s| s.as_str())
    }

    fn parse() -> Result<FirebaseConfig, serde_json::Error> {
        let root: Root = serde_json::from_str(GOOGLE_SERVICES_JSON)?;
        let client = root
            .client
            .into_iter()
            .next()
            .expect("google-services.json: `client` array is empty");

        let api_key = client
            .api_key
            .into_iter()
            .next()
            .expect("google-services.json: `api_key` array is empty")
            .current_key;

        // `client_type == 3` is the Web OAuth client — what Credential Manager's
        // `serverClientId` expects and what Firebase uses for ID token `aud`.
        let server_client_id = pick_web_client(&client.oauth_client)
            .or_else(|| {
                client
                    .services
                    .as_ref()
                    .and_then(|s| s.appinvite_service.as_ref())
                    .and_then(|a| pick_web_client(&a.other_platform_oauth_client))
            })
            .expect("google-services.json: no web OAuth client (type 3) found");

        Ok(FirebaseConfig {
            api_key,
            mobilesdk_app_id: client.client_info.mobilesdk_app_id,
            project_id: root.project_info.project_id,
            server_client_id,
        })
    }

    fn pick_web_client(clients: &[OAuthClient]) -> Option<String> {
        clients
            .iter()
            .find(|c| c.client_type == 3)
            .map(|c| c.client_id.clone())
    }
}

pub async fn sign_in() -> crate::common::Result<super::UserInfo> {
    sign_in_native().await
}
