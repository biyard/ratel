#![allow(unused_imports)]
mod app;
mod assets;
mod menu;
mod route;
mod views;

use assets::*;
use dioxus::prelude::*;

pub use app::App;

pub use menu::get_nav_item;

use common::types::*;
use route::Route;
