use super::*;
use crate::common::models::McpClientSecret;
use crate::common::types::Partition;

async fn create_mcp_secret(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> String {
    let (entity, raw_token) = McpClientSecret::new(user_pk.clone());
    entity.create(cli).await.unwrap();
    raw_token
}

async fn setup_mcp_test() -> (TestContext, String) {
    let ctx = TestContext::setup().await;
    let raw_token = create_mcp_secret(&ctx.ddb, &ctx.test_user.0.pk).await;
    (ctx, raw_token)
}

/// Send MCP JSON-RPC request via POST /mcp/{secret}.
/// Parses SSE response body to extract JSON-RPC response.
async fn mcp_request(
    app: axum::Router,
    secret: &str,
    jsonrpc_body: serde_json::Value,
) -> (axum::http::StatusCode, serde_json::Value) {
    let path = format!("/mcp/{}", secret);
    let body_bytes = serde_json::to_vec(&jsonrpc_body).unwrap();

    let req = axum::http::Request::builder()
        .uri(format!("http://localhost:8080{}", path))
        .method("POST")
        .header("content-type", "application/json")
        .header("accept", "application/json, text/event-stream")
        .body(axum::body::Body::from(body_bytes))
        .unwrap();

    let res: axum::http::Response<axum::body::Body> =
        tower::ServiceExt::oneshot(app, req).await.unwrap();

    let (parts, body) = res.into_parts();
    let bytes = axum::body::to_bytes(body, 10 * 1024 * 1024)
        .await
        .unwrap()
        .to_vec();
    let body_str = String::from_utf8(bytes).unwrap();

    (parts.status, parse_sse_json(&body_str))
}

/// Parse SSE body → JSON-RPC response. Handles both plain JSON and SSE format.
fn parse_sse_json(sse_body: &str) -> serde_json::Value {
    if let Ok(v) = serde_json::from_str::<serde_json::Value>(sse_body) {
        return v;
    }
    for line in sse_body.lines() {
        if let Some(data) = line.trim().strip_prefix("data:") {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(data.trim()) {
                return v;
            }
        }
    }
    serde_json::json!({ "_raw": sse_body })
}

/// Call an MCP tool via JSON-RPC (stateless mode — no session needed).
async fn mcp_tool_call(
    app: axum::Router,
    secret: &str,
    tool_name: &str,
    arguments: serde_json::Value,
) -> (axum::http::StatusCode, serde_json::Value) {
    mcp_request(
        app,
        secret,
        serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "tools/call",
            "params": { "name": tool_name, "arguments": arguments }
        }),
    )
    .await
}

/// Extract parsed JSON from MCP tool result content.
fn extract_tool_content(body: &serde_json::Value) -> serde_json::Value {
    let text = body
        .pointer("/result/content/0/text")
        .and_then(|v| v.as_str())
        .unwrap_or("{}");
    serde_json::from_str(text).unwrap_or_default()
}

#[tokio::test]
async fn test_mcp_tool_get_me() {
    let (ctx, token) = setup_mcp_test().await;
    let (status, body) = mcp_tool_call(ctx.app, &token, "get_me", serde_json::json!({})).await;
    assert_eq!(status, 200, "get_me: {:?}", body);
    assert!(body.get("result").is_some(), "should have result: {:?}", body);
}

#[tokio::test]
async fn test_mcp_tool_create_post() {
    let (ctx, token) = setup_mcp_test().await;
    let (status, body) =
        mcp_tool_call(ctx.app, &token, "create_post", serde_json::json!({})).await;
    assert_eq!(status, 200, "create_post: {:?}", body);
    let content = extract_tool_content(&body);
    assert!(content.get("post_pk").is_some(), "should have post_pk: {:?}", content);
}

#[tokio::test]
async fn test_mcp_tool_list_teams() {
    let (ctx, token) = setup_mcp_test().await;

    // Create a team so there is at least one team to list
    let team_username = format!("t{}", &uuid::Uuid::new_v4().simple().to_string()[..7]);
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/teams/create",
        headers: ctx.test_user.1.clone(),
        body: {
            "username": team_username,
            "nickname": "Test Team",
            "profile_url": "",
            "description": ""
        }
    };
    assert_eq!(status, 200, "create_team failed");

    // list_teams via MCP must return the newly created team
    let (status, body) =
        mcp_tool_call(ctx.app, &token, "list_teams", serde_json::json!({})).await;
    assert_eq!(status, 200, "list_teams: {:?}", body);

    let content = extract_tool_content(&body);
    let teams = content.as_array().expect("list_teams should return an array");
    assert!(
        !teams.is_empty(),
        "list_teams should return at least one team after creation: {:?}",
        body
    );
    let found = teams
        .iter()
        .any(|t| t.get("username").and_then(|v| v.as_str()) == Some(&team_username));
    assert!(found, "created team '{}' not found in list_teams result: {:?}", team_username, teams);
}

#[tokio::test]
async fn test_mcp_tool_invalid_secret() {
    let ctx = TestContext::setup().await;
    let (status, body) =
        mcp_tool_call(ctx.app, "invalid-secret", "create_post", serde_json::json!({})).await;

    // Tool call should produce an error (either MCP-level or tool-level)
    let has_error = body.get("error").is_some()
        || body.pointer("/result/isError").and_then(|v| v.as_bool()).unwrap_or(false)
        || status != 200;
    assert!(has_error, "invalid secret should error: status={}, body={:?}", status, body);
}

#[tokio::test]
async fn test_mcp_tool_create_and_get_post() {
    let (ctx, token) = setup_mcp_test().await;

    let (_, body) =
        mcp_tool_call(ctx.app.clone(), &token, "create_post", serde_json::json!({})).await;
    let post_pk = extract_tool_content(&body)["post_pk"].as_str().unwrap().to_string();

    let (status, body) = mcp_tool_call(
        ctx.app, &token, "get_post", serde_json::json!({ "post_id": post_pk }),
    ).await;
    assert_eq!(status, 200, "get_post: {:?}", body);
}

#[tokio::test]
async fn test_mcp_tool_create_publish_like() {
    let (ctx, token) = setup_mcp_test().await;

    let (_, body) =
        mcp_tool_call(ctx.app.clone(), &token, "create_post", serde_json::json!({})).await;
    let post_pk = extract_tool_content(&body)["post_pk"].as_str().unwrap().to_string();

    let (status, _) = mcp_tool_call(ctx.app.clone(), &token, "update_post", serde_json::json!({
        "post_id": post_pk, "title": "Test", "content": "<p>Hi</p>",
        "publish": true, "visibility": "Public"
    })).await;
    assert_eq!(status, 200);

    let (status, body) = mcp_tool_call(
        ctx.app, &token, "like_post", serde_json::json!({ "post_id": post_pk, "like": true }),
    ).await;
    assert_eq!(status, 200, "like_post: {:?}", body);
}

#[tokio::test]
async fn test_mcp_tool_delete_post() {
    let (ctx, token) = setup_mcp_test().await;

    let (_, body) =
        mcp_tool_call(ctx.app.clone(), &token, "create_post", serde_json::json!({})).await;
    let post_pk = extract_tool_content(&body)["post_pk"].as_str().unwrap().to_string();

    let (status, body) = mcp_tool_call(
        ctx.app, &token, "delete_post", serde_json::json!({ "post_id": post_pk }),
    ).await;
    assert_eq!(status, 200, "delete_post: {:?}", body);
}

#[tokio::test]
async fn test_mcp_tool_create_space() {
    let (ctx, token) = setup_mcp_test().await;

    let (_, body) =
        mcp_tool_call(ctx.app.clone(), &token, "create_post", serde_json::json!({})).await;
    let post_pk = extract_tool_content(&body)["post_pk"].as_str().unwrap().to_string();

    let (status, _) = mcp_tool_call(ctx.app.clone(), &token, "update_post", serde_json::json!({
        "post_id": post_pk, "title": "Space Post", "content": "<p>Space</p>",
        "publish": true, "visibility": "Public"
    })).await;
    assert_eq!(status, 200);

    let (status, body) = mcp_tool_call(
        ctx.app, &token, "create_space", serde_json::json!({ "post_id": post_pk }),
    ).await;
    assert_eq!(status, 200, "create_space: {:?}", body);
}
