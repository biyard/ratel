use serde::{Deserialize, Serialize};

use super::error::VotingError;
use super::types::{QuestionOptionCount, QuestionSelection, VoteBallot, VoterTag};
use crate::canister::storage::{BALLOTS, VOTE_COUNTS};

const SEP: char = '\u{1f}';

/// Per-voter ballot storage (internal representation).
/// ciphertext_hash, ciphertext_blob(암호문), submitted_at_ms, selections.
/// 저장 콘텐츠는 stable 버전과 동일 — 보관 위치만 heap(RAM).
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub(crate) struct VoterBallotData {
    pub ciphertext_hash: String,
    pub ciphertext_blob: Vec<u8>,
    pub submitted_at_ms: i64,
    pub selections: Vec<QuestionSelection>,
}

/// 투표 1건 = `(vote_key, voter_tag)` 개별 키. heap HashMap 을 그 자리에서 변경 → 직렬화/클론 없이 O(1).
fn ballot_key(vote_key: &str, voter_tag: &VoterTag) -> String {
    format!("{vote_key}{SEP}{}", voter_tag.0)
}

/// 집계 카운터 키 = `(vote_key, question, option)`.
fn count_key(vote_key: &str, q: u32, o: u32) -> String {
    format!("{vote_key}{SEP}{q}{SEP}{o}")
}

/// 투표 저장(insert/replace). heap 을 그 자리에서 변경 → O(1).
/// 저장 내용·집계 결과는 stable 버전과 동일하게 유지된다.
pub(crate) fn upsert(
    vote_key: &str,
    voter_tag: &VoterTag,
    ballot: &VoteBallot,
) -> Result<bool, VotingError> {
    if ballot.selections.is_empty() {
        return Err(VotingError::EmptyVotes);
    }

    let bkey = ballot_key(vote_key, voter_tag);
    let old = BALLOTS.with(|m| m.borrow().get(&bkey).cloned());
    let is_update = old.is_some();

    // 재투표면 이전 선택의 카운트를 감소(한 voter는 옵션당 1표)
    if let Some(old) = &old {
        for sel in &old.selections {
            let ckey = count_key(vote_key, sel.question_index, sel.option_index);
            VOTE_COUNTS.with(|m| {
                let mut m = m.borrow_mut();
                let cur = m.get(&ckey).copied().unwrap_or(0);
                if cur <= 1 {
                    m.remove(&ckey);
                } else {
                    m.insert(ckey, cur - 1);
                }
            });
        }
    }

    // 새 선택의 카운트 증가
    for sel in &ballot.selections {
        let ckey = count_key(vote_key, sel.question_index, sel.option_index);
        VOTE_COUNTS.with(|m| {
            let mut m = m.borrow_mut();
            let cur = m.get(&ckey).copied().unwrap_or(0);
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
    BALLOTS.with(|m| m.borrow_mut().insert(bkey, data));

    Ok(is_update)
}

/// 질문/옵션별 득표 수 — 기존 get_vote_counts 와 동일한 결과.
/// heap HashMap 은 정렬이 없으므로 prefix(`{vote_key}{SEP}`)로 필터링한다.
pub(crate) fn counts(vote_key: &str) -> Vec<QuestionOptionCount> {
    let prefix = format!("{vote_key}{SEP}");
    let plen = prefix.len();

    VOTE_COUNTS.with(|m| {
        m.borrow()
            .iter()
            .filter_map(|(k, &count)| {
                if count == 0 || !k.starts_with(&prefix) {
                    return None;
                }
                let rest = &k[plen..]; // "{question}{SEP}{option}"
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
    BALLOTS.with(|m| {
        m.borrow()
            .get(&ballot_key(vote_key, voter_tag))
            .map(|d| VoteBallot {
                ciphertext_hash: d.ciphertext_hash.clone(),
                ciphertext_blob: d.ciphertext_blob.clone(),
                submitted_at_ms: d.submitted_at_ms,
                selections: d.selections.clone(),
            })
    })
}
