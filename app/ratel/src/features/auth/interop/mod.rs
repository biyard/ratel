use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

mod wallet_connect;
use super::*;
pub use wallet_connect::*;
// ── Firebase interop ──────────────────────────────────────────────────

// static INITIALIZED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

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
#[cfg(not(feature = "web"))]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    unimplemented!("sign_in is only implemented for the web feature")
}

#[cfg(feature = "web")]
pub async fn sign_in() -> crate::common::Result<UserInfo> {
    let js_value = JsFuture::from(sign_in_promise())
        .await
        .map_err(|e| Error::Unknown(format!("{:?}", e)))?;
    serde_wasm_bindgen::from_value(js_value)
        .map_err(|e| Error::Unknown(format!("Failed to parse UserInfo: {}", e)))
}
