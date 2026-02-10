#![allow(unused_imports)]
mod assets;
mod menu;
mod route;
mod views;

use assets::*;
use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;

use common::types::*;
