use crate::types::Attribute;
use crate::{
    features::spaces::panels::SpacePanel,
    types::{EntityType, Partition},
};
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
pub struct SpacePanelResponse {
    pub pk: Partition,

    pub name: String,
    pub quotas: i64,
    pub participants: i64,
    pub attributes: Vec<Attribute>,
}

impl From<SpacePanel> for SpacePanelResponse {
    fn from(panel: SpacePanel) -> Self {
        Self {
            pk: match panel.sk {
                EntityType::SpacePanel(v) => Partition::Panel(v.to_string()),
                _ => Partition::Panel("".to_string()),
            },
            name: panel.name,
            quotas: panel.quotas,
            participants: panel.participants,
            attributes: panel.attributes,
        }
    }
}
