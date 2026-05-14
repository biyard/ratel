#[cfg(feature = "web")]
mod web;
#[cfg(feature = "web")]
pub use web::*;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
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
// - `target_os = "android"` provides one via `android::sign_in`
// - every other target (server build, non-android mobile build, default
//   `cargo check`) lands on this stub so the unconditional
//   `use ...interop::sign_in;` in `login_modal` keeps resolving.
#[cfg(not(any(feature = "web", target_os = "android")))]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    Err(AuthError::SignInUnsupportedOnPlatform.into())
}
