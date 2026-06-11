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

/// On WebView/mobile PortOne completes identity verification by redirecting the
/// whole window back to `redirectUrl` (the credentials page) with the result in
/// the query string — the original `request_identity_verification` future is
/// gone (the page reloaded). This reads that result on page load: it returns
/// the `identityVerificationId` once and clears the query so a refresh does not
/// re-trigger. Returns `None` on desktop (popup flow leaves no query param).
#[cfg(not(feature = "server"))]
pub async fn take_pg_return_id() -> Option<String> {
    use dioxus::document::eval as dx_eval;
    let mut runner = dx_eval(include_str!("web/take_pg_return.js"));
    runner.recv::<Option<String>>().await.ok().flatten()
}

/// Pops the PG pages the WebView redirect stacked in browser history, so "back"
/// from the returned credentials page exits to the pre-credentials page instead
/// of the verification screen. Call AFTER the credential is finalized. No-op
/// when there is no stashed history marker (e.g. desktop popup flow).
#[cfg(not(feature = "server"))]
pub async fn clear_kyc_history() {
    use dioxus::document::eval as dx_eval;
    let mut runner = dx_eval(include_str!("web/kyc_clear_history.js"));
    let _ = runner.recv::<bool>().await;
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
