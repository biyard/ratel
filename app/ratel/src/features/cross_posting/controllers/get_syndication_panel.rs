use crate::common::*;
use crate::features::auth::User;
use crate::features::cross_posting::models::{EngagementSnapshot, JobState, SyndicationJob};
use crate::features::cross_posting::types::{
    CrossPostingError, EngagementCountsView, SyndicationJobView, SyndicationPanelResponse,
};
use crate::features::posts::models::Post;

/// Author-only syndication panel (FR-7 #41–#45). Shows per-platform
/// dispatch state, retry diagnostics, and engagement counts for one of
/// the user's own posts. Returns `NotAuthorized` for posts whose
/// `user_pk` does not match the session user (Phase 1 supports only
/// user-owned cross-posting; team-owned posts have no
/// `PostSyndicationDirective` written and therefore no jobs to show).
#[get("/api/cross-posting/posts/{post_id}/syndication", user: User)]
pub async fn get_syndication_panel_handler(
    post_id: FeedPartition,
) -> Result<SyndicationPanelResponse> {
    let conf = crate::features::posts::config::get();
    let cli = conf.dynamodb();

    let post_pk: Partition = post_id.clone().into();

    let post = Post::get(cli, &post_pk, Some(EntityType::Post))
        .await
        .map_err(|e| {
            crate::error!("get_syndication_panel post lookup failed: {e}");
            CrossPostingError::ListFailed
        })?
        .ok_or(CrossPostingError::NotAuthorized)?;

    if post.user_pk != user.pk {
        return Err(CrossPostingError::NotAuthorized.into());
    }

    // Query SyndicationJob rows colocated under the post's pk.
    let sk_prefix = EntityType::SyndicationJob(String::new()).to_string();
    let opt = SyndicationJob::opt_with_bookmark(None).sk(sk_prefix).limit(10);
    let (jobs, _next): (Vec<SyndicationJob>, _) =
        SyndicationJob::query(cli, &post_pk, opt).await.map_err(|e| {
            crate::error!("get_syndication_panel jobs query failed: {e}");
            CrossPostingError::ListFailed
        })?;

    // Hydrate engagement counts for Published jobs. At most 3 rows in
    // Phase 1 (Bluesky / LinkedIn / Threads) so sequential reads are fine.
    let mut views = Vec::with_capacity(jobs.len());
    for job in jobs {
        let engagement = if job.state == JobState::Published {
            EngagementSnapshot::get(
                cli,
                &post_pk,
                Some(EntityType::EngagementSnapshot(job.platform.to_string())),
            )
            .await
            .ok()
            .flatten()
            .map(EngagementCountsView::from)
        } else {
            None
        };
        views.push(SyndicationJobView::from_job(job, engagement));
    }

    Ok(SyndicationPanelResponse { post_id, jobs: views })
}
