//! Stage 2 of the cross-posting pipeline — the **dispatcher**.
//!
//! Triggered by an EventBridge Pipe on SyndicationJob INSERT or MODIFY
//! whose `NewImage.state == "pending"`. The Pipe filter ensures we only
//! see jobs ready for dispatch (initial enqueue from Stage 1 OR a retry
//! sweep / user-initiated retry that flipped state Failed → Pending).
//!
//! Implements the 6-step idempotency-safe flow from the design doc:
//! 1. **Acquire lock** via conditional UpdateItem (state=Pending AND
//!    (lock unset OR TTL elapsed)). On `ConditionalCheckFailed` we exit —
//!    another invocation owns the dispatch.
//! 2. **Reconcile stolen lock**: when we just stole an expired lock, the
//!    previous attempt may have published before dying; probe the platform
//!    via `adapter.find_by_backlink` and adopt the result if found.
//! 3. **Privacy guard**: re-read `Post`; if it's no longer Public/
//!    Published, mark Skipped (FR-6 #39).
//! 4. **Resolve images + body**: take up to `platform.max_images()` from
//!    `post.urls`; format body via `format_for_platform` (Phase 1 has no
//!    `body_override`).
//! 5. **Publish** through the platform adapter.
//! 6. **Commit terminal state** (Published / Failed / Skipped) atomically
//!    with lock release. The commit's condition `dispatch_lock_id =
//!    :my_lock_id` prevents a stolen-lock holder from overwriting the
//!    legitimate finisher.
//!
//! Failure classification → retry policy (FR-5 #34):
//! - Retryable (`RateLimited` / `NetworkError` / `Unknown`): up to 3
//!   retries at 1m / 10m / 1h backoff via `dispatch_shard` sparse GSI
//!   (Stage 3 sweeper picks them up — wired in 1D).
//! - Non-retryable (`AuthExpired` / `ContentRejected`): terminal Failed
//!   immediately; user must reconnect or manually retry.

use crate::common::*;
use crate::common::utils::time;
use crate::features::cross_posting::models::{
    ConnectionStatus, ErrorCategory, JobState, LOCK_TTL_SEC, SocialConnection, SyndicationJob,
};
use crate::features::cross_posting::services::adapters::{
    BlueskyAdapter, CrossPostAdapter, ImageRef, LinkCard, PlatformError, PublishedRef,
};
use crate::features::cross_posting::services::{credentials, format, shard};
use crate::features::cross_posting::types::SocialPlatform;
use crate::features::posts::models::Post;
use crate::features::posts::types::{PostStatus, Visibility};
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_dynamodb::types::AttributeValue as AV;
use uuid::Uuid;

/// Backoff schedule for auto-retries (FR-5 #34): 1 min / 10 min / 1 h.
/// Index = `new_attempts - 1` after a failure. Length implicitly bounds
/// the total retry count to 3 (1 initial + 3 retries = 4 total calls).
const RETRY_BACKOFF_SEC: [i64; 3] = [60, 600, 3600];

/// Stage 2 entry point. Called from `EventBridgeEnvelope::proc` on
/// `DetailType::SyndicationJobReady`.
pub async fn handle_syndication_job_ready(job: SyndicationJob) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let table = table_name();
    let now = time::now();
    let now_secs = now / 1000;

    // ── (1) Acquire lock ───────────────────────────────────────────────
    let lock_id = Uuid::now_v7().to_string();
    let acquired = match try_acquire_lock(cli, &table, &job, &lock_id, now_secs).await? {
        Some(prev) => prev,
        None => {
            tracing::debug!(
                pk = ?job.pk, sk = ?job.sk,
                "dispatcher: lock not acquired (state changed or held within TTL) — exiting"
            );
            return Ok(());
        }
    };

    let stolen = acquired.had_existing_lock;
    if stolen {
        tracing::warn!(
            pk = ?job.pk, sk = ?job.sk,
            "dispatcher: stole expired lock — will reconcile via find_by_backlink"
        );
    }

    // ── (3-prep) Read Post for privacy guard + body formatting ────────
    let post = match Post::get(cli, &job.pk, Some(EntityType::Post)).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            tracing::error!(pk = ?job.pk, "dispatcher: Post not found — marking Skipped");
            commit_skipped(cli, &table, &job, &lock_id, now).await?;
            return Ok(());
        }
        Err(e) => {
            tracing::error!(pk = ?job.pk, error = %e, "dispatcher: Post lookup failed");
            // Don't release the lock — let TTL expire so the next invocation
            // can retry. Returning error makes Lambda invocation fail and
            // EventBridge will retry with the same event.
            return Err(e);
        }
    };

    // ── (3) Privacy guard re-check ─────────────────────────────────────
    if post.visibility != Some(Visibility::Public) || post.status != PostStatus::Published {
        tracing::info!(
            pk = ?job.pk,
            visibility = ?post.visibility,
            status = ?post.status,
            "dispatcher: post no longer public/published — marking Skipped"
        );
        commit_skipped(cli, &table, &job, &lock_id, now).await?;
        return Ok(());
    }

    // ── Adapter selection (Phase 1A: Bluesky only) ────────────────────
    let adapter = match job.platform {
        SocialPlatform::Bluesky => BlueskyAdapter::new(),
        SocialPlatform::LinkedIn | SocialPlatform::Threads => {
            tracing::warn!(
                pk = ?job.pk,
                platform = ?job.platform,
                "dispatcher: platform not implemented in Phase 1A — marking Failed (terminal)"
            );
            commit_failed_terminal(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::Unknown,
                "platform adapter not implemented in Phase 1A",
                now,
            )
            .await?;
            return Ok(());
        }
    };

    // ── Load + decrypt credentials ────────────────────────────────────
    let connection = SocialConnection::get(
        cli,
        &job.author_user_id,
        Some(EntityType::SocialConnection(job.platform.to_string())),
    )
    .await
    .map_err(|e| {
        tracing::error!(error = %e, "dispatcher: SocialConnection lookup failed");
        e
    })?;

    let connection = match connection {
        Some(c) if c.status == ConnectionStatus::Connected => c,
        Some(_) => {
            // Revoked / AuthExpired connection — non-retryable.
            commit_failed_terminal(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::AuthExpired,
                "connection no longer Connected (revoked or auth-expired)",
                now,
            )
            .await?;
            return Ok(());
        }
        None => {
            // Connection deleted between Stage 1 and Stage 2 — terminal.
            commit_failed_terminal(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::AuthExpired,
                "SocialConnection row not found",
                now,
            )
            .await?;
            return Ok(());
        }
    };

    let creds = match credentials::open_credentials(&connection.credential_ciphertext) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = %e, "dispatcher: credential decrypt failed");
            commit_failed_terminal(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::AuthExpired,
                "credential decrypt failed",
                now,
            )
            .await?;
            return Ok(());
        }
    };

    // ── (2) Stolen-lock reconcile ──────────────────────────────────────
    if stolen {
        match adapter.find_by_backlink(creds.clone(), &job.backlink_url).await {
            Ok(Some(prior)) => {
                tracing::info!(
                    pk = ?job.pk,
                    "dispatcher: reconciled — prior attempt published; adopting result"
                );
                commit_published(cli, &table, &job, &lock_id, &prior, now).await?;
                return Ok(());
            }
            Ok(None) => {
                tracing::debug!(
                    pk = ?job.pk,
                    "dispatcher: reconcile probe returned no prior post; proceeding with publish"
                );
            }
            Err(e) => {
                // Reconcile probe failed. Bias toward publishing (better to
                // risk a duplicate than to abandon a valid job). Log warn.
                tracing::warn!(
                    pk = ?job.pk,
                    error = ?e,
                    "dispatcher: find_by_backlink probe failed during reconcile — proceeding"
                );
            }
        }
    }

    // ── (4) Resolve images + body ──────────────────────────────────────
    let images: Vec<ImageRef> = post
        .urls
        .iter()
        .take(adapter.max_images())
        .map(|url| ImageRef::from_s3(url))
        .collect();

    let body = format::format_for_platform(&post, job.platform, &job.backlink_url);
    let body_len = body.chars().count() as i32;
    let link_card = build_link_card(&post, &job);

    // ── (5) Publish ────────────────────────────────────────────────────
    let result = adapter.publish(creds, body, images, link_card).await;

    // ── (6) Commit terminal state ──────────────────────────────────────
    match result {
        Ok(published) => {
            tracing::info!(
                pk = ?job.pk,
                platform = ?job.platform,
                external = %published.external_post_url,
                "dispatcher: publish ok"
            );
            commit_published_with_body_len(cli, &table, &job, &lock_id, &published, body_len, now)
                .await?;
            Ok(())
        }
        Err(err) => {
            let category = classify_platform_error(&err);
            let retryable = is_retryable(category);
            let new_attempts = job.attempts.saturating_add(1);
            let exhausted = (new_attempts as usize) > RETRY_BACKOFF_SEC.len();

            let msg = format!("{err}");

            if retryable && !exhausted {
                let backoff = RETRY_BACKOFF_SEC[(new_attempts as usize) - 1];
                let next_attempt_at = now_secs + backoff;
                let dispatch_shard = shard::shard_for(&post_id_inner(&job.pk));
                tracing::warn!(
                    pk = ?job.pk,
                    platform = ?job.platform,
                    ?category,
                    attempts = new_attempts,
                    next_attempt_at,
                    "dispatcher: publish failed; scheduling retry"
                );
                commit_failed_retryable(
                    cli,
                    &table,
                    &job,
                    &lock_id,
                    category,
                    &msg,
                    new_attempts,
                    &dispatch_shard,
                    next_attempt_at,
                    now,
                )
                .await?;
            } else {
                tracing::error!(
                    pk = ?job.pk,
                    platform = ?job.platform,
                    ?category,
                    attempts = new_attempts,
                    retryable,
                    "dispatcher: publish failed (terminal)"
                );
                commit_failed_terminal(cli, &table, &job, &lock_id, category, &msg, now).await?;
            }

            Ok(())
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Lock acquisition
// ─────────────────────────────────────────────────────────────────────────

struct LockAcquired {
    /// True when the row had a `dispatch_lock_id` set at the moment we
    /// acquired (TTL had expired). Triggers the reconcile probe in step 2.
    had_existing_lock: bool,
}

/// Conditional UpdateItem that sets `dispatch_lock_id` and
/// `lock_acquired_at` if either no lock is held OR the existing lock's
/// TTL has elapsed. State must still be Pending.
///
/// Returns:
/// - `Ok(Some(LockAcquired))` — we now own the lock
/// - `Ok(None)` — condition failed; another invocation owns it or state changed
/// - `Err(_)` — DynamoDB error (transient, propagate so Lambda retries)
async fn try_acquire_lock(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    now_secs: i64,
) -> Result<Option<LockAcquired>> {
    let ttl_threshold = now_secs - LOCK_TTL_SEC;

    let resp = cli
        .update_item()
        .table_name(table)
        .key("pk", AV::S(job.pk.to_string()))
        .key("sk", AV::S(job.sk.to_string()))
        .update_expression(
            "SET dispatch_lock_id = :lock_id, lock_acquired_at = :now, updated_at = :now_ms",
        )
        .condition_expression(
            "#state = :pending AND \
             (attribute_not_exists(dispatch_lock_id) OR lock_acquired_at < :ttl_threshold)",
        )
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_secs.to_string()))
        .expression_attribute_values(":now_ms", AV::N((now_secs * 1000).to_string()))
        .expression_attribute_values(":pending", AV::S(job_state_str(JobState::Pending).into()))
        .expression_attribute_values(":ttl_threshold", AV::N(ttl_threshold.to_string()))
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
        .send()
        .await;

    match resp {
        Ok(out) => {
            let had_existing_lock = out
                .attributes()
                .and_then(|a| a.get("dispatch_lock_id"))
                .is_some();
            Ok(Some(LockAcquired { had_existing_lock }))
        }
        Err(e) => {
            // ConditionalCheckFailedException → not an error; means another
            // dispatcher already owns the lock or state changed.
            let svc = e.into_service_error();
            if matches!(
                svc,
                aws_sdk_dynamodb::operation::update_item::UpdateItemError::ConditionalCheckFailedException(_)
            ) {
                return Ok(None);
            }
            tracing::error!(error = %svc, "dispatcher: try_acquire_lock unexpected DynamoDB error");
            Err(crate::common::Error::Internal)
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Commit helpers — all conditional on `dispatch_lock_id = :my_lock_id`
// so a stolen-lock holder cannot overwrite the legitimate finisher.
// ─────────────────────────────────────────────────────────────────────────

async fn commit_skipped(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    now_ms: i64,
) -> Result<()> {
    cli.update_item()
        .table_name(table)
        .key("pk", AV::S(job.pk.to_string()))
        .key("sk", AV::S(job.sk.to_string()))
        .update_expression(
            "SET #state = :state, updated_at = :now \
             REMOVE dispatch_shard, engagement_shard, dispatch_lock_id, lock_acquired_at",
        )
        .condition_expression("dispatch_lock_id = :my_lock_id")
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":state", AV::S(job_state_str(JobState::Skipped).into()))
        .expression_attribute_values(":my_lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_ms.to_string()))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!(error = %e.into_service_error(), "dispatcher: commit_skipped failed");
            crate::common::Error::Internal
        })
}

async fn commit_published(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    pubref: &PublishedRef,
    now_ms: i64,
) -> Result<()> {
    commit_published_with_body_len(cli, table, job, lock_id, pubref, 0, now_ms).await
}

async fn commit_published_with_body_len(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    pubref: &PublishedRef,
    body_len: i32,
    now_ms: i64,
) -> Result<()> {
    let now_secs = now_ms / 1000;
    let one_hour_later = now_secs + 3600;
    let engagement_shard = shard::shard_for(&post_id_inner(&job.pk));

    cli.update_item()
        .table_name(table)
        .key("pk", AV::S(job.pk.to_string()))
        .key("sk", AV::S(job.sk.to_string()))
        .update_expression(
            "SET #state = :state, \
                external_post_id = :ext_id, \
                external_post_url = :ext_url, \
                body_snapshot_len = :body_len, \
                engagement_shard = :eng_shard, \
                engagement_next_at = :eng_at, \
                updated_at = :now \
             REMOVE dispatch_shard, dispatch_lock_id, lock_acquired_at",
        )
        .condition_expression("dispatch_lock_id = :my_lock_id")
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":state", AV::S(job_state_str(JobState::Published).into()))
        .expression_attribute_values(":ext_id", AV::S(pubref.external_post_id.clone()))
        .expression_attribute_values(":ext_url", AV::S(pubref.external_post_url.clone()))
        .expression_attribute_values(":body_len", AV::N(body_len.to_string()))
        .expression_attribute_values(":eng_shard", AV::S(engagement_shard))
        .expression_attribute_values(":eng_at", AV::N(one_hour_later.to_string()))
        .expression_attribute_values(":my_lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_ms.to_string()))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!(error = %e.into_service_error(), "dispatcher: commit_published failed");
            crate::common::Error::Internal
        })
}

async fn commit_failed_retryable(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    category: ErrorCategory,
    message: &str,
    new_attempts: u8,
    dispatch_shard: &str,
    next_attempt_at_secs: i64,
    now_ms: i64,
) -> Result<()> {
    cli.update_item()
        .table_name(table)
        .key("pk", AV::S(job.pk.to_string()))
        .key("sk", AV::S(job.sk.to_string()))
        .update_expression(
            "SET #state = :state, \
                attempts = :attempts, \
                last_error_category = :cat, \
                last_error_message = :msg, \
                dispatch_shard = :shard, \
                next_attempt_at = :next_at, \
                updated_at = :now \
             REMOVE dispatch_lock_id, lock_acquired_at",
        )
        .condition_expression("dispatch_lock_id = :my_lock_id")
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":state", AV::S(job_state_str(JobState::Failed).into()))
        .expression_attribute_values(":attempts", AV::N(new_attempts.to_string()))
        .expression_attribute_values(":cat", AV::S(error_category_str(category).into()))
        .expression_attribute_values(":msg", AV::S(message.to_string()))
        .expression_attribute_values(":shard", AV::S(dispatch_shard.to_string()))
        .expression_attribute_values(":next_at", AV::N(next_attempt_at_secs.to_string()))
        .expression_attribute_values(":my_lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_ms.to_string()))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!(error = %e.into_service_error(), "dispatcher: commit_failed_retryable failed");
            crate::common::Error::Internal
        })
}

async fn commit_failed_terminal(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    category: ErrorCategory,
    message: &str,
    now_ms: i64,
) -> Result<()> {
    cli.update_item()
        .table_name(table)
        .key("pk", AV::S(job.pk.to_string()))
        .key("sk", AV::S(job.sk.to_string()))
        .update_expression(
            "SET #state = :state, \
                last_error_category = :cat, \
                last_error_message = :msg, \
                updated_at = :now \
             REMOVE dispatch_shard, engagement_shard, dispatch_lock_id, lock_acquired_at",
        )
        .condition_expression("dispatch_lock_id = :my_lock_id")
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":state", AV::S(job_state_str(JobState::Failed).into()))
        .expression_attribute_values(":cat", AV::S(error_category_str(category).into()))
        .expression_attribute_values(":msg", AV::S(message.to_string()))
        .expression_attribute_values(":my_lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_ms.to_string()))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!(error = %e.into_service_error(), "dispatcher: commit_failed_terminal failed");
            crate::common::Error::Internal
        })
}

// ─────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────

fn build_link_card(post: &Post, job: &SyndicationJob) -> LinkCard {
    let stripped = format::strip_html(&post.html_contents);
    let description: String = stripped.chars().take(200).collect();
    LinkCard {
        backlink_url: job.backlink_url.clone(),
        fallback_title: post.title.clone(),
        fallback_description: description,
        fallback_thumb_url: post.urls.first().cloned(),
    }
}

fn classify_platform_error(err: &PlatformError) -> ErrorCategory {
    match err {
        PlatformError::AuthExpired(_) => ErrorCategory::AuthExpired,
        PlatformError::RateLimited(_) => ErrorCategory::RateLimited,
        PlatformError::ContentRejected(_) => ErrorCategory::ContentRejected,
        PlatformError::NetworkError(_) => ErrorCategory::NetworkError,
        PlatformError::Unknown(_) => ErrorCategory::Unknown,
    }
}

fn is_retryable(category: ErrorCategory) -> bool {
    matches!(
        category,
        ErrorCategory::RateLimited | ErrorCategory::NetworkError | ErrorCategory::Unknown
    )
}

/// JobState's snake_case serde representation for use in
/// `:expression_attribute_values`. Manual mapping because we're building
/// the expression string by hand (the DynamoEntity macro's updater
/// doesn't expose conditional UpdateItem).
fn job_state_str(state: JobState) -> &'static str {
    match state {
        JobState::Pending => "pending",
        JobState::Published => "published",
        JobState::Failed => "failed",
        JobState::Skipped => "skipped",
    }
}

fn error_category_str(c: ErrorCategory) -> &'static str {
    match c {
        ErrorCategory::Unknown => "unknown",
        ErrorCategory::AuthExpired => "auth_expired",
        ErrorCategory::RateLimited => "rate_limited",
        ErrorCategory::ContentRejected => "content_rejected",
        ErrorCategory::NetworkError => "network_error",
    }
}

fn post_id_inner(pk: &Partition) -> String {
    match pk {
        Partition::Feed(id) => id.clone(),
        _ => pk.to_string(),
    }
}

fn table_name() -> String {
    let prefix = option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local");
    format!("{prefix}-main")
}
