#![allow(unused_imports)]

pub mod api_main;
pub mod config;
pub mod controllers;
pub mod error;
pub mod models;
pub mod proxy;
pub mod types;

pub use api_main::*;
pub use bdk::prelude::*;
pub use error::Error;
pub use schemars::JsonSchema;
pub use serde::{Deserialize, Serialize};
pub use serde_with::{DeserializeFromStr, SerializeDisplay};
pub use types::*;
pub type Result<T> = std::result::Result<T, Error>;

use by_axum::aide::axum::routing::*;
use by_axum::aide::{NoApi, OperationIo};
use by_axum::axum::*;
use by_axum::axum::{
    body::Body,
    http::request::Parts,
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    native_routing as nr,
    response::Response,
};
