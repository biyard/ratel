#![allow(unused)]
mod config;
mod controllers;
mod menu;
mod models;
mod route;
mod types;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use menu::{get_app_menu_items, get_nav_item};
pub use models::*;
pub use route::Route;
pub use types::*;

use common::*;

type Result<T> = common::Result<T>;
