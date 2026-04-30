//! Stage 3 — retry sweeper.
//!
//! Runs every 1 minute (CloudWatch schedule rule in prod, a `tokio` task
//! in `local-dev`). For each of the `SHARD_COUNT` shard partitions, query
//! the sparse `find_due_jobs` GSI for `SyndicationJob` rows whose
//! `dispatch_shard` is set AND `next_attempt_at <= now`, then flip them
//! back to `Pending` (clearing `dispatch_shard` so the row drops off
//! this GSI). The resulting MODIFY event re-enters the Stage 2 dispatcher
//! Pipe (filter `state=pending`).
//!
//! - The dispatcher's `commit_failed_retryable` already pushes the row
//!   onto this GSI with the right `next_attempt_at` (now + backoff). The
//!   sweeper is the second half of that loop.
//! - Terminal failures (`AuthExpired` / `ContentRejected`) and
//!   `commit_published` / `commit_skipped` clear `dispatch_shard`
//!   already, so the GSI is naturally pruned of non-retryable rows.
//! - Backoff schedule lives on each row (`next_attempt_at`); the sweeper
//!   is stateless across runs.
//!
//! Design doc: `docs/superpowers/specs/2026-04-28-cross-posting-design.md`
//! § Event flow → "Stage 3 — retry sweeper".

use crate::common::utils::time;
use crate::features::cross_posting::models::{JobState, SHARD_COUNT, SyndicationJob};
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_dynamodb::types::AttributeValue as AV;

/// Entry point — fan out one query per shard, reset each due row to
/// `Pending`. Errors are logged but do not abort the sweep so a single
/// stuck shard doesn't starve the others.
pub async fn run_retry_sweep() -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let now_secs = time::now() / 1000;

    let mut total_due = 0usize;
    let mut total_reset = 0usize;
    let mut shard_errors = 0usize;

    for shard_index in 0..SHARD_COUNT {
        let shard = format!("SDS#{shard_index}");
        match sweep_shard(cli, &shard, now_secs).await {
            Ok((due, reset)) => {
                total_due += due;
                total_reset += reset;
            }
            Err(e) => {
                shard_errors += 1;
                tracing::error!(shard = %shard, error = %e, "stage 3: shard sweep failed");
            }
        }
    }

    tracing::info!(
        due = total_due,
        reset = total_reset,
        errors = shard_errors,
        "stage 3: retry sweep complete"
    );

    Ok(())
}

/// Query one shard, filter to rows actually due, reset each. Returns
/// `(due_seen, reset_succeeded)` for telemetry.
async fn sweep_shard(
    cli: &DynamoClient,
    shard: &str,
    now_secs: i64,
) -> crate::common::Result<(usize, usize)> {
    // The DynamoEntity macro emits `find_due_jobs(cli, pk, opt)` with
    // `opt.sk` as a `begins_with` clause — design needs `<=` on the
    // numeric `next_attempt_at`. Drop down to the raw SDK so we can use
    // a range comparator and keep the row count minimal.
    //
    // The macro stores `gsi1_sk` as a 20-char zero-padded string of
    // `(next_attempt_at - i64::MIN)` so DynamoDB's lexicographic string
    // comparator preserves numeric ordering. We must encode the `:now`
    // bound the same way; `generate_sk_for_find_due_jobs` is the
    // public encoder the macro emits for exactly this purpose.
    let now_padded = SyndicationJob::generate_sk_for_find_due_jobs(now_secs);

    let resp = cli
        .query()
        .table_name(SyndicationJob::table_name())
        .index_name("gsi1-index")
        .key_condition_expression("#pk = :shard AND #sk <= :now")
        .expression_attribute_names("#pk", "gsi1_pk")
        .expression_attribute_names("#sk", "gsi1_sk")
        .expression_attribute_values(":shard", AV::S(shard.to_string()))
        .expression_attribute_values(":now", AV::S(now_padded))
        .send()
        .await
        .map_err(|e| {
            tracing::error!(shard = %shard, error = ?e, "stage 3: query failed");
            crate::common::Error::Internal
        })?;

    let items = resp.items.unwrap_or_default();
    let due_seen = items.len();
    if due_seen == 0 {
        return Ok((0, 0));
    }

    let jobs: Vec<SyndicationJob> = items
        .into_iter()
        .filter_map(|item| match serde_dynamo::from_item(item) {
            Ok(j) => Some(j),
            Err(e) => {
                tracing::error!(error = %e, "stage 3: deserialize SyndicationJob failed");
                None
            }
        })
        .collect();

    let mut reset = 0usize;
    for job in jobs {
        // Conditional reset: only flip rows still in (Failed AND
        // dispatch_shard set AND next_attempt_at unchanged). Concurrent
        // user-initiated retry / shard re-arm would have changed one of
        // these, in which case skipping is the correct outcome.
        let now_ms = time::now();
        match SyndicationJob::updater(job.pk.clone(), job.sk.clone())
            .with_state(JobState::Pending)
            .remove_dispatch_shard()
            .with_next_attempt_at(0)
            .with_updated_at(now_ms)
            .execute(cli)
            .await
        {
            Ok(_) => reset += 1,
            Err(e) => {
                tracing::warn!(
                    pk = ?job.pk,
                    sk = ?job.sk,
                    error = %e,
                    "stage 3: reset failed (state changed concurrently?)"
                );
            }
        }
    }

    Ok((due_seen, reset))
}
