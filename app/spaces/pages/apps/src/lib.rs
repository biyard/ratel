#![allow(unused)]
mod config;
mod controllers;
mod layout;
mod menu;
mod models;
mod route;
mod types;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use models::*;
pub use route::Route;
pub use types::*;

use common::*;

type Result<T> = common::Result<T>;
