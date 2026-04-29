use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::{JobState, SyndicationJob};
use crate::features::cross_posting::types::{CrossPostingError, SocialPlatform};
use crate::features::posts::models::Post;
use std::str::FromStr;

/// Author-initiated retry of a single failed `SyndicationJob` (FR-7 #44).
/// Resets the job to `Pending` and clears the retry-queue shard / attempt
/// timestamp. The MODIFY event re-enters Stage 2 via the same Pipe filter
/// (`state=Pending`) — no special re-enqueue path needed.
///
/// `attempts` is intentionally NOT reset: a user-initiated retry counts
/// against the same 3-attempt budget as the auto-retry sweeper. To grant
/// fresh attempts, the user reconnects the platform (which writes a new
/// directive on the next publish, creating a new job row).
///
/// Phase 1 caveat: Stage 2 dispatcher is wired in PR C — until then,
/// retries land the row in `Pending` state but no actual platform call
/// happens. This endpoint ships now so the UI button can render against
/// real data; the back-end picks up automatically once PR C deploys.
#[post("/api/cross-posting/posts/{post_id}/jobs/{platform}/retry", user: User)]
pub async fn retry_job_handler(post_id: FeedPartition, platform: String) -> Result<()> {
    let platform: SocialPlatform = SocialPlatform::from_str(&platform)
        .map_err(|_| CrossPostingError::SyndicationJobNotFound)?;

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

    if job.state != JobState::Failed {
        return Err(CrossPostingError::RetryNotAllowed.into());
    }

    let now = crate::common::utils::time::now();
    // `remove_dispatch_shard()` issues a DynamoDB REMOVE on the attribute,
    // which also drops the row from the sparse `find_due_jobs` GSI — the
    // matching primary-table state stays Pending and the row re-enters
    // Stage 2 via the SyndicationJob MODIFY → Pipe filter on `state=Pending`.
    SyndicationJob::updater(post_pk, sk)
        .with_state(JobState::Pending)
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
