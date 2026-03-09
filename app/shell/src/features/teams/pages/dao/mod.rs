#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod dto;
#[cfg(not(feature = "server"))]
pub mod interop;
pub mod layout;
pub mod models;

mod views;
pub use views::*;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
