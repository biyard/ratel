#![allow(unused_imports, dead_code)]
pub mod common;
mod app;
pub mod components;
pub mod config;
mod constants;
pub mod contexts;
pub mod interop;
pub mod layout;
mod route;
pub mod views;

pub use app::App;
pub use route::Route;

use crate::common::*;
use components::*;
use contexts::*;
use dioxus::prelude::*;
pub mod features;
pub use features::*;

type Result<T> = crate::common::Result<T>;

#[cfg(test)]
pub mod tests;
