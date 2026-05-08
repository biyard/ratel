use super::*;

use crate::common::models::reward::PendingReward;
use crate::common::services::pending_reward_retry::retry_pending_rewards;
use crate::common::types::{
    Partition, PendingRewardStatus, RewardKey, RewardUserBehavior, SpacePartition,
};

fn sample_reward_key(uid_seed: &str) -> RewardKey {
    RewardKey::from((
        SpacePartition(format!("space-{uid_seed}")),
        format!("action-{uid_seed}"),
        RewardUserBehavior::RespondPoll,
    ))
}

async fn fetch_one(
    ddb: &aws_sdk_dynamodb::Client,
    pending: &PendingReward,
) -> Option<PendingReward> {
    PendingReward::get(ddb, pending.pk.clone(), Some(pending.sk.clone()))
        .await
        .expect("get_item failed")
}

#[tokio::test]
async fn retry_triggers_award_call_for_pending_rows() {
    let ctx = TestContext::setup().await;

    let uid = uuid::Uuid::new_v4().to_string();
    let pending = PendingReward::new(
        &Partition::User(uid.clone()),
        &Partition::Space(format!("space-{uid}")),
        &sample_reward_key(&uid),
        20_000,
        "smoke",
        None,
    );
    pending.create(&ctx.ddb).await.expect("create failed");

    let stats = retry_pending_rewards(&ctx.ddb).await.expect("retry failed");

    assert!(stats.scanned >= 1, "should have scanned: {stats:?}");
    let in_response = stats
        .still_pending_rows
        .iter()
        .any(|r| r.target_pk.contains(&uid));
    assert!(
        in_response,
        "biyard call should have been triggered + failed (row in still_pending_rows): {stats:?}",
    );

    let after = fetch_one(&ctx.ddb, &pending)
        .await
        .expect("row should still exist after retry");
    assert_eq!(after.retry_count, 1, "retry_count should bump 0 → 1");
    assert!(!after.last_error.is_empty(), "last_error must record reason");
    assert_eq!(after.status, PendingRewardStatus::Pending);
}

#[tokio::test]
async fn retry_keeps_high_retry_count_rows_pending() {
    // No max-retry / Failed status — manual operator decides when to stop.
    // A row with retry_count=10 still gets re-tried and bumps to 11.
    let ctx = TestContext::setup().await;

    let uid = uuid::Uuid::new_v4().to_string();
    let mut pending = PendingReward::new(
        &Partition::User(uid.clone()),
        &Partition::Space(format!("space-{uid}")),
        &sample_reward_key(&uid),
        50_000,
        "high-retry",
        None,
    );
    pending.retry_count = 10;
    pending.create(&ctx.ddb).await.expect("create failed");

    let _ = retry_pending_rewards(&ctx.ddb).await.expect("retry failed");

    let after = fetch_one(&ctx.ddb, &pending).await.expect("row missing");
    assert_eq!(after.status, PendingRewardStatus::Pending);
    assert_eq!(after.retry_count, 11, "retry_count should keep incrementing");
}

#[tokio::test]
async fn retry_does_not_pick_completed_rows() {
    let ctx = TestContext::setup().await;

    let uid = uuid::Uuid::new_v4().to_string();
    let mut pending = PendingReward::new(
        &Partition::User(uid.clone()),
        &Partition::Space(format!("space-{uid}")),
        &sample_reward_key(&uid),
        10_000,
        "already-done",
        None,
    );
    pending.status = PendingRewardStatus::Completed;
    pending.create(&ctx.ddb).await.expect("create failed");

    let _ = retry_pending_rewards(&ctx.ddb).await.expect("retry failed");

    let after = fetch_one(&ctx.ddb, &pending).await.expect("row missing");
    assert_eq!(after.status, PendingRewardStatus::Completed);
    assert_eq!(after.retry_count, 0);
}
