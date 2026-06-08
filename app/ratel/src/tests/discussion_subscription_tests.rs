use super::*;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{
    EntityType, Partition, SpacePartition, SpacePostPartition, SpacePublishState, SpaceStatus,
    SpaceVisibility,
};
use crate::common::models::notification::UserInboxNotification;
use crate::common::types::InboxKind;
use crate::features::spaces::pages::actions::actions::discussion::{
    SpacePost, SpacePostSubscription,
};

async fn inbox_rows_for(ctx: &TestContext, user_pk: Partition) -> Vec<UserInboxNotification> {
    let (rows, _) = UserInboxNotification::query(
        &ctx.ddb,
        user_pk,
        UserInboxNotification::opt().sk("USER_INBOX_NOTIFICATION".to_string()),
    )
    .await
    .expect("query inbox");
    rows
}

/// Seed a published, ongoing, public space owned by `ctx.test_user` plus one
/// discussion (SpacePost). Returns (space_id, discussion_id) as raw id strings.
async fn seed_space_and_discussion(ctx: &TestContext) -> (String, String) {
    let space_id = uuid::Uuid::new_v4().to_string();
    let post_id = space_id.clone();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(post_id.clone());

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

    let post = crate::features::posts::models::Post {
        pk: post_pk.clone(),
        sk: EntityType::Post,
        title: "Sub Test".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    let discussion_id = uuid::Uuid::now_v7().to_string();
    let mut discussion = SpacePost::default();
    discussion.pk = space_pk.clone();
    discussion.sk = EntityType::SpacePost(discussion_id.clone());
    discussion.created_at = now;
    discussion.updated_at = now;
    discussion.title = "Test Discussion".to_string();
    discussion.user_pk = ctx.test_user.0.pk.clone();
    discussion.author_display_name = ctx.test_user.0.display_name.clone();
    discussion.author_username = ctx.test_user.0.username.clone();
    discussion.author_profile_url = ctx.test_user.0.profile_url.clone();
    discussion.create(&ctx.ddb).await.expect("create discussion");

    // The detail endpoint also loads the discussion's SpaceAction row.
    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::new(
        SpacePartition(space_id.clone()),
        discussion_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
    );
    space_action
        .create(&ctx.ddb)
        .await
        .expect("create space action");

    (space_id, discussion_id)
}

async fn join_space(ctx: &TestContext, space_id: &str, user_pk: Partition) {
    let participant = SpaceParticipant::new(Partition::Space(space_id.to_string()), user_pk);
    participant.create(&ctx.ddb).await.expect("join space");
}

async fn subscription_exists(
    ctx: &TestContext,
    discussion_id: &str,
    user_pk: &Partition,
) -> bool {
    let (pk, sk) =
        SpacePostSubscription::keys(&SpacePostPartition(discussion_id.to_string()), user_pk);
    SpacePostSubscription::get(&ctx.ddb, &pk, Some(sk))
        .await
        .expect("get subscription")
        .is_some()
}

#[tokio::test]
async fn test_subscribe_creates_row() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "subscribe: {:?}", body);
    assert!(
        subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await,
        "subscription row should exist after subscribe"
    );
}

#[tokio::test]
async fn test_subscribe_is_idempotent() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;
    let path = format!(
        "/api/spaces/{}/discussions/{}/subscribe",
        space_id, discussion_id
    );

    for _ in 0..2 {
        let (status, _, body) = crate::test_post! {
            app: ctx.app.clone(),
            path: &path,
            headers: ctx.test_user.1.clone(),
        };
        assert_eq!(status, 200, "subscribe twice: {:?}", body);
    }
    assert!(subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await);
}

#[tokio::test]
async fn test_participant_can_unsubscribe() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    // A non-author participant subscribes then unsubscribes — allowed.
    let (user2, headers2) = ctx.create_another_user().await;
    join_space(&ctx, &space_id, user2.pk.clone()).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: headers2.clone(),
    };
    assert_eq!(status, 200, "participant subscribe: {:?}", body);
    assert!(subscription_exists(&ctx, &discussion_id, &user2.pk).await);

    let (status, _, body) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: headers2.clone(),
    };
    assert_eq!(status, 200, "participant unsubscribe: {:?}", body);
    assert!(
        !subscription_exists(&ctx, &discussion_id, &user2.pk).await,
        "subscription row should be gone after participant unsubscribe"
    );
}

#[tokio::test]
async fn test_author_cannot_unsubscribe() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    // test_user owns the space → Creator (author).
    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };

    let (status, _, body) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "author must not be able to unsubscribe: {:?}", body);
    assert!(
        subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await,
        "author subscription must remain after a rejected unsubscribe"
    );
}

#[tokio::test]
async fn test_create_discussion_auto_subscribes_author() {
    let ctx = TestContext::setup().await;

    let space_id = uuid::Uuid::new_v4().to_string();
    let post_id = space_id.clone();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut space = SpaceCommon::default();
    space.pk = Partition::Space(space_id.clone());
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Ongoing);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = Partition::Feed(post_id.clone());
    space.user_pk = ctx.test_user.0.pk.clone();
    space.author_display_name = ctx.test_user.0.display_name.clone();
    space.author_profile_url = ctx.test_user.0.profile_url.clone();
    space.author_username = ctx.test_user.0.username.clone();
    space.create(&ctx.ddb).await.expect("create space");
    let post = crate::features::posts::models::Post {
        pk: Partition::Feed(post_id.clone()),
        sk: EntityType::Post,
        title: "AutoSub".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions", space_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "create discussion: {:?}", body);

    let discussion_sk = body["sk"].as_str().expect("sk in response");
    let discussion_id = discussion_sk
        .strip_prefix("SPACE_POST#")
        .unwrap_or(discussion_sk)
        .to_string();

    assert!(
        subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await,
        "author should be auto-subscribed to their new discussion (sk={discussion_sk})"
    );
}

#[tokio::test]
async fn test_detail_reports_subscribed_state() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/detail", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "detail before: {:?}", body);
    assert_eq!(
        body["subscribed"],
        serde_json::json!(false),
        "should be unsubscribed initially: {:?}",
        body
    );

    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/detail", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "detail after: {:?}", body);
    assert_eq!(
        body["subscribed"],
        serde_json::json!(true),
        "should be subscribed after subscribe: {:?}",
        body
    );
}

#[tokio::test]
async fn test_subscribe_requires_auth() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
    };
    assert_ne!(status, 200, "unauthenticated subscribe must fail");
}

#[tokio::test]
async fn test_comment_notifies_subscriber_not_author() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (subscriber, _headers) = ctx.create_another_user().await;
    {
        let sub = SpacePostSubscription::new(
            SpacePostPartition(discussion_id.clone()),
            SpacePartition(space_id.clone()),
            &subscriber.pk,
        );
        sub.create(&ctx.ddb).await.expect("seed subscription");
    }

    crate::common::utils::discussion_notification::send_discussion_comment_posted(
        &SpacePartition(space_id.clone()),
        &discussion_id,
        "Test Discussion",
        "SPACE_POST_COMMENT#test-comment-1",
        None,
        &ctx.test_user.0.pk.to_string(),
        &ctx.test_user.0.display_name,
        "hello everyone",
        "/spaces/x/discussions/y/comments/z",
    )
    .await
    .expect("fan-out");

    let sub_rows = inbox_rows_for(&ctx, subscriber.pk.clone()).await;
    let disc_rows: Vec<_> = sub_rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .collect();
    assert_eq!(
        disc_rows.len(),
        1,
        "subscriber should get one row: {:?}",
        sub_rows
    );

    let author_rows = inbox_rows_for(&ctx, ctx.test_user.0.pk.clone()).await;
    let author_disc: Vec<_> = author_rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .collect();
    assert!(
        author_disc.is_empty(),
        "commenter must not be notified: {:?}",
        author_rows
    );
}

#[tokio::test]
async fn test_mentioned_subscriber_gets_only_mention_not_subscription() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (mentioned, _h) = ctx.create_another_user().await;
    {
        let sub = SpacePostSubscription::new(
            SpacePostPartition(discussion_id.clone()),
            SpacePartition(space_id.clone()),
            &mentioned.pk,
        );
        sub.create(&ctx.ddb).await.expect("seed subscription");
    }

    let content = format!("hey @[{}](user:{})", mentioned.display_name, mentioned.pk);

    crate::common::utils::discussion_notification::send_discussion_comment_posted(
        &SpacePartition(space_id.clone()),
        &discussion_id,
        "Test Discussion",
        "SPACE_POST_COMMENT#test-comment-2",
        None,
        &ctx.test_user.0.pk.to_string(),
        &ctx.test_user.0.display_name,
        &content,
        "/spaces/x/discussions/y/comments/z",
    )
    .await
    .expect("fan-out");

    let rows = inbox_rows_for(&ctx, mentioned.pk.clone()).await;
    let disc_rows: Vec<_> = rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .collect();
    assert!(
        disc_rows.is_empty(),
        "mentioned subscriber must not get a subscription row: {:?}",
        rows
    );
}

#[tokio::test]
async fn test_reply_target_subscriber_gets_only_reply_row() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (parent_author, _h) = ctx.create_another_user().await;
    {
        let sub = SpacePostSubscription::new(
            SpacePostPartition(discussion_id.clone()),
            SpacePartition(space_id.clone()),
            &parent_author.pk,
        );
        sub.create(&ctx.ddb).await.expect("seed subscription");
    }
    let parent_uuid = uuid::Uuid::now_v7().to_string();
    let parent_sk = format!("SPACE_POST_COMMENT#{parent_uuid}");
    {
        use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;
        let mut parent = SpacePostComment::default();
        parent.pk = Partition::SpacePost(discussion_id.clone());
        parent.sk = EntityType::SpacePostComment(parent_uuid.clone());
        parent.content = "parent".to_string();
        parent.author_pk = parent_author.pk.clone();
        parent.author_display_name = parent_author.display_name.clone();
        parent.author_username = parent_author.username.clone();
        parent.author_profile_url = parent_author.profile_url.clone();
        parent.create(&ctx.ddb).await.expect("create parent comment");
    }

    crate::common::utils::discussion_notification::send_discussion_comment_posted(
        &SpacePartition(space_id.clone()),
        &discussion_id,
        "Test Discussion",
        "SPACE_POST_COMMENT_REPLY#reply-1",
        Some(&parent_sk),
        &ctx.test_user.0.pk.to_string(),
        &ctx.test_user.0.display_name,
        "my reply",
        "/spaces/x/discussions/y/comments/z",
    )
    .await
    .expect("fan-out");

    let rows = inbox_rows_for(&ctx, parent_author.pk.clone()).await;
    let reply_rows = rows
        .iter()
        .filter(|r| r.kind == InboxKind::ReplyOnComment)
        .count();
    let disc_rows = rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .count();
    assert_eq!(reply_rows, 1, "parent author should get one reply row: {:?}", rows);
    assert_eq!(disc_rows, 0, "no duplicate subscription row for reply target: {:?}", rows);
}
