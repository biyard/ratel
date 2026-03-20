use crate::canister::auth::require_controller;
use crate::sampling::error::SamplingError;
use crate::sampling::{self, ModelParams, SamplingInput, SamplingResult};
#[cfg(feature = "perf")]
use crate::sampling::SamplingWithMetrics;
use crate::sampling::store::ModelStore;

#[ic_cdk::update]
fn run_sampling(input: SamplingInput) -> SamplingResult {
    require_controller();
    sampling::pipeline::run(input)
        .unwrap_or_else(|e: SamplingError| ic_cdk::api::trap(&e.to_string()))
}

#[cfg(feature = "perf")]
#[ic_cdk::update]
fn run_sampling_with_metrics(input: SamplingInput) -> SamplingWithMetrics {
    require_controller();
    let (result, metrics) = sampling::pipeline::run_with_metrics(input)
        .unwrap_or_else(|e: SamplingError| ic_cdk::api::trap(&e.to_string()));
    SamplingWithMetrics { result, metrics }
}

#[ic_cdk::query]
fn get_model(id: String) -> Option<ModelParams> {
    ModelStore::load(&id)
}
