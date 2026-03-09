#![allow(unused_imports)]
mod config;
pub mod controllers;
mod menu;
pub mod models;
mod route;
pub mod utils;
mod views;

pub use menu::get_nav_item;
pub use models::*;
pub use route::Route;

use crate::common::*;
use dioxus::prelude::*;

type Result<T> = crate::common::Result<T>;
