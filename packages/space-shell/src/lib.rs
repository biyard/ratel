#![allow(unused)]
mod components;
pub mod config;
pub mod controllers;
pub mod layout;
pub mod route;
pub mod views;

pub use layout::SpaceLayout;
pub use views::Home;

pub use route::Route;

use common::*;
use components::*;
use dioxus::prelude::*;

type Result<T> = common::Result<T>;
type DioxusResult<T> = dioxus::prelude::Result<T>;

use space_page_actions as actions;
use space_page_apps as apps;
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
