#![allow(unused)]
pub mod assets;
mod components;
pub mod layout;
pub mod route;
pub mod views;

pub use layout::SpaceLayout;
pub use views::Home;

pub use assets::*;
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
