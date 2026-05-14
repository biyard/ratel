//! Tauri mobile shell integration.
//!
//! `types/` defines request/response DTOs shared between the dioxus-web bundle
//! (which serializes them to JSON for `window.__TAURI__.invoke(...)`) and the
//! native `app/ratel-tauri` shell (which deserializes them in `#[tauri::command]`
//! handlers). One definition, both ends — drift impossible.
//!
//! `interop/` defines the dioxus-web side bridges: a Rust `async fn` per native
//! call, plus an embedded JS driver that calls `window.__TAURI__.invoke(...)`.
//! Only compiled under `feature = "tauri-web"`.

#[cfg(feature = "tauri-web")]
pub mod invoke;

mod commands;

pub use commands::*;
