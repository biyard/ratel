//! Typed bridge to `window.__TAURI__.core.invoke`.
//!
//! All `crate::tauri::interop::*` modules go through this single helper.
//! `window.__TAURI__.core.invoke` is Tauri's stable IPC entry point — a
//! single uniformly-shaped function (`invoke(cmd, args) -> Promise<any>`)
//! that dispatches to every `#[tauri::command]` in the native shell.
//! Binding it once via wasm_bindgen avoids per-command JS driver files.

use serde::Serialize;
use serde::de::DeserializeOwned;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke, catch)]
    async fn raw_invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(thiserror::Error, Debug)]
pub enum InvokeError {
    #[error("serialize args: {0}")]
    Serialize(String),
    #[error("deserialize result: {0}")]
    Deserialize(String),
    #[error("command failed: {0}")]
    CommandFailed(String),
}

/// Call a Tauri command with typed args and a typed return.
///
/// `cmd` matches the command registered in `tauri::generate_handler![...]`
/// or — for plugin-provided commands — the `plugin:<name>|<method>` form.
pub async fn invoke<A, R>(cmd: &str, args: A) -> Result<R, InvokeError>
where
    A: Serialize,
    R: DeserializeOwned,
{
    let js_args = serde_wasm_bindgen::to_value(&args)
        .map_err(|e| InvokeError::Serialize(e.to_string()))?;
    let js_result = raw_invoke(cmd, js_args).await.map_err(|e| {
        let msg = e
            .as_string()
            .or_else(|| {
                // Errors from #[tauri::command] handlers come back as either
                // a plain string or a serde-serialized object. as_string()
                // catches the first; for the second, stringify with JSON.
                js_sys::JSON::stringify(&e).ok().and_then(|s| s.as_string())
            })
            .unwrap_or_else(|| format!("{e:?}"));
        InvokeError::CommandFailed(msg)
    })?;
    serde_wasm_bindgen::from_value(js_result)
        .map_err(|e| InvokeError::Deserialize(e.to_string()))
}
