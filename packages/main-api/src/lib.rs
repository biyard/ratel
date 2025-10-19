pub type Result<T> = std::result::Result<T, crate::error::Error>;
// pub type Error = dto::Error;
pub type Error = crate::error::Error;
pub type Error2 = crate::error::Error;

pub mod api_main;
pub mod config;
pub mod constants;
pub mod controllers;
pub mod error;
pub mod features;
pub mod models;
pub mod route;
// pub mod route_m3;
pub(crate) mod macros;
pub mod security;
pub mod types;
pub mod utils;

pub use bdk::prelude::*;

mod route_v3;
pub use route_v3::*;

pub mod features;
#[cfg(test)]
pub mod tests;
