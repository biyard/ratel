//! Per-source matchers and the AND-intersection driver they feed.
//!
//! Same logic backs the CREATE preview API and the auto-analysis stream
//! Lambda — neither path should re-derive how a chip "matches" a user
//! independently. Lifted here from `preview_analyze_report.rs` so the
//! two callers stay byte-identical.
//!
//! Each `match_*` returns `(matched_user_pks, matched_record_count,
//! matched_records)`: the user_pk set feeds the AND-intersection across
//! chips; the matched count feeds the preview's "해당되는 데이터 수"
//! tile (matches what the user sees in the records table — only rows
//! that actually pass the filter, NOT every scanned row); the record
//! list feeds the "사용된 raw data" view (preview sample + detail
//! records page) — frozen at report save time so the rows never drift
//! even if source data is later edited. The matched count is tracked
//! separately from `records.len()` because `records` is capped at
//! `PER_FILTER_RECORD_CAP` (1000) — the count keeps growing past the
//! cap so the "데이터 수" tile reflects the true matched total.
//!
//! Output `user_pks` are stringified so they survive HashSet
//! intersection without needing to derive `Hash` on `Partition`.

use crate::features::spaces::pages::apps::apps::analyzes::*;
use std::collections::HashSet;

/// Hard cap on how many matching records we record per filter. Reports
/// that match more than this are still saved — the count tiles stay
/// accurate via the scanned-count path — but only the first N rows are
/// retained for the records view. Picked to keep the report row well
/// under DynamoDB's 400KB per-item ceiling: 1000 refs × ~200 bytes
/// ≈ 200KB, leaving headroom for ~5 filters in one report.
pub const PER_FILTER_RECORD_CAP: usize = 1000;

/// Drive the chip set through every matcher and AND the per-chip user
/// sets together. Returns the intersection user_pks, the total count
/// of MATCHED records summed across all chips (matches the rows the
/// records page renders, not the rows scanned), AND the matched
/// record refs (capped per filter; flat list, each ref carries its
/// `filter_idx` pointing back into `filters`). Empty filter list is
/// the caller's responsibility to short-circuit (it would otherwise
/// return an empty set, which AND-collapses to the empty set rather
/// than "everyone").
pub async fn intersect_filters(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    filters: &[AnalyzeReportFilter],
) -> Result<(HashSet<String>, i64, Vec<MatchedRecordRef>)> {
    let mut intersection: Option<HashSet<String>> = None;
    let mut data_count: i64 = 0;
    let mut all_records: Vec<MatchedRecordRef> = Vec::new();

    for (idx, filter) in filters.iter().enumerate() {
        let (matched, matched_count, mut records) = match_filter(cli, space_pk, filter).await?;
        data_count += matched_count;
        // `match_*` doesn't know its position in the parent vec; tag the
        // refs here so the consumer can group rows under their chip.
        for r in records.iter_mut() {
            r.filter_idx = idx as u32;
        }
        all_records.append(&mut records);
        intersection = Some(match intersection {
            Some(prev) => prev.intersection(&matched).cloned().collect(),
            None => matched,
        });
    }

    Ok((intersection.unwrap_or_default(), data_count, all_records))
}

/// Walk the space's participants via the gsi2 (`find_by_space`) index
/// and return their user_pks. Used both as the unrestricted denominator
/// for empty-filter previews and as the candidate pool for quiz
/// matching (which has no quiz-only GSI).
pub async fn list_participant_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<Partition>> {
    use crate::common::models::space::SpaceParticipant;

    let mut bookmark: Option<String> = None;
    let mut user_pks: Vec<Partition> = Vec::new();

    loop {
        let mut opt = SpaceParticipant::opt().limit(100);
        if let Some(b) = bookmark.clone() {
            opt = opt.bookmark(b);
        }
        let (rows, next) = SpaceParticipant::find_by_space(cli, space_pk.clone(), opt).await?;
        for row in rows {
            if row.user_pk != Partition::None {
                user_pks.push(row.user_pk);
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok(user_pks)
}

pub async fn count_all_participants(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<i64> {
    Ok(list_participant_user_pks(cli, space_pk).await?.len() as i64)
}

async fn match_filter(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    filter: &AnalyzeReportFilter,
) -> Result<(HashSet<String>, i64, Vec<MatchedRecordRef>)> {
    match filter.source {
        AnalyzeFilterSource::Poll => match_poll(cli, space_pk, filter).await,
        AnalyzeFilterSource::Quiz => match_quiz(cli, space_pk, filter).await,
        AnalyzeFilterSource::Discussion => match_discussion(cli, filter).await,
        AnalyzeFilterSource::Follow => match_follow(cli, filter).await,
    }
}

async fn match_poll(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    filter: &AnalyzeReportFilter,
) -> Result<(HashSet<String>, i64, Vec<MatchedRecordRef>)> {
    use crate::features::spaces::pages::actions::actions::poll::{
        SpacePollUserAnswer, SpacePollUserAnswerQueryOption,
    };

    let q_idx: usize = match filter.question_id.parse() {
        Ok(v) => v,
        Err(_) => return Ok((HashSet::new(), 0, Vec::new())),
    };
    let o_idx: u32 = match filter.option_id.parse() {
        Ok(v) => v,
        Err(_) => return Ok((HashSet::new(), 0, Vec::new())),
    };
    let poll_sk = EntityType::SpacePoll(filter.item_id.clone());
    let gsi_sk = EntityType::SpacePollUserAnswer(space_pk.to_string(), poll_sk.to_string());

    let mut bookmark: Option<String> = None;
    let mut matched: HashSet<String> = HashSet::new();
    let mut records: Vec<MatchedRecordRef> = Vec::new();
    let mut matched_count: i64 = 0;

    loop {
        let opt = if let Some(b) = bookmark.clone() {
            SpacePollUserAnswerQueryOption::builder().bookmark(b)
        } else {
            SpacePollUserAnswerQueryOption::builder()
        };
        let (rows, next) = SpacePollUserAnswer::find_by_space_pk(cli, &gsi_sk, opt).await?;
        for row in rows {
            if let Some(answer) = row.answers.get(q_idx) {
                if answer.to_option_indices().contains(&o_idx) {
                    matched_count += 1;
                    let user_key = row
                        .user_pk
                        .as_ref()
                        .map(|pk| pk.to_string())
                        .unwrap_or_else(|| row.pk.to_string());
                    matched.insert(user_key.clone());
                    if records.len() < PER_FILTER_RECORD_CAP {
                        records.push(MatchedRecordRef {
                            source: AnalyzeFilterSource::Poll,
                            filter_idx: 0,
                            user_pk: user_key,
                            item_id: filter.item_id.clone(),
                            sub_id: String::new(),
                            question_idx: q_idx as u32,
                            option_idx: o_idx,
                        });
                    }
                }
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok((matched, matched_count, records))
}

/// Quiz attempts live on a `(user_pk, quiz_id)`-keyed GSI; there is no
/// index that yields every attempt for a given quiz alone. The
/// preview enumerates space participants and queries each user's
/// latest attempt (one O(1) GSI hit per user). All lookups run
/// concurrently via `futures::future::join_all` so wall-clock stays
/// bounded regardless of participant count — same fan-out pattern
/// used by `list_hot_spaces` and other multi-key dispatch paths.
async fn match_quiz(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    filter: &AnalyzeReportFilter,
) -> Result<(HashSet<String>, i64, Vec<MatchedRecordRef>)> {
    use crate::features::spaces::pages::actions::actions::quiz::SpaceQuizAttempt;

    let q_idx: usize = match filter.question_id.parse() {
        Ok(v) => v,
        Err(_) => return Ok((HashSet::new(), 0, Vec::new())),
    };
    let o_idx: u32 = match filter.option_id.parse() {
        Ok(v) => v,
        Err(_) => return Ok((HashSet::new(), 0, Vec::new())),
    };
    let quiz_id: SpaceQuizEntityType = filter.item_id.clone().into();

    let user_pks = list_participant_user_pks(cli, space_pk).await?;

    let lookups = user_pks.iter().map(|user_pk| {
        let quiz_id = quiz_id.clone();
        let user_pk = user_pk.clone();
        async move {
            let attempt =
                SpaceQuizAttempt::find_latest_by_quiz_user(cli, &quiz_id, &user_pk).await;
            (user_pk, attempt)
        }
    });

    let results = futures::future::join_all(lookups).await;

    let mut matched: HashSet<String> = HashSet::new();
    let mut records: Vec<MatchedRecordRef> = Vec::new();
    let mut matched_count: i64 = 0;
    for (user_pk, attempt_res) in results {
        let attempt = match attempt_res? {
            Some(a) => a,
            None => continue,
        };
        if let Some(answer) = attempt.answers.get(q_idx) {
            if answer.to_option_indices().contains(&o_idx) {
                matched_count += 1;
                let user_key = user_pk.to_string();
                matched.insert(user_key.clone());
                if records.len() < PER_FILTER_RECORD_CAP {
                    records.push(MatchedRecordRef {
                        source: AnalyzeFilterSource::Quiz,
                        filter_idx: 0,
                        user_pk: user_key,
                        item_id: filter.item_id.clone(),
                        sub_id: String::new(),
                        question_idx: q_idx as u32,
                        option_idx: o_idx,
                    });
                }
            }
        }
    }

    Ok((matched, matched_count, records))
}

async fn match_discussion(
    cli: &aws_sdk_dynamodb::Client,
    filter: &AnalyzeReportFilter,
) -> Result<(HashSet<String>, i64, Vec<MatchedRecordRef>)> {
    let post_pk = Partition::SpacePost(filter.item_id.clone());
    let keyword = filter.option_text.trim();
    if keyword.is_empty() {
        return Ok((HashSet::new(), 0, Vec::new()));
    }
    let needle = keyword.to_lowercase();

    let mut matched: HashSet<String> = HashSet::new();
    let mut records: Vec<MatchedRecordRef> = Vec::new();
    let mut matched_count: i64 = 0;

    // Walks top-level comments AND replies (`iter_post_comments`
    // covers both prefixes). Same keyword-in-body semantic as before
    // — just no longer reply-blind.
    let item_id = filter.item_id.clone();
    iter_post_comments(cli, post_pk, |row| {
        if row.content.to_lowercase().contains(&needle) {
            matched_count += 1;
            let user_key = row.author_pk.to_string();
            matched.insert(user_key.clone());
            if records.len() < PER_FILTER_RECORD_CAP {
                records.push(MatchedRecordRef {
                    source: AnalyzeFilterSource::Discussion,
                    filter_idx: 0,
                    user_pk: user_key,
                    item_id: item_id.clone(),
                    sub_id: row.sk.to_string(),
                    question_idx: 0,
                    option_idx: 0,
                });
            }
        }
    })
    .await?;

    Ok((matched, matched_count, records))
}

async fn match_follow(
    cli: &aws_sdk_dynamodb::Client,
    filter: &AnalyzeReportFilter,
) -> Result<(HashSet<String>, i64, Vec<MatchedRecordRef>)> {
    use crate::common::models::auth::UserFollow;

    // Follow chip's `option_id` carries the target user's partition key
    // string ("USER#..." or "TEAM#..."). Query every UserFollow row
    // pinned to that target — each row's `user_pk` is one follower.
    let target_pk_str = if filter.option_id.is_empty() {
        return Ok((HashSet::new(), 0, Vec::new()));
    } else {
        filter.option_id.clone()
    };
    let target_pk: Partition = match target_pk_str.parse() {
        Ok(p) => p,
        Err(_) => return Ok((HashSet::new(), 0, Vec::new())),
    };

    let mut bookmark: Option<String> = None;
    let mut matched: HashSet<String> = HashSet::new();
    let mut records: Vec<MatchedRecordRef> = Vec::new();
    let mut matched_count: i64 = 0;
    let follower_sk = EntityType::Follower(String::default()).to_string();

    loop {
        let mut opt = UserFollow::opt().sk(follower_sk.clone()).limit(100);
        if let Some(b) = bookmark.clone() {
            opt = opt.bookmark(b);
        }
        let (rows, next) = UserFollow::query(cli, target_pk.clone(), opt).await?;
        // Every UserFollow row pinned to this target IS a match —
        // there's no further filtering for follow chips, so scanned
        // count == matched count.
        matched_count += rows.len() as i64;
        for row in rows {
            let user_key = row.user_pk.to_string();
            matched.insert(user_key.clone());
            if records.len() < PER_FILTER_RECORD_CAP {
                records.push(MatchedRecordRef {
                    source: AnalyzeFilterSource::Follow,
                    filter_idx: 0,
                    user_pk: user_key,
                    item_id: target_pk_str.clone(),
                    sub_id: String::new(),
                    question_idx: 0,
                    option_idx: 0,
                });
            }
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
    }

    Ok((matched, matched_count, records))
}

/// Walk every top-level comment AND reply on a post, invoking
/// `visitor` for each row. Replies live under the same DDB partition
/// as the parent post but use a different sk prefix
/// (`SPACE_POST_COMMENT_REPLY#` vs `SPACE_POST_COMMENT#`), so a
/// single `begins_with("SPACE_POST_COMMENT#")` filter — which the
/// older callers in this module used — silently skipped every reply.
/// Two paginated queries cover both prefixes; merging happens
/// implicitly via the shared visitor closure.
pub async fn iter_post_comments<F>(
    cli: &aws_sdk_dynamodb::Client,
    post_pk: Partition,
    mut visitor: F,
) -> Result<()>
where
    F: FnMut(crate::features::spaces::pages::actions::actions::discussion::SpacePostComment),
{
    use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;

    for prefix in ["SPACE_POST_COMMENT#", "SPACE_POST_COMMENT_REPLY#"] {
        let mut bookmark: Option<String> = None;
        loop {
            let mut opt = SpacePostComment::opt().sk(prefix.to_string()).limit(100);
            if let Some(b) = bookmark.clone() {
                opt = opt.bookmark(b);
            }
            let (rows, next) = SpacePostComment::query(cli, post_pk.clone(), opt).await?;
            for row in rows {
                visitor(row);
            }
            match next {
                Some(b) => bookmark = Some(b),
                None => break,
            }
        }
    }
    Ok(())
}
