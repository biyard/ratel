mod components;
mod controllers;
mod i18n;
mod interop;
mod models;
mod utils;
mod views;

use dioxus::prelude::*;

pub use controllers::*;
pub use views::IncentivePoolPage as SpaceAppIncentivePoolPage;

use crate::common::*;

type Result<T> = crate::common::Result<T>;
