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
pub use crate::features::spaces::models::{
    CollectiveAttribute, PanelAttribute, PanelAttributeWithQuota, SpacePanelQuota,
    VerifiableAttribute, VerifiableAttributeWithQuota,
};
pub use crate::features::spaces::space_common::controllers::{UpdateSpaceRequest, update_space};
pub use crate::features::spaces::space_common::hooks::{use_space, use_space_role};
pub use crate::features::spaces::space_common::providers::use_space_context;
pub use crate::features::spaces::space_common::types::space_key;
