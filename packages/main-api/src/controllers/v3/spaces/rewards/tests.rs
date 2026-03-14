use tokio::time::sleep;

use crate::controllers::v3::me::memberships::change_membership::ChangeMembershipResponse;
use crate::controllers::v3::me::memberships::tests::seed_test_user_payment;
use crate::controllers::v3::spaces::participate_space::ParticipateSpaceResponse;
use crate::controllers::v3::spaces::polls::RespondPollSpaceResponse;
use crate::controllers::v3::spaces::polls::tests::setup_published_poll_space;
use crate::features::membership::*;
use crate::features::payment::*;
use crate::features::spaces::rewards::*;
use crate::tests::v3_setup::*;
use crate::types::SpacePartition;
use crate::types::*;
use crate::*;

/// Helper function to seed reward definitions
#[allow(dead_code)]
async fn default_poll_rewards(cli: &aws_sdk_dynamodb::Client) {
    let poll_respond_pk = Partition::Reward;
    if Reward::get(
        cli,
        poll_respond_pk.clone(),
        Some(RewardUserBehavior::RespondPoll),
    )
    .await
    .unwrap()
    .is_none()
    {
        let poll_reward = Reward::new(
            RewardUserBehavior::RespondPoll,
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
            "description": "Get points for responding to this poll",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200, "Failed to create reward. Response: {:?}", body);
    assert_eq!(body.description, "Get points for responding to this poll");
    assert_eq!(body.credits, 10);
    assert_eq!(body.points, 10_000);

    // Verify the reward was created in DB
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let space_reward = SpaceReward::get(&ddb, space_pk.clone(), Some(reward_key))
        .await
        .expect("SpaceReward should exist")
        .unwrap();

    assert_eq!(
        space_reward.description,
        "Get points for responding to this poll"
    );
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
    assert_eq!(body.items[0].description, "Description");
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
    assert_eq!(body.items[0].description, "Description");
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
    assert_eq!(body.items[0].description, "Description");
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
            "description": "Original Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Update the reward
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "sk": reward_key.to_string(),
            "description": "Updated Description",
            "credits": 15
        },
        response_type: SpaceRewardResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.description, "Updated Description");
    assert_eq!(body.credits, 15);

    // Verify the reward was updated in DB
    let space_reward = SpaceReward::get(&ddb, space_pk.clone(), Some(reward_key))
        .await
        .unwrap()
        .expect("SpaceReward should exist");

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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (status, _headers, body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "sk": reward_key.to_string(),
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Try to update reward with different user (user2) who doesn't have permission
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (status, _headers, _body) = put! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "sk": reward_key.to_string(),
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "sk": reward_key.to_string(),
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 200);
    // Verify the reward was deleted from DB
    let space_reward = SpaceReward::get(&ddb, space_pk.clone(), Some(reward_key))
        .await
        .expect("Request failed");
    assert!(space_reward.is_none()); // Should not exist

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
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "sk": reward_key.to_string(),
        },
        response_type: serde_json::Value
    };

    assert_eq!(status, 404); // Not found
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
            "description": "Description",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(status, 200);

    // Try to delete reward with different user (user2) who doesn't have permission
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (status, _headers, _body) = delete! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "sk": reward_key.to_string(),
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
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

    // user2 participates in the space before responding to poll
    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/participate", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "verifiable_presentation": ""
        },
        response_type: ParticipateSpaceResponse
    };
    assert_eq!(status, 200, "user2 failed to participate");

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
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (user_reward_pk, user_reward_sk) =
        UserReward::keys(user2.0.pk.clone(), reward_key).expect("Should create UserReward keys");
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

/// Integration test: Full flow from membership subscription to reward configuration
/// This test covers the complete user journey:
/// 1. User creates a poll space
/// 2. User subscribes to Pro membership (gains 40 credits)
/// 3. User configures a reward for poll responses (uses 10 credits)
/// 4. Another user responds to the poll
/// 5. The responding user receives the reward
#[tokio::test]
async fn test_full_flow_membership_to_reward_configuration() {
    // Step 1: Setup a published poll space
    let (ctx, space_pk, poll_sk, _questions) = setup_published_poll_space().await;
    let TestContextV3 {
        app,
        test_user,
        ddb,
        user2,
        ..
    } = ctx;

    // Step 2: Setup payment info for membership subscription
    seed_test_user_payment(&ddb, &test_user.0.pk).await;

    // Step 3: Subscribe to Pro membership
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/me/memberships",
        headers: test_user.1.clone(),
        body: {
            "membership": "Pro",
            "currency": "USD"
        },
        response_type: serde_json::Value
    };
    assert_eq!(
        status, 200,
        "Failed to subscribe to Pro membership. Response: {:?}",
        body
    );

    // Step 4: Verify membership was upgraded and credits were added
    let user_membership =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist after subscription");

    let membership_pk: Partition = user_membership.membership_pk.clone().into();
    let membership = Membership::get(&ddb, membership_pk, Some(EntityType::Membership))
        .await
        .unwrap()
        .expect("Membership should exist");

    assert_eq!(membership.tier, MembershipTier::Pro);
    assert_eq!(
        user_membership.remaining_credits, 40,
        "Pro membership should have 40 credits"
    );

    // Step 5: Create a reward for poll responses
    let (status, _headers, reward_body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
            "description": "Earn points for responding to our poll",
            "credits": 10
        },
        response_type: SpaceRewardResponse
    };
    assert_eq!(
        status, 200,
        "Failed to create reward. Response: {:?}",
        reward_body
    );
    assert_eq!(
        reward_body.description,
        "Earn points for responding to our poll"
    );
    assert_eq!(reward_body.credits, 10);
    assert_eq!(reward_body.points, 10_000); // Default points per reward

    // Step 6: Verify credits were deducted after reward creation
    let user_membership_after =
        UserMembership::get(&ddb, &test_user.0.pk, Some(EntityType::UserMembership))
            .await
            .unwrap()
            .expect("UserMembership should exist");
    assert_eq!(
        user_membership_after.remaining_credits, 30,
        "Credits should be 30 after creating reward (40 - 10)"
    );

    // Step 7: Another user (user2) participates and responds to the poll
    // user2 participates in the space before responding to poll
    let (status, _headers, _res) = post! {
        app: app,
        path: format!("/v3/spaces/{}/participate", space_pk.to_string()),
        headers: user2.1.clone(),
        body: {
            "verifiable_presentation": ""
        },
        response_type: ParticipateSpaceResponse
    };
    assert_eq!(status, 200, "user2 failed to participate");

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

    let (status, _headers, _response_body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/polls/{}/responses", space_pk.to_string(), poll_sk.to_string()),
        headers: user2.1.clone(),
        body: {
            "answers": answers.clone(),
        },
        response_type: RespondPollSpaceResponse
    };
    assert_eq!(status, 200, "Failed to respond to poll");

    // Step 8: Verify user2's reward claim was recorded
    let (status, _headers, rewards_list) = get! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: user2.1.clone(),
        response_type: ListItemsResponse<SpaceRewardResponse>
    };
    assert_eq!(status, 200);
    assert_eq!(rewards_list.items.len(), 1);
    assert_eq!(
        rewards_list.items[0].user_claims, 1,
        "User2 should have 1 claim after poll response"
    );

    // Step 9: Verify UserReward record was created for user2
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let (user_reward_pk, user_reward_sk) =
        UserReward::keys(user2.0.pk.clone(), reward_key).expect("Should create UserReward keys");
    let user_reward = UserReward::get(&ddb, user_reward_pk, Some(user_reward_sk))
        .await
        .unwrap()
        .expect("UserReward should exist for user2");

    assert_eq!(user_reward.total_claims, 1);
    assert_eq!(
        user_reward.total_points,
        10_000 * 10,
        "Total points should be 100,000 (10,000 points * 10 credits)"
    );

    // Step 10: Verify SpaceReward total_claims was incremented
    let reward_key = RewardKey::from((
        SpacePartition::from(space_pk.clone()),
        poll_sk.clone(),
        RewardUserBehavior::RespondPoll,
    ));
    let space_reward = SpaceReward::get(&ddb, space_pk.clone(), Some(reward_key))
        .await
        .unwrap()
        .expect("SpaceReward should exist");
    assert_eq!(
        space_reward.total_claims, 1,
        "SpaceReward total_claims should be 1"
    );
}

/// Integration test: Free user cannot create rewards (no credits)
#[tokio::test]
async fn test_free_user_cannot_create_rewards() {
    let (ctx, space_pk, poll_sk, _questions) = setup_published_poll_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Try to create reward without membership (Free tier has 0 credits)
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/rewards", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "action_key": poll_sk.to_string(),
            "behavior": "RespondPoll",
            "description": "No credits",
            "credits": 10
        },
        response_type: serde_json::Value
    };
    assert_eq!(
        status, 400,
        "Free user should not be able to create rewards. Response: {:?}",
        body
    );
}
