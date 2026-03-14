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

pub use crate::common::macros::dynamo_entity::DynamoEntity;
pub use crate::common::models::*;
pub use crate::common::types::*;
pub use crate::common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};

use crate::common::*;
use dioxus::prelude::*;

use serde::{Deserialize, Serialize};

type Result<T> = crate::common::Result<T>;
