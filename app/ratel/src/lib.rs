#![allow(unused_imports, dead_code)]
mod app;
pub mod common;
mod components;
pub mod config;
mod constants;
pub mod contexts;
pub mod interop;
pub mod layout;
pub mod root_layout;
mod route;
pub mod views;

pub use app::App;
pub use route::Route;

use crate::common::*;
pub use components::*;
use contexts::*;
pub mod features;
pub use features::*;

use dioxus::fullstack::{Loader, Loading};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;

#[cfg(test)]
pub mod tests;
