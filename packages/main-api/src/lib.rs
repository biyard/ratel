#![allow(unused_imports)]
pub type Result<T> = std::result::Result<T, crate::error::Error>;
pub type Error = crate::error::Error;

pub mod api_main;
pub mod config;
pub mod constants;
pub mod controllers;
pub mod error;
pub mod features;
pub(crate) mod macros;
pub mod models;
pub mod route;
pub mod security;
pub mod services;
pub mod types;
pub mod utils;

use crate::models::User;
use axum::extract::*;
pub use bdk::prelude::*;
use by_axum::aide::axum::routing::*;
use by_axum::aide::{NoApi, OperationIo};
use by_axum::axum::*;
use controllers::v3::*;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, warn};
use types::AppState;

#[cfg(test)]
pub mod tests;
