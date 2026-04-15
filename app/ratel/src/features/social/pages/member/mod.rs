#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod dto;
#[path = "views/i18n.rs"]
pub mod i18n;
pub use i18n::*;
pub mod hooks;
pub mod types;
pub use types::*;
pub mod layout;
pub mod models;
#[cfg(not(feature = "server"))]
pub mod interop;
#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;
mod views;
pub use views::*;

use crate::common::*;
use dioxus::prelude::*;

type Result<T> = crate::common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
