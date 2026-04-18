use super::*;
use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::common::types::{
    EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility,
};
use crate::features::spaces::pages::actions::actions::discussion::{
    SpacePost, SpacePostComment, ROOT_PARENT,
};

/// Helper: seed a public individual-owned space, a discussion post, and a
/// set of comments with controllable `created_at`. Returns the ids needed
/// to build the endpoint URL.
async fn seed_discussion_with_comments(
    ctx: &TestContext,
    comment_timestamps: &[i64],
) -> (String, String, Vec<String>) {
    let space_id = uuid::Uuid::new_v4().to_string();
    let post_id = space_id.clone();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(post_id.clone());

    // SpaceCommon owned by the test user (→ Creator role).
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

    // Minimal Post required by SpaceCommon extractor.
    let post = crate::features::posts::models::Post {
        pk: post_pk.clone(),
        sk: EntityType::Post,
        title: "Discussion Test".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    // SpacePost (the discussion) sits under the space partition.
    let discussion_id = uuid::Uuid::now_v7().to_string();
    let discussion_sk = EntityType::SpacePost(discussion_id.clone());
    let mut discussion = SpacePost::default();
    discussion.pk = space_pk.clone();
    discussion.sk = discussion_sk.clone();
    discussion.created_at = now;
    discussion.updated_at = now;
    discussion.title = "Test Discussion".to_string();
    discussion.user_pk = ctx.test_user.0.pk.clone();
    discussion.author_display_name = ctx.test_user.0.display_name.clone();
    discussion.author_username = ctx.test_user.0.username.clone();
    discussion.author_profile_url = ctx.test_user.0.profile_url.clone();
    discussion.create(&ctx.ddb).await.expect("create discussion");

    // SpacePostComment rows live in the partition keyed by the discussion SK
    // converted into Partition::SpacePost. Each comment gets its caller-specified
    // `created_at` so we can exercise the `since` filter.
    let author = SpaceUser::from(ctx.test_user.0.clone());
    let comment_pk = Partition::SpacePost(discussion_id.clone());
    let mut comment_ids = Vec::with_capacity(comment_timestamps.len());
    for ts in comment_timestamps {
        let comment_uuid = uuid::Uuid::now_v7().to_string();
        let mut comment = SpacePostComment::default();
        comment.pk = comment_pk.clone();
        comment.sk = EntityType::SpacePostComment(comment_uuid.clone());
        comment.space_pk = Some(space_pk.clone());
        comment.created_at = *ts;
        comment.updated_at = *ts;
        comment.content = format!("comment-{}", ts);
        comment.likes_align = format!("{:020}", 0);
        comment.updated_at_align = format!("{:020}", *ts);
        comment.parent_id_for_likes = ROOT_PARENT.to_string();
        comment.author_pk = ctx.test_user.0.pk.clone();
        comment.author_display_name = author.display_name.clone();
        comment.author_username = author.username.clone();
        comment.author_profile_url = author.profile_url.clone();
        comment.create(&ctx.ddb).await.expect("create comment");
        comment_ids.push(comment_uuid);
    }

    (space_id, discussion_id, comment_ids)
}

#[tokio::test]
async fn test_list_comments_without_since_returns_all_top_level() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id, _comment_ids) =
        seed_discussion_with_comments(&ctx, &[100, 200, 300]).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/spaces/{}/discussions/{}/comments",
            space_id, discussion_id
        ),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "list_comments: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(items.len(), 3, "should return all 3 comments: {:?}", body);
}

#[tokio::test]
async fn test_list_comments_with_since_filters_older_comments() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id, _comment_ids) =
        seed_discussion_with_comments(&ctx, &[100, 200, 300]).await;

    // since=200 ⇒ strictly greater-than, so only created_at=300 should return.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/spaces/{}/discussions/{}/comments?since=200",
            space_id, discussion_id
        ),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "list_comments?since=200: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert_eq!(
        items.len(),
        1,
        "since=200 should return only the newest comment: {:?}",
        body
    );
    assert!(
        items[0]["created_at"].as_i64() == Some(300),
        "returned comment should have created_at=300: {:?}",
        items[0]
    );
}

#[tokio::test]
async fn test_list_comments_with_future_since_returns_empty() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id, _) =
        seed_discussion_with_comments(&ctx, &[100, 200, 300]).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!(
            "/api/spaces/{}/discussions/{}/comments?since=9999",
            space_id, discussion_id
        ),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "list_comments?since=9999: {:?}", body);
    let items = body["items"].as_array().expect("items array");
    assert!(
        items.is_empty(),
        "since in the future should return no items: {:?}",
        body
    );
}
