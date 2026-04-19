use super::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

#[wasm_bindgen(module = "/src/features/auth/interop/web.js")]
extern "C" {
    pub fn init_firebase(conf: &JsValue);

    #[wasm_bindgen(js_name = signIn)]
    fn sign_in_promise() -> Promise;
}

pub async fn sign_in() -> crate::common::Result<super::UserInfo> {
    let js_value = JsFuture::from(sign_in_promise())
        .await
        .map_err(|_e| AuthError::UserInfoParseFailed)?;
    serde_wasm_bindgen::from_value(js_value).map_err(|_e| AuthError::UserInfoParseFailed.into())
}
