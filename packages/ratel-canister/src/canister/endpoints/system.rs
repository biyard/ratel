#[cfg(feature = "perf")]
#[ic_cdk::query]
fn get_cycles_balance() -> u64 {
    super::super::perf::cycles_balance()
}

#[cfg(feature = "perf")]
#[ic_cdk::query]
fn get_memory_usage() -> u64 {
    super::super::perf::heap_memory_bytes()
}

#[ic_cdk::query]
fn health() -> String {
    "ok".to_string()
}

#[ic_cdk::query]
fn version() -> String {
    build_version()
}

pub(super) fn build_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    match option_env!("COMMIT") {
        Some(commit) => format!("{}-{}", version, commit),
        None => version.to_string(),
    }
}
