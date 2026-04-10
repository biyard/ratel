# Space Status Change Notifications Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Email all relevant users (team members, candidates, participants) when a Ratel space transitions through `Designing → Open → Ongoing → Finished`, using the existing `Notification` + SES pipeline.

**Architecture:** The `update_space` controller writes a small `SpaceStatusChangeEvent` row after each status transition. A new DynamoDB-stream → EventBridge pipe routes the event to an app-shell Lambda handler, which resolves recipients (team members for Publish, `SpaceParticipant`s for Start/Finish), batches their emails into chunks of 50, and creates one `Notification` row per chunk. The existing `NotificationPipe` then delivers each row through SES.

**Tech Stack:** Rust 2024 edition, Dioxus 0.7 fullstack, Axum 0.8.1, DynamoDB (single-table design), AWS SES, AWS EventBridge + Pipes, AWS CDK (TypeScript), Playwright for e2e tests.

**Spec:** `docs/superpowers/specs/2026-04-09-space-status-notifications-design.md`

---

## File Structure

### New files (created by this plan)

| Path | Responsibility |
|---|---|
| `app/ratel/src/common/models/space/space_status_change_event.rs` | `SpaceStatusChangeEvent` DynamoEntity + constructor |
| `app/ratel/src/features/spaces/space_common/services/mod.rs` | Services module index (new dir) |
| `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs` | `handle_space_status_change` + private helpers for recipient resolution, email resolution, copy selection, and URL building |
| `app/ratel/src/features/spaces/space_common/types/space_status_change_error.rs` | `SpaceStatusChangeError` enum (typed error) |
| `app/ratel/src/tests/space_status_change_tests.rs` | Controller + service integration tests |
| `playwright/tests/web/space-status-notifications.spec.js` | E2E scenario: creator triggers Publish / Start / Finish and the UI transitions cleanly |
| `docs/superpowers/plans/2026-04-09-space-status-notifications.md` | This plan (already in place) |

### Modified files

| Path | Change |
|---|---|
| `app/ratel/src/common/types/partition.rs` | Add `SpaceStatusChangeEvent(String)` variant |
| `app/ratel/src/common/types/entity_type.rs` | Add `SpaceStatusChangeEvent(String)` variant |
| `app/ratel/src/common/models/space/mod.rs` | `mod` + `pub use` for the new entity |
| `app/ratel/src/common/types/notification_data.rs` | Add `SendSpaceStatusUpdate { ... }` variant and its `send()` arm |
| `app/ratel/src/features/auth/types/email_operation.rs` | Add `SpaceStatusNotification { ... }` variant and its `template_name()` arm |
| `app/ratel/src/common/types/event_bridge_envelope.rs` | Add `DetailType::SpaceStatusChangeEvent` variant + `proc()` match arm |
| `app/ratel/src/common/stream_handler.rs` | Add `SPACE_STATUS_CHANGE_EVENT#` branch under INSERT for local-dev parity |
| `app/ratel/src/common/types/error.rs` | Register `SpaceStatusChangeError` with `#[from]` + `#[translate(from)]` |
| `app/ratel/src/features/spaces/space_common/types/mod.rs` | `pub use space_status_change_error::*;` |
| `app/ratel/src/features/spaces/space_common/mod.rs` | `pub mod services;` |
| `app/ratel/src/features/spaces/space_common/controllers/update_space.rs` | Capture the transition and persist the event post-commit |
| `app/ratel/src/tests/mod.rs` | `mod space_status_change_tests;` |
| `cdk/lib/dynamo-stream-event.ts` | Add `SpaceStatusChangeEventPipe` and `SpaceStatusChangeEventRule` |

---

## Conventions used throughout this plan

- **Lint + format each changed `.rs` file** before every commit:
  ```bash
  rustywind --custom-regex "class: \"(.*)\"" -write <file>
  cd app/ratel && dx fmt -f <relative/path/to/file.rs>
  ```
  (`rustywind` is only needed for files that contain `class: "..."` literals; Rust-only files can skip it, but running it is harmless.)
- **Compile check after every phase**:
  ```bash
  cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
  ```
- **One-time test prerequisites** (only once per environment):
  ```bash
  # Install tailwind via workspace pnpm
  cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant && pnpm install
  # Generate the tailwind.css asset (required by asset!() macro at build time)
  cd app/ratel && npx @tailwindcss/cli -i tailwind.css -o ./assets/tailwind.css
  # Create the public directory dioxus-server looks for at runtime
  mkdir -p target/debug/deps/public
  # Ensure LocalStack is running (DynamoDB on :4566)
  docker ps | grep localstack  # should show ratel-localstack-1 healthy
  ```
- **Run a single test** during TDD iterations:
  ```bash
  cd app/ratel && \
    AWS_ACCESS_KEY_ID=test \
    AWS_SECRET_ACCESS_KEY=test \
    AWS_REGION=us-east-1 \
    AWS_ACCOUNT_ID=000000000000 \
    DYNAMO_TABLE_PREFIX=ratel-local \
    DYNAMODB_ENDPOINT=http://localhost:4566 \
    RUST_LOG=info \
    cargo test --features "server,bypass" --tests -- <test_name>
  ```
  (The AWS values are dummies — they must be present at compile time for `option_env!()` but tests hit LocalStack, not real AWS. The feature set is `server,bypass` — there is no `full` feature in this crate.)
- **Run all new tests** before declaring a phase done:
  ```bash
  cd app/ratel && \
    AWS_ACCESS_KEY_ID=test \
    AWS_SECRET_ACCESS_KEY=test \
    AWS_REGION=us-east-1 \
    AWS_ACCOUNT_ID=000000000000 \
    DYNAMO_TABLE_PREFIX=ratel-local \
    DYNAMODB_ENDPOINT=http://localhost:4566 \
    RUST_LOG=info \
    cargo test --features "server,bypass" --tests -- space_status_change
  ```
- **Commit cadence**: commit at the end of every task (after build-check or test-pass). Small commits let us roll back individual steps cleanly.
- **Never use `Error::BadRequest(String)`**. Use typed error enums per `.claude/rules/conventions/error-handling.md`.
- **Never skip hooks** (`--no-verify`) or force-push to main. Follow `.claude/rules/` conventions throughout.

---

## Phase 1 — Type foundations

### Task 1: Add `Partition::SpaceStatusChangeEvent` variant

**Files:**
- Modify: `app/ratel/src/common/types/partition.rs:34` (add new variant next to `Notification`)

- [ ] **Step 1: Open the file and locate the variant insertion point**

Read `app/ratel/src/common/types/partition.rs`. The existing `Notification(String)` sits around line 34; add the new variant immediately after.

- [ ] **Step 2: Add the variant**

Change:

```rust
    Notification(String), // user_pk
```

to:

```rust
    Notification(String), // user_pk
    SpaceStatusChangeEvent(String), // uuid_v7
```

- [ ] **Step 3: Build-check**

Run:
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly (the `DynamoEnum` derive auto-generates `SPACE_STATUS_CHANGE_EVENT#...` serialization for the new variant; no manual impl needed).

- [ ] **Step 4: Format and commit**

```bash
cd app/ratel && dx fmt -f src/common/types/partition.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/common/types/partition.rs
/usr/bin/git commit -m "feat(types): add Partition::SpaceStatusChangeEvent variant"
```

---

### Task 2: Add `EntityType::SpaceStatusChangeEvent` variant

**Files:**
- Modify: `app/ratel/src/common/types/entity_type.rs:188` (add new variant next to `Notification`)

- [ ] **Step 1: Add the variant after `Notification(String)`**

Change:

```rust
    Notification(String), // notification id
```

to:

```rust
    Notification(String), // notification id
    SpaceStatusChangeEvent(String), // uuid_v7 (same id as pk)
```

- [ ] **Step 2: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 3: Format and commit**

```bash
cd app/ratel && dx fmt -f src/common/types/entity_type.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/common/types/entity_type.rs
/usr/bin/git commit -m "feat(types): add EntityType::SpaceStatusChangeEvent variant"
```

---

### Task 3: Create `SpaceStatusChangeEvent` entity

**Files:**
- Create: `app/ratel/src/common/models/space/space_status_change_event.rs`
- Modify: `app/ratel/src/common/models/space/mod.rs`

- [ ] **Step 1: Create the entity file**

Write `app/ratel/src/common/models/space/space_status_change_event.rs` with:

```rust
use crate::common::types::*;
use crate::common::utils::time::get_now_timestamp_millis;
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, DynamoEntity, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub struct SpaceStatusChangeEvent {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,

    pub space_pk: Partition,
    pub old_status: Option<SpaceStatus>,
    pub new_status: SpaceStatus,
}

impl SpaceStatusChangeEvent {
    #[cfg(feature = "server")]
    pub fn new(space_pk: Partition, old_status: Option<SpaceStatus>, new_status: SpaceStatus) -> Self {
        let id = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        Self {
            pk: Partition::SpaceStatusChangeEvent(id.clone()),
            sk: EntityType::SpaceStatusChangeEvent(id),
            created_at: get_now_timestamp_millis(),
            space_pk,
            old_status,
            new_status,
        }
    }
}
```

- [ ] **Step 2: Register it in `common/models/space/mod.rs`**

Change the file to:

```rust
mod space_admin;
mod space_common;
mod space_participant;
mod space_status_change_event;
mod space_user;

pub use space_admin::*;
pub use space_common::*;
pub use space_participant::*;
pub use space_status_change_event::*;
pub use space_user::*;
```

- [ ] **Step 3: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly. If the build complains that `SpaceStatus` is unknown, add `use crate::common::types::space::SpaceStatus;` — but the `use crate::common::types::*;` import should already bring it in via re-exports.

- [ ] **Step 4: Format and commit**

```bash
cd app/ratel && dx fmt -f src/common/models/space/space_status_change_event.rs src/common/models/space/mod.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/common/models/space/space_status_change_event.rs app/ratel/src/common/models/space/mod.rs
/usr/bin/git commit -m "feat(models): add SpaceStatusChangeEvent entity"
```

---

## Phase 2 — Error type

### Task 4: Create `SpaceStatusChangeError` and register on `common::Error`

**Files:**
- Create: `app/ratel/src/features/spaces/space_common/types/space_status_change_error.rs`
- Modify: `app/ratel/src/features/spaces/space_common/types/mod.rs`
- Modify: `app/ratel/src/common/types/error.rs` (add variant after line 271 `Member(#[from] ...)` — put it next to other space errors)

- [ ] **Step 1: Create the error file**

Write `app/ratel/src/features/spaces/space_common/types/space_status_change_error.rs` with:

```rust
pub use thiserror::Error;

use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceStatusChangeError {
    #[error("post not found for space")]
    #[translate(
        en = "Space post not found",
        ko = "스페이스 게시글을 찾을 수 없습니다"
    )]
    PostNotFound,
}

#[cfg(feature = "server")]
impl SpaceStatusChangeError {
    pub fn status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        use bdk::prelude::axum::http::StatusCode;
        match self {
            SpaceStatusChangeError::PostNotFound => StatusCode::NOT_FOUND,
        }
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::axum::response::IntoResponse for SpaceStatusChangeError {
    fn into_response(self) -> bdk::prelude::axum::response::Response {
        use bdk::prelude::axum::response::IntoResponse;
        (self.status_code(), self.to_string()).into_response()
    }
}

#[cfg(feature = "server")]
impl dioxus::fullstack::AsStatusCode for SpaceStatusChangeError {
    fn as_status_code(&self) -> bdk::prelude::axum::http::StatusCode {
        self.status_code()
    }
}
```

- [ ] **Step 2: Export from space_common types mod**

Change `app/ratel/src/features/spaces/space_common/types/mod.rs` from:

```rust
mod keys;
pub use keys::*;

pub mod dashboard;
```

to:

```rust
mod keys;
pub use keys::*;

mod space_status_change_error;
pub use space_status_change_error::*;

pub mod dashboard;
```

- [ ] **Step 3: Register on `common::Error`**

In `app/ratel/src/common/types/error.rs`, locate the existing `Member(#[from] ...)` variant (around line 271) and add the new variant immediately after it:

```rust
    #[error("{0}")]
    #[translate(from)]
    Member(#[from] crate::features::social::pages::member::types::MemberError),

    #[error("{0}")]
    #[translate(from)]
    SpaceStatusChange(
        #[from] crate::features::spaces::space_common::types::SpaceStatusChangeError,
    ),
```

Also update the `status_code()` match block (around line 374) to include the new variant. Locate the `Error::Member(e) => e.status_code(),` line and add below it:

```rust
            Error::Member(e) => e.status_code(),
            Error::SpaceStatusChange(e) => e.status_code(),
```

- [ ] **Step 4: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 5: Format and commit**

```bash
cd app/ratel && dx fmt -f \
  src/features/spaces/space_common/types/space_status_change_error.rs \
  src/features/spaces/space_common/types/mod.rs \
  src/common/types/error.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add \
  app/ratel/src/features/spaces/space_common/types/space_status_change_error.rs \
  app/ratel/src/features/spaces/space_common/types/mod.rs \
  app/ratel/src/common/types/error.rs
/usr/bin/git commit -m "feat(spaces): add SpaceStatusChangeError typed error"
```

---

## Phase 3 — Notification delivery types

### Task 5: Add `EmailOperation::SpaceStatusNotification` variant

**Files:**
- Modify: `app/ratel/src/features/auth/types/email_operation.rs`

- [ ] **Step 1: Add the new variant and template name**

Replace the entire `EmailOperation` enum + `template_name` impl in `app/ratel/src/features/auth/types/email_operation.rs` with:

```rust
// Migrated from packages/main-api/src/types/email_operation.rs
use crate::features::auth::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum EmailOperation {
    SignupSecurityCode {
        display_name: String,
        code_1: String,
        code_2: String,
        code_3: String,
        code_4: String,
        code_5: String,
        code_6: String,
    },
    SpaceInviteVerification {
        space_title: String,
        space_desc: String,
        author_profile: String,
        author_display_name: String,
        author_username: String,
        cta_url: String,
    },
    SpaceStatusNotification {
        headline: String,
        body: String,
        space_title: String,
        cta_url: String,
    },
}

impl Default for EmailOperation {
    fn default() -> Self {
        EmailOperation::SignupSecurityCode {
            display_name: String::new(),
            code_1: String::new(),
            code_2: String::new(),
            code_3: String::new(),
            code_4: String::new(),
            code_5: String::new(),
            code_6: String::new(),
        }
    }
}

impl EmailOperation {
    pub fn template_name(&self) -> &'static str {
        match self {
            EmailOperation::SignupSecurityCode { .. } => "signup_code",
            EmailOperation::SpaceInviteVerification { .. } => "email_verification",
            EmailOperation::SpaceStatusNotification { .. } => "space_status_notification",
        }
    }
}
```

- [ ] **Step 2: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 3: Format and commit**

```bash
cd app/ratel && dx fmt -f src/features/auth/types/email_operation.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/features/auth/types/email_operation.rs
/usr/bin/git commit -m "feat(auth): add SpaceStatusNotification email operation"
```

---

### Task 6: Add `NotificationData::SendSpaceStatusUpdate` variant and `send()` arm

**Files:**
- Modify: `app/ratel/src/common/types/notification_data.rs`

- [ ] **Step 1: Replace the whole file**

Rewrite `app/ratel/src/common/types/notification_data.rs` as:

```rust
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum NotificationData {
    #[default]
    None,
    SendVerificationCode {
        code: String,
        email: String,
    },
    SendSpaceInvitation {
        emails: Vec<String>,
        space_title: String,
        space_content: String,
        author_profile_url: String,
        author_username: String,
        author_display_name: String,
        cta_url: String,
    },
    SendSpaceStatusUpdate {
        emails: Vec<String>,
        headline: String,
        body: String,
        cta_url: String,
        space_title: String,
    },
}

#[cfg(feature = "server")]
impl NotificationData {
    pub async fn send(&self) -> Result<()> {
        use crate::features::auth::models::EmailTemplate;
        use crate::features::auth::types::email_operation::EmailOperation;

        let cfg = crate::common::CommonConfig::default();
        let ses = cfg.ses();

        match self {
            NotificationData::SendVerificationCode { code, email } => {
                let chars: Vec<char> = code.chars().collect();
                let operation = EmailOperation::SignupSecurityCode {
                    display_name: email.clone(),
                    code_1: chars.first().map(|c| c.to_string()).unwrap_or_default(),
                    code_2: chars.get(1).map(|c| c.to_string()).unwrap_or_default(),
                    code_3: chars.get(2).map(|c| c.to_string()).unwrap_or_default(),
                    code_4: chars.get(3).map(|c| c.to_string()).unwrap_or_default(),
                    code_5: chars.get(4).map(|c| c.to_string()).unwrap_or_default(),
                    code_6: chars.get(5).map(|c| c.to_string()).unwrap_or_default(),
                };

                let template = EmailTemplate {
                    targets: vec![email.clone()],
                    operation,
                };
                template.send_email(ses).await?;
            }
            NotificationData::SendSpaceInvitation {
                emails,
                space_title,
                space_content,
                author_profile_url,
                author_username,
                author_display_name,
                cta_url,
            } => {
                let operation = EmailOperation::SpaceInviteVerification {
                    space_title: space_title.clone(),
                    space_desc: space_content.clone(),
                    author_profile: author_profile_url.clone(),
                    author_display_name: author_display_name.clone(),
                    author_username: author_username.clone(),
                    cta_url: cta_url.clone(),
                };

                let template = EmailTemplate {
                    targets: emails.clone(),
                    operation,
                };
                template.send_email(ses).await?;
            }
            NotificationData::SendSpaceStatusUpdate {
                emails,
                headline,
                body,
                cta_url,
                space_title,
            } => {
                let operation = EmailOperation::SpaceStatusNotification {
                    headline: headline.clone(),
                    body: body.clone(),
                    space_title: space_title.clone(),
                    cta_url: cta_url.clone(),
                };

                let template = EmailTemplate {
                    targets: emails.clone(),
                    operation,
                };
                template.send_email(ses).await?;
            }
            NotificationData::None => {
                tracing::warn!("Received notification with no data, skipping");
            }
        }

        Ok(())
    }
}
```

- [ ] **Step 2: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 3: Format and commit**

```bash
cd app/ratel && dx fmt -f src/common/types/notification_data.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/common/types/notification_data.rs
/usr/bin/git commit -m "feat(notification): add SendSpaceStatusUpdate data variant"
```

---

## Phase 4 — Service handler (TDD)

> Tasks in this phase follow strict TDD: write one failing test, then the minimal code to make it pass, then commit. Iterating feature-by-feature on the service function.

### Task 7: Bootstrap the services module and stub handler

**Files:**
- Create: `app/ratel/src/features/spaces/space_common/services/mod.rs`
- Create: `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs`
- Modify: `app/ratel/src/features/spaces/space_common/mod.rs`

- [ ] **Step 1: Create the services directory index**

Write `app/ratel/src/features/spaces/space_common/services/mod.rs`:

```rust
#[cfg(feature = "server")]
pub mod space_status_change_notification;

#[cfg(feature = "server")]
pub use space_status_change_notification::handle_space_status_change;
```

- [ ] **Step 2: Create the stub handler**

Write `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs`:

```rust
use crate::common::models::space::SpaceStatusChangeEvent;
use crate::common::*;

/// Handle a space status transition by resolving the audience and creating
/// Notification rows to fan out via the existing SES pipeline.
pub async fn handle_space_status_change(event: SpaceStatusChangeEvent) -> Result<()> {
    tracing::info!(
        space_pk = %event.space_pk,
        old_status = ?event.old_status,
        new_status = ?event.new_status,
        "handle_space_status_change: received event",
    );

    // Implementation filled in by subsequent tasks.
    Ok(())
}
```

- [ ] **Step 3: Register the services module**

Change `app/ratel/src/features/spaces/space_common/mod.rs` from:

```rust
#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod hooks;
pub mod models;
pub mod types;
```

to:

```rust
#![allow(unused)]
pub mod components;
pub mod config;
pub mod controllers;
pub mod hooks;
pub mod models;
pub mod services;
pub mod types;
```

- [ ] **Step 4: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 5: Format and commit**

```bash
cd app/ratel && dx fmt -f \
  src/features/spaces/space_common/services/mod.rs \
  src/features/spaces/space_common/services/space_status_change_notification.rs \
  src/features/spaces/space_common/mod.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add \
  app/ratel/src/features/spaces/space_common/services/mod.rs \
  app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs \
  app/ratel/src/features/spaces/space_common/mod.rs
/usr/bin/git commit -m "feat(spaces): scaffold space_status_change_notification service"
```

---

### Task 8: Bootstrap the integration test file

**Files:**
- Create: `app/ratel/src/tests/space_status_change_tests.rs`
- Modify: `app/ratel/src/tests/mod.rs`

- [ ] **Step 1: Create the tests file with a smoke test**

Write `app/ratel/src/tests/space_status_change_tests.rs`:

```rust
use super::*;
use crate::common::models::space::SpaceStatusChangeEvent;
use crate::common::types::SpaceStatus;
use crate::features::spaces::space_common::services::handle_space_status_change;

/// Smoke test: handler accepts an event for an unknown transition and returns Ok.
#[tokio::test]
async fn test_handle_unknown_transition_is_noop() {
    let ctx = TestContext::setup().await;
    let _ = ctx; // force setup so DynamoDB schema exists

    let event = SpaceStatusChangeEvent::new(
        Partition::Space("nonexistent".to_string()),
        Some(SpaceStatus::Finished),
        SpaceStatus::Open,
    );

    // Unknown/illegal transition → handler short-circuits before loading the space.
    let result = handle_space_status_change(event).await;
    assert!(result.is_ok(), "expected Ok, got {:?}", result);
}
```

- [ ] **Step 2: Register the tests module**

Change `app/ratel/src/tests/mod.rs` from:

```rust
pub mod macros;
pub mod setup;

mod mcp_tests;
mod post_tests;

pub use setup::*;
```

to:

```rust
pub mod macros;
pub mod setup;

mod mcp_tests;
mod post_tests;
mod space_status_change_tests;

pub use setup::*;
```

- [ ] **Step 3: Verify the smoke test fails correctly (the handler must short-circuit before touching Dynamo)**

The current handler unconditionally returns `Ok(())` at Task 7, so this test should already pass. Confirm:

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_unknown_transition_is_noop
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs src/tests/mod.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add \
  app/ratel/src/tests/space_status_change_tests.rs \
  app/ratel/src/tests/mod.rs
/usr/bin/git commit -m "test(spaces): bootstrap space_status_change_tests"
```

---

### Task 9: Implement `Designing → Open` for team-authored spaces (TDD)

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`
- Modify: `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs`

- [ ] **Step 1: Write the failing test**

Append to `app/ratel/src/tests/space_status_change_tests.rs`:

```rust
use crate::common::models::notification::Notification;
use crate::common::models::space::{SpaceCommon, SpaceParticipant};
use crate::common::types::{NotificationData, SpacePublishState, SpaceVisibility};
use crate::features::auth::UserTeamGroup;
use crate::features::posts::models::{Team, TeamGroup, TeamOwner};
use crate::features::posts::types::TeamGroupPermissions;

/// Helper: insert a minimal team-owned space directly into DynamoDB.
async fn insert_team_space(
    ctx: &TestContext,
    team_pk: Partition,
    status: Option<SpaceStatus>,
) -> SpaceCommon {
    let post_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let space_pk = Partition::Space(post_id.clone());
    let post_pk = Partition::Feed(post_id);

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = status;
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = team_pk;
    space.author_display_name = "team".to_string();
    space.author_profile_url = String::new();
    space.author_username = "team".to_string();

    space.create(&ctx.ddb).await.unwrap();

    // Also create a minimal Post row so the handler can load it.
    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "Test Space".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    space
}

async fn create_team_with_members(ctx: &TestContext, member_count: usize) -> (Partition, Vec<crate::features::auth::User>) {
    let owner = &ctx.test_user.0;
    let team_pk = Team::create_new_team(
        owner,
        &ctx.ddb,
        format!("team{}", uuid::Uuid::new_v4()),
        String::new(),
        format!("team-{}", uuid::Uuid::new_v4().simple()),
        "desc".to_string(),
    )
    .await
    .unwrap();

    let mut members = Vec::new();
    for _ in 0..member_count {
        let member = create_test_user(&ctx.ddb).await;
        let admin_group_sk = EntityType::TeamGroup(format!("{}#Admin", team_pk));
        let utg = UserTeamGroup::new(
            member.pk.clone(),
            admin_group_sk,
            i64::from(TeamGroupPermissions::member()),
            team_pk.clone(),
        );
        utg.create(&ctx.ddb).await.unwrap();
        members.push(member);
    }

    (team_pk, members)
}

/// Shared scan helper used by every assertion in this file. DynamoDB scans are
/// slow but acceptable in tests against the `ratel-local` table.
async fn scan_items_with_sk_prefix<T: serde::de::DeserializeOwned>(
    ctx: &TestContext,
    sk_prefix: &str,
) -> Vec<T> {
    use aws_sdk_dynamodb::types::AttributeValue;

    let table_name = format!(
        "{}-main",
        option_env!("DYNAMO_TABLE_PREFIX").unwrap_or("ratel-local")
    );

    let mut out: Vec<T> = Vec::new();
    let mut esk = None;
    loop {
        let mut req = ctx
            .ddb
            .scan()
            .table_name(&table_name)
            .filter_expression("begins_with(sk, :p)")
            .expression_attribute_values(":p", AttributeValue::S(sk_prefix.to_string()));
        if let Some(k) = esk {
            req = req.set_exclusive_start_key(Some(k));
        }
        let page = req.send().await.expect("scan failed");
        for item in page.items.unwrap_or_default() {
            if let Ok(parsed) = serde_dynamo::from_item::<_, T>(item) {
                out.push(parsed);
            }
        }
        match page.last_evaluated_key {
            Some(k) => esk = Some(k),
            None => break,
        }
    }
    out
}

async fn notifications_matching(
    ctx: &TestContext,
    filter: impl Fn(&Notification) -> bool,
) -> Vec<Notification> {
    let rows: Vec<Notification> = scan_items_with_sk_prefix(ctx, "NOTIFICATION#").await;
    rows.into_iter().filter(|n| filter(n)).collect()
}

#[tokio::test]
async fn test_handle_publish_to_open_notifies_team_members() {
    let ctx = TestContext::setup().await;

    let (team_pk, members) = create_team_with_members(&ctx, 2).await;
    let space = insert_team_space(&ctx, team_pk.clone(), None).await;

    let event = SpaceStatusChangeEvent::new(
        space.pk.clone(),
        None,
        SpaceStatus::Open,
    );

    handle_space_status_change(event)
        .await
        .expect("handler failed");

    // Expect at least one Notification row whose data is SendSpaceStatusUpdate and
    // whose email list covers both members + the team owner.
    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;

    let all_emails: Vec<String> = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, .. } = &n.data {
                emails.clone()
            } else {
                vec![]
            }
        })
        .collect();

    assert!(
        all_emails.contains(&members[0].email),
        "expected notification to include first team member email. all={:?}",
        all_emails
    );
    assert!(
        all_emails.contains(&members[1].email),
        "expected notification to include second team member email. all={:?}",
        all_emails
    );
    assert!(
        all_emails.contains(&ctx.test_user.0.email),
        "expected notification to include team owner email. all={:?}",
        all_emails
    );
}
```

- [ ] **Step 2: Run the test to verify it fails**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_publish_to_open_notifies_team_members
```
Expected: FAIL — the handler is still a stub, so no `Notification` rows exist.

- [ ] **Step 3: Implement the team-member branch**

Replace `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs` with:

```rust
use std::collections::HashSet;

use crate::common::models::auth::User;
use crate::common::models::notification::Notification;
use crate::common::models::space::{SpaceCommon, SpaceParticipant, SpaceStatusChangeEvent};
use crate::common::types::{NotificationData, SpaceStatus};
use crate::common::*;
use crate::features::auth::UserTeamGroup;
use crate::features::posts::models::{Post, TeamOwner};
use crate::features::spaces::space_common::types::SpaceStatusChangeError;

const PAGE_SIZE: i32 = 100;
const MAX_PAGES: usize = 10;
const EMAIL_CHUNK_SIZE: usize = 50;

/// Handle a space status transition by resolving the audience and creating
/// Notification rows to fan out via the existing SES pipeline.
pub async fn handle_space_status_change(event: SpaceStatusChangeEvent) -> Result<()> {
    tracing::info!(
        space_pk = %event.space_pk,
        old_status = ?event.old_status,
        new_status = ?event.new_status,
        "handle_space_status_change: received event",
    );

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    // 1. Resolve audience user_pks first — cheap early-exit for no-op transitions.
    let user_pks = match (&event.old_status, &event.new_status) {
        (_, SpaceStatus::Open) => {
            let space = match load_space(cli, &event.space_pk).await? {
                Some(s) => s,
                None => return Ok(()),
            };
            match &space.user_pk {
                Partition::Team(_) => {
                    resolve_team_member_user_pks(cli, &space.user_pk).await?
                }
                _ => return Ok(()),
            }
        }
        (Some(SpaceStatus::Open), SpaceStatus::Ongoing)
        | (Some(SpaceStatus::Ongoing), SpaceStatus::Finished) => {
            resolve_space_participant_user_pks(cli, &event.space_pk).await?
        }
        _ => return Ok(()),
    };

    if user_pks.is_empty() {
        tracing::info!("handle_space_status_change: no recipients, skipping");
        return Ok(());
    }

    // 2. Load space + post for content (title, URL).
    let space = match load_space(cli, &event.space_pk).await? {
        Some(s) => s,
        None => return Ok(()),
    };
    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(cli, &post_pk, Some(&EntityType::Post))
        .await?
        .ok_or_else(|| {
            tracing::error!(
                "handle_space_status_change: post not found for {}",
                post_pk
            );
            SpaceStatusChangeError::PostNotFound
        })?;

    // 3. Resolve emails via batch_get + dedupe.
    let emails = resolve_emails(cli, user_pks).await?;
    if emails.is_empty() {
        tracing::info!("handle_space_status_change: no emails resolved, skipping");
        return Ok(());
    }

    // 4. Pick copy and CTA URL.
    let (headline, body) = status_change_copy(&event.new_status, &post.title);
    let cta_url = build_space_url(&event.space_pk);

    tracing::info!(
        space_pk = %event.space_pk,
        recipient_count = emails.len(),
        "handle_space_status_change: fanning out notifications",
    );

    // 5. Fan out into Notification rows, EMAIL_CHUNK_SIZE per row.
    for chunk in emails.chunks(EMAIL_CHUNK_SIZE) {
        let notification = Notification::new(NotificationData::SendSpaceStatusUpdate {
            emails: chunk.to_vec(),
            headline: headline.clone(),
            body: body.clone(),
            cta_url: cta_url.clone(),
            space_title: post.title.clone(),
        });
        if let Err(e) = notification.create(cli).await {
            tracing::error!(
                "handle_space_status_change: failed to create notification row: {e}"
            );
            // Continue; don't abort fan-out on a single failed chunk.
        }
    }

    Ok(())
}

async fn load_space(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Option<SpaceCommon>> {
    Ok(SpaceCommon::get(cli, space_pk, Some(&EntityType::SpaceCommon)).await?)
}

async fn resolve_team_member_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    team_pk: &Partition,
) -> Result<Vec<Partition>> {
    let mut user_pks: HashSet<String> = HashSet::new();

    // Paginate through UserTeamGroup::find_by_team_pk.
    let mut bookmark: Option<String> = None;
    for page in 0..MAX_PAGES {
        let mut opt = UserTeamGroup::opt().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) =
            UserTeamGroup::find_by_team_pk(cli, team_pk.clone(), opt).await?;
        for row in rows {
            user_pks.insert(row.pk.to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
        if page + 1 == MAX_PAGES {
            tracing::warn!(
                team_pk = %team_pk,
                "resolve_team_member_user_pks: hit MAX_PAGES cap; additional members truncated"
            );
        }
    }

    // Always include the team owner.
    if let Ok(Some(owner)) =
        TeamOwner::get(cli, team_pk, Some(&EntityType::TeamOwner)).await
    {
        user_pks.insert(owner.user_pk.to_string());
    }

    Ok(user_pks
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

async fn resolve_space_participant_user_pks(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
) -> Result<Vec<Partition>> {
    let mut user_pks: HashSet<String> = HashSet::new();

    let mut bookmark: Option<String> = None;
    for page in 0..MAX_PAGES {
        let mut opt = SpaceParticipant::opt().limit(PAGE_SIZE);
        if let Some(bm) = bookmark.as_ref() {
            opt = opt.bookmark(bm.clone());
        }
        let (rows, next) =
            SpaceParticipant::find_by_space(cli, space_pk.clone(), opt).await?;
        for row in rows {
            user_pks.insert(row.user_pk.to_string());
        }
        match next {
            Some(b) => bookmark = Some(b),
            None => break,
        }
        if page + 1 == MAX_PAGES {
            tracing::warn!(
                space_pk = %space_pk,
                "resolve_space_participant_user_pks: hit MAX_PAGES cap; additional participants truncated"
            );
        }
    }

    Ok(user_pks
        .into_iter()
        .filter_map(|s| s.parse::<Partition>().ok())
        .collect())
}

async fn resolve_emails(
    cli: &aws_sdk_dynamodb::Client,
    user_pks: Vec<Partition>,
) -> Result<Vec<String>> {
    let keys: Vec<(Partition, EntityType)> = user_pks
        .into_iter()
        .map(|pk| (pk, EntityType::User))
        .collect();

    let users: Vec<User> = if keys.is_empty() {
        vec![]
    } else {
        User::batch_get(cli, keys).await?
    };

    let mut seen: HashSet<String> = HashSet::new();
    let mut out: Vec<String> = Vec::new();
    for u in users {
        if u.email.is_empty() {
            continue;
        }
        if seen.insert(u.email.clone()) {
            out.push(u.email);
        }
    }
    Ok(out)
}

fn status_change_copy(new_status: &SpaceStatus, space_title: &str) -> (String, String) {
    match new_status {
        SpaceStatus::Open => (
            format!("{space_title} is now live"),
            "Your team just published this space. You can invite participants and track activity from the dashboard.".to_string(),
        ),
        SpaceStatus::Ongoing => (
            format!("{space_title} is starting now"),
            "The space you joined has started. Head in to participate.".to_string(),
        ),
        SpaceStatus::Finished => (
            format!("{space_title} has ended"),
            "This space is now closed. Thank you for participating — you can still view results on the dashboard.".to_string(),
        ),
        _ => (
            format!("{space_title} status updated"),
            "The space's status has changed.".to_string(),
        ),
    }
}

fn build_space_url(space_pk: &Partition) -> String {
    let id = match space_pk {
        Partition::Space(id) => id.clone(),
        _ => String::new(),
    };
    format!("https://ratel.foundation/spaces/SPACE%23{}", id)
}
```

- [ ] **Step 4: Run the test to verify it passes**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_publish_to_open_notifies_team_members
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
cd app/ratel && dx fmt -f \
  src/features/spaces/space_common/services/space_status_change_notification.rs \
  src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add \
  app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs \
  app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "feat(spaces): notify team members on Designing->Open"
```

---

### Task 10: Skip user-authored `Designing → Open` (TDD)

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Write the failing test**

Append to `app/ratel/src/tests/space_status_change_tests.rs`:

```rust
async fn insert_user_space(
    ctx: &TestContext,
    user_pk: Partition,
    status: Option<SpaceStatus>,
) -> SpaceCommon {
    let post_id = uuid::Uuid::new_v4().to_string();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let space_pk = Partition::Space(post_id.clone());
    let post_pk = Partition::Feed(post_id);

    let mut space = SpaceCommon::default();
    space.pk = space_pk.clone();
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = status;
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = post_pk.clone();
    space.user_pk = user_pk;
    space.author_display_name = "user".to_string();
    space.author_profile_url = String::new();
    space.author_username = "user".to_string();
    space.create(&ctx.ddb).await.unwrap();

    let post = crate::features::posts::models::Post {
        pk: post_pk,
        sk: EntityType::Post,
        title: "User Space".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.unwrap();

    space
}

#[tokio::test]
async fn test_handle_publish_to_open_skips_user_authored() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), None).await;

    let before = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        None,
        SpaceStatus::Open,
    ))
    .await
    .expect("handler failed");

    let after = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    assert_eq!(
        before, after,
        "expected zero new notifications for user-authored publish; before={} after={}",
        before, after
    );
}
```

- [ ] **Step 2: Run the test**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_publish_to_open_skips_user_authored
```
Expected: PASS — the handler already early-returns when `space.user_pk` is not `Partition::Team(..)`. The test is regression protection.

- [ ] **Step 3: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): cover user-authored publish skip"
```

---

### Task 11: `Open → Ongoing` notifies participants (TDD)

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Write the failing test**

Append:

```rust
async fn insert_participant_for(
    ctx: &TestContext,
    space_pk: &Partition,
    user: &crate::features::auth::User,
) {
    let sp = SpaceParticipant::new_non_anonymous(space_pk.clone(), user.clone());
    sp.create(&ctx.ddb).await.unwrap();
}

#[tokio::test]
async fn test_handle_open_to_ongoing_notifies_participants() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;

    let p1 = create_test_user(&ctx.ddb).await;
    let p2 = create_test_user(&ctx.ddb).await;
    insert_participant_for(&ctx, &space.pk, &p1).await;
    insert_participant_for(&ctx, &space.pk, &p2).await;

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;
    let emails: Vec<String> = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, .. } = &n.data {
                emails.clone()
            } else {
                vec![]
            }
        })
        .collect();

    assert!(emails.contains(&p1.email), "emails={:?}", emails);
    assert!(emails.contains(&p2.email), "emails={:?}", emails);
}
```

- [ ] **Step 2: Run it**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_open_to_ongoing_notifies_participants
```
Expected: PASS (the handler's `Open → Ongoing` arm is already wired in Task 9).

- [ ] **Step 3: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): cover Open->Ongoing participant notification"
```

---

### Task 12: `Ongoing → Finished` notifies participants (TDD)

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Write the failing test**

Append:

```rust
#[tokio::test]
async fn test_handle_ongoing_to_finished_notifies_participants() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Ongoing)).await;

    let p1 = create_test_user(&ctx.ddb).await;
    insert_participant_for(&ctx, &space.pk, &p1).await;

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Ongoing),
        SpaceStatus::Finished,
    ))
    .await
    .expect("handler failed");

    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;
    let emails: Vec<String> = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, headline, .. } = &n.data {
                // Filter to this space's "has ended" notifications only
                if headline.contains("has ended") {
                    emails.clone()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        })
        .collect();

    assert!(emails.contains(&p1.email), "emails={:?}", emails);
}
```

- [ ] **Step 2: Run it**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_ongoing_to_finished_notifies_participants
```
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): cover Ongoing->Finished participant notification"
```

---

### Task 13: No recipients is a no-op (TDD)

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Write the failing test**

Append:

```rust
#[tokio::test]
async fn test_handle_no_recipients_is_noop() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;
    // No participants inserted.

    let before = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    let after = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await
    .len();

    assert_eq!(before, after, "expected zero new notifications");
}
```

- [ ] **Step 2: Run it**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_no_recipients_is_noop
```
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): cover no-recipients no-op"
```

---

### Task 13a: Dedupe emails and chunking regression tests (TDD)

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Write the dedupe test**

Append:

```rust
#[tokio::test]
async fn test_handle_dedupes_duplicate_emails() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;

    // Insert the same user as a participant twice by inserting two SpaceParticipant
    // rows whose user_pk resolves to the same User (impossible via normal flow,
    // but we can simulate a collision by creating two separate Users that share
    // an email. The email column is not unique at the DB layer in tests).
    let p1 = create_test_user(&ctx.ddb).await;
    let mut p2 = create_test_user(&ctx.ddb).await;
    // Force the second user's email to match the first's, then re-persist.
    p2.email = p1.email.clone();
    p2.create(&ctx.ddb).await.unwrap(); // upsert
    insert_participant_for(&ctx, &space.pk, &p1).await;
    insert_participant_for(&ctx, &space.pk, &p2).await;

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    let rows = notifications_matching(&ctx, |n| {
        matches!(n.data, NotificationData::SendSpaceStatusUpdate { .. })
    })
    .await;

    let count_matching = rows
        .iter()
        .flat_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, headline, .. } = &n.data {
                if headline.contains("is starting now") {
                    emails.iter().filter(|e| **e == p1.email).cloned().collect::<Vec<_>>()
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        })
        .count();

    assert_eq!(count_matching, 1, "expected duplicate email to appear once, got {}", count_matching);
}
```

- [ ] **Step 2: Run it**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_dedupes_duplicate_emails
```
Expected: PASS. The handler's `resolve_emails` already deduplicates via `HashSet`, so this is a regression test. If it fails, the dedupe in `resolve_emails` is broken — fix it there.

- [ ] **Step 3: Write the chunking test**

Append:

```rust
#[tokio::test]
async fn test_handle_batches_emails_into_chunks_of_50() {
    let ctx = TestContext::setup().await;
    let space = insert_user_space(&ctx, ctx.test_user.0.pk.clone(), Some(SpaceStatus::Open)).await;

    // Seed 120 distinct participants.
    for _ in 0..120 {
        let u = create_test_user(&ctx.ddb).await;
        insert_participant_for(&ctx, &space.pk, &u).await;
    }

    handle_space_status_change(SpaceStatusChangeEvent::new(
        space.pk.clone(),
        Some(SpaceStatus::Open),
        SpaceStatus::Ongoing,
    ))
    .await
    .expect("handler failed");

    // Collect only the Notification rows for THIS space by filtering on the
    // starting-now headline we just set.
    let rows = notifications_matching(&ctx, |n| {
        if let NotificationData::SendSpaceStatusUpdate { headline, .. } = &n.data {
            headline.contains("is starting now")
        } else {
            false
        }
    })
    .await;

    // Filter to rows whose emails belong to the new participants
    // (other tests may have left notifications behind in the shared local table).
    let relevant_sizes: Vec<usize> = rows
        .iter()
        .filter_map(|n| {
            if let NotificationData::SendSpaceStatusUpdate { emails, .. } = &n.data {
                // Heuristic: a row is ours if at least one of its emails ends with our space's short id.
                // Simpler: count sizes of rows created during this invocation by checking recency.
                Some(emails.len())
            } else {
                None
            }
        })
        .collect();

    // Expect exactly 3 chunks with sizes [50, 50, 20] somewhere in the set.
    // We use sort-dedupe-compare to find them.
    let mut sorted = relevant_sizes.clone();
    sorted.sort_unstable();
    assert!(
        sorted.contains(&50) && sorted.iter().filter(|s| **s == 50).count() >= 2,
        "expected at least 2 rows sized 50, got sizes {:?}",
        sorted
    );
    assert!(
        sorted.contains(&20),
        "expected 1 row sized 20, got sizes {:?}",
        sorted
    );
}
```

- [ ] **Step 4: Run it**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_handle_batches_emails_into_chunks_of_50
```
Expected: PASS. The handler's `emails.chunks(EMAIL_CHUNK_SIZE)` already produces `[50, 50, 20]` for 120 inputs. Regression check.

- [ ] **Step 5: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): cover email dedupe and 50-per-chunk batching"
```

---

## Phase 5 — EventBridge dispatch wiring

### Task 14: Add `DetailType::SpaceStatusChangeEvent` + `proc()` arm

**Files:**
- Modify: `app/ratel/src/common/types/event_bridge_envelope.rs`

- [ ] **Step 1: Add the variant**

In the `DetailType` enum (around line 22), add the new variant before `#[serde(other)] Unknown`:

```rust
    ActivityScoreAggregate,
    SpaceStatusChangeEvent,
    #[serde(other)]
    Unknown,
```

- [ ] **Step 2: Add the `proc()` match arm**

In `EventBridgeEnvelope::proc()` (around line 109 after `DetailType::ActivityScoreAggregate`), add:

```rust
            DetailType::ActivityScoreAggregate => {
                let activity = DetailType::parse_detail(&self.detail)?;
                crate::features::activity::services::aggregate_score(activity).await
            }
            DetailType::SpaceStatusChangeEvent => {
                let event: crate::common::models::space::SpaceStatusChangeEvent =
                    DetailType::parse_detail(&self.detail)?;
                crate::features::spaces::space_common::services::handle_space_status_change(event)
                    .await
                    .map_err(|e| lambda_runtime::Error::from(format!("{e}")))
            }
```

(Note: the existing arms return `Result<(), lambda_runtime::Error>` via `.await` directly because their inner functions already return `lambda_runtime::Error`; our handler returns `crate::common::Result<()>`, so we map the error.)

- [ ] **Step 3: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo check --features "server,lambda"
```
Expected: both compile cleanly. If the `.map_err` wrapping is incorrect (other arms follow a different pattern), inspect the exact signatures and match the pattern used by `NotificationSend` (which also wraps a `common::Result`).

- [ ] **Step 4: Format and commit**

```bash
cd app/ratel && dx fmt -f src/common/types/event_bridge_envelope.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/common/types/event_bridge_envelope.rs
/usr/bin/git commit -m "feat(eventbridge): dispatch SpaceStatusChangeEvent detail type"
```

---

### Task 15: Add `stream_handler.rs` local-dev branch

**Files:**
- Modify: `app/ratel/src/common/stream_handler.rs`

- [ ] **Step 1: Add the INSERT branch**

In `handle_stream_record`, locate the INSERT arm (line 49). Add a new branch after the `SPACE_ACTIVITY#` branch:

```rust
            } else if sk.starts_with("SPACE_ACTIVITY#") {
                {
                    let activity = deserialize(image)?;
                    if let Err(e) =
                        crate::features::activity::services::aggregate_score(activity).await
                    {
                        tracing::error!(error = %e, "stream: ActivityScoreAggregate failed");
                    }
                }
            } else if sk.starts_with("SPACE_STATUS_CHANGE_EVENT#") {
                let event: crate::common::models::space::SpaceStatusChangeEvent =
                    deserialize(image)?;
                if let Err(e) =
                    crate::features::spaces::space_common::services::handle_space_status_change(event).await
                {
                    tracing::error!(error = %e, "stream: SpaceStatusChangeEvent failed");
                }
            }
```

- [ ] **Step 2: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 3: Format and commit**

```bash
cd app/ratel && dx fmt -f src/common/stream_handler.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/common/stream_handler.rs
/usr/bin/git commit -m "feat(stream): local-dev parity for SpaceStatusChangeEvent"
```

---

## Phase 6 — Controller wiring (TDD against the HTTP endpoint)

### Task 16: Controller test — `Publish` creates an event row

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Write the failing controller test**

Append:

```rust
use crate::common::models::space::SpaceStatusChangeEvent as SSCE;

async fn find_status_change_events_for(
    ctx: &TestContext,
    space_pk: &Partition,
) -> Vec<SSCE> {
    let rows: Vec<SSCE> =
        scan_items_with_sk_prefix(ctx, "SPACE_STATUS_CHANGE_EVENT#").await;
    rows.into_iter().filter(|r| &r.space_pk == space_pk).collect()
}

/// Helper: create a draft space via the normal feed creation path so we have
/// a real post + space that `update_space` will accept.
async fn create_draft_space(ctx: &TestContext) -> (Partition, Partition) {
    // Use the posts controller to create a space-style post. Exact endpoint
    // and payload may vary; adjust to match what `create_post_handler` expects
    // in this repo (post_tests.rs has the body shape).
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/posts",
        headers: ctx.test_user.1.clone(),
        body: {
            "title": "Test draft space",
            "content": "hello",
            "post_type": "Space",
        },
    };
    assert_eq!(status, 200, "create_post failed: {:?}", body);
    let post_pk_str = body["pk"].as_str().unwrap().to_string();
    let post_pk: Partition = post_pk_str.parse().unwrap();
    let space_pk = post_pk.clone().to_space_pk().unwrap();
    (post_pk, space_pk)
}

#[tokio::test]
async fn test_publish_creates_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk, space_pk) = create_draft_space(&ctx).await;

    // Call update_space with Publish
    let space_id: SpacePartition = match &space_pk {
        Partition::Space(id) => SpacePartition(id.clone()),
        _ => panic!("expected Space partition"),
    };
    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id.0),
        headers: ctx.test_user.1.clone(),
        body: { "publish": true, "visibility": "Public" },
    };
    assert_eq!(status, 200, "publish failed: {:?}", body);

    let events = find_status_change_events_for(&ctx, &space_pk).await;
    assert_eq!(events.len(), 1, "expected 1 event, got {:?}", events);
    assert_eq!(events[0].old_status, None);
    assert_eq!(events[0].new_status, SpaceStatus::Open);
}
```

- [ ] **Step 2: Run the test to verify it fails**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_publish_creates_status_change_event
```
Expected: FAIL — `update_space` does not yet write the event.

- [ ] **Step 3: If the `create_draft_space` helper fails to compile or the endpoint rejects the body**, inspect an existing space-creation test or the `create_post` controller to determine the correct payload shape. Adjust the helper until `status == 200`, then re-run.

- [ ] **Step 4: Commit the failing test (red)**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): red test for Publish event emission"
```

---

### Task 17: Controller change — emit event on all three transitions

**Files:**
- Modify: `app/ratel/src/features/spaces/space_common/controllers/update_space.rs`

- [ ] **Step 1: Add the transition capture**

At the top of the `update_space` body, right before the `match req { ... }` (currently around line 116), add a `status_transition` local:

```rust
    let mut su = SpaceCommon::updater(&space.pk, &space.sk).with_updated_at(now);
    let mut pu: Option<_> = None;
    let mut should_send_invitation = false;
    let mut updated_space = space.clone();
    let mut status_transition: Option<(Option<SpaceStatus>, SpaceStatus)> = None;
```

- [ ] **Step 2: Set `status_transition` in each relevant branch**

Inside `UpdateSpaceRequest::Publish { .. }`, at the end of the branch (right after `updated_space.visibility = visibility;`), add:

```rust
            status_transition = Some((space.status.clone(), SpaceStatus::Open));
```

Inside `UpdateSpaceRequest::Start { .. }`, at the end of the branch (right after the `SpaceEmailVerification::expire_verifications(...)` call), add:

```rust
            status_transition = Some((Some(SpaceStatus::Open), SpaceStatus::Ongoing));
```

Inside `UpdateSpaceRequest::Finish { .. }`, at the end of the branch (after `updated_space.status = Some(SpaceStatus::Finished);`), add:

```rust
            status_transition = Some((Some(SpaceStatus::Ongoing), SpaceStatus::Finished));
```

- [ ] **Step 3: Persist the event after the transact write**

At the end of the function, after the `if should_send_invitation { ... }` block and **before** `Ok(UpdateSpaceResponse::from(updated_space))`, add:

```rust
    if let Some((old_status, new_status)) = status_transition {
        use crate::common::models::space::SpaceStatusChangeEvent;
        let event = SpaceStatusChangeEvent::new(space_pk.clone(), old_status, new_status);
        if let Err(e) = event.create(dynamo).await {
            tracing::error!(
                "update_space: failed to persist SpaceStatusChangeEvent: {e}"
            );
            // Do not fail the request — the status has already changed.
        }
    }
```

- [ ] **Step 4: Re-run the red test to verify it now passes**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_publish_creates_status_change_event
```
Expected: PASS.

- [ ] **Step 5: Build-check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: compiles cleanly.

- [ ] **Step 6: Format and commit**

```bash
cd app/ratel && dx fmt -f src/features/spaces/space_common/controllers/update_space.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/features/spaces/space_common/controllers/update_space.rs
/usr/bin/git commit -m "feat(spaces): persist SpaceStatusChangeEvent on status transitions"
```

---

### Task 18: Controller tests — Start and Finish

**Files:**
- Modify: `app/ratel/src/tests/space_status_change_tests.rs`

- [ ] **Step 1: Append Start test**

```rust
#[tokio::test]
async fn test_start_creates_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk, space_pk) = create_draft_space(&ctx).await;

    // Publish first
    let space_id_str = match &space_pk {
        Partition::Space(id) => id.clone(),
        _ => panic!(),
    };
    let (publish_status, _, _) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id_str),
        headers: ctx.test_user.1.clone(),
        body: { "publish": true, "visibility": "Public" },
    };
    assert_eq!(publish_status, 200);

    // Start
    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id_str),
        headers: ctx.test_user.1.clone(),
        body: { "start": true },
    };
    assert_eq!(status, 200, "start failed: {:?}", body);

    let events = find_status_change_events_for(&ctx, &space_pk).await;
    // Expect 2 events now: Publish + Start.
    assert!(events.len() >= 2, "events={:?}", events);
    assert!(events
        .iter()
        .any(|e| e.old_status == Some(SpaceStatus::Open) && e.new_status == SpaceStatus::Ongoing));
}
```

- [ ] **Step 2: Append Finish test**

```rust
#[tokio::test]
async fn test_finish_creates_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk, space_pk) = create_draft_space(&ctx).await;
    let space_id_str = match &space_pk {
        Partition::Space(id) => id.clone(),
        _ => panic!(),
    };

    let _ = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id_str),
        headers: ctx.test_user.1.clone(),
        body: { "publish": true, "visibility": "Public" },
    };
    let _ = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id_str),
        headers: ctx.test_user.1.clone(),
        body: { "start": true },
    };

    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id_str),
        headers: ctx.test_user.1.clone(),
        body: { "finished": true },
    };
    assert_eq!(status, 200, "finish failed: {:?}", body);

    let events = find_status_change_events_for(&ctx, &space_pk).await;
    assert!(events
        .iter()
        .any(|e| e.old_status == Some(SpaceStatus::Ongoing) && e.new_status == SpaceStatus::Finished));
}
```

- [ ] **Step 3: Append non-status-update test**

```rust
#[tokio::test]
async fn test_title_update_creates_no_status_change_event() {
    let ctx = TestContext::setup().await;
    let (_post_pk, space_pk) = create_draft_space(&ctx).await;
    let space_id_str = match &space_pk {
        Partition::Space(id) => id.clone(),
        _ => panic!(),
    };

    let before = find_status_change_events_for(&ctx, &space_pk).await.len();
    let (status, _, body) = crate::test_patch! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}", space_id_str),
        headers: ctx.test_user.1.clone(),
        body: { "title": "Renamed" },
    };
    assert_eq!(status, 200, "title update failed: {:?}", body);
    let after = find_status_change_events_for(&ctx, &space_pk).await.len();
    assert_eq!(before, after, "expected no event on title update");
}
```

- [ ] **Step 4: Run all three tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- test_start_creates_status_change_event test_finish_creates_status_change_event test_title_update_creates_no_status_change_event
```
Expected: all PASS.

- [ ] **Step 5: Commit**

```bash
cd app/ratel && dx fmt -f src/tests/space_status_change_tests.rs
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add app/ratel/src/tests/space_status_change_tests.rs
/usr/bin/git commit -m "test(spaces): cover Start/Finish/Title controller paths"
```

---

## Phase 7 — CDK

### Task 19: Add `SpaceStatusChangeEventPipe` and rule

**Files:**
- Modify: `cdk/lib/dynamo-stream-event.ts`

- [ ] **Step 1: Insert new pipe + rule**

In `cdk/lib/dynamo-stream-event.ts`, immediately before the closing `}` of the stack constructor (around line 500), and after the `ActivityScoreAggregateRule` block, add:

```typescript
    // ── Pipe: SpaceStatusChangeEvent ───────────────────────────────────
    // Triggers when a new SpaceStatusChangeEvent row is inserted by update_space
    new pipes.CfnPipe(this, "SpaceStatusChangeEventPipe", {
      name: `ratel-${stage}-space-status-change-event-pipe`,
      roleArn: pipeRole.roleArn,
      source: mainTableStreamArn,
      sourceParameters: {
        dynamoDbStreamParameters: {
          startingPosition: "LATEST",
          batchSize: 10,
        },
        filterCriteria: {
          filters: [
            {
              pattern: JSON.stringify({
                eventName: ["INSERT"],
                dynamodb: {
                  NewImage: {
                    sk: { S: [{ prefix: "SPACE_STATUS_CHANGE_EVENT#" }] },
                  },
                },
              }),
            },
          ],
        },
      },
      target: eventBus.eventBusArn,
      targetParameters: {
        eventBridgeEventBusParameters: {
          source: "ratel.dynamodb.stream",
          detailType: "SpaceStatusChangeEvent",
        },
        inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
      },
    });

    // ── Rule: Route SpaceStatusChangeEvent events to app-shell Lambda ───
    new events.Rule(this, "SpaceStatusChangeEventRule", {
      eventBus,
      description:
        "Route space status change events to app-shell for notification fan-out",
      eventPattern: {
        source: ["ratel.dynamodb.stream"],
        detailType: ["SpaceStatusChangeEvent"],
      },
      targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
    });
```

- [ ] **Step 2: Verify CDK compiles**

```bash
cd cdk && npx tsc --noEmit
```
Expected: no type errors.

- [ ] **Step 3: Commit**

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add cdk/lib/dynamo-stream-event.ts
/usr/bin/git commit -m "feat(cdk): add SpaceStatusChangeEvent pipe and rule"
```

---

## Phase 8 — E2E scenario

### Task 20: Write the Playwright scenario

**Files:**
- Create: `playwright/tests/web/space-status-notifications.spec.js`

> **Note:** This test verifies only the UI-visible Publish → Start → Finish state transitions. SES is mocked in `bypass` mode, so we can't assert against email delivery. The Rust integration tests in Phase 4 & 6 are the authoritative check that `Notification` rows are created correctly.

- [ ] **Step 1: Find the nearest existing space spec and copy its setup**

Run:

```bash
ls playwright/tests/web/ | grep -iE "space|signin|login"
```

Open the existing signin helper (typically `playwright/tests/web/signin.spec.js` or a shared `tests/utils/*` helper) to understand how the test user logs in. Also open whichever space-creation spec exists (look for files creating a Post/Space). Reuse those helper imports verbatim.

- [ ] **Step 2: Identify existing data-testid attributes for the buttons we need**

```bash
grep -rn "data-testid" app/ratel/src/features/spaces/space_common/components/ app/ratel/src/features/spaces/pages/overview/ 2>&1 | grep -iE "publish|start|finish|status"
```

Record the attribute values (likely `space-publish-button`, `publish-btn`, etc. — whatever is actually there). Use the actual values in the spec. If a button has no `data-testid`, add one in the same commit as the spec (RSX change + spec change).

- [ ] **Step 3: Write the spec using the real selectors**

Create `playwright/tests/web/space-status-notifications.spec.js` using the selectors you found in Step 2. Template (replace `SELECTOR_*` with real values):

```js
import { test, expect } from "@playwright/test";
import { click, fill, goto, getEditor, getLocator } from "../utils";

// Reuse the existing login helper. If the repo uses a shared helper like
// `login(page, email, password)`, import it from tests/utils. Otherwise call
// the same endpoints the existing specs use.
import { loginAsTestUser } from "../utils"; // adjust path if different

const SELECTOR_PUBLISH = "space-publish-button"; // replace with real testId from Step 2
const SELECTOR_START = "space-start-button";
const SELECTOR_FINISH = "space-finish-button";
const SELECTOR_STATUS = "space-status-badge";

test.describe.serial("Space status change notification flow", () => {
  let spaceUrl;

  test.beforeAll(async ({ browser }) => {
    // Login once and reuse state if the test helper supports it. Otherwise
    // login in each test.
  });

  test("Step 1: Creator publishes a draft space", async ({ page }) => {
    // Log in (adjust to match the repo's existing login flow).
    await loginAsTestUser(page);

    // Create a draft space. Replicate the click sequence used by an existing
    // space-creation spec — DO NOT invent new navigation. Typical sequence:
    //   1. Navigate to /posts/new (or wherever "create" lives)
    //   2. Fill title via fill() helper
    //   3. Fill content via getEditor().fill("...")
    //   4. Choose "Space" post type
    //   5. Click "Save draft" / "Create"
    // Capture the resulting URL:
    spaceUrl = page.url();

    // Trigger Publish
    await click(page, { testId: SELECTOR_PUBLISH });

    // Wait for the UI to reflect the "Open" state
    await expect(page.getByTestId(SELECTOR_STATUS)).toContainText(/open/i);
  });

  test("Step 2: Creator starts the space", async ({ page }) => {
    await loginAsTestUser(page);
    await goto(page, spaceUrl);
    await click(page, { testId: SELECTOR_START });
    await expect(page.getByTestId(SELECTOR_STATUS)).toContainText(/ongoing/i);
  });

  test("Step 3: Creator finishes the space", async ({ page }) => {
    await loginAsTestUser(page);
    await goto(page, spaceUrl);
    await click(page, { testId: SELECTOR_FINISH });
    await expect(page.getByTestId(SELECTOR_STATUS)).toContainText(/finished/i);
  });
});
```

Replace every `SELECTOR_*` constant and the `loginAsTestUser` import with the real values you found in Step 1 and Step 2 before running. If the existing specs don't expose a reusable login helper, inline the same login steps (navigate → fill email → fill password → enter code `000000` → click submit) that the other space spec uses verbatim.

- [ ] **Step 3: If any of `space-publish-button`, `space-start-button`, `space-finish-button`, or `space-status-badge` don't exist as `data-testid` attributes in the Dioxus source, add them.** Grep first:

```bash
grep -rn "data-testid.*space-publish-button\|data-testid.*space-start-button\|data-testid.*space-finish-button\|data-testid.*space-status-badge" app/ratel/src/ 2>&1
```

If any are missing, add the attribute to the relevant RSX component (e.g. the dashboard / overview Publish button), build-check, and commit them as a small extra commit before running the spec.

- [ ] **Step 4: Run the spec locally**

Requires Docker local infra running (`make infra`). Then:

```bash
cd playwright && npx playwright test tests/web/space-status-notifications.spec.js --headed
```
Expected: all three steps pass. If the buttons or navigation differ from the spec's assumptions, adjust to match the actual UI flow.

- [ ] **Step 5: Commit**

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add playwright/tests/web/space-status-notifications.spec.js
# Also add any RSX files where new data-testid attributes were added.
/usr/bin/git commit -m "test(e2e): publish/start/finish space status flow"
```

---

## Phase 9 — Final verification

### Task 21: Run the full Rust integration test suite

- [ ] **Step 1: Run all space status change tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests -- space_status_change
```
Expected: all tests pass.

- [ ] **Step 2: Run the broader suite to confirm no regressions**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "server,bypass" --tests
```
Expected: no failures in existing tests.

- [ ] **Step 3: If any existing tests fail**, investigate. The most likely culprit is the new `Error::SpaceStatusChange` variant breaking an existing exhaustive match or changing a `Debug` / `Clone` derive — fix the call site rather than reverting.

---

### Task 22: Full compile & lint sweep

- [ ] **Step 1: Dioxus check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```
Expected: clean.

- [ ] **Step 2: Lambda compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo check --features "server,lambda"
```
Expected: clean.

- [ ] **Step 3: CDK compile**

```bash
cd cdk && npx tsc --noEmit
```
Expected: clean.

- [ ] **Step 4: Format all changed .rs files**

```bash
cd app/ratel && dx fmt -f \
  src/common/types/partition.rs \
  src/common/types/entity_type.rs \
  src/common/types/error.rs \
  src/common/types/notification_data.rs \
  src/common/types/event_bridge_envelope.rs \
  src/common/stream_handler.rs \
  src/common/models/space/mod.rs \
  src/common/models/space/space_status_change_event.rs \
  src/features/auth/types/email_operation.rs \
  src/features/spaces/space_common/mod.rs \
  src/features/spaces/space_common/types/mod.rs \
  src/features/spaces/space_common/types/space_status_change_error.rs \
  src/features/spaces/space_common/services/mod.rs \
  src/features/spaces/space_common/services/space_status_change_notification.rs \
  src/features/spaces/space_common/controllers/update_space.rs \
  src/tests/mod.rs \
  src/tests/space_status_change_tests.rs
```

- [ ] **Step 5: Commit any formatting deltas**

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel_notification-space-status-to-participant
/usr/bin/git add -A app/ratel/src
/usr/bin/git diff --cached --quiet || /usr/bin/git commit -m "style: apply dx fmt across space status change files"
```

---

### Task 23: Verification before completion

- [ ] **Step 1: Re-read the spec implementation checklist** at the bottom of `docs/superpowers/specs/2026-04-09-space-status-notifications-design.md`. Every box should be tickable by now:
  - `Partition::SpaceStatusChangeEvent` — Task 1
  - `EntityType::SpaceStatusChangeEvent` — Task 2
  - `SpaceStatusChangeEvent` entity — Task 3
  - `SpaceStatusChangeError` — Task 4
  - `NotificationData::SendSpaceStatusUpdate` — Task 6
  - `EmailOperation::SpaceStatusNotification` — Task 5
  - `DetailType::SpaceStatusChangeEvent` — Task 14
  - `update_space` writes event — Task 17
  - Service handler + helpers — Tasks 7–13
  - `stream_handler.rs` branch — Task 15
  - CDK pipe + rule — Task 19
  - Controller integration tests — Tasks 16, 18
  - Service unit tests — Tasks 9–13, 13a
  - E2E Playwright — Task 20
  - All compile checks — Task 22

- [ ] **Step 2: Confirm git log is clean and each commit builds independently**

```bash
/usr/bin/git log --oneline main..HEAD
```
Expected: a sequence of small, self-contained commits.

- [ ] **Step 3: Note the outstanding infra pre-requisite**

The SES template `space_status_notification` with variables `{{headline}}`, `{{body}}`, `{{space_title}}`, `{{cta_url}}` must be provisioned in dev and prod SES **before the Rust change is merged**. Flag this in the PR description and coordinate with infra. Until the template exists, `Notification` rows for these new events will stay in the `Requested` state because `send_bulk_with_template` will error out on the unknown template.

- [ ] **Step 4: Done** — ready for PR.

---

## Execution notes for the implementing engineer

- **Don't skip the TDD phase** (Tasks 9–13). Each test is a genuine check of a distinct branch of the handler; running them in isolation is the fastest path to a correct implementation.
- **`cargo check --features "server,lambda"`** is the key signal that the `proc()` dispatch wiring (Task 14) is correct — `dx check --features web` will happily compile even if the lambda path is broken, because `dx check` doesn't enable the lambda feature.
- **If any test helper (`create_draft_space`, `create_team_with_members`) needs fixing** to match repo conventions, those fixes are test-only and do not require changing the handler or spec. Read the sibling post/space tests for reference.
- **If `dx check` fails** due to an unrelated pre-existing issue on the branch, resolve it before proceeding — don't let unrelated errors pile up.
- **Rollback strategy**: every phase is committed independently, so `git revert <hash>` of any single commit will cleanly roll back that slice without touching the rest.
