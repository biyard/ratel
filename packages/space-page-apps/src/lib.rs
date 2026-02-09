mod app;
mod assets;
mod menu;
mod route;
mod views;

pub use assets::*;
use dioxus::prelude::*;

pub use app::App;
pub use menu::get_nav_item;

use common::*;
use route::Route;
