# Notification Inbox Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship a user-wide notification inbox surfaced via a bell icon + slide-in panel on both the home page and every Space arena index page. Covers event kinds `ReplyOnComment`, `MentionInComment`, `SpaceStatusChanged`, `SpaceInvitation` (the four kinds already on email pipelines) for the MVP.

**Architecture:** A new `UserInboxNotification` DynamoEntity stores per-user inbox rows with a sparse GSI on unread status. A shared helper `common::utils::inbox::create_inbox_row` is called alongside each existing email-send site. Four REST endpoints power list / unread-count / mark-read / mark-all-read. The frontend is a `NotificationBell` (badge) + `NotificationPanel` (slide-in) pair reused on home and inside `SpaceIndexPage` via the existing `ActivePanel` pattern.

**Tech Stack:** Rust 2024 edition, Dioxus 0.7 fullstack (RSX macro, TailwindCSS v4), Axum 0.8.1, DynamoDB single-table + GSI, `rmcp` for MCP tools, Playwright for e2e.

**Spec:** `docs/superpowers/specs/2026-04-20-notification-inbox-design.md`

**Out of scope for this plan (follow-up plans):**
- Events E (`NewCommentOnMine`), F (`FollowedUserPosted`), G (`NewSpaceAction`), H (`RewardGranted`)
- EventBridge fan-out (CDK Pipes + Rules)
- Per-kind user preference toggles

These will be addressed in `docs/superpowers/plans/2026-04-21-notification-inbox-fanout.md` after this MVP lands.

---

## File Structure

### New files

| Path | Responsibility |
|---|---|
| `app/ratel/src/common/types/inbox_kind.rs` | `InboxKind` enum (MVP: 4 variants) + `InboxPayload` tagged enum |
| `app/ratel/src/common/models/notification/user_inbox_notification.rs` | `UserInboxNotification` DynamoEntity + constructors, mark-read helpers |
| `app/ratel/src/common/models/notification/inbox_dedup_marker.rs` | `InboxDedupMarker` DynamoEntity for idempotency (7-day TTL) |
| `app/ratel/src/common/utils/inbox.rs` | `create_inbox_row` + `create_inbox_row_once` shared helpers |
| `app/ratel/src/features/notifications/mod.rs` | Feature module index |
| `app/ratel/src/features/notifications/route.rs` | Route registration (`axum::Router`) |
| `app/ratel/src/features/notifications/types/mod.rs` | `mod error; mod response;` |
| `app/ratel/src/features/notifications/types/error.rs` | `NotificationsError` typed enum |
| `app/ratel/src/features/notifications/types/response.rs` | `InboxNotificationResponse`, `UnreadCountResponse`, `MarkAllReadResponse` |
| `app/ratel/src/features/notifications/controllers/mod.rs` | Controller module index |
| `app/ratel/src/features/notifications/controllers/list_inbox.rs` | `GET /api/inbox?unread_only&bookmark` |
| `app/ratel/src/features/notifications/controllers/get_unread_count.rs` | `GET /api/inbox/unread-count` |
| `app/ratel/src/features/notifications/controllers/mark_read.rs` | `POST /api/inbox/{inbox_id}/read` |
| `app/ratel/src/features/notifications/controllers/mark_all_read.rs` | `POST /api/inbox/read-all` |
| `app/ratel/src/features/notifications/hooks/mod.rs` | Hook module index |
| `app/ratel/src/features/notifications/hooks/use_inbox.rs` | `use_inbox` infinite query |
| `app/ratel/src/features/notifications/hooks/use_unread_count.rs` | `use_unread_count` loader + 60s polling |
| `app/ratel/src/features/notifications/components/mod.rs` | Component module index |
| `app/ratel/src/features/notifications/components/notification_bell/mod.rs` | Bell module |
| `app/ratel/src/features/notifications/components/notification_bell/component.rs` | Bell + badge + onclick |
| `app/ratel/src/features/notifications/components/notification_bell/style.css` | Bell / badge styling (dark+light space toggle) |
| `app/ratel/src/features/notifications/components/notification_panel/mod.rs` | Panel module |
| `app/ratel/src/features/notifications/components/notification_panel/component.rs` | Slide-in panel + list + "Mark all as read" |
| `app/ratel/src/features/notifications/components/notification_panel/style.css` | Panel styles |
| `app/ratel/src/features/notifications/components/notification_panel/notification_item/mod.rs` | Item module |
| `app/ratel/src/features/notifications/components/notification_panel/notification_item/component.rs` | Per-kind rendering + onclick |
| `app/ratel/src/features/notifications/components/notification_panel/notification_item/style.css` | Item styles |
| `app/ratel/src/features/notifications/i18n.rs` | EN/KO translations |
| `app/ratel/assets/design/notification-panel.html` | HTML mockup for user approval (HTML-first) |
| `app/ratel/src/tests/notifications_tests.rs` | Server integration tests |
| `app/ratel/src/tests/inbox_helper_tests.rs` | `create_inbox_row(_once)` tests |
| `playwright/tests/web/notifications.spec.js` | E2E scenario |

### Modified files

| Path | Change |
|---|---|
| `app/ratel/src/common/types/partition.rs` | Add `UserInboxNotification(String)` + `InboxDedupMarker(String)` variants |
| `app/ratel/src/common/types/entity_type.rs` | Add `UserInboxNotification(String)` + `InboxDedupMarker(String)` variants |
| `app/ratel/src/common/types/mod.rs` | `pub mod inbox_kind; pub use inbox_kind::*;` |
| `app/ratel/src/common/models/notification/mod.rs` | `mod user_inbox_notification; mod inbox_dedup_marker; pub use ...;` |
| `app/ratel/src/common/utils/mod.rs` | `pub mod inbox;` |
| `app/ratel/src/common/utils/reply_notification.rs` | Wire `create_inbox_row_once` per recipient (ReplyOnComment) |
| `app/ratel/src/common/utils/mention.rs` | Wire `create_inbox_row_once` per mention (MentionInComment) |
| `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs` | Wire `create_inbox_row` per participant (SpaceStatusChanged) |
| `app/ratel/src/features/spaces/pages/apps/apps/general/controllers/invite_space_participants.rs` | Wire `create_inbox_row` per invited user (SpaceInvitation) |
| `app/ratel/src/features/mod.rs` | `pub mod notifications;` |
| `app/ratel/src/app.rs` | Register `notifications::route::router()` |
| `app/ratel/src/common/types/error.rs` | Register `NotificationsError` with `#[from]` + `#[translate(from)]` |
| `app/ratel/src/common/mcp/server.rs` | Register `list_inbox` + `get_unread_count` tools |
| `app/ratel/src/common/components/navbar/component.rs` | Insert `NotificationBell` |
| `app/ratel/src/features/spaces/pages/index/component.rs` | Add `ActivePanel::Notifications`, render `NotificationPanel` |
| `app/ratel/src/features/spaces/pages/index/arena_topbar/component.rs` | Insert `NotificationBell` with onclick → `active_panel.set(ActivePanel::Notifications)` |
| `app/ratel/src/tests/mod.rs` | `mod notifications_tests; mod inbox_helper_tests;` |
| `Cargo.toml` at `app/ratel` | Add `notifications` feature (included in `full`) |

---

## Conventions used throughout this plan

**Per `.claude/rules/conventions/`:**
- Path params + DTOs use SubPartition types (`UserInboxNotificationEntityType`) — never raw `Partition`/`EntityType`.
- Typed error enums with `#[derive(Translate)]` — never `Error::BadRequest(String)`.
- Semantic color tokens (`bg-card-bg`, `text-foreground`, `bg-primary/10`) — never `text-gray-500` etc.
- Primitive components (`Button`, `Row`, `Col`, `Card`) — never raw `<button>`/`<div class="flex">`.
- All user-facing strings via `translate!`.
- Sleep via `crate::common::utils::time::sleep` — never `gloo_timers` direct.
- Navigation via `Route` enum, never format strings.
- Place `nav.push()` **after** all `.await` points in async handlers.

**Per commit:** lint + format every changed `.rs`:
```bash
rustywind --custom-regex "class: \"(.*)\"" --write <path>
cd app/ratel && dx fmt -f <relative/path/to/file.rs>
```

**Build check after each phase:**
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

**Single test run:**
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- <test_name>
```

**Commit cadence:** commit at the end of every task. Never skip hooks (`--no-verify`).

**Branch:** work continues on `feature/notification-reply-on-comment`. Push to `hackartists` fork per project convention.

---

## Phase 1 — Data model

### Task 1: Add Partition and EntityType variants

**Files:**
- Modify: `app/ratel/src/common/types/partition.rs`
- Modify: `app/ratel/src/common/types/entity_type.rs`

- [ ] **Step 1: Add `UserInboxNotification` + `InboxDedupMarker` variants to `Partition`**

In `app/ratel/src/common/types/partition.rs`, locate the `Notification(String)` variant and add directly below:
```rust
    UserInboxNotification(String), // user_pk
    InboxDedupMarker(String),      // user_pk
```

- [ ] **Step 2: Add matching `EntityType` variants**

In `app/ratel/src/common/types/entity_type.rs`, add near the existing `UserNotification(String)` variant:
```rust
    UserInboxNotification(String), // uuid_v7, time-ordered
    InboxDedupMarker(String),      // "{kind}#{source_hash}"
```

- [ ] **Step 3: Compile check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/partition.rs app/ratel/src/common/types/entity_type.rs
git commit -m "feat(notifications): add UserInboxNotification / InboxDedupMarker partition+entity variants"
```

---

### Task 2: Create `InboxKind` + `InboxPayload` types

**Files:**
- Create: `app/ratel/src/common/types/inbox_kind.rs`
- Modify: `app/ratel/src/common/types/mod.rs`

- [ ] **Step 1: Write the payload file**

Create `app/ratel/src/common/types/inbox_kind.rs`:
```rust
use crate::common::*;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(rename_all = "snake_case")]
pub enum InboxKind {
    ReplyOnComment,
    MentionInComment,
    SpaceStatusChanged,
    SpaceInvitation,
}

impl Default for InboxKind {
    fn default() -> Self {
        Self::ReplyOnComment
    }
}

impl InboxKind {
    pub fn as_prefix(&self) -> &'static str {
        match self {
            InboxKind::ReplyOnComment => "REPLY",
            InboxKind::MentionInComment => "MENTION",
            InboxKind::SpaceStatusChanged => "SPACE_STATUS",
            InboxKind::SpaceInvitation => "SPACE_INV",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum InboxPayload {
    ReplyOnComment {
        space_id: Option<SpacePartition>,
        post_id: Option<FeedPartition>,
        comment_preview: String,
        replier_name: String,
        replier_profile_url: String,
        cta_url: String,
    },
    MentionInComment {
        comment_preview: String,
        mentioned_by_name: String,
        cta_url: String,
    },
    SpaceStatusChanged {
        space_id: SpacePartition,
        space_title: String,
        new_status: SpaceStatus,
        cta_url: String,
    },
    SpaceInvitation {
        space_id: SpacePartition,
        space_title: String,
        inviter_name: String,
        cta_url: String,
    },
}

impl Default for InboxPayload {
    fn default() -> Self {
        Self::ReplyOnComment {
            space_id: None,
            post_id: None,
            comment_preview: String::new(),
            replier_name: String::new(),
            replier_profile_url: String::new(),
            cta_url: String::new(),
        }
    }
}

impl InboxPayload {
    pub fn kind(&self) -> InboxKind {
        match self {
            InboxPayload::ReplyOnComment { .. } => InboxKind::ReplyOnComment,
            InboxPayload::MentionInComment { .. } => InboxKind::MentionInComment,
            InboxPayload::SpaceStatusChanged { .. } => InboxKind::SpaceStatusChanged,
            InboxPayload::SpaceInvitation { .. } => InboxKind::SpaceInvitation,
        }
    }
}
```

- [ ] **Step 2: Register in `types/mod.rs`**

In `app/ratel/src/common/types/mod.rs`, add:
```rust
pub mod inbox_kind;
pub use inbox_kind::*;
```

- [ ] **Step 3: Compile check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/inbox_kind.rs app/ratel/src/common/types/mod.rs
git commit -m "feat(notifications): add InboxKind + InboxPayload types"
```

---

### Task 3: Create `UserInboxNotification` entity

**Files:**
- Create: `app/ratel/src/common/models/notification/user_inbox_notification.rs`
- Modify: `app/ratel/src/common/models/notification/mod.rs`

- [ ] **Step 1: Write the entity**

Create `app/ratel/src/common/models/notification/user_inbox_notification.rs`:
```rust
use crate::common::*;

pub const INBOX_TTL_DAYS: i64 = 90;
pub const UNREAD_SENTINEL: &str = "R";

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UserInboxNotification {
    #[dynamo(
        prefix = "UIN",
        index = "gsi1",
        name = "find_inbox_unread_by_user",
        pk
    )]
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // UserInboxNotification(uuid_v7)

    pub created_at: i64,

    /// Sparse GSI sort key. `"U#{created_at}"` while unread, `"R"` when read —
    /// entries with `"R"` are filtered out of the GSI query to implement cheap
    /// unread-only listing.
    #[dynamo(index = "gsi1", sk)]
    pub unread_created_at: String,

    pub is_read: bool,

    pub kind: InboxKind,
    pub payload: InboxPayload,

    /// DynamoDB TTL field (epoch seconds). `created_at_ms/1000 + 90*86400`.
    pub expires_at: i64,
}

#[cfg(feature = "server")]
impl UserInboxNotification {
    pub fn new(recipient_pk: Partition, payload: InboxPayload) -> Self {
        let uid = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
        let now_ms = crate::common::utils::time::get_now_timestamp_millis();
        let expires_at = (now_ms / 1000) + INBOX_TTL_DAYS * 86_400;

        Self {
            pk: recipient_pk,
            sk: EntityType::UserInboxNotification(uid),
            created_at: now_ms,
            unread_created_at: format!("U#{now_ms:020}"),
            is_read: false,
            kind: payload.kind(),
            payload,
            expires_at,
        }
    }
}
```

- [ ] **Step 2: Re-export in `notification/mod.rs`**

In `app/ratel/src/common/models/notification/mod.rs`:
```rust
mod notification;
mod user_inbox_notification;

pub use notification::*;
pub use user_inbox_notification::*;
```

- [ ] **Step 3: Compile check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```
Expected: PASS. The `DynamoEntity` derive generates `get`, `create`, `updater`, `find_by_pk`, `find_inbox_unread_by_user` (GSI) automatically.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/models/notification/user_inbox_notification.rs \
        app/ratel/src/common/models/notification/mod.rs
git commit -m "feat(notifications): add UserInboxNotification entity with sparse unread GSI"
```

---

### Task 4: Create `InboxDedupMarker` entity

**Files:**
- Create: `app/ratel/src/common/models/notification/inbox_dedup_marker.rs`
- Modify: `app/ratel/src/common/models/notification/mod.rs`

- [ ] **Step 1: Write the entity**

Create `app/ratel/src/common/models/notification/inbox_dedup_marker.rs`:
```rust
use crate::common::*;

pub const DEDUP_TTL_DAYS: i64 = 7;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct InboxDedupMarker {
    #[dynamo(prefix = "INBOX_DEDUP", pk)]
    pub pk: Partition, // User(user_id)

    pub sk: EntityType, // InboxDedupMarker("{kind_prefix}#{source_hash}")

    pub created_at: i64,

    /// DynamoDB TTL field (epoch seconds).
    pub expires_at: i64,
}

#[cfg(feature = "server")]
impl InboxDedupMarker {
    pub fn new(recipient_pk: Partition, kind: InboxKind, source_id: &str) -> Self {
        let now_ms = crate::common::utils::time::get_now_timestamp_millis();
        let expires_at = (now_ms / 1000) + DEDUP_TTL_DAYS * 86_400;
        let hash = Self::hash_source(source_id);
        Self {
            pk: recipient_pk,
            sk: EntityType::InboxDedupMarker(format!("{}#{hash}", kind.as_prefix())),
            created_at: now_ms,
            expires_at,
        }
    }

    fn hash_source(source_id: &str) -> String {
        use std::hash::{Hash, Hasher};
        let mut h = std::collections::hash_map::DefaultHasher::new();
        source_id.hash(&mut h);
        format!("{:016x}", h.finish())
    }
}
```

- [ ] **Step 2: Register in `notification/mod.rs`**

Append:
```rust
mod inbox_dedup_marker;
pub use inbox_dedup_marker::*;
```

- [ ] **Step 3: Compile check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/models/notification/inbox_dedup_marker.rs \
        app/ratel/src/common/models/notification/mod.rs
git commit -m "feat(notifications): add InboxDedupMarker entity for idempotency"
```

---

## Phase 2 — Shared helper

### Task 5: `create_inbox_row` + `create_inbox_row_once`

**Files:**
- Create: `app/ratel/src/common/utils/inbox.rs`
- Modify: `app/ratel/src/common/utils/mod.rs`
- Test: `app/ratel/src/tests/inbox_helper_tests.rs`
- Modify: `app/ratel/src/tests/mod.rs`

- [ ] **Step 1: Write the helper**

Create `app/ratel/src/common/utils/inbox.rs`:
```rust
#[cfg(feature = "server")]
use crate::common::*;
#[cfg(feature = "server")]
use crate::common::models::notification::{InboxDedupMarker, UserInboxNotification};

/// Write an inbox row. Non-fatal: logs `error!` on DynamoDB failure and returns Ok.
#[cfg(feature = "server")]
pub async fn create_inbox_row(
    recipient_pk: Partition,
    payload: InboxPayload,
) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let row = UserInboxNotification::new(recipient_pk.clone(), payload);
    if let Err(e) = row.create(cli).await {
        crate::error!(
            "create_inbox_row: failed for user={}: {e}",
            recipient_pk
        );
    }
    Ok(())
}

/// Idempotent inbox row creator. Uses `InboxDedupMarker` as a 7-day lock
/// keyed on `(recipient, kind, source_id)`. If the marker already exists,
/// the inbox row is skipped.
#[cfg(feature = "server")]
pub async fn create_inbox_row_once(
    recipient_pk: Partition,
    payload: InboxPayload,
    source_id: &str,
) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let kind = payload.kind();
    let marker = InboxDedupMarker::new(recipient_pk.clone(), kind, source_id);

    match InboxDedupMarker::get(cli, &marker.pk, Some(marker.sk.clone())).await {
        Ok(Some(_)) => {
            tracing::debug!(
                "create_inbox_row_once: skipped duplicate for user={} kind={:?} source={}",
                recipient_pk, kind, source_id
            );
            return Ok(());
        }
        Ok(None) => {}
        Err(e) => {
            crate::error!(
                "create_inbox_row_once: dedup lookup failed, proceeding anyway: {e}"
            );
        }
    }

    let row = UserInboxNotification::new(recipient_pk.clone(), payload);
    if let Err(e) = row.create(cli).await {
        crate::error!(
            "create_inbox_row_once: row create failed for user={}: {e}",
            recipient_pk
        );
        return Ok(());
    }

    if let Err(e) = marker.create(cli).await {
        crate::error!(
            "create_inbox_row_once: marker create failed (row created, dedup broken): {e}"
        );
    }
    Ok(())
}
```

- [ ] **Step 2: Register module**

In `app/ratel/src/common/utils/mod.rs`, add:
```rust
#[cfg(feature = "server")]
pub mod inbox;
```

- [ ] **Step 3: Write failing test**

Create `app/ratel/src/tests/inbox_helper_tests.rs`:
```rust
use super::*;
use crate::common::types::{InboxPayload, Partition};
use crate::common::utils::inbox::{create_inbox_row, create_inbox_row_once};
use crate::common::models::notification::UserInboxNotification;

fn dummy_payload() -> InboxPayload {
    InboxPayload::MentionInComment {
        comment_preview: "hello".into(),
        mentioned_by_name: "alice".into(),
        cta_url: "/posts/abc".into(),
    }
}

#[tokio::test]
async fn test_create_inbox_row_persists_row() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row(user_pk.clone(), dummy_payload())
        .await
        .unwrap();

    let (rows, _) = UserInboxNotification::find_by_pk(&ctx.ddb, &user_pk, UserInboxNotification::opt())
        .await
        .unwrap();
    assert_eq!(rows.len(), 1);
    assert!(!rows[0].is_read);
    assert!(rows[0].unread_created_at.starts_with("U#"));
}

#[tokio::test]
async fn test_create_inbox_row_once_dedups_on_same_source() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-123").await.unwrap();
    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-123").await.unwrap();

    let (rows, _) = UserInboxNotification::find_by_pk(&ctx.ddb, &user_pk, UserInboxNotification::opt())
        .await
        .unwrap();
    assert_eq!(rows.len(), 1, "second call should be deduped");
}

#[tokio::test]
async fn test_create_inbox_row_once_distinct_sources_produce_rows() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-1").await.unwrap();
    create_inbox_row_once(user_pk.clone(), dummy_payload(), "comment-2").await.unwrap();

    let (rows, _) = UserInboxNotification::find_by_pk(&ctx.ddb, &user_pk, UserInboxNotification::opt())
        .await
        .unwrap();
    assert_eq!(rows.len(), 2);
}
```

Register in `app/ratel/src/tests/mod.rs`:
```rust
mod inbox_helper_tests;
```

- [ ] **Step 4: Run tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- inbox_helper
```
Expected: 3 tests pass.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/common/utils/inbox.rs app/ratel/src/common/utils/mod.rs \
        app/ratel/src/tests/inbox_helper_tests.rs app/ratel/src/tests/mod.rs
git commit -m "feat(notifications): add create_inbox_row helpers with dedup marker"
```

---

## Phase 3 — Server API

### Task 6: Error + response DTOs + feature module scaffolding

**Files:**
- Create: `app/ratel/src/features/notifications/mod.rs`
- Create: `app/ratel/src/features/notifications/types/mod.rs`
- Create: `app/ratel/src/features/notifications/types/error.rs`
- Create: `app/ratel/src/features/notifications/types/response.rs`
- Modify: `app/ratel/src/features/mod.rs`
- Modify: `app/ratel/src/common/types/error.rs`

- [ ] **Step 1: Error type**

Create `app/ratel/src/features/notifications/types/error.rs`:
```rust
use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum NotificationsError {
    #[error("inbox entry not found")]
    #[translate(en = "Notification not found", ko = "알림을 찾을 수 없습니다")]
    InboxEntryNotFound,

    #[error("mark-read failed")]
    #[translate(en = "Failed to mark as read", ko = "읽음 처리에 실패했습니다")]
    MarkReadFailed,

    #[error("list failed")]
    #[translate(en = "Failed to load notifications", ko = "알림 불러오기에 실패했습니다")]
    ListFailed,
}
```

- [ ] **Step 2: Response DTOs**

Create `app/ratel/src/features/notifications/types/response.rs`:
```rust
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct InboxNotificationResponse {
    pub id: UserInboxNotificationEntityType,
    pub kind: InboxKind,
    pub payload: InboxPayload,
    pub is_read: bool,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl From<crate::common::models::notification::UserInboxNotification>
    for InboxNotificationResponse
{
    fn from(n: crate::common::models::notification::UserInboxNotification) -> Self {
        Self {
            id: match n.sk {
                EntityType::UserInboxNotification(id) => UserInboxNotificationEntityType(id),
                _ => UserInboxNotificationEntityType(String::new()),
            },
            kind: n.kind,
            payload: n.payload,
            is_read: n.is_read,
            created_at: n.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct UnreadCountResponse {
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MarkAllReadResponse {
    pub affected: i64,
    pub has_more: bool,
}
```

- [ ] **Step 3: Types mod**

Create `app/ratel/src/features/notifications/types/mod.rs`:
```rust
mod error;
mod response;

pub use error::*;
pub use response::*;
```

- [ ] **Step 4: Feature mod (skeleton for now)**

Create `app/ratel/src/features/notifications/mod.rs`:
```rust
pub mod types;

#[cfg(feature = "server")]
pub mod controllers;

#[cfg(feature = "server")]
pub mod route;

#[cfg(not(feature = "server"))]
pub mod hooks;

#[cfg(not(feature = "server"))]
pub mod components;

#[cfg(not(feature = "server"))]
pub mod i18n;

pub use types::*;
```

- [ ] **Step 5: Wire into features index**

In `app/ratel/src/features/mod.rs`, add:
```rust
pub mod notifications;
```

- [ ] **Step 6: Register error**

In `app/ratel/src/common/types/error.rs`, inside the `common::Error` enum, add:
```rust
    #[error("{0}")]
    #[translate(from)]
    Notifications(#[from] crate::features::notifications::types::NotificationsError),
```

- [ ] **Step 7: Compile check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```
Note: `controllers`, `route`, `hooks`, `components`, `i18n` modules don't exist yet — temporarily comment out those `pub mod` lines in `mod.rs`, or create empty module files with just `// placeholder`. Simpler: create placeholder files with `pub fn _placeholder() {}` so compilation succeeds. Remove placeholders in later tasks.

- [ ] **Step 8: Commit**

```bash
git add app/ratel/src/features/notifications/ app/ratel/src/features/mod.rs \
        app/ratel/src/common/types/error.rs
git commit -m "feat(notifications): scaffold feature module with error + response types"
```

---

### Task 7: `GET /api/inbox` list endpoint (TDD)

**Files:**
- Create: `app/ratel/src/features/notifications/controllers/mod.rs`
- Create: `app/ratel/src/features/notifications/controllers/list_inbox.rs`
- Modify: `app/ratel/src/tests/notifications_tests.rs` (create)
- Modify: `app/ratel/src/tests/mod.rs`

- [ ] **Step 1: Write failing integration test**

Create `app/ratel/src/tests/notifications_tests.rs`:
```rust
use super::*;
use crate::common::types::{InboxPayload, Partition};
use crate::common::utils::inbox::create_inbox_row;
use crate::features::notifications::types::InboxNotificationResponse;

fn reply_payload(content: &str) -> InboxPayload {
    InboxPayload::ReplyOnComment {
        space_id: None,
        post_id: None,
        comment_preview: content.into(),
        replier_name: "bob".into(),
        replier_profile_url: String::new(),
        cta_url: "/posts/xyz".into(),
    }
}

#[tokio::test]
async fn test_list_inbox_returns_rows_for_current_user() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row(user_pk.clone(), reply_payload("hi 1")).await.unwrap();
    create_inbox_row(user_pk.clone(), reply_payload("hi 2")).await.unwrap();

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox",
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<InboxNotificationResponse>,
    };
    assert_eq!(status, 200, "list: {:?}", body);
    assert_eq!(body.items.len(), 2);
    assert!(body.items.iter().all(|i| !i.is_read));
}

#[tokio::test]
async fn test_list_inbox_unauthenticated_fails() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: "/api/inbox",
    };
    assert_ne!(status, 200);
}
```

Register in `app/ratel/src/tests/mod.rs`:
```rust
mod notifications_tests;
```

- [ ] **Step 2: Run test — expect failure (route not registered)**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_list_inbox
```
Expected: 404 / FAIL.

- [ ] **Step 3: Implement the handler**

Create `app/ratel/src/features/notifications/controllers/mod.rs`:
```rust
pub mod list_inbox;
```

Create `app/ratel/src/features/notifications/controllers/list_inbox.rs`:
```rust
use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::notifications::types::{InboxNotificationResponse, NotificationsError};

#[get("/api/inbox?unread_only&bookmark")]
pub async fn list_inbox(
    session: Extension<tower_sessions::Session>,
    unread_only: Option<bool>,
    bookmark: Option<String>,
) -> Result<ListResponse<InboxNotificationResponse>> {
    let user_pk = require_user_pk(&session).await?;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let opts = UserInboxNotification::opt_with_bookmark(bookmark)
        .scan_index_forward(false)
        .limit(30);

    let (items, next) = if unread_only.unwrap_or(false) {
        UserInboxNotification::find_inbox_unread_by_user(cli, &user_pk, opts)
            .await
            .map_err(|e| {
                crate::error!("list_inbox unread GSI failed: {e}");
                NotificationsError::ListFailed
            })?
    } else {
        UserInboxNotification::find_by_pk(cli, &user_pk, opts)
            .await
            .map_err(|e| {
                crate::error!("list_inbox pk scan failed: {e}");
                NotificationsError::ListFailed
            })?
    };

    let items: Vec<InboxNotificationResponse> = items.into_iter().map(Into::into).collect();
    Ok((items, next).into())
}

async fn require_user_pk(session: &tower_sessions::Session) -> Result<Partition> {
    let user = crate::common::session::require_user(session).await?;
    Ok(user.pk)
}
```

Note: if `session::require_user` doesn't exist, use the existing session helper (search `features/auth` for the canonical pattern and copy). The `#[get]` macro imports and `Result` come via `crate::common::*`.

- [ ] **Step 4: Register route — stub the router for now**

Create `app/ratel/src/features/notifications/route.rs`:
```rust
use crate::common::*;

pub fn router() -> axum::Router {
    axum::Router::new()
        .route("/api/inbox", axum::routing::get(crate::features::notifications::controllers::list_inbox::list_inbox_handler))
}
```

Note: the generated handler ident follows `{fn_name}_handler` convention of the `#[get]` macro. Confirm by searching an existing controller (e.g., `get_post_handler`).

In `app/ratel/src/app.rs`, inside the app composition, add:
```rust
    .merge(crate::features::notifications::route::router())
```
(Place alongside the other `features::X::route::router()` merges.)

- [ ] **Step 5: Run test — expect pass**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_list_inbox
```
Expected: 2 tests pass.

- [ ] **Step 6: Lint + format**

```bash
cd app/ratel && dx fmt -f src/features/notifications/controllers/list_inbox.rs \
                       -f src/features/notifications/route.rs
```

- [ ] **Step 7: Commit**

```bash
git add app/ratel/src/features/notifications/controllers app/ratel/src/features/notifications/route.rs \
        app/ratel/src/app.rs app/ratel/src/tests/notifications_tests.rs app/ratel/src/tests/mod.rs
git commit -m "feat(notifications): add GET /api/inbox list endpoint"
```

---

### Task 8: `GET /api/inbox/unread-count`

**Files:**
- Create: `app/ratel/src/features/notifications/controllers/get_unread_count.rs`
- Modify: `app/ratel/src/features/notifications/controllers/mod.rs`
- Modify: `app/ratel/src/features/notifications/route.rs`
- Modify: `app/ratel/src/tests/notifications_tests.rs`

- [ ] **Step 1: Write failing test**

Append to `app/ratel/src/tests/notifications_tests.rs`:
```rust
use crate::features::notifications::types::UnreadCountResponse;

#[tokio::test]
async fn test_unread_count_reports_gsi_entries_capped_at_100() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    for i in 0..3 {
        create_inbox_row(user_pk.clone(), reply_payload(&format!("m{i}"))).await.unwrap();
    }

    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox/unread-count",
        headers: ctx.test_user.1.clone(),
        response_type: UnreadCountResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.count, 3);
}
```

- [ ] **Step 2: Run test — expect FAIL (404)**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_unread_count_reports
```
Expected: FAIL.

- [ ] **Step 3: Implement handler**

Create `app/ratel/src/features/notifications/controllers/get_unread_count.rs`:
```rust
use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::notifications::types::{NotificationsError, UnreadCountResponse};

const UNREAD_COUNT_CAP: i64 = 100;

#[get("/api/inbox/unread-count")]
pub async fn get_unread_count(
    session: Extension<tower_sessions::Session>,
) -> Result<UnreadCountResponse> {
    let user = crate::common::session::require_user(&session).await?;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let mut count: i64 = 0;
    let mut bookmark: Option<String> = None;
    for _ in 0..4 {
        let opts = UserInboxNotification::opt_with_bookmark(bookmark).limit(30);
        let (items, next) =
            UserInboxNotification::find_inbox_unread_by_user(cli, &user.pk, opts)
                .await
                .map_err(|e| {
                    crate::error!("unread-count GSI query failed: {e}");
                    NotificationsError::ListFailed
                })?;
        count += items.len() as i64;
        if count >= UNREAD_COUNT_CAP {
            return Ok(UnreadCountResponse { count: UNREAD_COUNT_CAP });
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }
    Ok(UnreadCountResponse { count })
}
```

- [ ] **Step 4: Register**

In `app/ratel/src/features/notifications/controllers/mod.rs`:
```rust
pub mod get_unread_count;
```

In `route.rs`, add the route:
```rust
        .route("/api/inbox/unread-count", axum::routing::get(crate::features::notifications::controllers::get_unread_count::get_unread_count_handler))
```

- [ ] **Step 5: Run test — expect PASS**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_unread_count
```

- [ ] **Step 6: Lint + commit**

```bash
cd app/ratel && dx fmt -f src/features/notifications/controllers/get_unread_count.rs \
                       -f src/features/notifications/route.rs
git add app/ratel/src/features/notifications app/ratel/src/tests/notifications_tests.rs
git commit -m "feat(notifications): add GET /api/inbox/unread-count"
```

---

### Task 9: `POST /api/inbox/{inbox_id}/read`

**Files:**
- Create: `app/ratel/src/features/notifications/controllers/mark_read.rs`
- Modify: `app/ratel/src/features/notifications/controllers/mod.rs`
- Modify: `app/ratel/src/features/notifications/route.rs`
- Modify: `app/ratel/src/tests/notifications_tests.rs`

- [ ] **Step 1: Failing test**

Append:
```rust
#[tokio::test]
async fn test_mark_read_flips_unread_sentinel() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    create_inbox_row(user_pk.clone(), reply_payload("hi")).await.unwrap();
    let (_, _, list_body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox",
        headers: ctx.test_user.1.clone(),
        response_type: ListResponse<InboxNotificationResponse>,
    };
    let id = &list_body.items[0].id.0;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/inbox/{id}/read"),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200);

    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox/unread-count",
        headers: ctx.test_user.1.clone(),
        response_type: UnreadCountResponse,
    };
    assert_eq!(body.count, 0);
}
```

- [ ] **Step 2: Run — FAIL**

- [ ] **Step 3: Handler**

Create `app/ratel/src/features/notifications/controllers/mark_read.rs`:
```rust
use crate::common::*;
use crate::common::models::notification::UserInboxNotification;
use crate::features::notifications::types::NotificationsError;

#[post("/api/inbox/{inbox_id}/read")]
pub async fn mark_read(
    session: Extension<tower_sessions::Session>,
    inbox_id: UserInboxNotificationEntityType,
) -> Result<()> {
    let user = crate::common::session::require_user(&session).await?;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let sk: EntityType = inbox_id.into();
    let existing = UserInboxNotification::get(cli, &user.pk, Some(sk.clone()))
        .await
        .map_err(|e| {
            crate::error!("mark_read get failed: {e}");
            NotificationsError::MarkReadFailed
        })?
        .ok_or(NotificationsError::InboxEntryNotFound)?;

    if existing.is_read {
        return Ok(());
    }

    UserInboxNotification::updater(user.pk.clone(), sk)
        .with_is_read(true)
        .with_unread_created_at(
            crate::common::models::notification::UNREAD_SENTINEL.to_string(),
        )
        .execute(cli)
        .await
        .map_err(|e| {
            crate::error!("mark_read update failed: {e}");
            NotificationsError::MarkReadFailed
        })?;
    Ok(())
}
```

- [ ] **Step 4: Register route**

In `controllers/mod.rs` + `route.rs`:
```rust
.route(
    "/api/inbox/{inbox_id}/read",
    axum::routing::post(crate::features::notifications::controllers::mark_read::mark_read_handler),
)
```

- [ ] **Step 5: Run test — PASS**

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/notifications app/ratel/src/tests/notifications_tests.rs
git commit -m "feat(notifications): add POST /api/inbox/{id}/read"
```

---

### Task 10: `POST /api/inbox/read-all`

**Files:**
- Create: `app/ratel/src/features/notifications/controllers/mark_all_read.rs`
- Modify: `app/ratel/src/features/notifications/controllers/mod.rs`
- Modify: `app/ratel/src/features/notifications/route.rs`
- Modify: `app/ratel/src/tests/notifications_tests.rs`

- [ ] **Step 1: Failing test**

Append:
```rust
use crate::features::notifications::types::MarkAllReadResponse;

#[tokio::test]
async fn test_read_all_marks_all_unread_and_reports_affected() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    for i in 0..3 {
        create_inbox_row(user_pk.clone(), reply_payload(&format!("m{i}"))).await.unwrap();
    }

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/inbox/read-all",
        headers: ctx.test_user.1.clone(),
        response_type: MarkAllReadResponse,
    };
    assert_eq!(status, 200);
    assert_eq!(body.affected, 3);
    assert!(!body.has_more);

    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/inbox/unread-count",
        headers: ctx.test_user.1.clone(),
        response_type: UnreadCountResponse,
    };
    assert_eq!(body.count, 0);
}
```

- [ ] **Step 2: Run — FAIL**

- [ ] **Step 3: Handler**

Create `app/ratel/src/features/notifications/controllers/mark_all_read.rs`:
```rust
use crate::common::*;
use crate::common::models::notification::{UserInboxNotification, UNREAD_SENTINEL};
use crate::features::notifications::types::{MarkAllReadResponse, NotificationsError};

const MAX_PAGES: usize = 5;
const PAGE_LIMIT: i32 = 30;

#[post("/api/inbox/read-all")]
pub async fn mark_all_read(
    session: Extension<tower_sessions::Session>,
) -> Result<MarkAllReadResponse> {
    let user = crate::common::session::require_user(&session).await?;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let mut affected = 0i64;
    let mut bookmark: Option<String> = None;
    let mut has_more = false;

    for page in 0..MAX_PAGES {
        let opts = UserInboxNotification::opt_with_bookmark(bookmark).limit(PAGE_LIMIT);
        let (items, next) = UserInboxNotification::find_inbox_unread_by_user(
            cli, &user.pk, opts,
        )
        .await
        .map_err(|e| {
            crate::error!("read-all GSI failed: {e}");
            NotificationsError::MarkReadFailed
        })?;

        for item in items {
            if let Err(e) = UserInboxNotification::updater(item.pk, item.sk)
                .with_is_read(true)
                .with_unread_created_at(UNREAD_SENTINEL.to_string())
                .execute(cli)
                .await
            {
                crate::error!("read-all per-row update failed: {e}");
                continue;
            }
            affected += 1;
        }

        match next {
            Some(b) => {
                bookmark = Some(b);
                if page == MAX_PAGES - 1 {
                    has_more = true;
                }
            }
            None => break,
        }
    }

    Ok(MarkAllReadResponse { affected, has_more })
}
```

- [ ] **Step 4: Register route in `controllers/mod.rs` + `route.rs`**

- [ ] **Step 5: Run — PASS**

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/notifications app/ratel/src/tests/notifications_tests.rs
git commit -m "feat(notifications): add POST /api/inbox/read-all with paged GSI walk"
```

---

## Phase 4 — MCP

### Task 11: Expose `list_inbox` + `get_unread_count` via MCP

**Files:**
- Modify: `app/ratel/src/features/notifications/controllers/list_inbox.rs`
- Modify: `app/ratel/src/features/notifications/controllers/get_unread_count.rs`
- Modify: `app/ratel/src/common/mcp/server.rs`

- [ ] **Step 1: Annotate controllers**

At the top of `list_inbox.rs`, above the `#[get]`:
```rust
#[mcp_tool(
    name = "list_inbox",
    description = "List current user's notification inbox. Returns paginated results ordered newest-first."
)]
```
Mark each parameter with `#[mcp(description = "...")]`.

Do the same for `get_unread_count.rs`:
```rust
#[mcp_tool(
    name = "get_unread_count",
    description = "Return the count of unread notifications in the current user's inbox (capped at 100)."
)]
```

- [ ] **Step 2: Register in MCP server**

In `app/ratel/src/common/mcp/server.rs`, inside the `#[tool_router] impl RatelMcpServer` block, add:
```rust
#[rmcp::tool(name = "list_inbox", description = "List the user's inbox, newest first.")]
async fn list_inbox(
    &self,
    Parameters(req): Parameters<crate::features::notifications::controllers::list_inbox::ListInboxMcpRequest>,
) -> McpResult {
    crate::features::notifications::controllers::list_inbox::list_inbox_mcp_handler(&self.mcp_secret, req).await
}

#[rmcp::tool(name = "get_unread_count", description = "Get the user's unread notification count.")]
async fn get_unread_count(&self) -> McpResult {
    crate::features::notifications::controllers::get_unread_count::get_unread_count_mcp_handler(&self.mcp_secret).await
}
```

- [ ] **Step 3: MCP test (append to `notifications_tests.rs`)**

```rust
#[tokio::test]
async fn test_mcp_list_inbox_lists_authenticated_user_rows() {
    let (ctx, token) = setup_mcp_test().await;
    let user_pk = ctx.test_user.0.pk.clone();
    create_inbox_row(user_pk.clone(), reply_payload("mcp-test")).await.unwrap();

    let (status, body) = mcp_tool_call(ctx.app, &token, "list_inbox", serde_json::json!({})).await;
    assert_eq!(status, 200);
    let content = extract_tool_content(&body);
    assert!(content["items"].as_array().unwrap().len() >= 1);
}
```

- [ ] **Step 4: Build + test**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_mcp_list_inbox
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/notifications app/ratel/src/common/mcp/server.rs \
        app/ratel/src/tests/notifications_tests.rs
git commit -m "feat(notifications): expose list_inbox + get_unread_count via MCP"
```

---

## Phase 5 — Wire existing email events to inbox (A–D)

### Task 12: Wire `ReplyOnComment` — write inbox row per recipient

**Files:**
- Modify: `app/ratel/src/common/utils/reply_notification.rs`

- [ ] **Step 1: Modify `send_reply_on_comment`**

Inside the recipient-resolution loop (around line 97–132 of `reply_notification.rs`), where each `user` is resolved, after the email has been pushed and BEFORE the final `send_email`, add an inbox-row call. Concretely, inside the existing `for pk in all_pks` loop, right after `emails.push(user.email);`:
```rust
            // Inbox row — idempotent per (recipient, parent_comment_sk).
            let payload = crate::common::types::InboxPayload::ReplyOnComment {
                space_id: None,
                post_id: None,
                comment_preview: build_preview(&parent_content),
                replier_name: replier_name.to_string(),
                replier_profile_url: String::new(),
                cta_url: cta_url.to_string(),
            };
            let dedup_source = format!("{parent_comment_sk}#{replier_pk}");
            if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
                pk.clone(),
                payload,
                &dedup_source,
            ).await {
                crate::error!("reply inbox row failed: {e}");
            }
```

(Note: we reuse the already-loaded `parent_content` from the outer scope. If the closure scope doesn't expose it, extract it to a variable before the loop.)

- [ ] **Step 2: Test (append to `notifications_tests.rs`)**

```rust
// Smoke test — exercises the code path. Real assertions live in the existing
// reply_to_comment controller tests (extended in Step 3 below) since they
// already set up parent comments.
#[tokio::test]
async fn test_reply_on_comment_send_helper_smoke() {
    let ctx = TestContext::setup().await;
    let (user2, _) = ctx.create_another_user().await;

    crate::common::utils::reply_notification::send_reply_on_comment(
        &crate::common::utils::reply_notification::ReplyCommentSource::Post,
        "POST_REPLY#none",
        "POST_COMMENT#none",
        &user2.pk.to_string(),
        "bob",
        "hi",
        "/posts/x",
    ).await.unwrap();
    // The helper early-returns when the parent comment is missing; this
    // merely verifies the new inbox code path compiles. The behavioural
    // assertion is in the reply_to_comment controller test below.
}
```

The end-to-end verification happens in the existing `post comment reply` and `space discussion reply` controller tests. Extend those tests (find them under `app/ratel/src/tests/`) to assert that after a reply, the parent author has 1 `UserInboxNotification` row.

- [ ] **Step 3: Compile + run existing reply tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- reply_to_comment
```
Expected: existing tests still pass. Add an assertion inside them for inbox rows.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/utils/reply_notification.rs app/ratel/src/tests/notifications_tests.rs
git commit -m "feat(notifications): write inbox row alongside reply email"
```

---

### Task 13: Wire `MentionInComment`

**Files:**
- Modify: `app/ratel/src/common/utils/mention.rs`

- [ ] **Step 1: Add inbox call next to existing Notification**

In `app/ratel/src/common/utils/mention.rs`, directly after the existing `Notification::new(crate::common::types::NotificationData::MentionInComment { ... })` (around line 152), add:
```rust
        let payload = crate::common::types::InboxPayload::MentionInComment {
            comment_preview: preview.clone(),
            mentioned_by_name: author_name.to_string(),
            cta_url: cta_url.clone(),
        };
        let dedup_source = format!("{}#{}", comment_sk_str, user.pk);
        if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
            user.pk.clone(),
            payload,
            &dedup_source,
        ).await {
            crate::error!("mention inbox row failed: {e}");
        }
```

Where `comment_sk_str` / `cta_url` / `author_name` / `user` are already in scope (search the file for these identifiers to confirm spelling — adjust as needed).

- [ ] **Step 2: Extend the existing mention tests under `app/ratel/src/tests/` to assert inbox rows created.**

- [ ] **Step 3: Compile + test**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- mention
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/utils/mention.rs
git commit -m "feat(notifications): write inbox row alongside mention email"
```

---

### Task 14: Wire `SpaceStatusChanged`

**Files:**
- Modify: `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs`

- [ ] **Step 1: Fan inbox rows inside `handle_space_status_change`**

In `handle_space_status_change` (file as in Read above), after the `user_pks` + `space` + `post` are all resolved and BEFORE the email chunk loop writes the `Notification` row, iterate over `user_pks` and write an inbox row for each recipient:
```rust
    // Per-recipient inbox rows. Idempotent on (user, space_pk, new_status).
    let cta_url = /* existing URL built for email */;
    let new_status = event.new_status;
    let space_title = space.title.clone();
    let space_id: SpacePartition = space.pk.clone().into();

    for user_pk in &user_pks {
        let payload = crate::common::types::InboxPayload::SpaceStatusChanged {
            space_id: space_id.clone(),
            space_title: space_title.clone(),
            new_status,
            cta_url: cta_url.clone(),
        };
        let dedup_source = format!("{}#{:?}", event.space_pk, new_status);
        if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
            user_pk.clone(),
            payload,
            &dedup_source,
        ).await {
            crate::error!("space-status inbox row failed: {e}");
        }
    }
```

Reuse whatever CTA URL the email already uses — search the existing function for the `cta_url` local.

- [ ] **Step 2: Extend `app/ratel/src/tests/space_status_change_tests.rs`**

Add an assertion after the event is handled:
```rust
    let (rows, _) = UserInboxNotification::find_by_pk(
        &ctx.ddb, &participant.pk, UserInboxNotification::opt()
    ).await.unwrap();
    assert_eq!(rows.len(), 1);
    assert!(matches!(rows[0].payload, InboxPayload::SpaceStatusChanged { .. }));
```

- [ ] **Step 3: Run existing + new tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- space_status_change
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs \
        app/ratel/src/tests/space_status_change_tests.rs
git commit -m "feat(notifications): write inbox rows for space status changes"
```

---

### Task 15: Wire `SpaceInvitation`

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/apps/apps/general/controllers/invite_space_participants.rs`

- [ ] **Step 1: Inspect existing email send**

Open the file and locate the `Notification::new(NotificationData::SendSpaceInvitation { ... })` site. The `invited_users` (or equivalent) collection is already resolved — we fan inbox rows over it.

- [ ] **Step 2: Add inbox-row call**

After the email notification is fired, add:
```rust
    for invitee in &invited_users {
        let payload = crate::common::types::InboxPayload::SpaceInvitation {
            space_id: SpacePartition::from(space.pk.clone()),
            space_title: space.title.clone(),
            inviter_name: inviter.name.clone(),
            cta_url: cta_url.clone(),
        };
        let dedup_source = format!("{}#{}", space.pk, invitee.pk);
        if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
            invitee.pk.clone(), payload, &dedup_source
        ).await {
            crate::error!("space-invitation inbox row failed: {e}");
        }
    }
```

Adjust identifiers (`invited_users`, `inviter`, `cta_url`) to match the existing controller's locals.

- [ ] **Step 3: Test**

Extend the existing invite-participants controller test (under `app/ratel/src/tests/`) to assert inbox rows are created for each invitee.

- [ ] **Step 4: Compile + test + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- invite_space_participants
git add app/ratel/src/features/spaces/pages/apps/apps/general/controllers/invite_space_participants.rs
git commit -m "feat(notifications): write inbox rows for space invitations"
```

---

## Phase 6 — Frontend hooks

### Task 16: `use_unread_count` hook with 60s polling

**Files:**
- Create: `app/ratel/src/features/notifications/hooks/mod.rs`
- Create: `app/ratel/src/features/notifications/hooks/use_unread_count.rs`

- [ ] **Step 1: Hook module**

Create `app/ratel/src/features/notifications/hooks/mod.rs`:
```rust
mod use_inbox;
mod use_unread_count;

pub use use_inbox::*;
pub use use_unread_count::*;
```

(The `use_inbox` file is created in Task 17 — for now create an empty placeholder: `pub fn _placeholder() {}` in `use_inbox.rs`.)

- [ ] **Step 2: Implement `use_unread_count`**

Create `app/ratel/src/features/notifications/hooks/use_unread_count.rs`:
```rust
use crate::common::*;
use crate::features::notifications::controllers::get_unread_count::get_unread_count;
use std::time::Duration;

const POLL_INTERVAL_SECS: u64 = 60;

#[derive(Clone, Copy)]
pub struct UnreadCount(pub Signal<i64>);

pub fn use_unread_count() -> Signal<i64> {
    use_hook(|| {
        let mut count = Signal::new(0i64);

        spawn(async move {
            loop {
                match get_unread_count().await {
                    Ok(resp) => count.set(resp.count),
                    Err(e) => tracing::debug!("use_unread_count poll failed: {e}"),
                }
                crate::common::utils::time::sleep(Duration::from_secs(POLL_INTERVAL_SECS)).await;
            }
        });

        count
    })
}
```

- [ ] **Step 3: Compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/notifications/hooks
git commit -m "feat(notifications): add use_unread_count hook with 60s polling"
```

---

### Task 17: `use_inbox` infinite-query hook

**Files:**
- Modify: `app/ratel/src/features/notifications/hooks/use_inbox.rs`

- [ ] **Step 1: Implement**

Replace `use_inbox.rs` with:
```rust
use crate::common::*;
use crate::common::hooks::use_infinite_query;
use crate::features::notifications::controllers::list_inbox::list_inbox;
use crate::features::notifications::types::InboxNotificationResponse;

pub fn use_inbox(
    unread_only: bool,
) -> crate::common::hooks::InfiniteQuery<InboxNotificationResponse> {
    use_infinite_query(
        ("inbox", unread_only),
        move |bookmark: Option<String>| async move {
            let res = list_inbox(Some(unread_only), bookmark).await?;
            Ok((res.items, res.bookmark))
        },
    )
}
```

Note: the exact `use_infinite_query` signature may vary — search `app/ratel/src/common/hooks/` for its definition and match the existing callers (e.g., ranking list, post list).

- [ ] **Step 2: Compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/notifications/hooks/use_inbox.rs
git commit -m "feat(notifications): add use_inbox infinite-query hook"
```

---

## Phase 7 — HTML-first UI mockup (approval checkpoint)

### Task 18: Create HTML mockup and block for user review

**Files:**
- Create: `app/ratel/assets/design/notification-panel.html`

- [ ] **Step 1: Write a standalone HTML mockup**

Create `app/ratel/assets/design/notification-panel.html` as a single self-contained file showing:
- A bell icon with a red badge `3` in a top-bar stub
- A right-slide panel (width 420px, full height) with:
  - Header: "Notifications" + "Mark all as read" button (right-aligned) + close × button
  - List of mock items covering each of the 4 kinds (reply, mention, space status, invitation) with realistic preview text
  - 1 read + 3 unread items — unread uses `bg-primary/10` and shows a dot
  - Relative timestamp ("2m ago", "3h ago") on right
  - Hover state on each item
- Empty-state variant (toggle button to switch)

Use dark-theme colors first, then show light-theme version via a toggle button (same `--dark` / `--light` space-toggle pattern as existing component CSS).

- [ ] **Step 2: Block for user approval**

Message the user:
> "Mockup at `app/ratel/assets/design/notification-panel.html`. Open in browser and let me know if you want any changes to layout, item rendering, or states. I'll convert to Dioxus once approved."

**Checkpoint — do not proceed to Task 19 until user approves.**

- [ ] **Step 3: Commit mockup**

```bash
git add app/ratel/assets/design/notification-panel.html
git commit -m "design(notifications): HTML mockup for inbox panel + bell"
```

---

## Phase 8 — Dioxus components

### Task 19: `NotificationBell` component

**Files:**
- Create: `app/ratel/src/features/notifications/components/mod.rs`
- Create: `app/ratel/src/features/notifications/components/notification_bell/mod.rs`
- Create: `app/ratel/src/features/notifications/components/notification_bell/component.rs`
- Create: `app/ratel/src/features/notifications/components/notification_bell/style.css`
- Create: `app/ratel/src/features/notifications/i18n.rs`

- [ ] **Step 1: Components mod**

```rust
// app/ratel/src/features/notifications/components/mod.rs
mod notification_bell;
mod notification_panel;
pub use notification_bell::*;
pub use notification_panel::*;
```

(`notification_panel` is created in Task 21; stub it now with `pub fn _p() {}` in a placeholder file so compilation passes — replace in Task 21.)

- [ ] **Step 2: Bell component**

Create `app/ratel/src/features/notifications/components/notification_bell/mod.rs`:
```rust
mod component;
pub use component::*;
```

Create `app/ratel/src/features/notifications/components/notification_bell/component.rs`:
```rust
use crate::common::*;
use crate::features::notifications::hooks::use_unread_count;

#[component]
pub fn NotificationBell(onclick: EventHandler<()>, #[props(default)] class: String) -> Element {
    let count = use_unread_count()();
    let label = if count >= 100 { "99+".to_string() } else { count.to_string() };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "notification-bell {class}",
            "aria-label": "Notifications",
            "data-testid": "notification-bell",
            onclick: move |_| onclick.call(()),
            lucide_dioxus::Bell { class: "w-6 h-6 [&>path]:stroke-icon-primary" }
            if count > 0 {
                span {
                    class: "notification-bell__badge",
                    "data-testid": "notification-bell-badge",
                    "{label}"
                }
            }
        }
    }
}
```

Create `app/ratel/src/features/notifications/components/notification_bell/style.css`:
```css
.notification-bell {
    --bell-bg: var(--dark, transparent) var(--light, transparent);
    --bell-hover: var(--dark, rgba(255,255,255,0.06)) var(--light, rgba(0,0,0,0.06));
    --badge-bg: #db2780;
    --badge-text: #ffffff;

    position: relative;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 40px;
    height: 40px;
    border-radius: 9999px;
    background: var(--bell-bg);
    transition: background 150ms ease;
    border: 0;
    cursor: pointer;
}
.notification-bell:hover { background: var(--bell-hover); }

.notification-bell__badge {
    position: absolute;
    top: 4px;
    right: 4px;
    min-width: 18px;
    height: 18px;
    padding: 0 5px;
    border-radius: 9999px;
    background: var(--badge-bg);
    color: var(--badge-text);
    font-size: 11px;
    font-weight: 600;
    line-height: 18px;
    text-align: center;
}
```

- [ ] **Step 3: i18n**

Create `app/ratel/src/features/notifications/i18n.rs`:
```rust
use dioxus_translate::*;

translate! {
    NotificationsTranslate;
    panel_title: { en: "Notifications", ko: "알림" },
    mark_all_read: { en: "Mark all as read", ko: "모두 읽음" },
    empty: { en: "No notifications yet", ko: "새 알림이 없습니다" },
    reply_title: { en: "{name} replied to your comment", ko: "{name}님이 답글을 남겼습니다" },
    mention_title: { en: "{name} mentioned you", ko: "{name}님이 나를 언급했습니다" },
    space_status_title: { en: "{space} is now {status}", ko: "{space}가 {status}로 변경되었습니다" },
    space_invite_title: { en: "{name} invited you to {space}", ko: "{name}님이 {space}에 초대했습니다" },
    relative_now: { en: "just now", ko: "방금" },
    relative_minute: { en: "{n}m ago", ko: "{n}분 전" },
    relative_hour: { en: "{n}h ago", ko: "{n}시간 전" },
    relative_day: { en: "{n}d ago", ko: "{n}일 전" },
}
```

- [ ] **Step 4: Build + format**

```bash
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/notifications/components/notification_bell/component.rs
cd app/ratel && dx fmt -f src/features/notifications/components/notification_bell/component.rs
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/notifications/components app/ratel/src/features/notifications/i18n.rs
git commit -m "feat(notifications): add NotificationBell component"
```

---

### Task 20: `NotificationItem` component (per-kind rendering)

**Files:**
- Create: `app/ratel/src/features/notifications/components/notification_panel/mod.rs`
- Create: `app/ratel/src/features/notifications/components/notification_panel/notification_item/mod.rs`
- Create: `app/ratel/src/features/notifications/components/notification_panel/notification_item/component.rs`
- Create: `app/ratel/src/features/notifications/components/notification_panel/notification_item/style.css`

- [ ] **Step 1: Panel mod (partial)**

`app/ratel/src/features/notifications/components/notification_panel/mod.rs`:
```rust
mod notification_item;
pub use notification_item::*;
```

(We'll add `mod component; pub use component::*;` in Task 21.)

- [ ] **Step 2: Item mod + component**

`notification_item/mod.rs`:
```rust
mod component;
pub use component::*;
```

`notification_item/component.rs`:
```rust
use crate::common::*;
use crate::features::notifications::i18n::NotificationsTranslate;
use crate::features::notifications::types::InboxNotificationResponse;
use dioxus_translate::*;

fn relative_time(now_ms: i64, then_ms: i64, tr: &NotificationsTranslate) -> String {
    let diff = (now_ms - then_ms).max(0);
    let secs = diff / 1000;
    let mins = secs / 60;
    let hours = mins / 60;
    let days = hours / 24;
    if secs < 60 { tr.relative_now.clone() }
    else if mins < 60 { tr.relative_minute.replace("{n}", &mins.to_string()) }
    else if hours < 24 { tr.relative_hour.replace("{n}", &hours.to_string()) }
    else { tr.relative_day.replace("{n}", &days.to_string()) }
}

#[component]
pub fn NotificationItem(
    item: ReadSignal<InboxNotificationResponse>,
    onclick: EventHandler<InboxNotificationResponse>,
) -> Element {
    let tr: NotificationsTranslate = use_translate();
    let lang = use_locale();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let data = item();
    let (title, body, avatar_url): (String, String, Option<String>) = match &data.payload {
        InboxPayload::ReplyOnComment { replier_name, comment_preview, replier_profile_url, .. } => (
            tr.reply_title.replace("{name}", replier_name),
            comment_preview.clone(),
            Some(replier_profile_url.clone()),
        ),
        InboxPayload::MentionInComment { mentioned_by_name, comment_preview, .. } => (
            tr.mention_title.replace("{name}", mentioned_by_name),
            comment_preview.clone(),
            None,
        ),
        InboxPayload::SpaceStatusChanged { space_title, new_status, .. } => (
            tr.space_status_title
                .replace("{space}", space_title)
                .replace("{status}", &new_status.translate(&lang)),
            String::new(),
            None,
        ),
        InboxPayload::SpaceInvitation { space_title, inviter_name, .. } => (
            tr.space_invite_title
                .replace("{name}", inviter_name)
                .replace("{space}", space_title),
            String::new(),
            None,
        ),
    };
    let is_unread = !data.is_read;
    let rel = relative_time(now, data.created_at, &tr);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        button {
            class: "notification-item",
            "aria-relevant": "{is_unread}",
            "data-testid": "notification-item",
            onclick: {
                let d = data.clone();
                move |_| onclick.call(d.clone())
            },
            div { class: "notification-item__avatar",
                if let Some(url) = avatar_url.filter(|u| !u.is_empty()) {
                    img { src: "{url}", alt: "" }
                } else {
                    lucide_dioxus::Bell { class: "w-5 h-5 [&>path]:stroke-icon-primary" }
                }
            }
            div { class: "notification-item__body",
                div { class: "notification-item__title", "{title}" }
                if !body.is_empty() {
                    div { class: "notification-item__preview", "{body}" }
                }
                div { class: "notification-item__time", "{rel}" }
            }
            if is_unread {
                span { class: "notification-item__dot", "data-testid": "unread-dot" }
            }
        }
    }
}
```

`notification_item/style.css`:
```css
.notification-item {
    --item-bg: var(--dark, transparent) var(--light, transparent);
    --item-hover: var(--dark, rgba(255,255,255,0.04)) var(--light, rgba(0,0,0,0.04));
    --item-unread: var(--dark, rgba(252,179,0,0.10)) var(--light, rgba(252,179,0,0.10));
    --item-title: var(--dark, #f0f0f5) var(--light, #12121a);
    --item-body: var(--dark, #8888a8) var(--light, #6b6b80);
    --item-dot: #fcb300;

    display: flex;
    gap: 12px;
    width: 100%;
    padding: 12px 16px;
    background: var(--item-bg);
    border: 0;
    border-bottom: 1px solid var(--dark, rgba(255,255,255,0.05)) var(--light, rgba(0,0,0,0.05));
    text-align: left;
    cursor: pointer;
    transition: background 120ms ease;
    color: var(--item-title);
}
.notification-item:hover { background: var(--item-hover); }
.notification-item[aria-relevant="true"] { background: var(--item-unread); }
.notification-item[aria-relevant="false"] { opacity: 0.7; }

.notification-item__avatar {
    width: 40px; height: 40px;
    border-radius: 9999px;
    flex-shrink: 0;
    background: var(--dark, rgba(255,255,255,0.06)) var(--light, rgba(0,0,0,0.06));
    display: flex; align-items: center; justify-content: center;
    overflow: hidden;
}
.notification-item__avatar img { width: 100%; height: 100%; object-fit: cover; }

.notification-item__body { flex: 1; min-width: 0; }
.notification-item__title { font-size: 14px; font-weight: 500; color: var(--item-title); }
.notification-item__preview {
    margin-top: 2px;
    font-size: 13px;
    color: var(--item-body);
    display: -webkit-box;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
    overflow: hidden;
}
.notification-item__time {
    margin-top: 4px;
    font-size: 11px;
    color: var(--item-body);
}
.notification-item__dot {
    width: 8px; height: 8px;
    border-radius: 9999px;
    background: var(--item-dot);
    flex-shrink: 0;
    align-self: center;
}
```

- [ ] **Step 3: Build + format**

```bash
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/notifications/components/notification_panel/notification_item/component.rs
cd app/ratel && dx fmt -f src/features/notifications/components/notification_panel/notification_item/component.rs
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/notifications/components/notification_panel
git commit -m "feat(notifications): add NotificationItem per-kind rendering"
```

---

### Task 21: `NotificationPanel` slide-in with "Mark all as read"

**Files:**
- Create: `app/ratel/src/features/notifications/components/notification_panel/component.rs`
- Create: `app/ratel/src/features/notifications/components/notification_panel/style.css`
- Modify: `app/ratel/src/features/notifications/components/notification_panel/mod.rs`

- [ ] **Step 1: Panel mod update**

```rust
mod component;
mod notification_item;
pub use component::*;
pub use notification_item::*;
```

- [ ] **Step 2: Panel component**

```rust
// component.rs
use crate::common::*;
use crate::features::notifications::components::notification_panel::notification_item::NotificationItem;
use crate::features::notifications::controllers::mark_all_read::mark_all_read;
use crate::features::notifications::controllers::mark_read::mark_read;
use crate::features::notifications::hooks::use_inbox;
use crate::features::notifications::i18n::NotificationsTranslate;
use crate::features::notifications::types::InboxNotificationResponse;

#[component]
pub fn NotificationPanel(
    open: bool,
    on_close: EventHandler<()>,
) -> Element {
    let tr: NotificationsTranslate = use_translate();
    let mut inbox = use_inbox(false);
    let nav = use_navigator();

    let on_item_click = move |item: InboxNotificationResponse| {
        let inbox_id = item.id.clone();
        let cta = match &item.payload {
            InboxPayload::ReplyOnComment { cta_url, .. } => cta_url.clone(),
            InboxPayload::MentionInComment { cta_url, .. } => cta_url.clone(),
            InboxPayload::SpaceStatusChanged { cta_url, .. } => cta_url.clone(),
            InboxPayload::SpaceInvitation { cta_url, .. } => cta_url.clone(),
        };
        spawn(async move {
            let _ = mark_read(inbox_id).await;
        });
        if !cta.is_empty() {
            // Route::from_path conversion — for MVP use absolute path routing:
            nav.push(cta);
        }
    };

    let on_mark_all = move |_| {
        spawn(async move {
            if let Err(e) = mark_all_read().await {
                tracing::error!("mark-all-read failed: {e}");
                return;
            }
            inbox.refetch();
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }
        div {
            class: "notification-panel",
            "data-open": "{open}",
            "data-testid": "notification-panel",
            div { class: "notification-panel__header",
                h3 { class: "notification-panel__title", "{tr.panel_title}" }
                button {
                    class: "notification-panel__mark-all",
                    "data-testid": "mark-all-read",
                    onclick: on_mark_all,
                    "{tr.mark_all_read}"
                }
                button {
                    class: "notification-panel__close",
                    "aria-label": "Close",
                    onclick: move |_| on_close.call(()),
                    lucide_dioxus::X { class: "w-5 h-5 [&>path]:stroke-icon-primary" }
                }
            }
            div { class: "notification-panel__list", "data-testid": "notification-list",
                if inbox.items().is_empty() && !inbox.is_loading() {
                    div { class: "notification-panel__empty", "{tr.empty}" }
                } else {
                    for item in inbox.items() {
                        NotificationItem {
                            key: "{item.id.0}",
                            item: item.clone(),
                            onclick: on_item_click,
                        }
                    }
                    {inbox.more_element()}
                }
            }
        }
    }
}
```

Note: The exact `InfiniteQuery` API (`.items()`, `.refetch()`, `.more_element()`, `.is_loading()`) should be verified against the real `use_infinite_query` in `common/hooks`. Adjust calls to match.

- [ ] **Step 3: Panel CSS**

```css
.notification-panel {
    --panel-bg: var(--dark, #0c0c1a) var(--light, #ffffff);
    --panel-border: var(--dark, rgba(255,255,255,0.06)) var(--light, rgba(0,0,0,0.08));
    --panel-title: var(--dark, #f0f0f5) var(--light, #12121a);
    --panel-shadow: var(--dark, rgba(0,0,0,0.4)) var(--light, rgba(0,0,0,0.08));
    --panel-accent: #fcb300;

    position: fixed;
    top: 0; right: 0; bottom: 0;
    width: 420px;
    max-width: 92vw;
    background: var(--panel-bg);
    border-left: 1px solid var(--panel-border);
    box-shadow: -8px 0 32px var(--panel-shadow);
    transform: translateX(100%);
    transition: transform 280ms cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    flex-direction: column;
    z-index: 110;
}
.notification-panel[data-open="true"] { transform: translateX(0); }

.notification-panel__header {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 16px 20px;
    border-bottom: 1px solid var(--panel-border);
}
.notification-panel__title {
    flex: 1;
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: var(--panel-title);
}
.notification-panel__mark-all {
    background: transparent;
    border: 0;
    color: var(--panel-accent);
    font-size: 13px;
    cursor: pointer;
    padding: 4px 8px;
}
.notification-panel__mark-all:hover {
    background: var(--dark, rgba(252,179,0,0.08)) var(--light, rgba(252,179,0,0.12));
    border-radius: 4px;
}
.notification-panel__close {
    background: transparent;
    border: 0;
    cursor: pointer;
    padding: 6px;
    border-radius: 9999px;
}

.notification-panel__list {
    flex: 1;
    overflow-y: auto;
}
.notification-panel__empty {
    padding: 40px 20px;
    text-align: center;
    color: var(--dark, #8888a8) var(--light, #6b6b80);
    font-size: 14px;
    font-style: italic;
}
```

- [ ] **Step 4: Build + format**

```bash
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/notifications/components/notification_panel/component.rs
cd app/ratel && dx fmt -f src/features/notifications/components/notification_panel/component.rs
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/notifications/components/notification_panel
git commit -m "feat(notifications): add NotificationPanel with mark-all-read"
```

---

### Task 22: Insert `NotificationBell` in Navbar

**Files:**
- Modify: `app/ratel/src/common/components/navbar/component.rs`

- [ ] **Step 1: Identify the user-area region**

The current Navbar component (from Read above) defines primitives wrapped around `dioxus_primitives::navbar`. The actual Ratel navbar used on the home page lives in a different file — locate it with:
```bash
grep -r "Navbar {" app/ratel/src --include="*.rs" -l
```
The bell should sit next to the user avatar in the top bar. Update that navbar (likely in `features/main_layout` or similar) to include:
```rust
NotificationBell {
    onclick: move |_| {
        // Toggle the panel — implementation detail depends on how navbar stores
        // panel state. For home, add a signal in the parent layout:
        notifications_open.toggle();
    },
}
```
And conditionally render `NotificationPanel { open: notifications_open(), on_close: ... }` near the root of the layout.

- [ ] **Step 2: Ensure `NotificationBell` + `NotificationPanel` are re-exported**

In `app/ratel/src/features/notifications/mod.rs`, confirm `pub use components::*;`.

- [ ] **Step 3: Build + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
git add app/ratel/src/common/components/navbar app/ratel/src/features/main_layout
git commit -m "feat(notifications): insert NotificationBell in main layout"
```

---

### Task 23: Insert `NotificationBell` + `ActivePanel::Notifications` in SpaceIndexPage

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/component.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/arena_topbar/component.rs`

- [ ] **Step 1: Extend `ActivePanel` enum**

In `app/ratel/src/features/spaces/pages/index/component.rs`, add variant:
```rust
pub enum ActivePanel {
    #[default]
    None,
    Overview,
    Leaderboard,
    Settings,
    Notifications,
}
```

Add matching `notifications_open = active_panel() == ActivePanel::Notifications` local.

Render the panel alongside the others:
```rust
            crate::features::notifications::components::NotificationPanel {
                open: notifications_open,
                on_close: move |_| active_panel.set(ActivePanel::None),
            }
```

- [ ] **Step 2: Add bell to `ArenaTopbar`**

In `arena_topbar/component.rs`, add an `active_panel: Signal<ActivePanel>` prop if not already present, and insert the bell next to the overview/leaderboard/settings buttons:
```rust
            crate::features::notifications::components::NotificationBell {
                onclick: move |_| active_panel.set(ActivePanel::Notifications),
            }
```

- [ ] **Step 3: Build + format**

```bash
cd app/ratel && dx fmt -f src/features/spaces/pages/index/component.rs \
                       -f src/features/spaces/pages/index/arena_topbar/component.rs
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index
git commit -m "feat(notifications): wire NotificationBell + Panel into Space arena"
```

---

## Phase 9 — E2E test

### Task 24: Playwright scenario

**Files:**
- Create: `playwright/tests/web/notifications.spec.js`

- [ ] **Step 1: Write the spec**

```js
import { test, expect } from "@playwright/test";
import { click, fill, goto, waitPopup } from "../utils";

test.describe.serial("Notification inbox", () => {
  test("Step 1: user1 replies to user2's comment → user2 sees badge", async ({ browser }) => {
    // User1 creates a post + comment, user2 replies.
    // (Reuse the existing test utilities that create posts + comments.)

    // Then switch to user2 and check the bell badge.
    const ctx = await browser.newContext({ storageState: { cookies: [], origins: [] } });
    const page = await ctx.newPage();
    try {
      await goto(page, "/");
      // Log in as user2 via existing utils
      // ...
      await expect(page.getByTestId("notification-bell-badge")).toBeVisible();
      await expect(page.getByTestId("notification-bell-badge")).toContainText("1");
    } finally {
      await ctx.close();
    }
  });

  test("Step 2: opening panel shows the item + clicking navigates", async ({ page }) => {
    await goto(page, "/");
    // assume logged in as user2 via previous test's state (serial)
    await click(page, { testId: "notification-bell" });
    await expect(page.getByTestId("notification-panel")).toHaveAttribute("data-open", "true");
    await expect(page.getByTestId("notification-item").first()).toBeVisible();

    await click(page, { testId: "notification-item" });
    // Navigation verified by URL change:
    await page.waitForURL(/\/(posts|spaces)\//);
  });

  test("Step 3: 'Mark all as read' clears the badge", async ({ page }) => {
    await goto(page, "/");
    await click(page, { testId: "notification-bell" });
    await click(page, { testId: "mark-all-read" });
    await expect(page.getByTestId("notification-bell-badge")).toBeHidden();
  });
});
```

- [ ] **Step 2: Run**

```bash
cd playwright && npx playwright test tests/web/notifications.spec.js --headed
```
Expected: 3 tests pass.

- [ ] **Step 3: Commit**

```bash
git add playwright/tests/web/notifications.spec.js
git commit -m "test(notifications): add e2e scenario for inbox badge + panel + mark-all-read"
```

---

## Phase 10 — Final verification

### Task 25: Full build matrix + Playwright against prod image

- [ ] **Step 1: All cargo checks**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features mobile
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```
All must pass clean (no warnings).

- [ ] **Step 2: Full test suite**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass"
```

- [ ] **Step 3: Full Playwright against prod-built image**

Follow `.claude/rules/workflows/fix-pr-playwright.md` Step 5 — build the local Docker image and run the full suite with `CI=true`.

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-local ENV=local RUSTFLAGS='-D warnings' \
DYNAMO_ENDPOINT=http://localstack:4566 \
AWS_ACCESS_KEY_ID=test AWS_SECRET_ACCESS_KEY=test \
AWS_REGION=ap-northeast-2 AWS_ACCOUNT_ID=test \
COMMIT=inbox-test ECR=ratel/app-shell \
  make build-testing && COMMIT=inbox-test ECR=ratel/app-shell make docker

cd /home/hackartist/data/devel/github.com/biyard/ratel
COMMIT=inbox-test make testing

cd playwright && CI=true make test
```
Zero failures required before push.

- [ ] **Step 4: Push to `hackartists` fork**

```bash
git push hackartists feature/notification-reply-on-comment
```

- [ ] **Step 5: Open PR to `biyard/ratel:dev`**

Via `gh pr create` targeting base `dev` and head `hackartists:feature/notification-reply-on-comment`.

---

## Follow-up plans (not in this plan)

Once this MVP lands, the next plan `docs/superpowers/plans/2026-04-21-notification-inbox-fanout.md` will add:

- **Event E** `NewCommentOnMine` — inline trigger in post/space-comment controllers
- **Event F** `FollowedUserPosted` — CDK Pipe + Rule for Feed INSERT, handler fan-out over follower GSI
- **Event G** `NewSpaceAction` — CDK Pipe + Rule for SpacePoll/Quiz/Discussion INSERT, handler fan-out over participants
- **Event H** `RewardGranted` — inline trigger in reward grant service

That plan will extend `InboxKind` + `InboxPayload` with 4 more variants and add 2 EventBridge `DetailType` variants + stream-handler branches for local-dev parity.
