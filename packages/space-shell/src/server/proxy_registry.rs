use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

const RESERVED_PATHS: &[&str] = &["spaces", "api", "admin", "assets"];

#[derive(Clone)]
pub struct ProxyRegistry {
    inner: Arc<RwLock<HashMap<String, String>>>,
}

impl ProxyRegistry {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register(&self, base_name: &str, endpoint: &str) -> Result<(), String> {
        if RESERVED_PATHS.contains(&base_name) {
            return Err(format!("'{}' is a reserved path", base_name));
        }
        if base_name.is_empty() {
            return Err("base_name cannot be empty".to_string());
        }
        if endpoint.is_empty() {
            return Err("endpoint cannot be empty".to_string());
        }
        let endpoint = endpoint.trim_end_matches('/').to_string();
        self.inner
            .write()
            .await
            .insert(base_name.to_string(), endpoint);
        Ok(())
    }

    pub async fn unregister(&self, base_name: &str) -> bool {
        self.inner.write().await.remove(base_name).is_some()
    }

    pub async fn get(&self, base_name: &str) -> Option<String> {
        self.inner.read().await.get(base_name).cloned()
    }

    pub async fn list(&self) -> HashMap<String, String> {
        self.inner.read().await.clone()
    }

    /// Extracts the first path segment and looks it up in the registry.
    /// Returns `Some((endpoint, full_path))` if matched.
    pub async fn match_path(&self, path: &str) -> Option<String> {
        let trimmed = path.trim_start_matches('/');
        let base_name = trimmed.split('/').next()?;
        if base_name.is_empty() {
            return None;
        }
        self.get(base_name).await
    }
}
