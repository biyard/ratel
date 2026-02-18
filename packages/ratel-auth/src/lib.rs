#![allow(unused_imports, dead_code)]
mod components;
pub mod controllers;
pub mod interop;
pub mod models;
mod route;
pub mod utils;
mod views;

pub use components::*;
pub use models::*;
pub use route::Route;

// Re-export common types needed by models (available via `use crate::*;`)
pub use common::macros::dynamo_entity::DynamoEntity;
pub use common::models::*;
pub use common::types::*;
pub use common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};
pub use serde::{Deserialize, Serialize};

// Re-export DynamoDB transaction macros from common
#[cfg(feature = "server")]
pub use common::transact_write;
#[cfg(feature = "server")]
pub use common::transact_write_all_items;
#[cfg(feature = "server")]
pub use common::transact_write_all_items_with_failover;
#[cfg(feature = "server")]
pub use common::transact_write_items;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
