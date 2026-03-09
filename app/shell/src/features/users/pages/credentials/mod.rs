#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod hooks;
pub mod layout;
pub mod models;
pub mod provider;
#[cfg(feature = "server")]
pub mod services;

#[cfg(not(feature = "server"))]
pub mod interop;
#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;
mod views;
pub use views::*;

pub use provider::Provider;

use crate::common::*;
use dioxus::prelude::*;

type Result<T> = crate::common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
