pub mod components;
#[cfg(feature = "server")]
mod config;
mod controllers;
pub mod i18n;
mod interop;
mod models;
mod types;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use views::FileManagerPage as SpaceAppFilePage;

use crate::common::*;

type Result<T> = crate::common::Result<T>;
