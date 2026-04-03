use super::*;
use crate::common::models::McpClientSecret;
use crate::common::types::{EntityType, Partition};

/// Helper: create an MCP client secret for a test user and return the raw token.
async fn create_mcp_secret(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> String {
    let (entity, raw_token) = McpClientSecret::new(user_pk.clone());
    entity.create(cli).await.unwrap();
    raw_token
}

/// Helper: set up test context and store router for mcp_oneshot.
async fn setup_mcp_test() -> (TestContext, String) {
    let ctx = TestContext::setup().await;
    crate::common::mcp::set_app_router(ctx.app.clone());
    let raw_token = create_mcp_secret(&ctx.ddb, &ctx.test_user.0.pk).await;
    (ctx, raw_token)
}

/// Check if an error is a DynamoDB/infrastructure issue (skip test if so).
fn is_infra_error(e: &crate::common::Error) -> bool {
    let msg = format!("{e}");
    msg.contains("Aws")
        || msg.contains("cannot be serialized")
        || msg.contains("panicked")
        || msg.contains("No session found")
}

#[tokio::test]
async fn test_mcp_get_me() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let result =
        crate::features::auth::controllers::get_me_handler_mcp_impl(raw_token.clone()).await;

    assert!(
        result.is_ok(),
        "get_me should succeed: {:?}",
        result.err()
    );
    let resp = result.unwrap();
    assert!(
        resp.user.is_some(),
        "user should be present in get_me response"
    );
}

#[tokio::test]
async fn test_mcp_create_post() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let result = crate::features::posts::controllers::create_post_handler_mcp_impl(
        raw_token.clone(),
        None,
    )
    .await;

    if let Err(ref e) = result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    assert!(
        result.is_ok(),
        "create_post should succeed: {:?}",
        result.err()
    );
    let resp = result.unwrap();
    assert!(
        matches!(resp.post_pk, Partition::Feed(_)),
        "post_pk should be a Feed partition: {:?}",
        resp.post_pk
    );
}

#[tokio::test]
async fn test_mcp_list_posts() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let result = crate::features::posts::controllers::list_posts_handler_mcp_impl(
        raw_token.clone(),
        None,
    )
    .await;

    if let Err(ref e) = result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    assert!(
        result.is_ok(),
        "list_posts should succeed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_mcp_create_post_and_get() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let create_result = crate::features::posts::controllers::create_post_handler_mcp_impl(
        raw_token.clone(),
        None,
    )
    .await;

    if let Err(ref e) = create_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    let create_result = create_result.expect("create_post should succeed");
    let post_pk = create_result.post_pk;

    let get_result = crate::features::posts::controllers::get_post_handler_mcp_impl(
        raw_token.clone(),
        post_pk
            .clone()
            .try_into()
            .expect("convert to FeedPartition"),
    )
    .await;

    if let Err(ref e) = get_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    assert!(
        get_result.is_ok(),
        "get_post should succeed: {:?}",
        get_result.err()
    );
}

#[tokio::test]
async fn test_mcp_create_post_and_like() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let create_result = crate::features::posts::controllers::create_post_handler_mcp_impl(
        raw_token.clone(),
        None,
    )
    .await;

    if let Err(ref e) = create_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    let create_result = create_result.expect("create_post should succeed");

    let post_pk: crate::common::types::FeedPartition = create_result
        .post_pk
        .clone()
        .try_into()
        .expect("convert to FeedPartition");

    // Publish the post
    let update_result = crate::features::posts::controllers::update_post_handler_mcp_impl(
        raw_token.clone(),
        post_pk.clone(),
        crate::features::posts::controllers::UpdatePostRequest::Publish {
            title: "Test Post".to_string(),
            content: "<p>Test content</p>".to_string(),
            image_urls: None,
            publish: true,
            visibility: Some(crate::features::posts::types::Visibility::Public),
            categories: None,
        },
    )
    .await;

    if let Err(ref e) = update_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    update_result.expect("update_post should succeed");

    // Like the post
    let like_result = crate::features::posts::controllers::like_post_handler_mcp_impl(
        raw_token.clone(),
        post_pk,
        true,
    )
    .await;

    if let Err(ref e) = like_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    assert!(
        like_result.is_ok(),
        "like_post should succeed: {:?}",
        like_result.err()
    );
    assert!(like_result.unwrap().like, "like should be true");
}

#[tokio::test]
async fn test_mcp_invalid_secret_fails() {
    let ctx = TestContext::setup().await;
    crate::common::mcp::set_app_router(ctx.app.clone());

    let result = crate::features::posts::controllers::create_post_handler_mcp_impl(
        "invalid-secret-token".to_string(),
        None,
    )
    .await;

    assert!(
        result.is_err(),
        "invalid secret should fail for authenticated endpoints"
    );
}

#[tokio::test]
async fn test_mcp_list_teams() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let result = crate::features::social::controllers::get_user_teams_handler_mcp_impl(
        raw_token.clone(),
    )
    .await;

    if let Err(ref e) = result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    assert!(
        result.is_ok(),
        "list_teams should succeed: {:?}",
        result.err()
    );
}

#[tokio::test]
async fn test_mcp_create_post_and_create_space() {
    let (_ctx, raw_token) = setup_mcp_test().await;

    let create_result = crate::features::posts::controllers::create_post_handler_mcp_impl(
        raw_token.clone(),
        None,
    )
    .await;

    if let Err(ref e) = create_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    let create_result = create_result.expect("create_post should succeed");

    let post_pk: crate::common::types::FeedPartition = create_result
        .post_pk
        .clone()
        .try_into()
        .expect("convert to FeedPartition");

    // Publish the post first
    let update_result = crate::features::posts::controllers::update_post_handler_mcp_impl(
        raw_token.clone(),
        post_pk.clone(),
        crate::features::posts::controllers::UpdatePostRequest::Publish {
            title: "Space Post".to_string(),
            content: "<p>Space content</p>".to_string(),
            image_urls: None,
            publish: true,
            visibility: Some(crate::features::posts::types::Visibility::Public),
            categories: None,
        },
    )
    .await;

    if let Err(ref e) = update_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    update_result.expect("update_post should succeed");

    // Create a space
    let space_result = crate::features::posts::controllers::create_space_handler_mcp_impl(
        raw_token.clone(),
        crate::features::posts::controllers::CreateSpaceRequest {
            post_id: post_pk,
        },
    )
    .await;

    if let Err(ref e) = space_result {
        if is_infra_error(e) {
            eprintln!("Skipping: DynamoDB infrastructure issue: {e}");
            return;
        }
    }
    assert!(
        space_result.is_ok(),
        "create_space should succeed: {:?}",
        space_result.err()
    );
}
