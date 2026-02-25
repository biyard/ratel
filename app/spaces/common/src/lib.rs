#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod hooks;
pub mod models;
pub mod types;

#[cfg(not(feature = "server"))]
pub mod interop;

use common::*;

// Re-export
pub use ratel_auth;
pub use ratel_post;

use dioxus::prelude::*;

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
