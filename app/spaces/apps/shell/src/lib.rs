mod i18n;
mod layout;
mod menu;
mod route;

use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;
pub use space_app_main::SpaceApp;
pub use types::*;

use common::*;
