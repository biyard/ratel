#[cfg(feature = "web")]
mod web;
#[cfg(feature = "web")]
pub use web::*;

// Android-native auth (Firebase + Google Sign-In) is part of the legacy
// dioxus-native mobile build. Only compile it when actually targeting that
// build — the Tauri shell (which compiles app-shell for android with
// `tauri-types` only) must not pull in this module because it embeds a
// gitignored google-services.json via include_str!.
#[cfg(all(target_os = "android", feature = "mobile"))]
mod android;
#[cfg(all(target_os = "android", feature = "mobile"))]
pub use android::*;

mod wallet_connect;
use super::*;
pub use wallet_connect::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub access_token: String,
    pub id_token: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}

// Fallback used when no platform-specific implementation is active:
// - `feature = "web"` provides one via `web::sign_in`
// - `target_os = "android"` + `feature = "mobile"` provides one via `android::sign_in`
// - every other target (server build, non-android mobile build, default
//   `cargo check`, tauri shell pulling app-shell as a type-only dep on android)
//   lands on this stub so the unconditional `use ...interop::sign_in;` in
//   `login_modal` keeps resolving.
#[cfg(not(any(feature = "web", all(target_os = "android", feature = "mobile"))))]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    Err(AuthError::SignInUnsupportedOnPlatform.into())
}
