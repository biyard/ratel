pub mod controllers;

pub mod models;
pub use models::*;

pub mod types;
pub use types::*;

pub mod config;

pub mod route;
pub mod views;
use dioxus::prelude::*;

pub mod menu;
pub use menu::get_nav_item;

pub use route::Route;

pub use common::*;

type Result<T> = common::Result<T>;
