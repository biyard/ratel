#[cfg(feature = "server")]
mod config;
mod controllers;
mod route;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use route::Route;

use common::*;
