use serde::{Deserialize, Serialize};

use super::error::VotingError;
use super::types::{QuestionOptionCount, QuestionSelection, VoteBallot, VoterTag};
use crate::canister::storage::{StorableBallot, StringKey, BALLOTS, VOTE_COUNTS};

const SEP: char = '\u{1f}';

/// Per-voter ballot storage (internal representation).
/// ciphertext_hash, ciphertext_blob(암호문), submitted_at_ms, selections.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct VoterBallotData {
    pub ciphertext_hash: String,
    pub ciphertext_blob: Vec<u8>,
    pub submitted_at_ms: i64,
    pub selections: Vec<QuestionSelection>,
}

fn ballot_key(vote_key: &str, voter_tag: &VoterTag) -> StringKey {
    StringKey(format!("{vote_key}{SEP}{}", voter_tag.0))
}

fn count_key(vote_key: &str, q: u32, o: u32) -> StringKey {
    StringKey(format!("{vote_key}{SEP}{q}{SEP}{o}"))
}

pub(crate) fn upsert(
    vote_key: &str,
    voter_tag: &VoterTag,
    ballot: &VoteBallot,
) -> Result<bool, VotingError> {
    if ballot.selections.is_empty() {
        return Err(VotingError::EmptyVotes);
    }

    let bkey = ballot_key(vote_key, voter_tag);
    let old = BALLOTS.with(|m| m.borrow().get(&bkey));
    let is_update = old.is_some();

    if let Some(old) = &old {
        for sel in &old.0.selections {
            let ckey = count_key(vote_key, sel.question_index, sel.option_index);
            VOTE_COUNTS.with(|m| {
                let mut m = m.borrow_mut();
                let cur = m.get(&ckey).unwrap_or(0);
                if cur <= 1 {
                    m.remove(&ckey);
                } else {
                    m.insert(ckey.clone(), cur - 1);
                }
            });
        }
    }

    // 새 선택의 카운트 증가
    for sel in &ballot.selections {
        let ckey = count_key(vote_key, sel.question_index, sel.option_index);
        VOTE_COUNTS.with(|m| {
            let mut m = m.borrow_mut();
            let cur = m.get(&ckey).unwrap_or(0);
            m.insert(ckey, cur + 1);
        });
    }

    // ballot 전체 저장 (암호문 ciphertext_blob 포함 — 콘텐츠 보존)
    let data = VoterBallotData {
        ciphertext_hash: ballot.ciphertext_hash.clone(),
        ciphertext_blob: ballot.ciphertext_blob.clone(),
        submitted_at_ms: ballot.submitted_at_ms,
        selections: ballot.selections.clone(),
    };
    BALLOTS.with(|m| m.borrow_mut().insert(bkey, StorableBallot(data)));

    Ok(is_update)
}

/// 질문/옵션별 득표 수 — 기존 get_vote_counts 와 동일한 결과.
pub(crate) fn counts(vote_key: &str) -> Vec<QuestionOptionCount> {
    let start = StringKey(format!("{vote_key}{SEP}"));
    // 0x1F 다음 바이트(0x20)를 상한으로 → 해당 vote_key 의 카운터 키만 범위 스캔
    let end = StringKey(format!("{vote_key}\u{20}"));
    let plen = start.0.len();

    VOTE_COUNTS.with(|m| {
        m.borrow()
            .range(start..end)
            .filter_map(|entry| {
                let count = entry.value();
                if count == 0 {
                    return None;
                }
                let k = entry.key();
                let rest = &k.0[plen..]; // "{question}{SEP}{option}"
                let mut it = rest.split(SEP);
                let q: u32 = it.next()?.parse().ok()?;
                let o: u32 = it.next()?.parse().ok()?;
                Some(QuestionOptionCount {
                    question_index: q,
                    option_index: o,
                    count,
                })
            })
            .collect()
    })
}

pub(crate) fn ballot_by_voter(vote_key: &str, voter_tag: &VoterTag) -> Option<VoteBallot> {
    BALLOTS
        .with(|m| m.borrow().get(&ballot_key(vote_key, voter_tag)))
        .map(|sb| {
            let d = sb.0;
            VoteBallot {
                ciphertext_hash: d.ciphertext_hash,
                ciphertext_blob: d.ciphertext_blob,
                submitted_at_ms: d.submitted_at_ms,
                selections: d.selections,
            }
        })
}
