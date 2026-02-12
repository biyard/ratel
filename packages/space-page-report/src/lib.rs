#![allow(unused_imports)]
mod menu;
mod route;
mod views;
#[cfg(feature = "server")]
mod config;
#[cfg(feature = "server")]
#[path = "../apis/mod.rs"]
pub mod apis;

use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;

use common::types::*;
