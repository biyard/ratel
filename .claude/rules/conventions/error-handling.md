---
globs: ["app/ratel/src/**/types/error.rs", "app/ratel/src/common/types/**/*.rs"]
---

# Error Handling

## Rule: Typed error enums — never `Error::BadRequest(String)`

Define domain-specific error enum with `Translate` derive, register in `common::Error` with `#[from]` + `#[translate(from)]`.

## Option A: Feature-specific error enum

For errors specific to a single feature module:

```rust
// 1. Define in features/<module>/types/error.rs
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum MyFeatureError {
    #[error("internal msg")]
    #[translate(en = "User message", ko = "사용자 메시지")]
    SpecificError,
}

// 2. Register in common::Error
#[error("{0}")]
#[translate(from)]  // delegates translation to inner type
MyFeature(#[from] MyFeatureError),

// 3. Use in controllers
return Err(MyFeatureError::SpecificError.into());
```

See `SpaceRewardError` pattern in `app/ratel/src/common/types/reward/error.rs`.

## Option B: Direct variant on common::Error

For simple, cross-cutting errors that don't warrant a separate enum, add a **unit variant** (no parameters) directly to `common::Error` in `app/ratel/src/common/types/error.rs`:

```rust
// In common::Error enum
#[error("unauthorized access")]
#[translate(en = "Unauthorized access", ko = "권한이 없습니다")]
Unauthorized,

// Use in controllers
return Err(Error::Unauthorized.into());
```

**Important**: Direct variants must be unit variants — no `String` or other parameters. If you need parameterized messages, use Option A (feature-specific error enum).

## Rule: Log details server-side, return generic errors to users

Server functions must log the specific error with `crate::error!` **before** converting to a unit error type. Users should never see internal details (DB errors, stack traces, etc.).

```rust
// GOOD — log detail, return generic unit error
let result = entity.create(&cli).await.map_err(|e| {
    crate::error!("failed to create entity: {e}");
    MyFeatureError::CreateFailed
})?;

// BAD — leaking internal details to user
let result = entity.create(&cli).await
    .map_err(|e| Error::BadRequest(format!("DynamoDB error: {e}")))?;
```

This ensures:
- **Server logs** contain detailed error context for debugging (`crate::error!`)
- **User responses** contain only safe, translated messages via unit error variants
