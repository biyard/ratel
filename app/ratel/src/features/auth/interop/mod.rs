#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use wasm_bindgen_futures::JsFuture;
#[cfg(feature = "web")]
use web_sys::js_sys::Promise;

mod native_signin;
mod wallet_connect;
use super::*;
pub use native_signin::sign_in_native;
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

/// Google sign-in entry point.
///
/// - **Web**: Firebase `signInWithPopup` over a wasm-bindgen FFI.
/// - **Android**: Credential Manager API via the Kotlin `GoogleAuthPlugin`
///   (manganis FFI). See [`native_signin::sign_in_native`].
/// - **Other non-web targets**: returns `SignInUnsupportedOnPlatform`.
#[cfg(all(not(feature = "web"), target_os = "android"))]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    sign_in_native().await
}

#[cfg(all(not(feature = "web"), not(target_os = "android")))]
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
