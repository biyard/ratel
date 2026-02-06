use crate::controllers::v3::rewards::list_rewards::ListRewardsResponse;
use crate::controllers::v3::spaces::CreateSpaceResponse;
use crate::features::spaces::discussions::dto::{
    CreateDiscussionResponse, DeleteDiscussionResponse, GetDiscussionResponse,
    ListDiscussionResponse, SpaceDiscussionMemberResponse,
};
use crate::features::spaces::rewards::{RewardAction, RewardUserBehavior};
use crate::types::{Partition, SpaceType};
use crate::*;
use crate::{
    controllers::v3::posts::CreatePostResponse,
    tests::v3_setup::{TestContextV3, setup_v3},
};
use axum::AxumRouter;

#[tokio::test]
async fn test_list_rewards() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/rewards",
        response_type: ListRewardsResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1, "Should have 1 reward type");

    // Check that all reward types exist
    let poll_respond = body
        .items
        .iter()
        .find(|r| r.reward_behavior == RewardUserBehavior::RespondPoll);
    assert!(
        poll_respond.is_some(),
        "PollRespond reward type should exist"
    );
    assert_eq!(poll_respond.unwrap().point, 10_000);
}

#[tokio::test]
async fn test_list_rewards_filtered_by_poll() {
    let TestContextV3 { app, .. } = TestContextV3::setup().await;

    let (status, _headers, body) = get! {
        app: app,
        path: "/v3/rewards?action=poll",
        response_type: ListRewardsResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.items.len(), 1, "Should have 1 poll reward type");
    assert_eq!(
        body.items[0].reward_behavior,
        RewardUserBehavior::RespondPoll
    );
}
