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
//! Failure handling (revised from the original design doc spec):
//! - First attempt fails → **inline retry once** within the same lock.
//!   Catches transient `IncompleteMessage` / one-shot `RateLimited`
//!   without spawning a separate retry pipeline.
//! - If the inline retry also fails → commit terminal Failed and post
//!   an inbox notification to the author. The Syndication panel's
//!   "Retry now" CTA is the only path back into Pending.
//! - There is **no automatic retry sweeper** (Stage 3 was removed in
//!   favour of notifications). `AuthExpired` failures route the
//!   notification CTA to Settings → Connections instead of the post
//!   detail.

use crate::common::models::notification::UserInboxNotification;
use crate::common::types::InboxPayload;
use crate::common::utils::time;
use crate::common::*;
use crate::features::cross_posting::models::{
    ConnectionStatus, ErrorCategory, JobState, LOCK_TTL_SEC, SocialConnection, SyndicationJob,
};
use crate::features::cross_posting::services::adapters::{
    BlueskyAdapter, CrossPostAdapter, DecryptedCredentials, ImageRef, LinkCard, LinkedInAdapter,
    PlatformError, PublishedRef,
};
use crate::features::cross_posting::services::{credentials, format, shard};
use crate::features::cross_posting::types::{CrossPostingError, SocialPlatform};
use crate::features::posts::models::Post;
use crate::features::posts::types::{PostStatus, Visibility};
use aws_sdk_dynamodb::Client as DynamoClient;
use aws_sdk_dynamodb::types::AttributeValue as AV;
use uuid::Uuid;

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

    // ── Adapter selection ─────────────────────────────────────────────
    // Boxed trait object so each platform can return its own concrete type.
    // Threads (1C) still falls through to the not-implemented branch until
    // its adapter and OAuth flow land.
    let adapter: Box<dyn CrossPostAdapter> = match job.platform {
        SocialPlatform::Bluesky => Box::new(BlueskyAdapter::new()),
        SocialPlatform::LinkedIn => Box::new(LinkedInAdapter::new()),
        SocialPlatform::Threads => {
            tracing::warn!(
                pk = ?job.pk,
                platform = ?job.platform,
                "dispatcher: platform adapter not implemented yet — marking Failed"
            );
            let msg = "platform adapter not implemented yet".to_string();
            commit_failed(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::Unknown,
                &msg,
                job.attempts.saturating_add(1),
                now,
            )
            .await?;
            notify_failure(cli, &job, ErrorCategory::Unknown, Some(msg)).await;
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
            let msg = "connection no longer Connected (revoked or auth-expired)".to_string();
            commit_failed(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::AuthExpired,
                &msg,
                job.attempts.saturating_add(1),
                now,
            )
            .await?;
            notify_failure(cli, &job, ErrorCategory::AuthExpired, Some(msg)).await;
            return Ok(());
        }
        None => {
            // Connection deleted between Stage 1 and Stage 2 — terminal.
            let msg = "SocialConnection row not found".to_string();
            commit_failed(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::AuthExpired,
                &msg,
                job.attempts.saturating_add(1),
                now,
            )
            .await?;
            notify_failure(cli, &job, ErrorCategory::AuthExpired, Some(msg)).await;
            return Ok(());
        }
    };

    let creds = match credentials::open_credentials(&connection.credential_ciphertext) {
        Ok(c) => c,
        Err(e) => {
            tracing::error!(error = %e, "dispatcher: credential decrypt failed");
            let msg = "credential decrypt failed".to_string();
            commit_failed(
                cli,
                &table,
                &job,
                &lock_id,
                ErrorCategory::AuthExpired,
                &msg,
                job.attempts.saturating_add(1),
                now,
            )
            .await?;
            notify_failure(cli, &job, ErrorCategory::AuthExpired, Some(msg)).await;
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

    // ── (5) Publish — inline retry once on retryable failures ────────
    // First attempt. If it fails we may retry once within the same lock:
    //   * AuthExpired → call `refreshSession` to rotate the Bluesky access
    //     token, persist the new ciphertext on `SocialConnection`, then
    //     retry publish with the rotated creds. spec FR-5 #35.
    //   * Other retryable categories (network / rate-limit / unknown) →
    //     retry with the same creds.
    // If both attempts fail, commit terminal Failed below.
    let mut current_creds = creds;
    let mut result = adapter
        .publish(
            current_creds.clone(),
            body.clone(),
            images.clone(),
            link_card.clone(),
        )
        .await;

    if let Err(err) = &result {
        let category = classify_platform_error(err);
        if matches!(category, ErrorCategory::AuthExpired) {
            tracing::warn!(
                pk = ?job.pk,
                platform = ?job.platform,
                "dispatcher: publish hit AuthExpired — attempting credential refresh",
            );
            // Trait dispatch — each adapter knows its own refresh API
            // (Bluesky refreshSession, LinkedIn OAuth refresh, Threads
            // long-lived token extension). Adapters that don't support
            // refresh fall through the default impl and return AuthExpired,
            // collapsing this branch to the same Failed commit below.
            match adapter.try_refresh_credentials(current_creds.clone()).await {
                Ok(refreshed) => {
                    if let Err(persist_err) =
                        persist_refreshed_credentials(cli, &connection, &refreshed).await
                    {
                        // DB write failed but we have the new tokens in hand;
                        // proceed with the publish anyway. Worst case the next
                        // publish will hit AuthExpired again and refresh
                        // re-runs (refresh APIs are typically idempotent).
                        tracing::warn!(
                            pk = ?job.pk,
                            platform = ?job.platform,
                            error = %persist_err,
                            "dispatcher: refreshed creds but persist failed — proceeding with rotated tokens",
                        );
                    } else {
                        tracing::info!(
                            pk = ?job.pk,
                            platform = ?job.platform,
                            "dispatcher: credential refresh ok — retrying publish with rotated tokens",
                        );
                    }
                    current_creds = refreshed;
                    result = adapter
                        .publish(
                            current_creds.clone(),
                            body.clone(),
                            images.clone(),
                            link_card.clone(),
                        )
                        .await;
                }
                Err(refresh_err) => {
                    tracing::warn!(
                        pk = ?job.pk,
                        platform = ?job.platform,
                        error = %refresh_err,
                        "dispatcher: credential refresh failed — leaving publish error as AuthExpired",
                    );
                    // Drop through; `result` keeps the original AuthExpired
                    // error which `commit_failed` + `notify_failure` will
                    // route to Settings → Connections.
                }
            }
        } else if is_retryable(category) {
            tracing::warn!(
                pk = ?job.pk,
                platform = ?job.platform,
                ?category,
                "dispatcher: first publish failed (retryable) — retrying inline"
            );
            result = adapter
                .publish(current_creds, body, images, link_card)
                .await;
        }
    }

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
            let new_attempts = job.attempts.saturating_add(1);
            let msg = format!("{err}");

            tracing::error!(
                pk = ?job.pk,
                platform = ?job.platform,
                ?category,
                attempts = new_attempts,
                "dispatcher: publish failed after inline retry"
            );

            commit_failed(cli, &table, &job, &lock_id, category, &msg, new_attempts, now).await?;
            notify_failure(cli, &job, category, Some(msg)).await;
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
        // `attribute_type(field, "NULL")` covers the case where Stage 1
        // factory wrote the SyndicationJob with `dispatch_lock_id:
        // Option<String> = None`, which DynamoEntity serializes as an
        // explicit `Null(true)` attribute (not a missing attribute). Without
        // this branch the brand-new job's first lock-acquire would always
        // fail ConditionalCheckFailed because:
        //   - `attribute_not_exists(dispatch_lock_id)` is false (the
        //     attribute is present, just NULL-typed)
        //   - `lock_acquired_at < :ttl_threshold` is false (also NULL,
        //     which can't be compared to a Number)
        .condition_expression(
            "#state = :pending AND \
             (attribute_not_exists(dispatch_lock_id) \
              OR attribute_type(dispatch_lock_id, :null_type) \
              OR lock_acquired_at < :ttl_threshold)",
        )
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_secs.to_string()))
        .expression_attribute_values(":now_ms", AV::N((now_secs * 1000).to_string()))
        .expression_attribute_values(":pending", AV::S(job_state_str(JobState::Pending).into()))
        .expression_attribute_values(":ttl_threshold", AV::N(ttl_threshold.to_string()))
        .expression_attribute_values(":null_type", AV::S("NULL".to_string()))
        .return_values(aws_sdk_dynamodb::types::ReturnValue::AllOld)
        .send()
        .await;

    match resp {
        Ok(out) => {
            // A brand-new job written by `services/factory.rs` carries
            // `dispatch_lock_id: Option<String> = None`, which DynamoEntity
            // serializes as an explicit NULL-typed attribute (not a missing
            // attribute). `is_some()` here would treat that NULL as a real
            // prior lock, sending every first-dispatch through the reconcile
            // probe — which on LinkedIn surfaces a 403 (ugcPosts FINDER
            // permission), and on Bluesky burns an extra `listRecords` call
            // per publish. Only a String-typed attribute means a real
            // dispatcher previously held the lock (and its TTL elapsed).
            let had_existing_lock = out
                .attributes()
                .and_then(|a| a.get("dispatch_lock_id"))
                .map(|v| !matches!(v, AV::Null(_)))
                .unwrap_or(false);
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
            Err(CrossPostingError::DispatchLockFailed.into())
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
            CrossPostingError::CommitFailed.into()
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
            CrossPostingError::CommitFailed.into()
        })
}

/// Commit a terminal Failed state (retry-sweeper-free model). The row
/// stays in the panel with `last_error_*` populated so the user's
/// "Retry now" CTA on the post-detail panel can flip it back to
/// Pending; the inbox notification (`notify_failure`) is the user-
/// visible signal that this happened.
async fn commit_failed(
    cli: &DynamoClient,
    table: &str,
    job: &SyndicationJob,
    lock_id: &str,
    category: ErrorCategory,
    message: &str,
    new_attempts: u8,
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
                updated_at = :now \
             REMOVE dispatch_shard, engagement_shard, dispatch_lock_id, lock_acquired_at",
        )
        .condition_expression("dispatch_lock_id = :my_lock_id")
        .expression_attribute_names("#state", "state")
        .expression_attribute_values(":state", AV::S(job_state_str(JobState::Failed).into()))
        .expression_attribute_values(":attempts", AV::N(new_attempts.to_string()))
        .expression_attribute_values(":cat", AV::S(error_category_str(category).into()))
        .expression_attribute_values(":msg", AV::S(message.to_string()))
        .expression_attribute_values(":my_lock_id", AV::S(lock_id.to_string()))
        .expression_attribute_values(":now", AV::N(now_ms.to_string()))
        .send()
        .await
        .map(|_| ())
        .map_err(|e| {
            tracing::error!(error = %e.into_service_error(), "dispatcher: commit_failed failed");
            CrossPostingError::CommitFailed.into()
        })
}

/// Best-effort inbox notification for the post author. Errors here MUST
/// NOT block dispatch — the row state is already correct on DynamoDB,
/// the worst case is the user not seeing the bell ring.
///
/// `cta_url`:
/// - `AuthExpired` → `/{post_owner_username}/settings/connections` would
///   need a user lookup we don't have here, so we fall back to the
///   post-detail page; the panel's per-platform card handles
///   AuthExpired by hiding "Retry now" and surfacing a Reconnect hint.
/// - everything else → `/posts/{post_id}` so the author can hit "Retry
///   now" directly from the Syndication panel.
async fn notify_failure(
    cli: &DynamoClient,
    job: &SyndicationJob,
    category: ErrorCategory,
    error_message: Option<String>,
) {
    let post_id = post_id_inner(&job.pk).to_string();
    let cta_url = format!("/posts/{}", post_id);

    let payload = InboxPayload::CrossPostingFailed {
        post_id,
        platform: job.platform,
        error_category: category,
        error_message,
        cta_url,
    };

    let inbox = UserInboxNotification::new(job.author_user_id.clone(), payload);
    if let Err(e) = inbox.create(cli).await {
        tracing::warn!(
            pk = ?job.pk,
            error = %e,
            "dispatcher: inbox notification failed (non-fatal)"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────────────────

fn build_link_card(post: &Post, job: &SyndicationJob) -> LinkCard {
    let stripped = post.body.to_plain_text();
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

/// AEAD-seal rotated credentials and persist them onto
/// `SocialConnection.credential_ciphertext`. Platform-agnostic: the actual
/// refresh call lives on `CrossPostAdapter::try_refresh_credentials`.
async fn persist_refreshed_credentials(
    cli: &DynamoClient,
    connection: &SocialConnection,
    new_creds: &DecryptedCredentials,
) -> std::result::Result<(), PlatformError> {
    let sealed = credentials::seal_credentials(new_creds)
        .map_err(|e| PlatformError::Unknown(format!("seal after refresh failed: {e}")))?;

    SocialConnection::updater(connection.pk.clone(), connection.sk.clone())
        .with_credential_ciphertext(sealed)
        .with_updated_at(time::now())
        .execute(cli)
        .await
        .map_err(|e| {
            PlatformError::Unknown(format!("SocialConnection update after refresh: {e}"))
        })?;

    Ok(())
}
