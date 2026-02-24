#![allow(unused)]
mod config;
mod controllers;
mod i18n;
mod layout;
mod menu;
mod models;
mod route;
mod types;

use dioxus::prelude::*;

pub use controllers::*;
pub use menu::get_nav_item;
pub use models::*;
pub use route::Route;
pub use types::*;

use common::*;

type Result<T> = common::Result<T>;
