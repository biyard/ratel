mod menu;
mod route;
mod types;
mod views;
use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;

use common::*;
use types::*;

// type Result<T> = common::Result<T>;
