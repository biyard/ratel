use crate::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceAnalyze {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,

    pub lda_topics: Vec<TopicRow>,
    #[serde(default)]
    pub lda_html_contents: Option<String>,

    pub network: NetworkGraph,
    #[serde(default)]
    pub network_html_contents: Option<String>,

    pub tf_idf: Vec<TfidfRow>,
    #[serde(default)]
    pub tf_idf_html_contents: Option<String>,

    #[serde(default)]
    pub metadata_url: Option<String>,
}

impl SpaceAnalyze {
    pub fn new(
        space_pk: Partition,
        lda_topics: Vec<TopicRow>,
        network: NetworkGraph,
        tf_idf: Vec<TfidfRow>,
    ) -> Self {
        let now = get_now_timestamp_millis();

        Self {
            pk: space_pk,
            sk: EntityType::SpaceAnalyze,
            created_at: now,
            lda_topics,
            lda_html_contents: None,
            network,
            network_html_contents: None,
            tf_idf,
            tf_idf_html_contents: None,

            metadata_url: None,
        }
    }
}
