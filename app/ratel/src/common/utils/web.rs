use dioxus::document::eval;

use crate::*;

pub async fn copy_text(text: &str) -> Result<()> {
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window().ok_or(Error::OnlyWebFunction)?;
    let promise = window.navigator().clipboard().write_text(text);

    JsFuture::from(promise)
        .await
        .map_err(|_| Error::OnlyWebFunction)?;

    Ok(())
}

pub async fn invoke_with_args<ARG: serde::Serialize, T: serde::de::DeserializeOwned>(
    method: &str,
    args: ARG,
) -> Result<T> {
    let mut runner = eval(include_str!("web/invoke_with_args.js"));
    runner
        .send(serde_json::json!({
            "method": method,
            "args": args,
        }))
        .map_err(|e| {
            error!("Failed to send method {}: {}", method, e);
            Error::OnlyWebFunction
        })?;

    runner.recv::<T>().await.map_err(|e| {
        error!("Failed to invoke method {}: {}", method, e);
        Error::OnlyWebFunction
    })
}

/// Invoke a Tauri command directly via the WebView IPC
/// (`window.__TAURI_INTERNALS__.invoke`), bypassing the `window.ratel` JS
/// dispatch table. Used by the tauri-web HTTP transport to reach the native
/// `api_request` command (persistent reqwest cookie jar) so API calls don't go
/// through the in-WebView fetch, which iOS WKWebView strips the cross-site
/// session cookie from.
pub async fn invoke_tauri<ARG: serde::Serialize, T: serde::de::DeserializeOwned>(
    method: &str,
    args: ARG,
) -> Result<T> {
    let mut runner = eval(include_str!("web/invoke_tauri.js"));
    runner
        .send(serde_json::json!({
            "method": method,
            "args": args,
        }))
        .map_err(|e| {
            error!("Failed to send tauri invoke {}: {}", method, e);
            Error::OnlyWebFunction
        })?;

    runner.recv::<T>().await.map_err(|e| {
        error!("Failed tauri invoke {}: {}", method, e);
        Error::OnlyWebFunction
    })
}

pub async fn invoke_with_empty<T: serde::de::DeserializeOwned>(method: &str) -> Result<T> {
    let mut runner = eval(include_str!("web/invoke_with_args.js"));
    runner
        .send(serde_json::json!({
            "method": method,
        }))
        .map_err(|e| {
            error!("Failed to send method {}: {}", method, e);
            Error::OnlyWebFunction
        })?;
    runner.recv::<T>().await.map_err(|e| {
        error!("Failed to invoke method {}: {}", method, e);
        Error::OnlyWebFunction
    })
}
