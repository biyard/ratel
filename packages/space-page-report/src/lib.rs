#![allow(unused_imports)]
mod config;
pub mod controllers;
mod menu;
mod route;
pub mod utils;
mod views;

use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;

use common::*;
