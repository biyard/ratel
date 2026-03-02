pub mod controllers;

pub mod models;
pub use models::*;

pub mod types;
pub use types::*;

pub mod components;
pub mod config;

pub mod route;
pub mod views;
use dioxus::prelude::*;

pub use route::Route;

pub use common::*;

type Result<T> = common::Result<T>;
