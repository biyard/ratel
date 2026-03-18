use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use super::error::VotingError;
use super::types::{QuestionVote, VoterTag};
use crate::canister::storage::{StorableVoteData, StringKey, VOTE_DATA};

/// Vote data for a single option within a question.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct OptionData {
    pub count: u64,
    /// voter_tag → encoded vote blob
    pub votes: BTreeMap<VoterTag, Vec<u8>>,
}

/// All vote data for a single question.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct QuestionData {
    /// option_index → OptionData
    pub options: BTreeMap<u32, OptionData>,
}

/// Aggregate vote data stored as a single document per voting context (poll/quiz).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct VoteData {
    /// question_index → QuestionData
    pub questions: BTreeMap<u32, QuestionData>,
    /// Set of voter tags that have voted
    pub voters: BTreeSet<VoterTag>,
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
        self.voters.contains(voter_tag)
    }

    pub fn submit(
        &mut self,
        voter_tag: &VoterTag,
        votes: &[QuestionVote],
    ) -> Result<(), VotingError> {
        if votes.is_empty() {
            return Err(VotingError::EmptyVotes);
        }
        if self.has_voter(voter_tag) {
            return Err(VotingError::DuplicateVoter(voter_tag.to_string()));
        }

        self.voters.insert(voter_tag.clone());
        for vote in votes {
            let encoded = candid::encode_one(vote)
                .map_err(|e| VotingError::EncodeFailed(e.to_string()))?;
            let option = self
                .questions
                .entry(vote.question_index)
                .or_default()
                .options
                .entry(vote.option_index)
                .or_default();
            option.count += 1;
            option.votes.insert(voter_tag.clone(), encoded);
        }
        Ok(())
    }

    pub fn update(
        &mut self,
        voter_tag: &VoterTag,
        votes: &[QuestionVote],
    ) -> Result<(), VotingError> {
        if votes.is_empty() {
            return Err(VotingError::EmptyVotes);
        }
        if !self.has_voter(voter_tag) {
            return Err(VotingError::VoterNotFound(voter_tag.to_string()));
        }

        // Remove old votes
        for question in self.questions.values_mut() {
            for option in question.options.values_mut() {
                if option.votes.remove(voter_tag).is_some() {
                    option.count = option.count.saturating_sub(1);
                }
            }
        }

        // Insert new votes
        for vote in votes {
            let encoded = candid::encode_one(vote)
                .map_err(|e| VotingError::EncodeFailed(e.to_string()))?;
            let option = self
                .questions
                .entry(vote.question_index)
                .or_default()
                .options
                .entry(vote.option_index)
                .or_default();
            option.count += 1;
            option.votes.insert(voter_tag.clone(), encoded);
        }
        Ok(())
    }

    pub fn counts(&self) -> Vec<QuestionOptionCount> {
        let mut results = Vec::new();
        for (&qi, question) in &self.questions {
            for (&oi, option) in &question.options {
                if option.count > 0 {
                    results.push(QuestionOptionCount {
                        question_index: qi,
                        option_index: oi,
                        count: option.count,
                    });
                }
            }
        }
        results
    }

    pub fn votes_by_voter(&self, voter_tag: &VoterTag) -> Vec<QuestionVote> {
        let mut results = Vec::new();
        for question in self.questions.values() {
            for option in question.options.values() {
                if let Some(blob) = option.votes.get(voter_tag) {
                    if let Ok(vote) = candid::decode_one::<QuestionVote>(blob) {
                        results.push(vote);
                    }
                }
            }
        }
        results
    }
}

use super::types::QuestionOptionCount;
