#![allow(unused)]
mod controllers;
mod i18n;
mod layout;
mod menu;
mod route;
mod types;

use dioxus::prelude::*;

pub use controllers::*;
pub use menu::get_nav_item;
pub use route::Route;
pub use space_app_all_apps::SpaceApp;
pub use types::*;

use common::*;

type Result<T> = common::Result<T>;
