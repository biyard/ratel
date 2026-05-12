#![allow(unused_imports, dead_code, ambiguous_glob_reexports)]
mod app;
pub mod common;
pub mod components;
pub mod config;
mod constants;
pub mod contexts;
pub mod interop;
pub mod root_layout;
mod route;
pub mod views;

pub use app::App;
pub use route::Route;

pub use crate::common::*;
pub use components::*;
pub use contexts::*;
pub mod features;
pub use features::*;

pub use dioxus::fullstack::{Loader, Loading};

use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
use features::auth::{OptionalUser, User};

#[cfg(test)]
pub mod tests;
