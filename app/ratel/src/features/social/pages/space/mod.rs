#![allow(unused)]
pub mod config;
pub mod controllers;
pub mod hooks;
#[cfg(not(feature = "server"))]
pub mod interop;
pub mod layout;
pub mod models;
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
