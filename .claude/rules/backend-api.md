---
globs: ["packages/main-api/**/*.rs"]
---

# Backend API Rules

Rules for working with the main-api Axum backend in `packages/main-api/`.

## Test Macros

Tests use custom HTTP macros from `src/tests/macros.rs`. Test files are `tests.rs` inside controller modules.

```rust
#[tokio::test]
async fn test_example() {
    let TestContextV3 { app, test_user, .. } = TestContextV3::setup().await;

    // GET with auth
    let (status, _headers, body) = get! {
        app: app,
        path: format!("/v3/posts/{}", pk),
        headers: test_user.1.clone(),
        response_type: PostDetailResponse
    };
    assert_eq!(status, 200);

    // POST with body
    let (status, _headers, body) = post! {
        app: app,
        path: "/v3/posts",
        headers: test_user.1.clone(),
        body: { "title": "Test", "content": "<p>Content</p>" },
        response_type: CreatePostResponse
    };
}
```

Available macros: `get!`, `post!`, `patch!`, `put!`, `delete!`
Parameters: `app:`, `path:`, `headers:` (optional), `body:` (optional), `response_type:` (optional, defaults to `serde_json::Value`)
Returns: `(StatusCode, HeaderMap, ResponseBody)`

## TestContextV3 Fields

- `app` — Axum app instance
- `test_user` — `(User, HeaderMap)` for authenticated requests
- `now` — current timestamp
- `ddb` — DynamoDB client

## Test Best Practices

- Test with and without authentication
- Cover success, error, and permission cases
- Always run `make test` before committing

## Server-Side Pagination

- Add query parameters (e.g., `active_only: Option<bool>`) instead of changing handler semantics
- Hard-cap DynamoDB page scanning loops (`max_pages = 5`)
- Preserve bookmark on cap (set `bookmark = next_bookmark`, not `None`)
- Don't use `.take(remaining)` in filtered collection — collect all matching, truncate post-loop

## Controller Server Functions

Use `#[get("/path")]`, `#[post("/path")]`, `#[patch("/path")]` attributes.
Server derives: `#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]`
