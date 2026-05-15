#![allow(unused_imports)]
pub mod assets;
pub mod components;
pub mod config;
pub mod contexts;
pub mod controllers;
pub mod dev_tools;
pub mod hooks;
mod provider;
pub mod providers;
mod run;
mod traits;
pub mod types;

pub use config::*;
pub use provider::*;

pub use by_macros::*;

pub use dioxus_translate;
pub use dioxus_translate::*;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{DeserializeFromStr, SerializeDisplay};
pub use strum::*;
pub use types::*;
pub mod logger;

pub mod fullstack;
pub use fullstack::*;

pub use dioxus::logger::tracing::{debug, error, info, warn};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub use icons;
pub use lucide_dioxus;

pub mod macros;
#[cfg(feature = "server")]
pub mod middlewares;
pub mod models;
pub mod services;
pub mod utils;
pub use macros::{DynamoEntity, DynamoEnum};
pub use regex;

pub use components::*;
pub use contexts::*;
pub use dev_tools::*;
pub use dioxus;
pub use providers::*;
pub use run::*;

// Shadow `dioxus::fullstack::{get, post, put, patch, delete}` with our
// own variants. Same syntax as the dioxus macros for `!tauri-web`, and
// emit a reqwest stub for `tauri-web` so the bundle bypasses dioxus's
// hydration-coupled RPC transport.
//
// Must come AFTER `dioxus::prelude::*` so the specific names win.
pub use by_macros::{delete, get, patch, post, put};

// NOTE: it replaces dioxus::prelude::* and should be used after it.
pub use components::SuspenseBoundary;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[cfg(feature = "server")]
pub mod migrations;

#[cfg(feature = "server")]
pub mod stream_handler;

#[cfg(all(feature = "server", feature = "local-dev"))]
pub mod stream_poller;

#[cfg(all(feature = "server", feature = "local-dev"))]
pub mod design_preview;

#[cfg(feature = "server")]
pub mod mcp;
#[cfg(feature = "server")]
pub mod server_lib;

#[cfg(feature = "server")]
pub use server_lib::*;

pub use chrono;
pub use percent_encoding;
pub use serde;
pub use serde_json;
pub use serde_wasm_bindgen;
pub use serde_with;
pub use wasm_bindgen;
pub use wasm_bindgen_futures;
pub use web_sys;
