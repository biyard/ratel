use dioxus::{
    fullstack::{Loader, Loading},
    prelude::*,
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub type QueryKey = Vec<String>;

#[derive(Clone)]
pub struct QueryStore {
    versions: Signal<HashMap<QueryKey, u64>>,
}

impl QueryStore {
    fn new() -> Self {
        Self {
            versions: Signal::new(HashMap::new()),
        }
    }

    fn version(&self, key: &QueryKey) -> u64 {
        self.versions.read().get(key).copied().unwrap_or_default()
    }

    /// Invalidate all queries whose key starts with the given prefix.
    ///
    /// e.g. `invalidate(&["Space"])` invalidates
    /// `["Space"]`, `["Space", "UUID"]`, `["Space", "UUID", "actions"]`, etc.
    pub fn invalidate(&mut self, prefix: &[impl AsRef<str>]) {
        let prefix: QueryKey = prefix.iter().map(|s| s.as_ref().to_string()).collect();
        let mut versions = self.versions.write();

        let keys_to_bump: Vec<QueryKey> = versions
            .keys()
            .filter(|k| k.starts_with(&prefix))
            .cloned()
            .collect();

        for key in keys_to_bump {
            let next = versions
                .get(&key)
                .copied()
                .unwrap_or_default()
                .saturating_add(1);
            versions.insert(key, next);
        }

        if !versions.contains_key(&prefix) {
            versions.insert(prefix, 1);
        }
    }

    pub fn clear(&mut self) {
        let mut versions = self.versions.write();
        for version in versions.values_mut() {
            *version = version.saturating_add(1);
        }
    }
}

pub fn use_query_store() -> QueryStore {
    use_root_context(QueryStore::new)
}

#[allow(clippy::result_large_err)]
#[track_caller]
pub fn use_query<F, T, E>(
    key: &[impl AsRef<str>],
    mut future: impl FnMut() -> F + 'static,
) -> dioxus::prelude::Result<Loader<T>, Loading>
where
    F: std::future::Future<Output = std::result::Result<T, E>> + 'static,
    T: 'static + Clone + PartialEq + Serialize + DeserializeOwned,
    E: Into<dioxus::CapturedError> + 'static,
{
    let query = use_query_store();
    let key: QueryKey = key.iter().map(|s| s.as_ref().to_string()).collect();

    use_loader(move || {
        let _version = query.version(&key);
        future()
    })
}
