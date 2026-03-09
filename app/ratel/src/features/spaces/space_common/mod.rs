#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod hooks;
pub mod models;
pub mod types;

#[cfg(not(feature = "server"))]
pub mod interop;
pub mod providers;

use crate::common::*;

// Re-export
pub use crate::features::auth as ratel_auth;
pub use crate::features::posts as ratel_post;

use dioxus::prelude::*;

type Result<T> = crate::common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
