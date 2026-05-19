use serde::{Deserialize, Serialize};

use crate::common::models::reward::PendingReward;
use crate::common::types::PendingRewardStatus;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::*;

const BATCH_LIMIT: i32 = 50;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RetryStats {
    pub scanned: usize,
    pub succeeded: usize,
    pub still_pending: usize,
    pub still_pending_rows: Vec<PendingFailureSnapshot>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PendingFailureSnapshot {
    pub sk: String,
    pub target_pk: String,
    pub amount: i64,
    pub retry_count: i64,
    pub last_error: String,
    pub created_at: i64,
}

#[cfg(feature = "server")]
pub async fn retry_pending_rewards(cli: &aws_sdk_dynamodb::Client) -> Result<RetryStats> {
    let cfg = crate::common::CommonConfig::default();
    let biyard = cfg.biyard();

    let mut stats = RetryStats::default();
    let mut bookmark: Option<String> = None;

    loop {
        let opt = PendingReward::opt_with_bookmark(bookmark.clone()).limit(BATCH_LIMIT);
        let (items, next) =
            PendingReward::find_by_status(cli, PendingRewardStatus::Pending, opt).await?;

        for item in items {
            stats.scanned += 1;

            // Anchor the Biyard transaction to the original month so a
            // backfilled outage row doesn't get billed against a later month.
            // An invalid `created_at` would break this invariant, so we
            // fail-fast rather than silently fall back to the current month.
            let month = format_yyyy_mm(item.created_at)?;
            let description = retry_description(&item);
            let target_pk = item.target_pk.clone();
            let owner_pk = item.owner_pk.clone();

            match biyard
                .award_points(
                    target_pk.clone(),
                    item.amount,
                    description.clone(),
                    Some(month),
                )
                .await
            {
                Ok(user_res) => {
                    // Backfill UserRewardHistory.transaction_id/month for
                    // the row created during the original award attempt
                    // (now that Biyard sync is finally proven by this
                    // retry). Reconstruct the history sk by reading
                    // `SpaceReward.period` so we can derive the same
                    // TimeKey the original award used.
                    //
                    // Best-effort: if SpaceReward is gone (reward
                    // re-configured / removed) or the history row was
                    // never written, just log and move on — Biyard /
                    // PendingReward state is the source of truth for
                    // "did the user get their points?", and we already
                    // know the answer is yes.
                    if let (Some(space_pk_sub), Some(action_id)) = (
                        item.reward_key.space_pk.clone(),
                        item.reward_key.action_id.clone(),
                    ) {
                        use crate::common::models::reward::UserRewardHistory;
                        use crate::common::types::{
                            CompositePartition, UserRewardHistoryKey,
                        };
                        use crate::features::spaces::space_common::models::space_reward::SpaceReward;

                        let behavior = item.reward_key.behavior.clone();
                        match SpaceReward::get_by_action(
                            cli,
                            space_pk_sub,
                            action_id,
                            behavior,
                        )
                        .await
                        {
                            Ok(space_reward) => {
                                let time_key = space_reward
                                    .period
                                    .to_time_key(item.created_at);
                                let history_pk = CompositePartition(
                                    item.target_pk.clone(),
                                    Partition::Reward,
                                );
                                let history_sk = UserRewardHistoryKey(
                                    item.reward_key.clone(),
                                    time_key,
                                );

                                // Confirm the row exists before updating —
                                // `updater().execute()` is upsert-shaped, so
                                // a wrong sk would otherwise create a blank
                                // row instead of failing. The extra read is
                                // cheap and keeps the data clean if
                                // `SpaceReward.period` changed between award
                                // and retry.
                                match UserRewardHistory::get(
                                    cli,
                                    &history_pk,
                                    Some(&history_sk),
                                )
                                .await
                                {
                                    Ok(Some(_)) => {
                                        if let Err(e) =
                                            UserRewardHistory::updater(&history_pk, &history_sk)
                                                .with_transaction_id(
                                                    user_res.transaction_id.clone(),
                                                )
                                                .with_month(user_res.month.clone())
                                                .execute(cli)
                                                .await
                                        {
                                            tracing::warn!(
                                                pending_sk = %item.sk,
                                                history_sk = %history_sk,
                                                error = %e,
                                                "UserRewardHistory updater failed after retry succeeded",
                                            );
                                        }
                                    }
                                    Ok(None) => {
                                        tracing::warn!(
                                            pending_sk = %item.sk,
                                            history_sk = %history_sk,
                                            "UserRewardHistory row not found for retry-history backfill (period changed since award?)",
                                        );
                                    }
                                    Err(e) => {
                                        tracing::warn!(
                                            pending_sk = %item.sk,
                                            error = %e,
                                            "UserRewardHistory get failed during retry-history backfill",
                                        );
                                    }
                                }
                            }
                            Err(e) => {
                                tracing::warn!(
                                    pending_sk = %item.sk,
                                    error = %e,
                                    "SpaceReward lookup for retry-history backfill failed",
                                );
                            }
                        }
                    }

                    if let Some(owner) = owner_pk {
                        if owner != target_pk && item.amount > 0 {
                            let owner_amount = item.amount * 10 / 100;
                            if owner_amount > 0 {
                                if let Err(e) = biyard
                                    .award_points(
                                        owner.clone(),
                                        owner_amount,
                                        description.clone(),
                                        Some(user_res.month.clone()),
                                    )
                                    .await
                                {
                                    tracing::error!(
                                        owner_pk = %owner,
                                        sk = %item.sk,
                                        error = %e,
                                        "owner referral retry failed (main award succeeded)",
                                    );
                                }
                            }
                        }
                    }
                    // Mark COMPLETED so the next retry sweep skips this
                    // row. If this fails after a successful Biyard award,
                    // a subsequent sweep would re-award (double-pay), so
                    // surface it loudly for an operator to reconcile.
                    if let Err(e) = mark_completed(cli, &item).await {
                        tracing::error!(
                            sk = %item.sk,
                            error = %e,
                            "CRITICAL: biyard award succeeded but mark_completed failed — may double-pay on next retry",
                        );
                    }
                    stats.succeeded += 1;
                }
                Err(e) => {
                    let last_error = format!("{e}");
                    if let Err(bump_err) = bump_retry(cli, &item, &last_error).await {
                        tracing::error!(
                            sk = %item.sk,
                            error = %bump_err,
                            "failed to bump retry counter — counter and last_error may be stale",
                        );
                    }
                    let next_retry_count = item.retry_count + 1;
                    stats.still_pending += 1;
                    stats.still_pending_rows.push(PendingFailureSnapshot {
                        sk: item.sk.to_string(),
                        target_pk: item.target_pk.to_string(),
                        amount: item.amount,
                        retry_count: next_retry_count,
                        last_error: last_error.clone(),
                        created_at: item.created_at,
                    });
                    tracing::warn!(
                        sk = %item.sk,
                        retry = next_retry_count,
                        error = %e,
                        "pending reward retry failed",
                    );
                }
            }
        }

        bookmark = next;
        if bookmark.is_none() {
            break;
        }
    }

    tracing::info!(
        scanned = stats.scanned,
        succeeded = stats.succeeded,
        still_pending = stats.still_pending,
        "pending reward retry completed",
    );

    Ok(stats)
}

fn retry_description(item: &PendingReward) -> String {
    let marker = format!("retry-{}", item.sk);
    if item.description.is_empty() {
        marker
    } else {
        format!("{} | {}", item.description, marker)
    }
}

fn format_yyyy_mm(ts_ms: i64) -> Result<String> {
    let secs = ts_ms / 1000;
    let dt = chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0).ok_or_else(|| {
        crate::error!("invalid created_at_ms: {ts_ms} — cannot derive month");
        Error::InvalidFormat
    })?;
    Ok(dt.format("%Y-%m").to_string())
}

#[cfg(feature = "server")]
async fn mark_completed(
    cli: &aws_sdk_dynamodb::Client,
    item: &PendingReward,
) -> Result<()> {
    let now = get_now_timestamp_millis();
    PendingReward::updater(&item.pk, &item.sk)
        .with_status(PendingRewardStatus::Completed)
        .with_updated_at(now)
        .execute(cli)
        .await?;
    Ok(())
}

#[cfg(feature = "server")]
async fn bump_retry(
    cli: &aws_sdk_dynamodb::Client,
    item: &PendingReward,
    last_error: &str,
) -> Result<()> {
    let now = get_now_timestamp_millis();
    PendingReward::updater(&item.pk, &item.sk)
        .with_updated_at(now)
        .with_retry_count(item.retry_count + 1)
        .with_last_error(last_error.to_string())
        .execute(cli)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_yyyy_mm_returns_utc_month() {
        assert_eq!(format_yyyy_mm(1776817749390).unwrap(), "2026-04");
        assert_eq!(format_yyyy_mm(1735689600000).unwrap(), "2025-01");
    }

    #[test]
    fn format_yyyy_mm_rejects_out_of_range_timestamp() {
        // i64::MAX seconds is far past chrono's representable range.
        assert!(format_yyyy_mm(i64::MAX).is_err());
    }
}
