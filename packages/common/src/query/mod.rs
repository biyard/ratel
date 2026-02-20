use dioxus::{
    fullstack::{Loader, Loading},
    prelude::*,
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::HashMap;

pub type QueryKey = Vec<String>;

#[derive(Clone, Copy)]
pub struct QueryStore {
    versions: Store<HashMap<QueryKey, u64>>,
}

impl QueryStore {
    fn new() -> Self {
        Self {
            versions: Store::new(HashMap::new()),
        }
    }

    fn version(&self, key: &QueryKey) -> u64 {
        self.versions
            .get(key.clone())
            .map(|v| *v.read())
            .unwrap_or_default()
    }

    /// Invalidate all queries whose key starts with the given prefix.
    ///
    /// e.g. `invalidate(&["Space"])` invalidates
    /// `["Space"]`, `["Space", "UUID"]`, `["Space", "UUID", "actions"]`, etc.
    pub fn invalidate(&mut self, prefix: &[impl AsRef<str>]) {
        let prefix: QueryKey = prefix.iter().map(|s| s.as_ref().to_string()).collect();

        let mut has_exact = false;
        for (k, mut v) in self.versions.iter() {
            if *k == prefix {
                has_exact = true;
            }
            if k.starts_with(&prefix) {
                let next = (*v.read()).saturating_add(1);
                v.set(next);
            }
        }

        if !has_exact {
            self.versions.insert(prefix, 1);
        }
    }

    pub fn clear(&mut self) {
        for (_, mut version) in self.versions.iter() {
            let next = (*version.read()).saturating_add(1);
            version.set(next);
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
