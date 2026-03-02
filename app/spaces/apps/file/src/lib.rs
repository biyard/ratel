pub mod components;
#[cfg(feature = "server")]
mod config;
mod controllers;
pub mod i18n;
mod interop;
mod models;
mod route;
mod types;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use route::Route;

use common::*;

type Result<T> = common::Result<T>;
