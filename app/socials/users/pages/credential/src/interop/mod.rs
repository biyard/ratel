use common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::{
    controllers::{CredentialResponse, SignAttributesRequest, sign_attributes_handler},
    *,
};
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
) -> Result<CredentialResponse> {
    let promise = request_identity_verification(store_id, channel_key, prefix);
    let value = JsFuture::from(promise).await.map_err(|e| {
        error!("Failed to request identity verification: {:?}", e);
        Error::PortOneRequestFailure
    })?;

    debug!("PortOne response: {:?}", value);

    let id = value
        .as_string()
        .ok_or_else(|| Error::PortOneInicisInvalidIdentity)?;

    sign_attributes_handler(SignAttributesRequest::PortOne { id }).await
}
