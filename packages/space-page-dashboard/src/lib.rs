pub mod api;
pub mod components;
#[cfg(feature = "server")]
mod config;
mod menu;
pub mod route;
pub mod types;
pub mod views;

#[cfg(feature = "server")]
pub mod models;

pub use types::*;
use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;
pub use common::Error;

use common::*;
use components::*;
