use crate::*;

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct NetworkGraph {
    pub nodes: Vec<NetworkCentralityRow>,
    pub edges: Vec<NetworkEdgeRow>,
}
