pub mod controllers;
#[path = "views/i18n.rs"]
pub mod i18n;

mod views;
pub use i18n::*;
pub use views::*;

use crate::common::*;
