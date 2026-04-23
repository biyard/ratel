use super::*;
use crate::common::models::space::SpaceCommon;
use crate::common::types::{
    EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility,
};

/// Helper: seed a public individual-owned space so the `SpaceCommon`
/// extractor succeeds and the test user is recognised as Creator.
/// Returns the raw space id (the portion after the `SPACE#` prefix).
async fn seed_creator_space(ctx: &TestContext) -> String {
    let space_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(space_id.clone());

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Ongoing);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = ctx.test_user.0.pk.clone();
    space.author_display_name = ctx.test_user.0.display_name.clone();
    space.author_profile_url = ctx.test_user.0.profile_url.clone();
    space.author_username = ctx.test_user.0.username.clone();
    space.create(&ctx.ddb).await.expect("create space");

    // Minimal Post row so SpaceCommon extraction side-paths (if any) do not 404.
    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "Meet Action Test Space".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    space_id
}

#[tokio::test]
async fn test_create_meet_admin_success() {
    let ctx = TestContext::setup().await;
    let space_id = seed_creator_space(&ctx).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/meets", space_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "create_meet should succeed: {:?}", body);
    assert!(body["sk"].as_str().is_some(), "response must include sk");
}

#[tokio::test]
async fn test_create_meet_unauthenticated() {
    let ctx = TestContext::setup().await;
    let space_id = seed_creator_space(&ctx).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/spaces/{}/meets", space_id),
    };
    assert_ne!(status, 200, "unauthenticated create_meet should fail");
}
