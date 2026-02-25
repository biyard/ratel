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

    fn register(&mut self, key: &QueryKey) {
        let mut prefix = Vec::with_capacity(key.len());
        for part in key {
            prefix.push(part.clone());
            if self.versions.get(prefix.clone()).is_none() {
                self.versions.insert(prefix.clone(), 0);
            }
        }
    }

    fn stamp(&self, key: &QueryKey) -> u64 {
        let mut prefix = Vec::with_capacity(key.len());
        let mut stamp = 0_u64;

        for (depth, part) in key.iter().enumerate() {
            prefix.push(part.clone());
            let version = self
                .versions
                .get(prefix.clone())
                .map(|v| *v.read())
                .unwrap_or_default();

            stamp = stamp
                .wrapping_mul(1_000_003)
                .wrapping_add(version ^ ((depth as u64) + 1));
        }

        stamp
    }

    pub fn invalidate(&mut self, prefix: &[impl AsRef<str>]) {
        let prefix: QueryKey = prefix.iter().map(|s| s.as_ref().to_string()).collect();
        self.register(&prefix);

        if let Some(mut version) = self.versions.get(prefix) {
            let next = (*version.read()).saturating_add(1);
            version.set(next);
        }
    }

    pub fn clear(&mut self) {
        for (_, mut version) in self.versions.iter() {
            let next = (*version.read()).saturating_add(1);
            version.set(next);
        }
    }
}

pub fn query_provider() -> QueryStore {
    use_context_provider(QueryStore::new)
}

fn use_query_store() -> QueryStore {
    try_consume_context::<QueryStore>()
        .expect("#[PROVIDER NEEDED] use_query_store must be used in a `query_provider`")
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

    use_effect(use_reactive((&key,), {
        let mut query = query;
        move |(key,)| {
            query.register(&key);
        }
    }));

    use_loader(use_reactive((&key,), move |(key,)| {
        let _version = query.stamp(&key);
        future()
    }))
}

pub fn invalidate_query(key: &[impl AsRef<str>]) {
    let mut query = use_query_store();
    query.invalidate(key);
}
