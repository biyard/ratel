use crate::features::spaces::polls::{Poll, PollResponse, PollUserAnswer};
use crate::features::spaces::rewards::{
    RewardAction, RewardCondition, RewardKey, RewardPeriod, RewardUserBehavior, SpaceReward,
    UserReward, UserRewardHistory, UserRewardHistoryKey, UserRewardHistoryQueryOption,
};
use crate::types::*;
use crate::utils::time::get_now_timestamp_millis;
use crate::{
    models::{feed::Post, space::SpaceCommon},
    tests::{create_test_user, get_test_aws_config},
    types::{Answer, ChoiceQuestion, EntityType, Question},
    utils::aws::DynamoClient,
};

// Helper function to create test space
async fn create_test_space(
    cli: &aws_sdk_dynamodb::Client,
    user: &crate::models::user::User,
) -> SpaceCommon {
    let post = Post::new(
        "This is a test post".to_string(),
        "Content of the post".to_string(),
        crate::types::PostType::Post,
        user.clone(),
    );

    let common = SpaceCommon::new(post.clone());
    common
        .create(cli)
        .await
        .expect("failed to create space common");
    common
}

// Helper function to create test poll
async fn create_test_poll(cli: &aws_sdk_dynamodb::Client, space: &SpaceCommon) -> Poll {
    let space_id = match space.pk.clone() {
        crate::types::Partition::Space(id) => id,
        _ => panic!("space pk must be Partition::Space"),
    };

    let questions = vec![Question::SingleChoice(ChoiceQuestion {
        title: "Test Question".to_string(),
        description: Some("Choose one option".to_string()),
        image_url: None,
        options: vec!["A".to_string(), "B".to_string()],
        is_required: Some(true),
        allow_other: None,
    })];

    let sk = EntityType::SpacePoll(space_id);
    let poll = Poll::new(space.pk.clone(), Some(sk.clone()))
        .unwrap()
        .with_questions(questions);
    poll.create(cli).await.expect("failed to create poll");
    poll
}

#[tokio::test]
async fn test_reward_period_daily() {
    let aws_config = get_test_aws_config();
    let cli = DynamoClient::mock(aws_config.clone()).client;
    let user = create_test_user(&cli).await;
    let space = create_test_space(&cli, &user).await;
    let poll = create_test_poll(&cli, &space).await;
    let biyard = crate::services::biyard::Biyard::new();
    let credits = 1;
    let point = 10_000;
    let period = RewardPeriod::Daily;
    let condition = RewardCondition::None;

    // Create a reward with Daily period
    let reward = SpaceReward::new(
        space.pk.clone().into(),
        poll.sk.clone(),
        RewardUserBehavior::RespondPoll,
        "Can be claimed once per day".to_string(),
        credits,
        point,
        period,
        condition,
    );
    reward.create(&cli).await.unwrap();

    // First award on same day - should succeed
    let result1 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result1.is_ok(), "First award should succeed");
    let user_reward1 = result1.unwrap();
    assert_eq!(user_reward1.total_claims, 1);
    assert_eq!(user_reward1.total_points, reward.get_amount());

    // Second award on same day - should fail (duplicate)
    let result2 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(
        result2.is_err(),
        "Second award on same day should fail due to duplicate"
    );

    // Simulate next day by modifying the time_key check
    // Note: In real scenario, you would wait for next day or mock time
    // For this test, we verify the history was created with today's time_key
    let now = get_now_timestamp_millis();
    let today_key = RewardPeriod::Daily.to_time_key(now);

    let history = UserRewardHistory::new(user.pk.clone(), reward.clone());
    let history_pk = history.pk.clone();
    let history_sk = UserRewardHistoryKey(reward.sk.clone(), today_key);

    // Verify history exists for today
    let histories = UserRewardHistory::query(
        &cli,
        history_pk.clone(),
        UserRewardHistoryQueryOption::builder().sk(history_sk.to_string()),
    )
    .await
    .unwrap();
    assert_eq!(histories.0.len(), 1, "Should have one history entry");
    assert_eq!(histories.0[0].point, reward.get_amount());
}

#[tokio::test]
async fn test_reward_condition_max_user_claims() {
    let aws_config = get_test_aws_config();
    let cli = DynamoClient::mock(aws_config.clone()).client;
    let user = create_test_user(&cli).await;
    let space = create_test_space(&cli, &user).await;
    let poll = create_test_poll(&cli, &space).await;
    let biyard = crate::services::biyard::Biyard::new();
    let credits = 1;
    let point = 10_000;
    let period = RewardPeriod::Unlimited;
    let condition = RewardCondition::MaxUserClaims(2);
    // Create a reward with MaxUserClaims(2) - user can claim only 2 times
    let reward = SpaceReward::new(
        space.pk.clone().into(),
        poll.sk.clone(),
        RewardUserBehavior::RespondPoll,
        "Can be claimed only 2 times per user".to_string(),
        credits,
        point,
        period,
        condition,
    );
    reward.create(&cli).await.unwrap();

    // First claim - should succeed
    let result1 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result1.is_ok(), "First claim should succeed");
    let user_reward1 = result1.unwrap();
    assert_eq!(user_reward1.total_claims, 1);
    assert_eq!(user_reward1.total_points, reward.get_amount());

    // Second claim - should succeed
    let result2 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result2.is_ok(), "Second claim should succeed");
    let user_reward2 = result2.unwrap();
    assert_eq!(user_reward2.total_claims, 2);
    assert_eq!(user_reward2.total_points, reward.get_amount() * 2);

    // Third claim - should fail (MaxUserClaims reached)
    let result3 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result3.is_err(), "Third claim should fail");
    match result3 {
        Err(crate::Error::SpaceRewardMaxUserClaimsReached) => {
            // Expected error
        }
        _ => panic!("Expected SpaceRewardMaxUserClaimsReached error"),
    }

    // Verify UserReward still has only 2 claims
    let (pk, sk) = UserReward::keys(user.pk.clone(), reward.sk.clone()).unwrap();
    let final_user_reward = UserReward::get(&cli, pk, Some(sk)).await.unwrap().unwrap();
    assert_eq!(final_user_reward.total_claims, 2);
    assert_eq!(final_user_reward.total_points, reward.get_amount() * 2);
}

#[tokio::test]
async fn test_user_reward_award_flow() {
    let aws_config = get_test_aws_config();
    let cli = DynamoClient::mock(aws_config.clone()).client;
    let user = create_test_user(&cli).await;
    let space = create_test_space(&cli, &user).await;
    let poll = create_test_poll(&cli, &space).await;
    let biyard = crate::services::biyard::Biyard::new();

    let credits = 2;
    let point = 10_000;
    let period = RewardPeriod::Unlimited;
    let condition = RewardCondition::None;
    // Create a SpaceReward
    let reward = SpaceReward::new(
        space.pk.clone().into(),
        poll.sk.clone(),
        RewardUserBehavior::RespondPoll,
        "Testing full award flow".to_string(),
        credits,
        point,
        period,
        condition,
    );
    reward.create(&cli).await.unwrap();

    let initial_reward_point = reward.get_amount();

    // Call UserReward::award()
    let result = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result.is_ok(), "Award should succeed");
    let user_reward = result.unwrap();

    // 1. Verify UserReward was created
    assert_eq!(user_reward.total_claims, 1);
    assert_eq!(user_reward.total_points, initial_reward_point);

    // 2. Verify UserReward persisted in DB
    let (ur_pk, ur_sk) = UserReward::keys(user.pk.clone(), reward.sk.clone()).unwrap();
    let fetched_user_reward = UserReward::get(&cli, ur_pk.clone(), Some(ur_sk.clone()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(fetched_user_reward.total_claims, 1);
    assert_eq!(fetched_user_reward.total_points, initial_reward_point);

    // 3. Verify SpaceReward total_claims and total_points increased
    let sr_pk: Partition = space.pk.clone().into();
    let sr_sk = reward.sk.clone();
    let updated_space_reward = SpaceReward::get(&cli, sr_pk, Some(sr_sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_space_reward.total_claims, 1);
    assert_eq!(updated_space_reward.total_points, initial_reward_point);

    // 4. Test second award to verify incremental updates
    let result2 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result2.is_ok(), "Second award should succeed");
    let user_reward2 = result2.unwrap();
    assert_eq!(user_reward2.total_claims, 2);
    assert_eq!(user_reward2.total_points, initial_reward_point * 2);
}

#[tokio::test]
async fn test_biyard_transaction_rollback_on_duplicate() {
    let aws_config = get_test_aws_config();
    let cli = DynamoClient::mock(aws_config.clone()).client;
    let user = create_test_user(&cli).await;
    let space = create_test_space(&cli, &user).await;
    let poll = create_test_poll(&cli, &space).await;
    let biyard = crate::services::biyard::Biyard::new();

    let credits = 1;
    let point = 10_000;
    let period = RewardPeriod::Once;
    let condition = RewardCondition::None;
    // Create a reward with Once period (can only be claimed once)
    let reward = SpaceReward::new(
        space.pk.clone().into(),
        poll.sk.clone(),
        RewardUserBehavior::RespondPoll,
        "Can only be claimed once".to_string(),
        credits,
        point,
        period,
        condition,
    );
    reward.create(&cli).await.unwrap();

    // First award - should succeed and Biyard points awarded
    let result1 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(result1.is_ok(), "First award should succeed");

    // Verify first history was created
    let history = UserRewardHistory::new(user.pk.clone(), reward.clone());
    let history_pk = history.pk.clone();
    let history_sk = history.sk.clone();
    let opt = UserRewardHistoryQueryOption::builder().sk(history_sk.to_string());
    let histories1 = UserRewardHistory::query(&cli, history_pk.clone(), opt)
        .await
        .unwrap();
    assert_eq!(
        histories1.0.len(),
        1,
        "Should have one history entry after first award"
    );

    // Second award - should fail because of Once period (duplicate history)
    // The transaction will fail, and Biyard points should be rolled back
    let result2 = UserReward::award(&cli, &biyard, reward.clone(), user.pk.clone(), None).await;
    assert!(
        result2.is_err(),
        "Second award should fail due to duplicate history"
    );
    match result2 {
        Err(crate::Error::SpaceRewardAlreadyClaimedInPeriod) => {
            // Expected error when transaction fails
        }
        _ => panic!("Expected SpaceRewardAlreadyClaimedInPeriod error"),
    }
    let opt = UserRewardHistoryQueryOption::builder().sk(history_sk.to_string());

    // Verify no additional history was created
    let histories2 = UserRewardHistory::query(&cli, history_pk.clone(), opt)
        .await
        .unwrap();
    assert_eq!(
        histories2.0.len(),
        1,
        "Should still have only one history entry after failed second award"
    );

    // Verify UserReward still shows only 1 claim
    let (ur_pk, ur_sk) = UserReward::keys(user.pk.clone(), reward.sk.clone()).unwrap();
    let user_reward = UserReward::get(&cli, ur_pk, Some(ur_sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(user_reward.total_claims, 1);
    assert_eq!(user_reward.total_points, reward.get_amount());

    // Verify SpaceReward still shows only 1 claim
    let sr_pk: Partition = space.pk.clone().into();
    let sr_sk = reward.sk.clone();
    let space_reward = SpaceReward::get(&cli, sr_pk, Some(sr_sk))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(space_reward.total_claims, 1);
    assert_eq!(space_reward.total_points, reward.get_amount());

    // Note: In the real implementation, Biyard service receives a rollback call
    // with negative points (-reward.point) to revert the transaction.
    // In this noop implementation, we verify the data consistency in DynamoDB.
}
