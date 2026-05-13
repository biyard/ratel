//! Dioxus-web side bridges for Tauri commands.
//!
//! Each sub-module provides a Rust `async fn` that sends a JSON payload to the
//! native shell via `window.__TAURI__.invoke(...)` using a tiny embedded JS
//! driver (the `dioxus::document::eval` channel pattern).
//!
//! Populated by Task 3.1 (open_external_url bridge).
