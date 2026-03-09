#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod models;
pub mod route;
pub mod services;

#[cfg(not(feature = "server"))]
pub mod interop;
mod views;
pub use views::*;

pub use route::Route;

pub use common::macros::dynamo_entity::DynamoEntity;
pub use common::models::*;
pub use common::types::*;
pub use common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};

use common::*;
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

type Result<T> = common::Result<T>;
