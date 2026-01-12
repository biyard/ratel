use crate::*;

#[derive(
    Debug, Clone, Default, Serialize, Deserialize, aide::OperationIo, schemars::JsonSchema,
)]
pub struct NetworkCentralityRow {
    pub node: String,
    pub degree_centrality: f64,
    pub betweenness_centrality: f64,
}
