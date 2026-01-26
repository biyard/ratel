use crate::{tests::v3_setup::TestContextV3, *};

#[tokio::test]
async fn test_get_team_rewards_when_member() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Get team PK (using hiteam which test_user should be a member of)
    let team_pk = "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61";

    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/teams/{}/points", team_pk),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(status, 200, "Team member should be able to view rewards");
}

#[tokio::test]
async fn test_list_team_point_transactions_when_member() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // Get team PK (using hiteam which test_user should be a member of)
    let team_pk = "TEAM#d4004e2b-093a-41cc-9f21-b8fb3e5e9f61";

    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/teams/{}/points/transactions", team_pk),
        headers: test_user.1.clone(),
        response_type: serde_json::Value
    };

    assert_eq!(
        status, 200,
        "Team member should be able to view transactions"
    );

    // Verify response structure
    assert!(
        body.get("items").is_some(),
        "Response should have items field"
    );
}
