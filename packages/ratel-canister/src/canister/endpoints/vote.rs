use crate::canister::auth::require_controller;
use crate::voting::error::VotingError;
use crate::voting::store;
use crate::voting::{QuestionOptionCount, SubmitVoteResult, VoteBallot, VoteKey, VoterTag};

fn trap(err: VotingError) -> ! {
    ic_cdk::api::trap(&err.to_string())
}

#[cfg(feature = "perf")]
thread_local! {
    static LAST_UPSERT_INSTRUCTIONS: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

#[ic_cdk::update]
fn upsert_vote(vote_key: String, voter_tag: String, ballot: VoteBallot) -> SubmitVoteResult {
    require_controller();

    let voter_tag = VoterTag(voter_tag);
    store::upsert(&vote_key, &voter_tag, &ballot).unwrap_or_else(|e| trap(e));

    #[cfg(feature = "perf")]
    LAST_UPSERT_INSTRUCTIONS.with(|c| c.set(crate::canister::perf::instruction_counter()));

    SubmitVoteResult {
        record_id: format!("{}:{}", vote_key, voter_tag),
        vote_key: VoteKey(vote_key),
    }
}

#[cfg(feature = "perf")]
#[ic_cdk::query]
fn last_upsert_instructions() -> u64 {
    LAST_UPSERT_INSTRUCTIONS.with(|c| c.get())
}

#[ic_cdk::query]
fn get_vote_counts(vote_key: String) -> Vec<QuestionOptionCount> {
    store::counts(&vote_key)
}

#[ic_cdk::query]
fn get_ballot_by_tag(vote_key: String, voter_tag: String) -> Option<VoteBallot> {
    store::ballot_by_voter(&vote_key, &VoterTag(voter_tag))
}
