#![allow(unused)]
mod controllers;
mod hooks;
mod models;
mod route;
mod types;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use hooks::*;
pub use models::*;
pub use route::Route;
pub use types::*;

use common::*;

type Result<T> = common::Result<T>;
