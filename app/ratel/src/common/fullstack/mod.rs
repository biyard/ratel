#[cfg(feature = "tauri-web")]
mod tauri_web;

#[cfg(feature = "tauri-web")]
pub use tauri_web::*;

#[cfg(feature = "tauri-web")]
pub use by_macros::{delete, get, patch, post, put};

// Under web/server, alias dioxus-fullstack's types so call sites that
// `use crate::common::fullstack::{Loader, Form, ServerFnError, ...}` work
// regardless of which build is active.
#[cfg(not(feature = "tauri-web"))]
pub use dioxus::fullstack::{Form, Loader, Loading};

#[cfg(not(feature = "tauri-web"))]
pub use dioxus::prelude::{use_loader, use_server_cached, use_server_future, ServerFnError};

#[cfg(not(feature = "tauri-web"))]
pub use dioxus::prelude::*;
