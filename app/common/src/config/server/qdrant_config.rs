use crate::utils::aws::QdrantClient;
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
