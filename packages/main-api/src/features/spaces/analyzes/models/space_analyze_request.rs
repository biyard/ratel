use crate::time::get_now_timestamp_millis;
use crate::*;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, DynamoEntity, JsonSchema, Default)]
pub struct SpaceAnalyzeRequest {
    #[dynamo(index = "gsi1", name = "find_by_analyze_finish", pk)]
    pub pk: Partition,
    pub sk: EntityType,
    pub created_at: i64,
    pub lda_topics: usize,
    pub tf_idf_keywords: usize,
    pub network_top_nodes: usize,
    #[serde(default)]
    pub remove_topics: Vec<String>,
    pub analyze_finish: bool,
    #[serde(default)]
    #[dynamo(index = "gsi1", name = "find_by_analyze_finish", sk)]
    pub analyze_finish_key: String,
}

impl SpaceAnalyzeRequest {
    pub fn new(
        space_pk: Partition,
        lda_topics: usize,
        tf_idf_keywords: usize,
        network_top_nodes: usize,

        remove_topics: Vec<String>,
    ) -> Self {
        let now = get_now_timestamp_millis();
        let uuid = sorted_uuid();
        let analyze_finish_key = Self::finish_key(false);

        Self {
            pk: space_pk,
            sk: EntityType::SpaceAnalyzeRequest(uuid),

            created_at: now,
            lda_topics,
            tf_idf_keywords,
            network_top_nodes,
            remove_topics,
            analyze_finish: false,
            analyze_finish_key,
        }
    }

    pub fn set_analyze_finish(&mut self, finished: bool) {
        self.analyze_finish = finished;
        self.analyze_finish_key = Self::finish_key(finished);
    }

    pub fn pending_key() -> String {
        Self::finish_key(false)
    }

    fn finish_key(finished: bool) -> String {
        if finished {
            "1".to_string()
        } else {
            "0".to_string()
        }
    }
}
