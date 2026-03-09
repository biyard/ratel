#![allow(unused_imports)]
pub mod controllers;
mod menu;
mod route;
mod views;

pub use menu::get_nav_item;
pub use route::Route;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
