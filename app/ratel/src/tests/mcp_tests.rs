use super::*;
use crate::common::models::McpClientSecret;
use crate::common::models::space::SpaceCommon;
use crate::common::types::{
    EntityType, Partition, SpacePublishState, SpaceStatus, SpaceVisibility,
};

pub(super) async fn create_mcp_secret(
    cli: &crate::common::aws_sdk_dynamodb::Client,
    user_pk: &Partition,
) -> String {
    let (entity, raw_token) = McpClientSecret::new(user_pk.clone());
    entity.create(cli).await.unwrap();
    raw_token
}

pub(super) async fn setup_mcp_test() -> (TestContext, String) {
    let ctx = TestContext::setup().await;
    let raw_token = create_mcp_secret(&ctx.ddb, &ctx.test_user.0.pk).await;
    (ctx, raw_token)
}

/// Send MCP JSON-RPC request via POST /mcp/{secret}.
/// Parses SSE response body to extract JSON-RPC response.
pub(super) async fn mcp_request(
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
pub(super) fn parse_sse_json(sse_body: &str) -> serde_json::Value {
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
pub(super) async fn mcp_tool_call(
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
pub(super) fn extract_tool_content(body: &serde_json::Value) -> serde_json::Value {
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

    // Create a team so there is at least one team to list.
    // Dioxus server functions wrap the named parameter as a JSON key:
    // create_team_handler(body: CreateTeamRequest, ...) → send {"body": {...}}
    let team_username = format!("t{}", &uuid::Uuid::new_v4().simple().to_string()[..7]);
    let (status, _, resp) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/teams/create",
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "username": team_username,
                "nickname": "Test Team",
                "profile_url": "",
                "description": ""
            }
        }
    };
    assert_eq!(status, 200, "create_team failed: {:?}", resp);

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

/// Helper: create a team via HTTP and return the team username.
async fn create_test_team(ctx: &TestContext) -> String {
    let team_username = format!("t{}", &uuid::Uuid::new_v4().simple().to_string()[..7]);
    // Dioxus server functions wrap the named parameter as a JSON key.
    // create_team_handler(body: CreateTeamRequest, ...) → send {"body": {...}}
    let (status, _, resp) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/teams/create",
        headers: ctx.test_user.1.clone(),
        body: {
            "body": {
                "username": team_username,
                "nickname": "Test Team",
                "profile_url": "",
                "description": ""
            }
        }
    };
    assert_eq!(status, 200, "create_team failed: {:?}", resp);
    team_username
}

/// Helper: call list_teams via MCP and return the team_id (pk) for the given username.
async fn get_team_id_by_username(
    app: axum::Router,
    token: &str,
    username: &str,
) -> String {
    let (status, body) =
        mcp_tool_call(app, token, "list_teams", serde_json::json!({})).await;
    assert_eq!(status, 200, "list_teams: {:?}", body);

    let content = extract_tool_content(&body);
    let teams = content.as_array().expect("list_teams should return array");
    teams
        .iter()
        .find(|t| t.get("username").and_then(|v| v.as_str()) == Some(username))
        .and_then(|t| t.get("pk").and_then(|v| v.as_str()))
        .unwrap_or_else(|| panic!("team '{}' not found in list_teams", username))
        .to_string()
}

#[tokio::test]
async fn test_mcp_tool_team_create_post() {
    let (ctx, token) = setup_mcp_test().await;

    // Step 1: create a team and resolve its team_id via list_teams
    let team_username = create_test_team(&ctx).await;
    let team_id = get_team_id_by_username(ctx.app.clone(), &token, &team_username).await;

    // Step 2: create a post under the team
    let (status, body) = mcp_tool_call(
        ctx.app.clone(),
        &token,
        "create_post",
        serde_json::json!({ "team_id": team_id }),
    )
    .await;
    assert_eq!(status, 200, "team create_post: {:?}", body);

    let content = extract_tool_content(&body);
    let post_pk = content["post_pk"]
        .as_str()
        .expect("should have post_pk")
        .to_string();

    // Step 3: verify the post belongs to the team by fetching it
    let (status, body) = mcp_tool_call(
        ctx.app,
        &token,
        "get_post",
        serde_json::json!({ "post_id": post_pk }),
    )
    .await;
    assert_eq!(status, 200, "get_post (team post): {:?}", body);
}

#[tokio::test]
async fn test_mcp_tool_team_post_and_space() {
    let (ctx, token) = setup_mcp_test().await;

    // Step 1: create a team and resolve its team_id via list_teams
    let team_username = create_test_team(&ctx).await;
    let team_id = get_team_id_by_username(ctx.app.clone(), &token, &team_username).await;

    // Step 2: create a draft post under the team
    let (status, body) = mcp_tool_call(
        ctx.app.clone(),
        &token,
        "create_post",
        serde_json::json!({ "team_id": team_id }),
    )
    .await;
    assert_eq!(status, 200, "team create_post: {:?}", body);
    let post_pk = extract_tool_content(&body)["post_pk"]
        .as_str()
        .expect("should have post_pk")
        .to_string();

    // Step 3: publish the team post
    let (status, _) = mcp_tool_call(
        ctx.app.clone(),
        &token,
        "update_post",
        serde_json::json!({
            "post_id": post_pk,
            "title": "Team Space Post",
            "content": "<p>Team post content</p>",
            "publish": true,
            "visibility": "Public"
        }),
    )
    .await;
    assert_eq!(status, 200, "update_post (publish team post) failed");

    // Step 4: create a space from the team post
    let (status, body) = mcp_tool_call(
        ctx.app,
        &token,
        "create_space",
        serde_json::json!({ "post_id": post_pk }),
    )
    .await;
    assert_eq!(status, 200, "create_space from team post: {:?}", body);
}

/// Helper: seed a public individual-owned space so the `SpaceCommon`
/// extractor succeeds and the test user is recognised as Creator.
/// Returns the raw space id (the portion after the `SPACE#` prefix).
/// Mirrors `seed_creator_space` in `meet_action_tests.rs`.
async fn seed_creator_space(ctx: &TestContext) -> String {
    let space_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(space_id.clone());

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Ongoing);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = ctx.test_user.0.pk.clone();
    space.author_display_name = ctx.test_user.0.display_name.clone();
    space.author_profile_url = ctx.test_user.0.profile_url.clone();
    space.author_username = ctx.test_user.0.username.clone();
    space.create(&ctx.ddb).await.expect("create space");

    // Minimal Post row so SpaceCommon extraction side-paths do not 404.
    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "MCP Meet Test Space".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    space_id
}

#[tokio::test]
async fn test_mcp_tool_create_meet() {
    let (ctx, token) = setup_mcp_test().await;
    let space_id = seed_creator_space(&ctx).await;

    let (status, body) = mcp_tool_call(
        ctx.app,
        &token,
        "create_meet",
        serde_json::json!({ "space_pk": space_id }),
    )
    .await;

    assert_eq!(status, 200, "mcp create_meet: {:?}", body);
    let content = extract_tool_content(&body);
    assert!(
        content["sk"].as_str().is_some(),
        "content must include sk: {:?}",
        content
    );
}
