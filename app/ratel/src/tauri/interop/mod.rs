//! Web-side bridges to Tauri native commands.
//!
//! Each submodule exposes one or more `pub async fn` callers paired with a
//! JS driver embedded via `include_str!`. The Rust side sends JSON via
//! `dioxus::document::eval`, the JS calls `window.__TAURI__.invoke(...)`,
//! and the response comes back as the same DTOs defined in
//! `crate::tauri::types`.

pub mod external_url;
