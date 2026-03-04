#![allow(unused)]
pub mod components;
pub mod controllers;
pub mod hooks;
pub mod layout;
pub mod models;
pub mod route;

#[cfg(not(feature = "server"))]
pub mod interop;

mod views;

pub use models::*;
pub use route::Route;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
