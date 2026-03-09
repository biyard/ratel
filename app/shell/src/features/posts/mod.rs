#![allow(unused_imports, dead_code)]
pub mod components;
pub mod config;
mod constants;
pub mod controllers;
pub mod interop;
pub mod models;
mod provider;
mod route;
pub mod types;
pub mod utils;
pub mod views;

#[cfg(feature = "server")]
pub mod services;

pub use provider::Provider;
pub use route::Route;

// Re-export common types needed by models (available via `use crate::*;`)
pub use crate::common::macros::dynamo_entity::DynamoEntity;
pub use crate::common::models::*;
pub use crate::common::types::*;
pub use crate::common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};
pub use serde::{Deserialize, Serialize};

use crate::common::*;
use dioxus::prelude::*;

type Result<T> = crate::common::Result<T>;

#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;
