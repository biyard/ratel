#![allow(unused_imports, dead_code)]
mod components;
mod config;
pub mod constants;
pub mod context;
pub mod controllers;
pub mod hooks;
pub mod interop;
pub mod models;
pub mod provider;
mod route;
pub mod types;
pub mod utils;
mod views;

pub use components::*;
pub use models::*;
pub use route::Route;

// Re-export common types needed by models (available via `use crate::*;`)
pub use crate::common::macros::dynamo_entity::DynamoEntity;
pub use crate::common::models::*;
pub use crate::common::types::*;
pub use crate::common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};
pub use context::*;
pub use serde::{Deserialize, Serialize};

pub use provider::Provider as AuthProvider;

// Re-export DynamoDB transaction macros from common (macro_export places them at crate root)
#[cfg(feature = "server")]
pub use crate::transact_write;
#[cfg(feature = "server")]
pub use crate::transact_write_all_items;
#[cfg(feature = "server")]
pub use crate::transact_write_all_items_with_failover;
#[cfg(feature = "server")]
pub use crate::transact_write_items;

use crate::common::*;
use dioxus::prelude::*;

type Result<T, E = Error> = crate::common::Result<T, E>;
