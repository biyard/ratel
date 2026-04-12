use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::js_sys::Promise;

use crate::features::auth::*;

#[cfg(feature = "web")]
pub fn wallet_connect_initialize(config: &crate::common::WalletConnectConfig) {
    wallet_initialize(
        &config.project_id,
        &config.app_name,
        &config.app_description,
        &config.app_url,
    );
}

// ── Wallet (WalletConnect) interop ────────────────────────────────────

#[wasm_bindgen(js_namespace = ["window", "ratel", "auth", "wallet"])]
extern "C" {
    #[wasm_bindgen(js_name = initialize)]
    pub fn wallet_initialize(
        project_id: &str,
        app_name: &str,
        app_description: &str,
        app_url: &str,
    );

    #[wasm_bindgen(js_name = connect)]
    fn wallet_connect_promise() -> Promise;

    #[wasm_bindgen(js_name = signMessage)]
    fn wallet_sign_message_promise(message: &str) -> Promise;

    #[wasm_bindgen(js_name = getAddress)]
    fn wallet_get_address_promise() -> Promise;

    #[wasm_bindgen(js_name = disconnect)]
    fn wallet_disconnect_promise() -> Promise;

    #[wasm_bindgen(js_name = isConnected)]
    pub fn wallet_is_connected() -> bool;

    #[wasm_bindgen(js_name = openWalletApp)]
    fn wallet_open_app_promise() -> Promise;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnectResult {
    pub address: String,
    pub chain_id: u64,
}

/// Connect wallet via QR — returns address + chain_id, keeps session alive
pub async fn wallet_connect() -> crate::common::Result<WalletConnectResult> {
    let js_value = JsFuture::from(wallet_connect_promise())
        .await
        .map_err(|_e| AuthError::WalletConnectFailed)?;

    serde_wasm_bindgen::from_value(js_value)
        .map_err(|_e| AuthError::WalletConnectFailed.into())
}

/// Sign a message using the active session
pub async fn wallet_sign_message(message: &str) -> crate::common::Result<String> {
    let js_value = JsFuture::from(wallet_sign_message_promise(message))
        .await
        .map_err(|_e| AuthError::WalletConnectFailed)?;

    js_value
        .as_string()
        .ok_or_else(|| AuthError::WalletConnectFailed.into())
}

pub async fn wallet_get_address() -> crate::common::Result<Option<String>> {
    let js_value = JsFuture::from(wallet_get_address_promise())
        .await
        .map_err(|_e| AuthError::WalletConnectFailed)?;
    Ok(js_value.as_string())
}

/// Open the wallet app (deep link or AppKit modal) for pending sign requests
pub async fn wallet_open_app() -> crate::common::Result<()> {
    JsFuture::from(wallet_open_app_promise())
        .await
        .map_err(|_e| AuthError::WalletConnectFailed)?;
    Ok(())
}

pub async fn wallet_disconnect() -> crate::common::Result<()> {
    JsFuture::from(wallet_disconnect_promise())
        .await
        .map_err(|_e| AuthError::WalletConnectFailed)?;
    Ok(())
}
