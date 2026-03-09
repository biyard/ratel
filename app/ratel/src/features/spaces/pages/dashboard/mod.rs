pub mod components;
#[cfg(feature = "server")]
mod config;
pub mod controllers;
pub mod i18n;
mod menu;
pub mod route;
pub mod types;
pub mod views;

#[cfg(feature = "server")]
pub mod models;

use dioxus::prelude::*;
pub use types::*;

pub use crate::common::Error;
pub use menu::get_nav_item;
pub use route::Route;

use crate::common::*;
use components::*;
