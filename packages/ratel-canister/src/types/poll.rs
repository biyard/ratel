use serde::{Deserialize, Serialize};

/// A single vote entry for one question+option, submitted to the canister.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct QuestionVote {
    pub question_index: u32,
    pub option_index: u32,
    pub ciphertext_hash: String,
    pub ciphertext_blob: Vec<u8>,
    pub voter_tag: String,
    pub submitted_at_ms: i64,
}

/// Result returned after successfully submitting a vote.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct SubmitVoteResult {
    pub record_id: String,
    pub poll_sk: String,
}

/// Per-option vote count for a poll question.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct QuestionOptionCount {
    pub question_index: u32,
    pub option_index: u32,
    pub count: u64,
}
