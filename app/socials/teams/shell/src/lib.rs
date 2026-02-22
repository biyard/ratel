#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod hooks;
pub mod layout;
pub mod models;
pub mod route;

#[cfg(not(feature = "server"))]
pub mod interop;
#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;
mod views;

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
type DioxusResult<T> = dioxus::prelude::Result<T>;
