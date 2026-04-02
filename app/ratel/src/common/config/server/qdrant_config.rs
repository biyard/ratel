use crate::common::utils::aws::QdrantClient;
use dioxus::fullstack::Lazy;

pub static QDRANT_CLIENT: Lazy<QdrantClient> = Lazy::new(|| async move {
    let url = option_env!("QDRANT_URL")
        .unwrap_or("http://qdrant:6333")
        .to_string();

    let api_key = option_env!("QDRANT_API_KEY").map(|s| s.to_string());

    let prefix = option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local");
    let collection_name = format!("{}-posts", prefix);

    dioxus::Ok(QdrantClient::new(url, collection_name, api_key))
});

#[derive(Debug, Clone, Copy)]
pub struct QdrantConfig {
    pub endpoint: &'static str,
    pub api_key: &'static str,
    pub prefix: &'static str,
}

impl Default for QdrantConfig {
    fn default() -> Self {
        QdrantConfig {
            endpoint: option_env!("QDRANT_URL").unwrap_or("http://qdrant:6333"),
            api_key: option_env!("QDRANT_API_KEY").unwrap_or(""),
            prefix: option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local"),
        }
    }
}
