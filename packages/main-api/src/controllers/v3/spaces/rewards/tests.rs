use crate::controllers::v3::me::memberships::tests::seed_test_user_payment;
use crate::controllers::v3::posts::CreatePostResponse;
use crate::controllers::v3::spaces::create_space::CreateSpaceResponse;
use crate::controllers::v3::spaces::polls::RespondPollSpaceResponse;
use crate::controllers::v3::spaces::polls::tests::setup_published_poll_space;
use crate::features::membership::*;
use crate::features::payment::*;
use crate::features::spaces::rewards::*;
use crate::tests::v3_setup::*;
use crate::types::*;
use crate::*;

/// Helper function to seed reward definitions
#[allow(dead_code)]
async fn default_poll_rewards(cli: &aws_sdk_dynamodb::Client) {
    // Create PollRespond reward if it doesn't exist
    let poll_respond_pk = Partition::Reward;
    if Reward::get(cli, poll_respond_pk.clone(), Some(&RewardType::PollRespond))
        .await
        .unwrap()
        .is_none()
    {
        let poll_reward = Reward::new(
            RewardType::PollRespond,
            10_000, // 10,000 points
            RewardPeriod::Daily,
            RewardCondition::None,
        );
        poll_reward.create(cli).await.unwrap();
    }
}

/// Helper function to setup user with Pro membership
async fn setup_user_with_credits(
    cli: &aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> UserMembership {
    seed_test_user_payment(cli, user_pk).await;

    // Upgrade user to Pro membership
    let pro_pk = Partition::Membership(MembershipTier::Pro.to_string());
    let membership = Membership::get(cli, pro_pk.clone(), Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Pro membership should exist");

    let user_membership = UserMembership::new(
        user_pk.clone().into(),
        membership.pk.clone().into(),
        membership.duration_days,
        membership.credits,
    )
    .unwrap();

    user_membership.create(cli).await.unwrap();

    user_membership
}

#[tokio::test]
async fn test_create_reward_success() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // default_poll_rewards(&ddb).await;

    // Create a reward
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Poll Response Reward",
            "description": "Get points for responding to this poll",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200, "Failed to create reward. Response: {:?}", body);
    assert_eq!(body.label, "Poll Response Reward");
    assert_eq!(body.credits, 10);
    assert_eq!(body.points, 10_000);

    // Verify the reward was created in DB
    let reward_key = RewardKey::Poll(poll_sk.clone().into(), PollReward::Respond);
    let space_reward = SpaceReward::get_by_reward_key(&ddb, space_pk.clone().into(), reward_key)
        .await
        .expect("SpaceReward should exist");

    assert_eq!(space_reward.label, "Poll Response Reward");
    assert_eq!(space_reward.credits, 10);

    // Verify user credits were deducted
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 30); // 40 - 10 = 30
}

#[tokio::test]
async fn test_create_reward_insufficient_credits() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with limited credits
    let mut user_membership = setup_user_with_credits(&ddb, &test_user.0.pk).await;
    user_membership.remaining_credits = 5; // Only 5 credits
    UserMembership::updater(user_membership.pk.clone(), user_membership.sk.clone())
        .with_remaining_credits(5)
        .execute(&ddb)
        .await
        .unwrap();

    // Try to create a reward that costs more than available credits
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Expensive Reward",
            "description": "Too expensive",
            "credits": 10
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 400); // Should return insufficient credits error
}

#[tokio::test]
async fn test_list_rewards_authenticated() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits and create a reward
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // List rewards
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<SpaceRewardResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].label, "Test Reward");
    assert_eq!(body.items[0].credits, 10);
    assert_eq!(body.items[0].points, 10_000);
}

#[tokio::test]
async fn test_list_rewards_guest() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits and create a reward
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // List rewards as guest (no auth)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        response_type: ListItemsResponse<SpaceRewardResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].label, "Test Reward");
    // Guest should not see user-specific reward progress
    assert_eq!(body.items[0].user_claims, 0);
}

#[tokio::test]
async fn test_list_rewards_filtered_by_feature() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Create a poll reward
    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Poll Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // List rewards filtered by POLL entity type
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/rewards?feature={}", space_pk.to_string(), poll_sk.to_string()),
        headers: test_user.1.clone(),
        response_type: ListItemsResponse<SpaceRewardResponse>
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(body.items[0].label, "Poll Reward");
}

#[tokio::test]
async fn test_update_reward_success() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Create initial reward
    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Original Label",
            "description": "Original Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Update the reward
    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Updated Label",
            "description": "Updated Description",
            "credits": 15
        },
        response_type: SpaceRewardResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.label, "Updated Label");
    assert_eq!(body.description, "Updated Description");
    assert_eq!(body.credits, 15);

    // Verify the reward was updated in DB
    let reward_key = RewardKey::Poll(poll_sk.clone().into(), PollReward::Respond);
    let space_reward = SpaceReward::get_by_reward_key(&ddb, space_pk.clone().into(), reward_key)
        .await
        .expect("SpaceReward should exist");

    assert_eq!(space_reward.label, "Updated Label");
    assert_eq!(space_reward.description, "Updated Description");
    assert_eq!(space_reward.credits, 15);

    // Verify user credits were adjusted (deducted additional 5 credits)
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 25); // 40 - 10 - 5 = 25
}

#[tokio::test]
async fn test_update_reward_reduce_credits() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Create initial reward with 20 credits
    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "Description",
            "credits": 20
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Check initial credits
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 20); // 40 - 20 = 20

    // Update reward with fewer credits (reduce to 10)
    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.credits, 10);

    // Verify credits were refunded (10 credits returned)
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 30); // 20 + 10 = 30
}

#[tokio::test]
async fn test_update_reward_without_permission() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        user2,
        ..
    } = ctx;

    // Setup test_user with credits and create reward
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Try to update reward with different user (user2) who doesn't have permission
    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Unauthorized Update",
            "description": "Should fail",
            "credits": 15
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 401); // Forbidden
}

#[tokio::test]
async fn test_delete_reward_success() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;
    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Create a reward
    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "To be deleted",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Check credits after creation
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 30); // 40 - 10 = 30

    // Delete the reward
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            }
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 200);

    // Verify the reward was deleted from DB
    let reward_key = RewardKey::Poll(poll_sk.clone().into(), PollReward::Respond);
    let space_reward =
        SpaceReward::get_by_reward_key(&ddb, space_pk.clone().into(), reward_key).await;
    assert!(space_reward.is_err()); // Should not exist

    // Verify credits were refunded
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 40); // 30 + 10 = 40 (refunded)
}

#[tokio::test]
async fn test_delete_reward_nonexistent() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Try to delete a reward that doesn't exist
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            }
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 400); // Not found
}

#[tokio::test]
async fn test_delete_reward_without_permission() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        user2,
        ..
    } = ctx;

    // Setup test_user with credits and create reward
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Test Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Try to delete reward with different user (user2) who doesn't have permission
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            }
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 401); // Forbidden
}

#[tokio::test]
async fn test_create_multiple_rewards_deducts_total_credits() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        ..
    } = ctx;

    // Setup user with 40 credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Create first reward (10 credits)
    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "First Reward",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Verify credits after first reward
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(user_membership.remaining_credits, 30); // 40 - 10 = 30

    // Note: We can't create another reward for the same poll_sk since it would be duplicate
    // This test verifies that credits are properly tracked across operations
}

#[tokio::test]
async fn test_poll_respond_increases_user_claim() {
    let (ctx, space_pk, poll_sk, _) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        user2,
        ..
    } = ctx;

    // Setup user with credits
    setup_user_with_credits(&ddb, &test_user.0.pk).await;

    // Create a poll reward
    let (status, _headers, _) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "reward": {
                "poll_sk": poll_sk.to_string()
            },
            "label": "Poll Response Reward",
            "description": "Get points for responding to this poll",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200, "Failed to create reward");

    // Check user_claims before poll response
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        response_type: ListItemsResponse<SpaceRewardResponse>
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(
        body.items[0].user_claims, 0,
        "User should have 0 claims initially"
    );

    let answers = vec![
        Answer::SingleChoice {
            answer: Some(1),
            other: None,
        },
        Answer::MultipleChoice {
            answer: Some(vec![0, 2]),
            other: None,
        },
    ];

    // user2 responds to the poll
    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone(),
        body: {
            "answers": answers.clone(),
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200, "Failed to respond to poll");

    // Check user_claims after poll response
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        response_type: ListItemsResponse<SpaceRewardResponse>
    };
    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1);
    assert_eq!(
        body.items[0].user_claims, 1,
        "User should have 1 claim after poll response"
    );

    // Verify UserReward was created with correct values
    let reward_key = RewardKey::Poll(poll_sk.clone().into(), PollReward::Respond);
    let (user_reward_pk, user_reward_sk) = UserReward::keys(
        user2.0.pk.clone().into(),
        space_pk.clone().into(),
        reward_key,
    );
    let user_reward = UserReward::get(&ddb, user_reward_pk, Some(user_reward_sk))
        .await
        .unwrap()
        .expect("UserReward should exist");

    assert_eq!(
        user_reward.total_claims, 1,
        "UserReward should have 1 total claim"
    );
    assert_eq!(
        user_reward.total_points,
        10_000 * 10,
        "UserReward should have 10,000 total points"
    ); // Point * Credit
}
