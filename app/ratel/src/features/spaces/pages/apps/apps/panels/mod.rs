mod components;
mod controllers;
mod types;
mod views;

use dioxus::prelude::*;

pub use components::*;
pub use controllers::*;
pub use types::*;
pub use views::HomePage as SpacePanelsAppPage;

pub use crate::common::attribute::{Age, Gender};
use crate::common::*;
pub use crate::features::spaces::models::*;
pub use crate::features::spaces::space_common::controllers::*;
pub use crate::features::spaces::space_common::hooks::*;
pub use crate::features::spaces::space_common::providers::*;
pub use crate::features::spaces::space_common::types::*;
