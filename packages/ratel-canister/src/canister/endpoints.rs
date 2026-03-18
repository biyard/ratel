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

// ─── Poll Voting ────────────────────────────────────────────────────

#[ic_cdk::update]
fn submit_poll_vote(poll_sk: String, votes: Vec<QuestionVote>) -> SubmitVoteResult {
    require_controller();

    if votes.is_empty() {
        ic_cdk::api::trap("votes cannot be empty");
    }

    let voter_tag = &votes[0].voter_tag;

    // Check for duplicate submission
    if storage::poll_voter_exists(&poll_sk, voter_tag) {
        ic_cdk::api::trap("duplicate vote: this voter has already submitted");
    }

    // Store each vote entry and increment counts
    for vote in &votes {
        if vote.voter_tag != *voter_tag {
            ic_cdk::api::trap("all votes in a submission must have the same voter_tag");
        }

        let encoded =
            candid::encode_one(vote).unwrap_or_else(|e| ic_cdk::api::trap(&e.to_string()));

        storage::poll_vote_insert(
            &poll_sk,
            vote.question_index,
            vote.option_index,
            &vote.voter_tag,
            encoded,
        );
        storage::poll_count_increment(&poll_sk, vote.question_index, vote.option_index);
    }

    // Mark voter as submitted
    storage::poll_voter_mark(&poll_sk, voter_tag);

    let record_id = format!("{}:{}", poll_sk, voter_tag);
    SubmitVoteResult { record_id, poll_sk }
}

#[ic_cdk::query]
fn get_poll_vote_counts(poll_sk: String) -> Vec<QuestionOptionCount> {
    storage::poll_counts_by_poll(&poll_sk)
        .into_iter()
        .map(|(qi, oi, count)| QuestionOptionCount {
            question_index: qi,
            option_index: oi,
            count,
        })
        .collect()
}

#[ic_cdk::query]
fn get_poll_vote_by_tag(poll_sk: String, voter_tag: String) -> Vec<QuestionVote> {
    storage::poll_votes_by_voter(&poll_sk, &voter_tag)
        .into_iter()
        .filter_map(|(_qi, _oi, blob)| candid::decode_one::<QuestionVote>(&blob).ok())
        .collect()
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
