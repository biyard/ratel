// Only the Tauri WebView build needs the reqwest-based HTTP transport;
// regular dev/prod web builds use dioxus-fullstack's native RPC, so
// don't compile this module at all there (avoids accidentally baking
// `MOBILE_API_URL` into the web wasm bundle).
#[cfg(feature = "tauri-web")]
mod tauri_web;

#[cfg(feature = "tauri-web")]
pub use tauri_web::*;

pub use by_macros::{delete, get, patch, post, put};
pub use dioxus::prelude::use_hook as use_server_cached;
