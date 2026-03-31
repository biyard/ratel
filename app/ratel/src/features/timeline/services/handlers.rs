use crate::common::models::space::SpaceCommon;
use crate::features::posts::models::Post;
use crate::features::timeline::*;

/// Handle a TimelineUpdate event — fan out a new post to followers.
pub async fn handle_timeline_event(post: Post) -> Result<()> {
    tracing::info!(
        "Timeline update: post_pk={}, author_pk={}, created_at={}",
        post.pk,
        post.user_pk,
        post.created_at
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    super::fan_out_timeline_entries(cli, &post.pk, &post.user_pk, post.created_at).await
}

/// Handle a PopularPostUpdate event — fan out to 2nd-degree followers if popular.
pub async fn handle_popular_post_event(post: Post) -> Result<()> {
    if !super::is_popular(post.likes, post.comments, post.shares) {
        return Ok(());
    }

    tracing::info!(
        "Popular post fan-out: post_pk={}, likes={}, comments={}, shares={}",
        post.pk,
        post.likes,
        post.comments,
        post.shares
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    super::fan_out_popular_post(cli, &post.pk, &post.user_pk, post.created_at).await
}

/// Handle a PopularSpaceUpdate event — fan out to 2nd-degree followers if popular.
pub async fn handle_popular_space_event(space: SpaceCommon) -> Result<()> {
    if !super::is_popular_space(space.participants) {
        return Ok(());
    }

    tracing::info!(
        "Popular space fan-out: space_pk={}, participants={}",
        space.pk,
        space.participants
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    super::fan_out_popular_space(
        cli,
        &space.pk,
        &space.post_pk,
        &space.user_pk,
        space.created_at,
    )
    .await
}
