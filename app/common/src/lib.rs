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

pub use dioxus::logger::tracing::{debug, error, info, warn};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub use icons;
pub use lucide_dioxus;

pub mod macros;
#[cfg(feature = "server")]
pub mod middlewares;
pub mod models;
pub mod query;
pub mod services;
pub mod utils;
pub use macros::dynamo_entity::DynamoEntity;

pub use components::*;
pub use contexts::*;
pub use dev_tools::*;
pub use dioxus;
pub use providers::*;
pub use query::*;
pub use run::*;

use dioxus::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

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
