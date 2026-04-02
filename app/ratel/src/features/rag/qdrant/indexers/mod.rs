pub mod reply_indexer;
pub mod material_indexer;

pub use reply_indexer::*;
pub use material_indexer::*;

use crate::common::utils::aws::QdrantClient;

/// Create a per-discussion Qdrant client using QdrantConfig from ServerConfig.
pub fn get_discussion_qdrant_client(space_id: &str, discussion_sk: &str) -> QdrantClient {
    let cfg = crate::common::CommonConfig::default();
    let qcfg = cfg.qdrant;
    let collection = format!("{}-aimod-{}-{}", qcfg.prefix, space_id, discussion_sk);
    let api_key = if qcfg.api_key.is_empty() {
        None
    } else {
        Some(qcfg.api_key.to_string())
    };
    QdrantClient::new(qcfg.endpoint.to_string(), collection, api_key)
}
