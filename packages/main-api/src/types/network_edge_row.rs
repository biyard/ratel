use crate::*;

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct NetworkEdgeRow {
    pub source: String,
    pub target: String,
    pub weight: u32,
}
