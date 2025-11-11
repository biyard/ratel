use crate::features::spaces::panels::PanelAttribute;
use crate::types::*;
use bdk::prelude::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpacePanels {
    pub pk: Partition,
    pub sk: EntityType,

    pub quotas: i64,
    pub remains: i64,
    pub attributes: Vec<PanelAttribute>,
}

impl SpacePanels {
    pub fn new(space_pk: Partition, quotas: i64, attributes: Vec<PanelAttribute>) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpacePanels,
            quotas,
            remains: quotas,
            attributes,
        }
    }
}
