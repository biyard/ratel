#![allow(unused_imports, dead_code)]
mod app;
pub mod common;
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
pub mod features;
pub use features::*;

use dioxus::fullstack::{Loader, Loading};

#[cfg(test)]
pub mod tests;
