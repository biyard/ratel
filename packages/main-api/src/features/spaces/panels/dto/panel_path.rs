use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize, OperationIo, JsonSchema)]
pub struct PanelPathParam {
    pub space_pk: Partition,
    pub panel_sk: EntityType,
}

pub type PanelPath = Path<PanelPathParam>;
