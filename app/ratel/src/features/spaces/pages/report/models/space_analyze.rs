use crate::features::spaces::pages::report::*;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(
    feature = "server",
    derive(DynamoEntity, schemars::JsonSchema, aide::OperationIo)
)]
pub struct SpaceAnalyze {
    pub pk: Partition,
    pub sk: EntityType,

    #[serde(default)]
    pub created_at: i64,

    #[serde(default)]
    pub lda_topics: Vec<TopicRow>,
    #[serde(default)]
    pub network: NetworkGraph,
    #[serde(default)]
    pub tf_idf: Vec<TfidfRow>,
    #[serde(default)]
    pub remove_topics: Vec<String>,

    #[serde(default)]
    pub html_contents: Option<String>,

    #[serde(default)]
    pub metadata_url: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TopicRow {
    pub topic: String,
    pub keyword: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct TfidfRow {
    pub keyword: String,
    pub tf_idf: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NetworkGraph {
    pub nodes: Vec<NetworkCentralityRow>,
    pub edges: Vec<NetworkEdgeRow>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NetworkCentralityRow {
    pub node: String,
    pub degree_centrality: f64,
    pub betweenness_centrality: f64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct NetworkEdgeRow {
    pub source: String,
    pub target: String,
    pub weight: u32,
}
