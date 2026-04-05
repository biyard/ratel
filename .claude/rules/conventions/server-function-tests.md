---
globs: ["app/ratel/src/tests/**/*.rs"]
---

# Server Function Tests

Integration tests for server functions (controllers) live in `app/ratel/src/tests/`.

## Test Infrastructure

### TestContext

```rust
use super::*;

let ctx = TestContext::setup().await;
// ctx.app     — Axum Router (consumed by oneshot, clone for multiple calls)
// ctx.ddb     — DynamoDB client
// ctx.test_user — (User, HeaderMap) with session cookie

// Create additional users:
let (user2, headers2) = ctx.create_another_user().await;
```

### Test Macros

All macros use `tower::ServiceExt::oneshot` internally. Default response type is `serde_json::Value`.

```rust
// POST with JSON body + auth headers
let (status, headers, body) = crate::test_post! {
    app: ctx.app.clone(),
    path: "/api/posts",
    headers: ctx.test_user.1.clone(),
    body: { "title": "Test" },
};

// GET with auth headers + typed response
// Path params use SubPartition types (FeedPartition, SpacePartition, etc.)
// which allow omitting the prefix — no need to URL-encode "POST#"
let (status, _, body) = crate::test_get! {
    app: ctx.app.clone(),
    path: "/api/posts/abc",
    headers: ctx.test_user.1.clone(),
    response_type: MyResponseType,
};

// POST without auth (error case)
let (status, _, _) = crate::test_post! {
    app: ctx.app.clone(),
    path: "/api/posts",
};
```

Available macros: `test_get!`, `test_post!`, `test_put!`, `test_patch!`, `test_delete!`

### Macro Arguments (in order)

| Arg | Required | Description |
|-----|----------|-------------|
| `app` | Yes | Axum Router (clone if reused) |
| `path` | Yes | API path (SubPartition types allow omitting prefix; URL-encode `#` as `%23` only for raw Partition keys) |
| `headers` | No | `HeaderMap` with session cookie |
| `body` | No | JSON body as `{ key: value }` |
| `response_type` | No | Deserialize target (default: `serde_json::Value`) |

## Test File Organization

- Create `app/ratel/src/tests/<feature>_tests.rs` for each feature
- Register in `app/ratel/src/tests/mod.rs`: `mod <feature>_tests;`

## Test Pattern

```rust
use super::*;

#[tokio::test]
async fn test_create_and_get_my_entity() {
    let ctx = TestContext::setup().await;

    // Create
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/my-entities",
        headers: ctx.test_user.1.clone(),
        body: { "name": "Test" },
    };
    assert_eq!(status, 200, "create: {:?}", body);
    let entity_pk = body["pk"].as_str().unwrap();

    // Get — path params use SubPartition types (e.g. FeedPartition),
    // so pass the ID directly without the prefix
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/my-entities/{}", entity_pk),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "get: {:?}", body);
}

#[tokio::test]
async fn test_unauthenticated_access() {
    let ctx = TestContext::setup().await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/my-entities",
    };
    assert_ne!(status, 200, "unauthenticated request should fail");
}
```

## Key Rules

- Clone `ctx.app` when making multiple requests in one test
- Clone `ctx.test_user.1` (headers) for each request
- Path params should use SubPartition types (`FeedPartition`, `SpacePartition`, `{Name}EntityType`, etc.) which allow omitting the prefix — pass just the ID
- URL-encode `#` as `%23` only when passing raw `Partition` keys (rare — prefer SubPartition types in API DTOs and path params)
- Test both success and error cases (unauthenticated, invalid input, unauthorized role)
- `bypass` feature is required for test login (verification code `"000000"`)

## Running Tests

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features "full,bypass" -- <test_name>
```
