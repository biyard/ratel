---
globs: ["packages/main-api/src/controllers/**/*.rs", "app/ratel/src/features/**/controllers/**/*.rs"]
---

# Backend API Conventions

## Controller Server Functions

Use `#[get("/path")]`, `#[post("/path")]`, `#[patch("/path")]` attributes.
Server derives: `#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]`

## Server-Side Pagination

- Add query parameters (e.g., `active_only: Option<bool>`) instead of changing handler semantics
- Hard-cap DynamoDB page scanning loops (`max_pages = 5`)
- Preserve bookmark on cap (set `bookmark = next_bookmark`, not `None`)
- Don't use `.take(remaining)` in filtered collection — collect all matching, truncate post-loop

## Server-side Validation

Never use `i32::MAX` / `i64::MAX` as defaults. Define shared constants for upper bounds (e.g., `MAX_TOTAL_ATTEMPTS: i64 = 100`). Validate at write path, clamp at read path.
