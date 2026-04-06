---
globs: ["app/ratel/src/common/mcp/**/*.rs", "app/ratel/src/features/**/controllers/*.rs"]
---

# MCP Tools

How to expose a server function as an MCP tool in the Ratel MCP server.

## Architecture

```
Controller fn (#[mcp_tool] + #[post/get]) → generates _mcp_impl + _mcp_handler + McpRequest struct
                                                           ↓
RatelMcpServer (#[rmcp::tool] in server.rs) → calls _mcp_handler or _mcp_impl
                                                           ↓
mcp_oneshot (oneshot.rs) → routes request through Axum router internally
```

## Step 1: Annotate the Controller

Add `#[mcp_tool]` **above** the route attribute on your controller function. Use `#[mcp(description = "...")]` on parameters.

```rust
#[mcp_tool(name = "update_poll", description = "Update a poll. Requires creator role.")]
#[post("/api/spaces/{space_pk}/polls/{poll_sk}", role: SpaceUserRole)]
pub async fn update_poll(
    #[mcp(description = "Space partition key")]
    space_pk: SpacePartition,
    #[mcp(description = "Poll sort key")]
    poll_sk: SpacePollEntityType,
    #[mcp(description = "Poll update data as JSON")]
    req: UpdatePollRequest,
) -> Result<String> {
    // implementation
}
```

This generates:
- `UpdatePollMcpRequest` struct (derives `JsonSchema` for MCP schema)
- `update_poll_mcp_impl(mcp_secret, ...)` — sends oneshot HTTP request through Axum router
- `update_poll_mcp_handler(mcp_secret, req)` — calls `_mcp_impl` and converts to `McpResult`

## Step 2: Register in RatelMcpServer

Add an `#[rmcp::tool]` method in `app/ratel/src/common/mcp/server.rs` inside the `#[tool_router] impl RatelMcpServer` block:

```rust
#[rmcp::tool(
    name = "update_poll",
    description = "Update a poll. Requires creator role."
)]
async fn update_poll(
    &self,
    Parameters(req): Parameters<crate::features::spaces::...::UpdatePollMcpRequest>,
) -> McpResult {
    crate::features::spaces::...::update_poll_mcp_handler(&self.mcp_secret, req).await
}
```

### For tools without parameters

If `#[mcp_tool]` generates no `McpRequest` struct (no params beyond user/session), the handler takes only `mcp_secret`:

```rust
async fn list_teams(&self) -> McpResult {
    crate::features::social::controllers::get_user_teams_handler_mcp_handler(&self.mcp_secret).await
}
```

### For tools with custom conversion logic

Some tools (e.g., `update_post`) need to transform MCP request fields before calling `_mcp_impl`. Define the `McpRequest` struct by hand in `server.rs` and call `_mcp_impl` directly:

```rust
#[rmcp::tool(name = "update_post", description = "Update a post.")]
async fn update_post(&self, Parameters(req): Parameters<UpdatePostMcpRequest>) -> McpResult {
    // custom transformation
    update_post_handler_mcp_impl(self.mcp_secret.clone(), req.post_id, update_req)
        .await
        .into_mcp()
}
```

## Key Types

| Type | Location | Purpose |
|------|----------|---------|
| `McpResult` | `common/mcp/server.rs` | `Result<CallToolResult, ErrorData>` |
| `IntoMcpResult` | `common/mcp/server.rs` | Converts `common::Result<T: Serialize>` to `McpResult` |
| `Parameters<T>` | `rmcp` crate | Extracts and validates MCP tool parameters |

## Naming Conventions

| Source | Generated |
|--------|-----------|
| `fn create_post_handler` | `CreatePostHandlerMcpRequest`, `create_post_handler_mcp_impl`, `create_post_handler_mcp_handler` |
| `fn update_poll` | `UpdatePollMcpRequest`, `update_poll_mcp_impl`, `update_poll_mcp_handler` |

## Parameter Classification

The `#[mcp_tool]` macro classifies params by matching against the route path:
- **Path params**: `{space_pk}` in route → `space_pk` param mapped to path
- **Query params**: `?bookmark` in route → `bookmark` param mapped to query string
- **Body params**: everything else → serialized as JSON body

## Testing

Add integration tests in `app/ratel/src/tests/mcp_tests.rs` for every new tool.

### Test Helpers

```rust
use super::*;

// Setup: creates test context + MCP client secret
let (ctx, token) = setup_mcp_test().await;

// Call a tool (stateless JSON-RPC via POST /mcp/{secret})
let (status, body) = mcp_tool_call(ctx.app.clone(), &token, "tool_name", serde_json::json!({
    "param": "value"
})).await;

// Extract parsed JSON from tool result content
let content = extract_tool_content(&body);
```

### Test Pattern

```rust
#[tokio::test]
async fn test_mcp_tool_my_tool() {
    let (ctx, token) = setup_mcp_test().await;

    // Setup: create prerequisite entities if needed
    let (_, body) =
        mcp_tool_call(ctx.app.clone(), &token, "create_post", serde_json::json!({})).await;
    let post_pk = extract_tool_content(&body)["post_pk"].as_str().unwrap().to_string();

    // Act: call the tool under test
    let (status, body) = mcp_tool_call(
        ctx.app, &token, "my_tool", serde_json::json!({ "post_id": post_pk }),
    ).await;

    // Assert
    assert_eq!(status, 200, "my_tool: {:?}", body);
    assert!(body.get("result").is_some(), "should have result: {:?}", body);
}
```

### Key Rules

- Clone `ctx.app` if calling multiple tools in one test (Router is consumed by oneshot)
- Use `extract_tool_content(&body)` to parse the JSON inside the MCP tool result
- Chain setup calls (create_post → update_post → create_space) when testing tools that require existing entities
- Test error cases with invalid secrets or missing params

## Checklist

- [ ] `#[mcp_tool(name, description)]` added above route attribute
- [ ] `#[mcp(description = "...")]` on each parameter
- [ ] `#[rmcp::tool]` method added in `server.rs` `#[tool_router] impl` block
- [ ] Tool name matches between `#[mcp_tool]` and `#[rmcp::tool]`
- [ ] Import `_mcp_handler` or `_mcp_impl` + request type in `server.rs`
- [ ] Integration test added in `mcp_tests.rs`
- [ ] Build passes: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web`
