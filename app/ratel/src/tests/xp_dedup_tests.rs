use super::*;

/// Test: calling record_activity twice with the same (space, user, action_id)
/// for a Poll should only create one SpaceActivity record.
#[tokio::test]
async fn test_poll_xp_dedup() {
    let ctx = TestContext::setup().await;

    let space_id = crate::common::types::SpacePartition(uuid::Uuid::new_v4().to_string());
    let author = crate::features::activity::types::AuthorPartition::from(ctx.test_user.0.pk.clone());
    let action_id = uuid::Uuid::new_v4().to_string();

    let data = crate::features::activity::types::SpaceActivityData::Poll {
        poll_id: action_id.clone(),
        answered_optional_count: 0,
    };

    // First call — should succeed
    let result = crate::features::activity::controllers::record_activity(
        &ctx.ddb,
        space_id.clone(),
        author.clone(),
        action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Poll,
        data.clone(),
        "testuser".to_string(),
        "".to_string(),
    )
    .await;
    assert!(result.is_ok(), "first record_activity failed: {:?}", result);

    // Second call (duplicate) — should be silently skipped
    let result = crate::features::activity::controllers::record_activity(
        &ctx.ddb,
        space_id.clone(),
        author.clone(),
        action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Poll,
        data.clone(),
        "testuser".to_string(),
        "".to_string(),
    )
    .await;
    assert!(result.is_ok(), "second record_activity failed: {:?}", result);

    // Verify only 1 SpaceActivity exists
    let pk = crate::common::types::CompositePartition(space_id.clone(), author.clone());
    let sk_prefix = format!("SPACE_ACTIVITY#{}#", action_id);
    let opt = crate::features::activity::models::SpaceActivity::opt().sk(sk_prefix).limit(10);
    let (items, _) = crate::features::activity::models::SpaceActivity::query(&ctx.ddb, pk, opt)
        .await
        .expect("query failed");

    assert_eq!(items.len(), 1, "expected 1 activity, got {}", items.len());
}

/// Test: calling record_activity twice with the same (space, user, action_id)
/// for Follow should only create one SpaceActivity record.
#[tokio::test]
async fn test_follow_xp_dedup() {
    let ctx = TestContext::setup().await;

    let space_id = crate::common::types::SpacePartition(uuid::Uuid::new_v4().to_string());
    let author = crate::features::activity::types::AuthorPartition::from(ctx.test_user.0.pk.clone());
    let action_id = uuid::Uuid::new_v4().to_string();

    let data = crate::features::activity::types::SpaceActivityData::Follow {
        follow_id: action_id.clone(),
    };

    // Two calls
    crate::features::activity::controllers::record_activity(
        &ctx.ddb, space_id.clone(), author.clone(), action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
        data.clone(), "testuser".to_string(), "".to_string(),
    ).await.expect("first failed");

    crate::features::activity::controllers::record_activity(
        &ctx.ddb, space_id.clone(), author.clone(), action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
        data.clone(), "testuser".to_string(), "".to_string(),
    ).await.expect("second failed");

    let pk = crate::common::types::CompositePartition(space_id.clone(), author.clone());
    let sk_prefix = format!("SPACE_ACTIVITY#{}#", action_id);
    let opt = crate::features::activity::models::SpaceActivity::opt().sk(sk_prefix).limit(10);
    let (items, _) = crate::features::activity::models::SpaceActivity::query(&ctx.ddb, pk, opt)
        .await.expect("query failed");

    assert_eq!(items.len(), 1, "expected 1 activity, got {}", items.len());
}

/// Test: two different comments on the same discussion should each create
/// their own SpaceActivity (not deduplicated against each other).
#[tokio::test]
async fn test_discussion_different_comments_not_deduped() {
    let ctx = TestContext::setup().await;

    let space_id = crate::common::types::SpacePartition(uuid::Uuid::new_v4().to_string());
    let author = crate::features::activity::types::AuthorPartition::from(ctx.test_user.0.pk.clone());
    let action_id = uuid::Uuid::new_v4().to_string();
    let comment_id_1 = uuid::Uuid::new_v4().to_string();
    let comment_id_2 = uuid::Uuid::new_v4().to_string();

    let data1 = crate::features::activity::types::SpaceActivityData::Discussion {
        discussion_id: crate::common::types::SpacePostPartition(action_id.clone()),
        comment_id: crate::common::types::SpacePostCommentEntityType(comment_id_1.clone()),
        is_first_contribution: true,
    };
    let data2 = crate::features::activity::types::SpaceActivityData::Discussion {
        discussion_id: crate::common::types::SpacePostPartition(action_id.clone()),
        comment_id: crate::common::types::SpacePostCommentEntityType(comment_id_2.clone()),
        is_first_contribution: false,
    };

    // Comment 1
    crate::features::activity::controllers::record_activity(
        &ctx.ddb, space_id.clone(), author.clone(), action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
        data1.clone(), "testuser".to_string(), "".to_string(),
    ).await.expect("comment 1 failed");

    // Comment 2 (different comment_id — should NOT be deduped)
    crate::features::activity::controllers::record_activity(
        &ctx.ddb, space_id.clone(), author.clone(), action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
        data2.clone(), "testuser".to_string(), "".to_string(),
    ).await.expect("comment 2 failed");

    // Both should exist
    let pk = crate::common::types::CompositePartition(space_id.clone(), author.clone());
    let sk_prefix = format!("SPACE_ACTIVITY#{}#", action_id);
    let opt = crate::features::activity::models::SpaceActivity::opt().sk(sk_prefix).limit(10);
    let (items, _) = crate::features::activity::models::SpaceActivity::query(&ctx.ddb, pk, opt)
        .await.expect("query failed");

    assert_eq!(items.len(), 2, "expected 2 activities (one per comment), got {}", items.len());
}

/// Test: same discussion comment submitted twice should be deduplicated.
#[tokio::test]
async fn test_discussion_same_comment_deduped() {
    let ctx = TestContext::setup().await;

    let space_id = crate::common::types::SpacePartition(uuid::Uuid::new_v4().to_string());
    let author = crate::features::activity::types::AuthorPartition::from(ctx.test_user.0.pk.clone());
    let action_id = uuid::Uuid::new_v4().to_string();
    let comment_id = uuid::Uuid::new_v4().to_string();

    let data = crate::features::activity::types::SpaceActivityData::Discussion {
        discussion_id: crate::common::types::SpacePostPartition(action_id.clone()),
        comment_id: crate::common::types::SpacePostCommentEntityType(comment_id.clone()),
        is_first_contribution: true,
    };

    // Two calls with same comment_id
    crate::features::activity::controllers::record_activity(
        &ctx.ddb, space_id.clone(), author.clone(), action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
        data.clone(), "testuser".to_string(), "".to_string(),
    ).await.expect("first failed");

    crate::features::activity::controllers::record_activity(
        &ctx.ddb, space_id.clone(), author.clone(), action_id.clone(),
        crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
        data.clone(), "testuser".to_string(), "".to_string(),
    ).await.expect("second failed");

    let pk = crate::common::types::CompositePartition(space_id.clone(), author.clone());
    let dedup = format!("{}#comment#{}", action_id, comment_id);
    let sk_prefix = format!("SPACE_ACTIVITY#{}#", dedup);
    let opt = crate::features::activity::models::SpaceActivity::opt().sk(sk_prefix).limit(10);
    let (items, _) = crate::features::activity::models::SpaceActivity::query(&ctx.ddb, pk, opt)
        .await.expect("query failed");

    assert_eq!(items.len(), 1, "expected 1 activity, got {}", items.len());
}
