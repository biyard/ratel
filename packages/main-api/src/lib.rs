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
pub mod security;
pub mod services;
pub mod types;
pub mod utils;

use crate::error::*;
use crate::models::User;
use axum::extract::*;
pub use bdk::prelude::*;
use btracing::{notify, notify_error};
use by_axum::aide::axum::routing::*;
use by_axum::aide::{NoApi, OperationIo};
use by_axum::axum::*;
use by_axum::axum::{
    body::Body,
    http::StatusCode,
    http::request::Parts,
    middleware::{self, Next},
    native_routing as nr,
    response::Response,
};
use constants::*;
use controllers::v3::*;
use features::migration::*;
use schemars::JsonSchema_repr;
use serde::{Deserialize, Serialize};
use ssi::prelude::*;
use tracing::{debug, error, info, warn};
use types::AppState;
use types::*;
use utils::time;
use validator::Validate;

#[cfg(test)]
pub mod tests;
