#![allow(unused_imports, dead_code)]
pub mod config;
mod constants;
pub mod interop;
mod route;
pub mod views;

pub use route::Route;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;

#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;
