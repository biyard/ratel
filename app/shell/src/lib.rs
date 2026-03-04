#![allow(unused_imports, dead_code)]
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

use common::*;
use components::*;
use contexts::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;

#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;

#[cfg(test)]
pub mod tests;
