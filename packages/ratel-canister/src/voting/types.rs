use std::fmt;

use serde::{Deserialize, Serialize};

/// Opaque identifier for a voter (e.g., hashed user ID).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct VoterTag(pub String);

impl fmt::Display for VoterTag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for VoterTag {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for VoterTag {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Key that identifies a voting context (poll, quiz, etc.).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct VoteKey(pub String);

impl fmt::Display for VoteKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl From<String> for VoteKey {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl AsRef<str> for VoteKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// A lightweight question+option selection (no ciphertext duplication).
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct QuestionSelection {
    pub question_index: u32,
    pub option_index: u32,
}

/// A complete vote ballot submitted by one voter for one poll.
/// Ciphertext is stored once, selections reference individual question+option choices.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct VoteBallot {
    pub ciphertext_hash: String,
    pub ciphertext_blob: Vec<u8>,
    pub submitted_at_ms: i64,
    pub selections: Vec<QuestionSelection>,
}

/// Result returned after successfully submitting a vote.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct SubmitVoteResult {
    pub record_id: String,
    pub vote_key: VoteKey,
}

/// Per-option vote count for a question.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[cfg_attr(feature = "candid", derive(candid::CandidType))]
pub struct QuestionOptionCount {
    pub question_index: u32,
    pub option_index: u32,
    pub count: u64,
}
