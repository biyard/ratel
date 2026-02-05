use crate::features::spaces::rewards::{Reward, RewardAction, RewardCondition, RewardPeriod};
use crate::types::*;
use crate::{get, put, tests::v3_setup::TestContextV3};

/// Helper function to delete a reward if it exists
async fn delete_reward_if_exists(cli: &aws_sdk_dynamodb::Client, action: RewardAction) {
    if Reward::get_by_reward_action(cli, &action).await.is_ok() {
        let pk = Partition::Reward;
        Reward::delete(cli, pk, Some(action)).await.unwrap();
    }
}

#[tokio::test]
async fn test_upsert_reward_create() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardAction::PollRespond).await;

    let (status, _headers, body) = put! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "action": "PollRespond",
            "point": 100,
            "period": "Daily",
            "condition": "None"
        },
        response_type: Reward
    };

    assert_eq!(status, 200);
    assert_eq!(body.sk, RewardAction::PollRespond);
    assert_eq!(body.point, 100);
    assert_eq!(body.period, RewardPeriod::Daily);

    // Cleanup
    delete_reward_if_exists(&ddb, RewardAction::PollRespond).await;
}

#[tokio::test]
async fn test_upsert_reward_update() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Create a reward first
    delete_reward_if_exists(&ddb, RewardAction::PollRespond).await;

    let reward = Reward::new(
        RewardAction::PollRespond,
        100,
        RewardPeriod::Daily,
        RewardCondition::None,
    );
    reward.create(&ddb).await.unwrap();

    // Upsert (update) with new values
    let (status, _headers, body) = put! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "action": "PollRespond",
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
    delete_reward_if_exists(&ddb, RewardAction::PollRespond).await;
}

#[tokio::test]
async fn test_upsert_reward_unauthorized() {
    let TestContextV3 {
        app,
        ddb,
        test_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardAction::PollRespond).await;

    let (status, _headers, _body) = put! {
        app: app,
        path: "/m3/rewards",
        headers: test_user.1,
        body: {
            "action": "PollRespond",
            "point": 100,
            "period": "Daily",
            "condition": "None"
        }
    };

    assert_eq!(status, 401); // Middleware returns 401 for non-admin users

    // Cleanup
    delete_reward_if_exists(&ddb, RewardAction::PollRespond).await;
}

#[tokio::test]
async fn test_upsert_reward_idempotent() {
    let TestContextV3 {
        app,
        ddb,
        admin_user,
        ..
    } = TestContextV3::setup().await;

    // Ensure reward doesn't exist
    delete_reward_if_exists(&ddb, RewardAction::None).await;

    // First upsert
    let (status1, _headers1, body1) = put! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "action": "None",
            "point": 50,
            "period": "Once",
            "condition": {"MaxClaims": 100}
        },
        response_type: Reward
    };

    assert_eq!(status1, 200);
    assert_eq!(body1.point, 50);

    // Second upsert with same values (idempotent)
    let (status2, _headers2, body2) = put! {
        app: app,
        path: "/m3/rewards",
        headers: admin_user.1.clone(),
        body: {
            "action": "None",
            "point": 50,
            "period": "Once",
            "condition": {"MaxClaims": 100}
        },
        response_type: Reward
    };

    assert_eq!(status2, 200);
    assert_eq!(body2.point, 50);
    assert_eq!(body2.period, RewardPeriod::Once);

    // Cleanup
    delete_reward_if_exists(&ddb, RewardAction::None).await;
}
