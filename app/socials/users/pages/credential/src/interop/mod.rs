use common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::*;
#[wasm_bindgen(js_namespace = ["window", "ratel", "ratel_user_credential"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);

    #[wasm_bindgen(js_name = requestIdentityVerification)]
    pub fn request_identity_verification(
        store_id: &str,
        channel_key: &str,
        prefix: &str,
    ) -> Promise;
}

pub async fn request_identity_verification_async(
    store_id: &str,
    channel_key: &str,
    prefix: &str,
) -> Result<String> {
    let promise = request_identity_verification(store_id, channel_key, prefix);
    let value = JsFuture::from(promise)
        .await
        .map_err(|e| Error::Unknown(format_js_error(e)))?;

    value
        .as_string()
        .ok_or_else(|| Error::Unknown("Invalid PortOne response".to_string()))
}

fn format_js_error(err: JsValue) -> String {
    if let Some(msg) = err.as_string() {
        msg
    } else {
        format!("{:?}", err)
    }
}
