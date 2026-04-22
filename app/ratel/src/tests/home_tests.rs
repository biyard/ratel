use super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{
    EntityType, Partition, SpacePartition, SpacePublishState, SpaceStatus, SpaceVisibility,
};
use crate::features::auth::User;
use crate::features::posts::models::Post;
use crate::features::spaces::space_common::controllers::HotSpaceResponse;
use crate::features::timeline::models::{TimelineEntry, TimelineReason};

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
        space_pk: Some(space_pk),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    space
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

async fn seed_timeline_entry(
    ctx: &TestContext,
    viewer: &User,
    post_pk: &Partition,
    author_pk: &Partition,
    reason: TimelineReason,
) {
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let entry = TimelineEntry::new(&viewer.pk, post_pk, author_pk, now, reason);
    entry.create(&ctx.ddb).await.unwrap();
}

fn space_id_str(pk: &Partition) -> String {
    Into::<SpacePartition>::into(pk.clone()).to_string()
}

// ----- My Spaces ---------------------------------------------------------

#[tokio::test]
async fn test_my_spaces_filters_to_active_status() {
    let ctx = TestContext::setup().await;
    let user = ctx.test_user.0.clone();

    let designing = seed_space(&ctx, &user, SpaceStatus::Designing).await;
    seed_participant(&ctx, &designing.pk, &user, None).await;

    let ongoing = seed_space(&ctx, &user, SpaceStatus::Ongoing).await;
    seed_participant(&ctx, &ongoing.pk, &user, None).await;

    let finished = seed_space(&ctx, &user, SpaceStatus::Finished).await;
    seed_participant(&ctx, &finished.pk, &user, None).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/home/my-spaces",
        headers: ctx.test_user.1.clone(),
        response_type: crate::common::types::ListResponse<HotSpaceResponse>,
    };
    assert_eq!(status, 200, "my-spaces: {:?}", body);

    let returned: Vec<String> = body.items.iter().map(|i| i.space_id.to_string()).collect();
    assert!(
        returned.contains(&space_id_str(&ongoing.pk)),
        "Ongoing space should appear: {:?}",
        returned
    );
    assert!(
        !returned.contains(&space_id_str(&designing.pk)),
        "Designing space must be filtered out: {:?}",
        returned
    );
    assert!(
        !returned.contains(&space_id_str(&finished.pk)),
        "Finished space must be filtered out: {:?}",
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
    // Positive case for the fan-in path: a TimelineEntry with reason Following
    // should resolve back to its SpaceCommon and surface the space in Hot.
    let ctx = TestContext::setup().await;
    let author = ctx.test_user.0.clone();
    let (viewer, viewer_headers) = ctx.create_another_user().await;

    let space = seed_space(&ctx, &author, SpaceStatus::Ongoing).await;
    seed_timeline_entry(
        &ctx,
        &viewer,
        &space.post_pk,
        &author.pk,
        TimelineReason::Following,
    )
    .await;

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
        "Following timeline entry should surface the space in Hot: {:?}",
        body.items
    );
}
