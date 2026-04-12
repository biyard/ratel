use crate::common::wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use super::{
    controllers::{sign_attributes_handler, CredentialResponse, SignAttributesRequest},
    *,
};
#[wasm_bindgen(js_namespace = ["window", "ratel", "user_credential"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn initialize(config: &JsValue);

    #[wasm_bindgen(js_name = request_identity_verification)]
    pub fn request_identity_verification(
        store_id: &str,
        channel_key: &str,
        prefix: &str,
    ) -> Promise;
}

#[cfg(feature = "bypass")]
pub async fn verify_identity(
    _store_id: &str,
    _channel_key: &str,
    _prefix: &str,
) -> Result<CredentialResponse> {
    sign_attributes_handler(SignAttributesRequest::PortOne { id: "".to_string() }).await
}

#[cfg(not(feature = "bypass"))]
pub async fn verify_identity(
    store_id: &str,
    channel_key: &str,
    prefix: &str,
) -> Result<CredentialResponse> {
    debug!(
        "Requesting identity verification with store_id: {}, channel_key: {}, prefix: {}",
        store_id, channel_key, prefix
    );
    let promise = request_identity_verification(store_id, channel_key, prefix);
    debug!("Received promise from request_identity_verification");
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
