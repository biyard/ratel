#![allow(unused)]
mod controllers;
mod models;
mod route;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use models::*;
pub use route::Route;

use common::*;

type Result<T> = common::Result<T>;
