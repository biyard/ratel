#![allow(unused)]
pub mod config;
pub mod controllers;
pub mod dto;
pub mod hooks;
pub mod layout;
pub mod models;
pub mod route;
#[cfg(not(feature = "server"))]
pub mod web;

#[cfg(feature = "server")]
pub mod server;

pub use layout::SpaceLayout;

pub use route::Route;

use common::*;
use dioxus::prelude::*;

// Re-export common types/macros for model derives.
pub use common::macros::dynamo_entity::DynamoEntity;
pub use common::types::*;
pub use common::{DeserializeFromStr, DynamoEnum, EnumProp, SerializeDisplay};

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

pub use hooks::*;
use serde::{Deserialize, Serialize};
use space_action_shell as actions;
use space_app_shell as apps;
use space_page_dashboard as dashboard;
use space_page_overview as overview;
use space_page_report as report;

#[cfg(feature = "server")]
#[derive(Clone)]
pub struct AppState {
    pub upstream_url: String,
}

#[cfg(feature = "server")]
use dioxus::fullstack::{axum::extract::FromRef, FullstackContext};

#[cfg(feature = "server")]
impl FromRef<FullstackContext> for AppState {
    fn from_ref(state: &FullstackContext) -> Self {
        state.extension::<AppState>().unwrap()
    }
}
