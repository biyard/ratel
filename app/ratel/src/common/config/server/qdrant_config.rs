use dioxus::fullstack::Lazy;

pub static QDRANT_CLIENT: Lazy<qdrant_client::Qdrant> = Lazy::new(|| async move {
    let cfg = QdrantConfig::default();
    let mut builder = qdrant_client::Qdrant::from_url(cfg.endpoint);
    if !cfg.api_key.is_empty() {
        builder = builder.api_key(cfg.api_key);
    }
    let client = builder
        .build()
        .map_err(|e| dioxus::prelude::ServerFnError::new(format!("Qdrant client init: {e}")))?;
    dioxus::Ok(client)
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
            endpoint: option_env!("QDRANT_URL").unwrap_or("http://localhost:6334"),
            api_key: option_env!("QDRANT_API_KEY").unwrap_or(""),
            prefix: option_env!("QDRANT_PREFIX").unwrap_or("ratel-local"),
        }
    }
}
