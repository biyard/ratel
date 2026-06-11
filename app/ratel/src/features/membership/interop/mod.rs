use crate::common::wasm_bindgen::prelude::*;
use crate::common::web_sys::js_sys::Promise;

#[wasm_bindgen(
    js_namespace = ["window", "ratel", "membership"],
    js_name = requestIdentityVerification
)]
extern "C" {
    pub fn request_identity_verification(
        store_id: &str,
        channel_key: &str,
        prefix: &str,
    ) -> Promise;
}

/// Stash the membership tier the user is purchasing right before the PortOne
/// identity-verification redirect (WebView/mobile), so the return handler can
/// resume the purchase flow after the page reloads. No-op outside Tauri.
#[cfg(not(feature = "server"))]
pub async fn stash_membership_tier(tier: &str) {
    use dioxus::document::eval as dx_eval;
    let mut runner = dx_eval(include_str!("web/stash_membership_tier.js"));
    let _ = runner.send(serde_json::json!(tier));
    let _ = runner.recv::<bool>().await;
}

/// On a WebView/mobile redirect return to /membership, read the
/// `identityVerificationId` (captured by index.html before Dioxus strips the
/// query) together with the stashed tier. Returns `(identityVerificationId,
/// tier)` once, then clears the markers. `None` on desktop (popup flow).
#[cfg(not(feature = "server"))]
pub async fn take_membership_return() -> Option<(String, String)> {
    use dioxus::document::eval as dx_eval;
    let mut runner = dx_eval(include_str!("web/take_membership_return.js"));
    runner
        .recv::<Option<(String, String)>>()
        .await
        .ok()
        .flatten()
}
