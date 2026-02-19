use dioxus::{
    fullstack::{Loader, Loading},
    prelude::*,
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub type QueryKey = Vec<String>;

#[derive(Clone)]
pub struct QueryStore {
    versions: Signal<HashMap<String, u64>>,
}

impl QueryStore {
    fn new() -> Self {
        Self {
            versions: Signal::new(HashMap::new()),
        }
    }

    fn version(&self, key: &str) -> u64 {
        self.versions.read().get(key).copied().unwrap_or_default()
    }

    pub fn invalidate(&mut self, key: impl Into<String>) {
        let key = key.into();

        {
            let mut versions = self.versions.write();
            let next = versions
                .get(&key)
                .copied()
                .unwrap_or_default()
                .saturating_add(1);
            versions.insert(key.clone(), next);
        }
    }

    pub fn clear(&mut self) {
        {
            let mut versions = self.versions.write();
            for version in versions.values_mut() {
                *version = version.saturating_add(1);
            }
        }
    }
}

pub fn use_query_store() -> QueryStore {
    // Shared per app root.
    use_root_context(QueryStore::new)
}

#[allow(clippy::result_large_err)]
#[track_caller]
pub fn use_query<F, T, E>(
    key: impl Into<String>,
    mut future: impl FnMut() -> F + 'static,
) -> dioxus::prelude::Result<Loader<T>, Loading>
where
    F: std::future::Future<Output = std::result::Result<T, E>> + 'static,
    T: 'static + Clone + PartialEq + Serialize + DeserializeOwned,
    E: Into<dioxus::CapturedError> + 'static,
{
    let query = use_query_store();
    let key = key.into();

    use_loader(move || {
        // Reactive dependency for this key.
        let _version = query.version(&key);

        future()
    })
}
