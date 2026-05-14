//! Web-side bridges to Tauri native commands.
//!
//! Each submodule exposes a `pub async fn` that calls `crate::tauri::invoke`
//! against a registered `#[tauri::command]` (or plugin command via
//! `plugin:<name>|<method>`) in the native shell. Args/results flow through
//! `serde_wasm_bindgen` against the `window.__TAURI__.core.invoke` API.

pub mod external_url;
pub mod google_sign_in;
