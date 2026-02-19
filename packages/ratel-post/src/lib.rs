#![allow(unused_imports, dead_code)]
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

pub use provider::Provider;
pub use route::Route;

// Re-export common types needed by models (available via `use crate::*;`)
pub use common::macros::dynamo_entity::DynamoEntity;
pub use common::models::*;
pub use common::types::*;
pub use common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};
pub use serde::{Deserialize, Serialize};

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;

#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;
