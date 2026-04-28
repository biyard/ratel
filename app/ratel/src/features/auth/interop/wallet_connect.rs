use dioxus::document::eval as dx_eval;

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

pub fn wallet_initialize(project_id: &str, app_name: &str, app_description: &str, app_url: &str) {
    let runner = dx_eval(include_str!("web/wc_init.js"));

    if let Err(e) = runner.send(serde_json::json!({
        "projectId": project_id,
        "appName": app_name,
        "appDescription": app_description,
        "appUrl": app_url,
    })) {
        error!("Failed to initialize WalletConnect: {:?}", e);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletConnectResult {
    pub address: String,
    pub chain_id: u64,
}

/// Connect wallet via QR — returns address + chain_id, keeps session alive
pub async fn wallet_connect() -> crate::common::Result<WalletConnectResult> {
    let mut runner = dx_eval(include_str!("web/wc_connect.js"));
    runner
        .recv::<Option<WalletConnectResult>>()
        .await
        .map_err(|_| AuthError::WalletConnectFailed)?
        .ok_or_else(|| AuthError::WalletConnectFailed.into())
}

/// Sign a message using the active session
pub async fn wallet_sign_message(message: &str) -> crate::common::Result<String> {
    let mut runner = dx_eval(include_str!("web/wc_sign_message.js"));
    runner
        .send(serde_json::json!(message))
        .map_err(|_| AuthError::WalletConnectFailed)?;
    runner
        .recv::<Option<String>>()
        .await
        .map_err(|_| AuthError::WalletConnectFailed)?
        .ok_or_else(|| AuthError::WalletConnectFailed.into())
}

pub async fn wallet_get_address() -> crate::common::Result<Option<String>> {
    let mut runner = dx_eval(include_str!("web/wc_get_address.js"));
    runner
        .recv::<Option<String>>()
        .await
        .map_err(|_| AuthError::WalletConnectFailed.into())
}

/// Open the wallet app (deep link or AppKit modal) for pending sign requests
pub async fn wallet_open_app() -> crate::common::Result<()> {
    let mut runner = dx_eval(include_str!("web/wc_open_app.js"));
    let ok = runner
        .recv::<bool>()
        .await
        .map_err(|_| AuthError::WalletConnectFailed)?;
    if ok {
        Ok(())
    } else {
        Err(AuthError::WalletConnectFailed.into())
    }
}

pub async fn wallet_disconnect() -> crate::common::Result<()> {
    let mut runner = dx_eval(include_str!("web/wc_disconnect.js"));
    let ok = runner
        .recv::<bool>()
        .await
        .map_err(|_| AuthError::WalletConnectFailed)?;
    if ok {
        Ok(())
    } else {
        Err(AuthError::WalletConnectFailed.into())
    }
}

pub async fn wallet_is_connected() -> bool {
    let mut runner = dx_eval(include_str!("web/wc_is_connected.js"));
    runner.recv::<bool>().await.unwrap_or(false)
}
