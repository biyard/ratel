use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::error::VotingError;
use super::types::{QuestionOptionCount, QuestionSelection, VoteBallot, VoterTag};
use crate::canister::storage::{StorableVoteData, StringKey, VOTE_DATA};

/// Per-voter ballot storage (internal representation).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct VoterBallotData {
    pub ciphertext_hash: String,
    pub ciphertext_blob: Vec<u8>,
    pub submitted_at_ms: i64,
    pub selections: Vec<QuestionSelection>,
}

/// Aggregate vote data stored as a single document per voting context (poll/quiz).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct VoteData {
    /// voter_tag → ballot data (ciphertext stored once per voter)
    pub ballots: BTreeMap<VoterTag, VoterBallotData>,
    /// question_index → option_index → set of voter tags (for fast counting)
    pub counts: BTreeMap<u32, BTreeMap<u32, BTreeSet<VoterTag>>>,
}

impl VoteData {
    pub fn load(key: &str) -> Self {
        VOTE_DATA.with(|m| {
            m.borrow()
                .get(&StringKey(key.to_string()))
                .map(|v| v.0)
                .unwrap_or_default()
        })
    }

    pub fn save(&self, key: &str) {
        VOTE_DATA.with(|m| {
            m.borrow_mut()
                .insert(StringKey(key.to_string()), StorableVoteData(self.clone()));
        });
    }

    pub fn has_voter(&self, voter_tag: &VoterTag) -> bool {
        self.ballots.contains_key(voter_tag)
    }

    /// Insert or replace a ballot for a voter.
    pub fn upsert(
        &mut self,
        voter_tag: &VoterTag,
        ballot: &VoteBallot,
    ) -> Result<bool, VotingError> {
        if ballot.selections.is_empty() {
            return Err(VotingError::EmptyVotes);
        }

        let is_update = self.has_voter(voter_tag);

        // Remove old selections from counts if updating
        if is_update {
            if let Some(old) = self.ballots.get(voter_tag) {
                for sel in &old.selections {
                    if let Some(options) = self.counts.get_mut(&sel.question_index) {
                        if let Some(voters) = options.get_mut(&sel.option_index) {
                            voters.remove(voter_tag);
                        }
                    }
                }
            }
        }

        // Insert new selections into counts
        for sel in &ballot.selections {
            self.counts
                .entry(sel.question_index)
                .or_default()
                .entry(sel.option_index)
                .or_default()
                .insert(voter_tag.clone());
        }

        // Store ballot (ciphertext stored once)
        self.ballots.insert(
            voter_tag.clone(),
            VoterBallotData {
                ciphertext_hash: ballot.ciphertext_hash.clone(),
                ciphertext_blob: ballot.ciphertext_blob.clone(),
                submitted_at_ms: ballot.submitted_at_ms,
                selections: ballot.selections.clone(),
            },
        );

        Ok(is_update)
    }

    pub fn counts(&self) -> Vec<QuestionOptionCount> {
        let mut results = Vec::new();
        for (&qi, options) in &self.counts {
            for (&oi, voters) in options {
                let count = voters.len() as u64;
                if count > 0 {
                    results.push(QuestionOptionCount {
                        question_index: qi,
                        option_index: oi,
                        count,
                    });
                }
            }
        }
        results
    }

    pub fn ballot_by_voter(&self, voter_tag: &VoterTag) -> Option<VoteBallot> {
        self.ballots.get(voter_tag).map(|data| VoteBallot {
            ciphertext_hash: data.ciphertext_hash.clone(),
            ciphertext_blob: data.ciphertext_blob.clone(),
            submitted_at_ms: data.submitted_at_ms,
            selections: data.selections.clone(),
        })
    }
}
