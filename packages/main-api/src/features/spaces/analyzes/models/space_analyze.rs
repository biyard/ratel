use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceAnalyze {
    pub pk: Partition,
    pub sk: EntityType,

    pub lda_topics: Option<Vec<TopicRow>>,
    pub network: Option<NetworkGraph>,
    pub tf_idf: Option<Vec<TfidfRow>>,
}

impl SpaceAnalyze {
    pub fn new(
        space_pk: Partition,
        lda_topics: Option<Vec<TopicRow>>,
        network: Option<NetworkGraph>,
        tf_idf: Option<Vec<TfidfRow>>,
    ) -> Self {
        Self {
            pk: space_pk,
            sk: EntityType::SpaceAnalyze,
            lda_topics,
            network,
            tf_idf,
        }
    }
}
