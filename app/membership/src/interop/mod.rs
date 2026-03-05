use common::wasm_bindgen::prelude::*;
use common::web_sys::js_sys::Promise;

#[wasm_bindgen(
    js_namespace = ["window", "ratel", "ratel_membership"],
    js_name = requestIdentityVerification
)]
extern "C" {
    pub fn request_identity_verification(
        store_id: &str,
        channel_key: &str,
        prefix: &str,
    ) -> Promise;
}
