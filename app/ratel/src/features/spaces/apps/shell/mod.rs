mod i18n;
mod layout;
mod menu;
mod route;

use dioxus::prelude::*;

pub use menu::get_nav_item;
pub use route::Route;
pub use crate::features::spaces::pages::apps::SpaceApp;
pub use types::*;

use crate::common::*;
