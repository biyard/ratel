#![allow(unused_imports, dead_code, ambiguous_glob_reexports)]
mod app;
pub mod common;
pub mod components;
pub mod config;
mod constants;
pub mod contexts;
pub mod interop;
pub mod root_layout;
mod route;
pub mod views;

pub use app::App;
pub use route::Route;

pub use crate::common::*;
pub use components::*;
pub use contexts::*;
pub mod features;
pub use features::*;

#[cfg(any(feature = "tauri-types", feature = "tauri-web"))]
pub mod tauri;

pub use dioxus::fullstack::{Loader, Loading};

// `axum` is pulled in transitively through `dioxus-fullstack` and exposed at
// `dioxus::fullstack::axum`. Re-export it here so all in-crate paths can use
// `crate::axum::Router`, `crate::axum::routing::post`, etc. without going
// through `dioxus::fullstack::axum::...` everywhere.
#[cfg(feature = "server")]
pub use dioxus::fullstack::axum;

// `schemars` is pulled in transitively through `rmcp`. The `JsonSchema`
// derive macro emits code that references the `schemars` crate by its
// unqualified name (`schemars::...`), so we need to expose it at the crate
// root for `#[derive(rmcp::schemars::JsonSchema)]` callsites to resolve.
#[cfg(feature = "server")]
pub use rmcp::schemars;

use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use features::auth::{OptionalUser, User};

#[cfg(test)]
pub mod tests;
