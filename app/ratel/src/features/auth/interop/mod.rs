#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use wasm_bindgen_futures::JsFuture;
#[cfg(feature = "web")]
use web_sys::js_sys::Promise;

mod wallet_connect;
use super::*;
pub use wallet_connect::*;
// ── Firebase interop ──────────────────────────────────────────────────

#[cfg(feature = "web")]
#[wasm_bindgen(js_namespace = ["window","ratel", "auth", "firebase"])]
extern "C" {
    pub fn init_firebase(conf: &JsValue);

    #[wasm_bindgen(js_name = signIn)]
    fn sign_in_promise() -> Promise;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub access_token: String,
    pub id_token: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub photo_url: Option<String>,
}

/// Google / Firebase sign-in.
///
/// Only wired for the web feature today. On mobile the embedded Android
/// WebView cannot run Firebase's `signInWithPopup` (Google blocks OAuth in
/// WebViews), so we surface a clear error instead of panicking. Native
/// Google Sign-In via Custom Tabs or the GMS SDK is tracked as follow-up.
#[cfg(not(feature = "web"))]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    Err(AuthError::SignInUnsupportedOnPlatform.into())
}

#[cfg(feature = "web")]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    let js_value = JsFuture::from(sign_in_promise())
        .await
        .map_err(|_e| AuthError::UserInfoParseFailed)?;
    serde_wasm_bindgen::from_value(js_value)
        .map_err(|_e| AuthError::UserInfoParseFailed.into())
}
