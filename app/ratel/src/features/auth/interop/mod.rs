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

#[cfg(feature = "server")]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    Err(AuthError::SignInUnsupportedOnPlatform.into())
}
