use crate::features::spaces::rewards::{
    Reward, RewardAction, RewardCondition, RewardPeriod, RewardUserBehavior,
};
use crate::{get, post, put, tests::v3_setup::TestContextV3};
use crate::{patch, types::*};

/// Helper function to delete a reward if it exists
async fn delete_reward_if_exists(cli: &aws_sdk_dynamodb::Client, behavior: RewardUserBehavior) {
    let pk = Partition::Reward;
    if Reward::get(cli, pk.clone(), Some(behavior.clone()))
        .await
        .unwrap()
        .is_some()
    {
        Reward::delete(cli, pk, Some(behavior)).await.unwrap();
    }
}

#[tokio::test]
async fn test_create_reward() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;

    let (status, _headers, body) = post! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "behavior": "RespondPoll",
            "point": 100,
            "period": "Daily",
            "condition": "None"
        },
        response_type: Reward
    };

    assert_eq!(status, 200);
    assert_eq!(body.sk, RewardUserBehavior::RespondPoll);
    assert_eq!(body.point, 100);
    assert_eq!(body.period, RewardPeriod::Daily);

    // Cleanup
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
}

#[tokio::test]
async fn test_update_reward() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create a reward first
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;

    let reward = Reward::new(
        RewardUserBehavior::RespondPoll,
        100,
        RewardPeriod::Daily,
        RewardCondition::None,
    );
    reward.create(&ddb).await.unwrap();

    // Update with new values
    let (status, _headers, body) = patch! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "behavior": "RespondPoll",
            "point": 200,
            "period": "Weekly",
            "condition": {"MaxUserClaims": 10}
        },
        response_type: Reward
    };

    assert_eq!(status, 200);
    assert_eq!(body.point, 200);
    assert_eq!(body.period, RewardPeriod::Weekly);
    assert_eq!(body.condition, RewardCondition::MaxUserClaims(10));

    // Cleanup
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
}

#[tokio::test]
async fn test_create_duplicate_reward() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create a reward first
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
    let reward = Reward::new(
        RewardUserBehavior::RespondPoll,
        100,
        RewardPeriod::Daily,
        RewardCondition::None,
    );
    reward.create(&ddb).await.unwrap();

    // Try to create duplicate
    let (status, _headers, _body) = post! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "behavior": "RespondPoll",
            "point": 200,
            "period": "Weekly",
            "condition": "None"
        }
    };

    assert_eq!(status, 409); // Conflict

    // Cleanup
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
}

#[tokio::test]
async fn test_update_nonexistent_reward() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;

    // Try to update non-existent reward
    let (status, _headers, _body) = patch! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "behavior": "RespondPoll",
            "point": 200,
            "period": "Weekly",
            "condition": "None"
        }
    };

    assert_eq!(status, 404); // Not found

    // Cleanup
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
}

#[tokio::test]
async fn test_create_reward_unauthorized() {
    let TestContextV3 {
        app,
        ddb,
        test_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;

    let (status, _headers, _body) = post! {
        app: app,
        path: "/m3/rewards",
        headers: test_user.1,
        body: {
            "behavior": "RespondPoll",
            "point": 100,
            "period": "Daily",
            "condition": "None"
        }
    };

    assert_eq!(status, 401); // Middleware returns 401 for non-admin users

    // Cleanup
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
}

#[tokio::test]
async fn test_create_then_update_flow() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;

    // Create reward
    let (status1, _headers1, body1) = post! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "behavior": "RespondPoll",
            "point": 50,
            "period": "Once",
            "condition": {"MaxClaims": 100}
        },
        response_type: Reward
    };

    assert_eq!(status1, 200);
    assert_eq!(body1.point, 50);
    assert_eq!(body1.period, RewardPeriod::Once);

    // Update reward with different values
    let (status2, _headers2, body2) = patch! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "behavior": "RespondPoll",
            "point": 75,
            "period": "Daily",
            "condition": {"MaxClaims": 200}
        },
        response_type: Reward
    };

    assert_eq!(status2, 200);
    assert_eq!(body2.point, 75);
    assert_eq!(body2.period, RewardPeriod::Daily);

    // Cleanup
    delete_reward_if_exists(&ddb, RewardUserBehavior::RespondPoll).await;
}
