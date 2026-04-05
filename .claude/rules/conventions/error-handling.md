---
globs: ["app/ratel/src/**/types/error.rs", "app/ratel/src/common/types/**/*.rs"]
---

# Error Handling

## Rule: Typed error enums — never `Error::BadRequest(String)`

Define domain-specific error enum with `Translate` derive, register in `common::Error` with `#[from]` + `#[translate(from)]`.

## Pattern

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

## Reference

See `SpaceRewardError` pattern in `app/ratel/src/common/types/reward/error.rs`.
