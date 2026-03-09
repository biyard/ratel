#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod layout;
pub mod route;

pub mod pages;
use pages::*;
mod views;

pub use components::*;
pub use route::*;

use common::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use serde::{Deserialize, Serialize};
