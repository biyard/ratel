#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod dto;
pub mod hooks;
pub mod i18n;
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
pub use i18n::*;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
