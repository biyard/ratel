use crate::canister::auth::require_controller;
use crate::voting::error::VotingError;
use crate::voting::store::VoteData;
use crate::voting::{QuestionOptionCount, QuestionVote, SubmitVoteResult, VoteKey, VoterTag};

fn trap(err: VotingError) -> ! {
    ic_cdk::api::trap(&err.to_string())
}

#[ic_cdk::update]
fn upsert_vote(vote_key: String, voter_tag: VoterTag, votes: Vec<QuestionVote>) -> SubmitVoteResult {
    require_controller();

    let mut data = VoteData::load(&vote_key);
    data.upsert(&voter_tag, &votes).unwrap_or_else(|e| trap(e));
    data.save(&vote_key);

    SubmitVoteResult {
        record_id: format!("{}:{}", vote_key, voter_tag),
        vote_key: VoteKey(vote_key),
    }
}

#[ic_cdk::query]
fn get_vote_counts(vote_key: String) -> Vec<QuestionOptionCount> {
    VoteData::load(&vote_key).counts()
}

#[ic_cdk::query]
fn get_vote_by_tag(vote_key: String, voter_tag: VoterTag) -> Vec<QuestionVote> {
    VoteData::load(&vote_key).votes_by_voter(&voter_tag)
}
