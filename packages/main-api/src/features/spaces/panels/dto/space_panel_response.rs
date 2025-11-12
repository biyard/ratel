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
    pub pk: Partition,

    pub quotas: i64,
    pub attributes: Vec<PanelAttribute>,

    pub panel_quotas: Vec<SpacePanelQuota>,
}

impl From<SpacePanels> for SpacePanelsResponse {
    fn from(panel: SpacePanels) -> Self {
        Self {
            pk: panel.pk,
            quotas: panel.quotas,
            attributes: panel.attributes,
            panel_quotas: vec![],
        }
    }
}
