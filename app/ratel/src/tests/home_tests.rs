use super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{
    EntityType, Partition, SpacePartition, SpacePublishState, SpaceStatus, SpaceVisibility,
};
use crate::features::auth::User;
use crate::features::posts::models::Post;
use crate::features::spaces::space_common::controllers::HotSpaceResponse;

async fn seed_space(ctx: &TestContext, author: &User, status: SpaceStatus) -> SpaceCommon {
    let id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let space_pk = Partition::Space(id.clone());
    let post_pk = Partition::Feed(id);

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(status);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = author.pk.clone();
    space.author_display_name = author.display_name.clone();
    space.author_username = author.username.clone();
    space.create(&ctx.ddb).await.unwrap();

    let post = Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: format!("Test Space {}", space.pk),
        space_pk: Some(space_pk.clone()),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    // Hot stream is read-only; both the global rank table and the per-viewer
    // UserHotSpace fanout are populated by `space_fanout::upsert_hot_space`.
    // Production paths (publish / start / status changes) trigger this via
    // stream events — here we call it directly so seeded spaces actually
    // surface in /api/home/hot-spaces for any viewer in the fanout set.
    crate::features::spaces::space_common::services::space_fanout::upsert_hot_space(
        &ctx.ddb, &space_pk,
    )
    .await;

    space
}

/// Seed a follower → target relationship and re-run hot-space fanout so
/// the follower shows up in the target's `UserHotSpace` rows. Used by the
/// "logged-in viewer surfaces a fanned-out space" test.
async fn seed_follow_and_refanout(
    ctx: &TestContext,
    follower: &User,
    target: &User,
    space_pk: &Partition,
) {
    let (follower_record, following_record) =
        crate::common::models::auth::UserFollow::new_follow_records(
            follower.pk.clone(),
            target.pk.clone(),
        );
    follower_record.create(&ctx.ddb).await.unwrap();
    following_record.create(&ctx.ddb).await.unwrap();

    // Re-fanout so the new follower lands in this space's UserHotSpace set.
    crate::features::spaces::space_common::services::space_fanout::upsert_hot_space(
        &ctx.ddb, space_pk,
    )
    .await;
}

async fn seed_participant(
    ctx: &TestContext,
    space_pk: &Partition,
    user: &User,
    last_activity_at: Option<i64>,
) {
    let mut sp = SpaceParticipant::new_non_anonymous(space_pk.clone(), user.clone());
    sp.last_activity_at = last_activity_at;
    sp.create(&ctx.ddb).await.unwrap();
}


fn space_id_str(pk: &Partition) -> String {
    Into::<SpacePartition>::into(pk.clone()).to_string()
}

// ----- My Spaces ---------------------------------------------------------

#[tokio::test]
async fn test_my_spaces_ranks_active_before_inactive() {
    let ctx = TestContext::setup().await;
    let user = ctx.test_user.0.clone();

    let designing = seed_space(&ctx, &user, SpaceStatus::Designing).await;
    seed_participant(&ctx, &designing.pk, &user, Some(9_000)).await;

    let ongoing = seed_space(&ctx, &user, SpaceStatus::Ongoing).await;
    seed_participant(&ctx, &ongoing.pk, &user, Some(1_000)).await;

    let finished = seed_space(&ctx, &user, SpaceStatus::Finished).await;
    seed_participant(&ctx, &finished.pk, &user, Some(5_000)).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/home/my-spaces",
        headers: ctx.test_user.1.clone(),
        response_type: crate::common::types::ListResponse<HotSpaceResponse>,
    };
    assert_eq!(status, 200, "my-spaces: {:?}", body);

    let returned: Vec<String> = body.items.iter().map(|i| i.space_id.to_string()).collect();
    let ongoing_id = space_id_str(&ongoing.pk);
    let designing_id = space_id_str(&designing.pk);
    let finished_id = space_id_str(&finished.pk);

    assert!(
        returned.contains(&ongoing_id),
        "Ongoing space should appear: {:?}",
        returned
    );
    assert!(
        returned.contains(&designing_id),
        "Designing space should still appear for history access: {:?}",
        returned
    );
    assert!(
        returned.contains(&finished_id),
        "Finished space should still appear for history access: {:?}",
        returned
    );

    let ongoing_idx = returned.iter().position(|id| id == &ongoing_id).unwrap();
    let designing_idx = returned.iter().position(|id| id == &designing_id).unwrap();
    let finished_idx = returned.iter().position(|id| id == &finished_id).unwrap();
    assert!(
        ongoing_idx < designing_idx && ongoing_idx < finished_idx,
        "Ongoing must rank above non-active spaces regardless of activity time: {:?}",
        returned
    );
}

#[tokio::test]
async fn test_my_spaces_sorts_by_last_activity() {
    let ctx = TestContext::setup().await;
    let user = ctx.test_user.0.clone();

    let older = seed_space(&ctx, &user, SpaceStatus::Ongoing).await;
    let newer = seed_space(&ctx, &user, SpaceStatus::Ongoing).await;

    // Pin explicit activity times so the sort is deterministic regardless of
    // the order BatchGetItem returns rows.
    seed_participant(&ctx, &older.pk, &user, Some(1_000)).await;
    seed_participant(&ctx, &newer.pk, &user, Some(2_000)).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/home/my-spaces",
        headers: ctx.test_user.1.clone(),
        response_type: crate::common::types::ListResponse<HotSpaceResponse>,
    };
    assert_eq!(status, 200, "my-spaces: {:?}", body);
    assert!(body.items.len() >= 2, "expected both spaces");

    let newer_id = space_id_str(&newer.pk);
    let older_id = space_id_str(&older.pk);
    let newer_idx = body
        .items
        .iter()
        .position(|i| i.space_id.to_string() == newer_id);
    let older_idx = body
        .items
        .iter()
        .position(|i| i.space_id.to_string() == older_id);
    assert!(
        newer_idx.is_some() && older_idx.is_some(),
        "both must appear in response"
    );
    assert!(
        newer_idx.unwrap() < older_idx.unwrap(),
        "newer activity (2000) should rank above older (1000)"
    );
}

#[tokio::test]
async fn test_my_spaces_requires_auth() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: "/api/home/my-spaces",
    };
    assert_ne!(status, 200, "unauthenticated request must not succeed");
}

// ----- Hot Spaces --------------------------------------------------------

#[tokio::test]
async fn test_hot_spaces_logged_out_uses_public_fallback() {
    let ctx = TestContext::setup().await;
    let author = ctx.test_user.0.clone();

    let space = seed_space(&ctx, &author, SpaceStatus::Ongoing).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: "/api/home/hot-spaces",
        response_type: crate::common::types::ListResponse<HotSpaceResponse>,
    };
    assert_eq!(status, 200, "hot-spaces (anon): {:?}", body);

    let target = space_id_str(&space.pk);
    assert!(
        body.items.iter().any(|i| i.space_id.to_string() == target),
        "anon fallback should include the public space: {:?}",
        body.items
    );
}

#[tokio::test]
async fn test_hot_spaces_logged_in_excludes_non_fanned_out_public_space() {
    let ctx = TestContext::setup().await;
    let author = ctx.test_user.0.clone();
    let (_viewer, viewer_headers) = ctx.create_another_user().await;

    let _public_space = seed_space(&ctx, &author, SpaceStatus::Ongoing).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/home/hot-spaces",
        headers: viewer_headers,
        response_type: crate::common::types::ListResponse<HotSpaceResponse>,
    };
    assert_eq!(status, 200, "hot-spaces: {:?}", body);
    assert!(
        body.items.is_empty(),
        "viewer outside the fan-out graph must see no Hot spaces: {:?}",
        body.items
    );
}

#[tokio::test]
async fn test_hot_spaces_logged_in_surfaces_following_entry() {
    // Positive case for the per-viewer fanout path: when the viewer follows
    // the author, `space_fanout::upsert_hot_space` writes a UserHotSpace
    // row in the viewer's namespace and the space surfaces in their Hot
    // stream.
    let ctx = TestContext::setup().await;
    let author = ctx.test_user.0.clone();
    let (viewer, viewer_headers) = ctx.create_another_user().await;

    let space = seed_space(&ctx, &author, SpaceStatus::Ongoing).await;
    seed_follow_and_refanout(&ctx, &viewer, &author, &space.pk).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/home/hot-spaces",
        headers: viewer_headers,
        response_type: crate::common::types::ListResponse<HotSpaceResponse>,
    };
    assert_eq!(status, 200, "hot-spaces: {:?}", body);

    let target = space_id_str(&space.pk);
    assert!(
        body.items.iter().any(|i| i.space_id.to_string() == target),
        "Following relationship should surface the space in Hot: {:?}",
        body.items
    );
}
