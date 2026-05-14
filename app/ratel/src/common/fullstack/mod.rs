#[cfg(feature = "tauri-web")]
mod tauri_web;

#[cfg(feature = "tauri-web")]
pub use tauri_web::*;

#[cfg(not(feature = "tauri-web"))]
pub use dioxus::fullstack::{Loader, Loading};

#[cfg(not(feature = "tauri-web"))]
pub use dioxus::prelude::use_loader;
