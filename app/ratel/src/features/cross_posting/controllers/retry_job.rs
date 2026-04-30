use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::{JobState, SyndicationJob};
use crate::features::cross_posting::types::{CrossPostingError, SocialPlatform};
use crate::features::posts::models::Post;

/// Author-initiated retry of a single `SyndicationJob` (FR-7 #44).
///
/// Resets the job to `Pending` and clears the dispatch shard / next-attempt
/// timestamp. The MODIFY event re-enters Stage 2 via the same Pipe filter
/// (`state=Pending`) — no special re-enqueue path needed.
///
/// `Published` is the only blocked state — re-publishing an already-published
/// job risks double posting on the platform. Every other state retries:
///   - `Failed` (any category, including `auth_expired`) — user-initiated.
///   - `Pending` — covers stuck rows where the dispatcher Lambda died
///     mid-flight or EventBridge dropped the event; UI exposes this so the
///     author isn't dependent on infra recovery.
///   - `Skipped` — privacy guard re-runs in the dispatcher; if the post is
///     still private the row settles back to Skipped (idempotent no-op).
///
/// `attempts` is reset to 0 — the auto-retry sweeper that previously owned
/// the 3-attempt budget was removed when failure handling moved to inline
/// retry + inbox notification, so the counter is purely informational now.
#[post("/api/cross-posting/posts/{post_id}/jobs/{platform}/retry", user: User)]
pub async fn retry_job_handler(post_id: FeedPartition, platform: SocialPlatform) -> Result<()> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.into();

    let post = Post::get(cli, &post_pk, Some(EntityType::Post))
        .await
        .map_err(|e| {
            crate::error!("retry_job post lookup failed: {e}");
            CrossPostingError::UpdateFailed
        })?
        .ok_or(CrossPostingError::NotAuthorized)?;

    if post.user_pk != user.pk {
        return Err(CrossPostingError::NotAuthorized.into());
    }

    let sk = EntityType::SyndicationJob(platform.to_string());
    let job = SyndicationJob::get(cli, &post_pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("retry_job lookup failed: {e}");
            CrossPostingError::UpdateFailed
        })?
        .ok_or(CrossPostingError::SyndicationJobNotFound)?;

    if job.state == JobState::Published {
        return Err(CrossPostingError::RetryNotAllowed.into());
    }

    let now = crate::common::utils::time::now();
    // `remove_dispatch_shard()` issues a DynamoDB REMOVE on the attribute,
    // which also drops the row from the sparse `find_due_jobs` GSI — the
    // matching primary-table state stays Pending and the row re-enters
    // Stage 2 via the SyndicationJob MODIFY → Pipe filter on `state=Pending`.
    SyndicationJob::updater(post_pk, sk)
        .with_state(JobState::Pending)
        .with_attempts(0)
        .remove_dispatch_shard()
        .with_next_attempt_at(0)
        .with_updated_at(now)
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("retry_job update failed: {e}");
            CrossPostingError::UpdateFailed
        })?;

    Ok(())
}
