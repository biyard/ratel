mod controllers;
pub mod i18n;
mod route;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use route::Route;

use crate::common::*;

type Result<T> = crate::common::Result<T>;
