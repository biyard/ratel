use crate::controllers::v3::posts::CreatePostResponse;
use crate::controllers::v3::spaces::create_space::CreateSpaceResponse;
use crate::features::spaces::reports::dto::{
    CreateReportRequest, CreateReportResponse, GetPricingChallengeRequest,
    GetPricingChallengeResponse, GetReportResponse, PublishReportResponse, SetPricingRequest,
    SetPricingResponse, UpdateReportRequest,
};
use crate::tests::v3_setup::TestContextV3;
use crate::types::{Partition, ReportPublishState};
use crate::*;

/// Helper function to setup a deliberation space for testing reports
/// Returns (TestContextV3, space_pk)
pub async fn setup_deliberation_space() -> (TestContextV3, Partition) {
    let ctx = TestContextV3::setup().await;
    let TestContextV3 { app, test_user, .. } = ctx.clone();

    // Create a post first
    let (_status, _headers, create_post_res) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        response_type: CreatePostResponse
    };

    let post_pk = create_post_res.post_pk;

    // Publish the post
    let (_status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/posts/{}", post_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Deliberation Post",
            "content": "<p>This is a deliberation post for report testing</p>",
            "publish": true
        }
    };

    // Create a deliberation space (type 1)
    let (status, _headers, create_space_res) = post! {
        app: app,
        path: "/v3/spaces",
        headers: test_user.1.clone(),
        body: {
            "space_type": 1,
            "post_pk": post_pk,
        },
        response_type: CreateSpaceResponse
    };
    assert_eq!(status, 200);

    let space_pk = create_space_res.space_pk;

    (ctx, space_pk)
}

#[tokio::test]
async fn test_create_report() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Report",
            "content": "<p>This is the report content</p>",
            "summary": "A brief summary of the report"
        },
        response_type: CreateReportResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.title, "Test Report");
    assert_eq!(body.publish_state, ReportPublishState::Draft);
}

#[tokio::test]
async fn test_create_report_without_auth() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, .. } = ctx;

    // Try to create a report without authentication
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        body: {
            "title": "Test Report",
            "content": "<p>This is the report content</p>"
        }
    };

    // Should fail with 401 or similar
    assert!(status != 200);
}

#[tokio::test]
async fn test_get_report() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report first
    let (_status, _headers, _create_body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Report",
            "content": "<p>This is the report content</p>",
            "summary": "A brief summary"
        },
        response_type: CreateReportResponse
    };

    // Get the report (using /draft endpoint for author access)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/reports/draft", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: GetReportResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.title, "Test Report");
    assert_eq!(body.content, "<p>This is the report content</p>");
    assert_eq!(body.summary, Some("A brief summary".to_string()));
    assert_eq!(body.publish_state, ReportPublishState::Draft);
}

#[tokio::test]
async fn test_get_nonexistent_report() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Try to get a report that doesn't exist (using /draft endpoint)
    let (status, _headers, _body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/reports/draft", space_pk.to_string()),
        headers: test_user.1.clone()
    };

    // Should return 404
    assert_eq!(status, 404);
}

#[tokio::test]
async fn test_update_report() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report first
    let (_status, _headers, _create_body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Original Title",
            "content": "<p>Original content</p>"
        },
        response_type: CreateReportResponse
    };

    // Update the report (using /draft endpoint)
    let (status, _headers, _body) = patch! {
        app: app,
        path: format!("/v3/spaces/{}/reports/draft", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Updated Title",
            "content": "<p>Updated content</p>",
            "summary": "New summary"
        }
    };

    assert_eq!(status, 200);

    // Verify the update (using /draft endpoint)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/reports/draft", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: GetReportResponse
    };

    assert_eq!(status, 200);
    assert_eq!(body.title, "Updated Title");
    assert_eq!(body.content, "<p>Updated content</p>");
    assert_eq!(body.summary, Some("New summary".to_string()));
}

#[tokio::test]
async fn test_get_pricing_challenge() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report first
    let (_status, _headers, _create_body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Report",
            "content": "<p>Content</p>"
        },
        response_type: CreateReportResponse
    };

    // Get pricing challenge
    let (status, _headers, body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports/pricing/challenge", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "recipient_address": "0x1234567890123456789012345678901234567890"
        },
        response_type: GetPricingChallengeResponse
    };

    assert_eq!(status, 200);
    assert!(!body.message.is_empty());
    assert!(!body.nonce.is_empty());
    assert!(body.expires_at > 0);
    assert!(body.message.contains("0x1234567890123456789012345678901234567890"));
}

#[tokio::test]
async fn test_get_pricing_challenge_invalid_address() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report first
    let (_status, _headers, _create_body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Report",
            "content": "<p>Content</p>"
        },
        response_type: CreateReportResponse
    };

    // Try with invalid address
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports/pricing/challenge", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "recipient_address": "invalid_address"
        }
    };

    // Should fail with 400 bad request
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_create_report_already_exists() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report first
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "First Report",
            "content": "<p>Content</p>"
        },
        response_type: CreateReportResponse
    };
    assert_eq!(status, 200);

    // Try to create another report for the same space
    let (status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Second Report",
            "content": "<p>Content</p>"
        }
    };

    // Should fail because report already exists
    assert!(status != 200);
}

#[tokio::test]
async fn test_report_revenue_split() {
    let (ctx, space_pk) = setup_deliberation_space().await;
    let TestContextV3 { app, test_user, .. } = ctx;

    // Create a report
    let (_status, _headers, _body) = post! {
        app: app,
        path: format!("/v3/spaces/{}/reports", space_pk.to_string()),
        headers: test_user.1.clone(),
        body: {
            "title": "Test Report",
            "content": "<p>Content</p>"
        },
        response_type: CreateReportResponse
    };

    // Get the report and check that it doesn't have revenue split yet (no price set)
    // (using /draft endpoint for author access)
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/spaces/{}/reports/draft", space_pk.to_string()),
        headers: test_user.1.clone(),
        response_type: GetReportResponse
    };

    assert_eq!(status, 200);
    assert!(body.price_dollars.is_none());
    assert!(body.revenue_split.is_none());
}
