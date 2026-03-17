use candid::CandidType;
use serde::Deserialize;

use crate::canister::storage;
use crate::error::SamplingError;
use crate::pipeline;
use crate::types::*;

fn require_controller() {
    if !ic_cdk::api::is_controller(&ic_cdk::api::msg_caller()) {
        ic_cdk::api::trap("unauthorized: controller only");
    }
}

#[ic_cdk::update]
fn run_sampling(input: SamplingInput) -> SamplingResult {
    require_controller();
    pipeline::run(input).unwrap_or_else(|e: SamplingError| ic_cdk::api::trap(&e.to_string()))
}

#[cfg(feature = "perf")]
#[ic_cdk::update]
fn run_sampling_with_metrics(input: SamplingInput) -> SamplingWithMetrics {
    require_controller();
    let (result, metrics) = pipeline::run_with_metrics(input)
        .unwrap_or_else(|e: SamplingError| ic_cdk::api::trap(&e.to_string()));
    SamplingWithMetrics { result, metrics }
}

#[ic_cdk::query]
fn get_model(id: String) -> Option<ModelParams> {
    storage::load(&id)
}

#[cfg(feature = "perf")]
#[ic_cdk::query]
fn get_cycles_balance() -> u64 {
    super::perf::cycles_balance()
}

#[cfg(feature = "perf")]
#[ic_cdk::query]
fn get_memory_usage() -> u64 {
    super::perf::heap_memory_bytes()
}

#[ic_cdk::query]
fn health() -> String {
    "ok".to_string()
}

#[ic_cdk::query]
fn version() -> String {
    build_version()
}

fn build_version() -> String {
    let version = env!("CARGO_PKG_VERSION");
    match option_env!("COMMIT") {
        Some(commit) => format!("{}-{}", version, commit),
        None => version.to_string(),
    }
}

// ─── HTTP Gateway ────────────────────────────────────────────────────

#[derive(CandidType, Deserialize)]
pub(super) struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[derive(CandidType)]
pub(super) struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
}

#[ic_cdk::query]
fn http_request(req: HttpRequest) -> HttpResponse {
    let path = req.url.split('?').next().unwrap_or(&req.url);

    match path {
        "/version" => HttpResponse {
            status_code: 200,
            headers: vec![("content-type".into(), "text/plain".into())],
            body: build_version().into_bytes(),
        },
        "/health" => HttpResponse {
            status_code: 200,
            headers: vec![("content-type".into(), "text/plain".into())],
            body: b"ok".to_vec(),
        },
        _ => HttpResponse {
            status_code: 404,
            headers: vec![("content-type".into(), "text/plain".into())],
            body: b"not found".to_vec(),
        },
    }
}
