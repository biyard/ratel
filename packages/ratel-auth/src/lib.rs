#![allow(unused)]
pub mod config;
pub mod constants;
pub mod types;

#[cfg(feature = "server")]
pub mod controllers;
#[cfg(feature = "server")]
pub(crate) mod macros;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod utils;

#[cfg(feature = "server")]
pub mod server;

#[cfg(not(feature = "server"))]
pub mod web;

pub mod route;

pub use route::Route;

use common::*;
use dioxus::prelude::*;
