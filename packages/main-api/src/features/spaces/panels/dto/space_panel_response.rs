use crate::features::spaces::panels::PanelAttribute;
use crate::features::spaces::panels::SpacePanelQuota;
use crate::features::spaces::panels::SpacePanels;
use crate::types::Attribute;
use crate::types::{EntityType, Partition};
use bdk::prelude::*;

#[derive(
    Debug,
    Clone,
    Default,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    aide::OperationIo,
)]
pub struct SpacePanelsResponse {
    pub quotas: i64,
    pub remains: i64,
}
