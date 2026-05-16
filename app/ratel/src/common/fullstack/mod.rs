mod tauri_web;

pub use tauri_web::*;

pub use by_macros::{delete, get, patch, post, put};
pub use dioxus::prelude::use_hook as use_server_cached;
