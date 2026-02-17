use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::*;

#[wasm_bindgen(js_namespace = ["window", "ratel", "auth", "firebase"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);

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

pub async fn sign_in() -> common::Result<UserInfo> {
    let js_value = JsFuture::from(sign_in_promise())
        .await
        .map_err(|e| Error::Unknown(format!("{:?}", e)))?;
    serde_wasm_bindgen::from_value(js_value)
        .map_err(|e| Error::Unknown(format!("Failed to parse UserInfo: {}", e)))
}
