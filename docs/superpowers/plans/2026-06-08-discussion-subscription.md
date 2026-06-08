# Discussion Subscription Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Let space members subscribe to a single discussion and receive one in-app notification + one email per comment/reply (deduped: mention > reply-target > subscriber), via a toggle button in the discussion top bar.

**Architecture:** New `SpacePostSubscription` DynamoEntity stored under the discussion's `SpacePost` partition (direct Query, never Scan). Subscribe/unsubscribe controllers + author auto-subscribe on discussion creation. Fan-out is async: comment controllers fire one `Notification` row → existing DynamoDB Stream → `NotificationData::DiscussionCommentPosted::send()` resolves recipients in a single pass with a shared `seen` set. Frontend adds `subscribed` to the detail response and a toggle button driven by `UseDiscussionArena`.

**Tech Stack:** Rust (edition 2024), Dioxus 0.7 fullstack, DynamoDB (DynamoEntity derive), AWS SESv2, Playwright.

**Spec:** [docs/superpower/2026-06-08-discussion-subscription.md](../../superpower/2026-06-08-discussion-subscription.md)

**Build/verify commands (run from `app/ratel`):**
- `DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
- `DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web`
- `DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web`
- Tests: `DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- <name>`
- Lint a changed file: `rustywind --custom-regex "class: \"(.*)\"" --write <file>` then `dx fmt -f <file>`

**Conventions reminder:** `id` (SubPartition) in API params/DTOs; typed error enums; semantic color tokens; primitive components; `translate!` for all UI strings; components consume `UseDiscussionArena`, never `_handler`s directly; all CSS in `app/ratel/assets/main.css`.

---

## Phase 1 — Data model

### Task 1: Add `SpacePostSubscription` variant to the `EntityType` enum

**Files:**
- Modify: `app/ratel/src/common/types/entity_type.rs` (near the `SpacePost*` variants, ~line 156-159)

- [ ] **Step 1: Add the enum variant**

In `entity_type.rs`, find:

```rust
    SpacePost(String),
    SpacePostComment(String),
    SpacePostCommentReply(String, String),
    SpacePostCommentLike(String, String),
```

Add a new variant immediately after `SpacePostCommentLike`:

```rust
    SpacePost(String),
    SpacePostComment(String),
    SpacePostCommentReply(String, String),
    SpacePostCommentLike(String, String),
    SpacePostSubscription(String),
```

The enum already derives `SubPartition` and `DynamoEnum`, so this auto-generates the `SpacePostSubscriptionEntityType` SubPartition type with DynamoDB prefix `SPACE_POST_SUBSCRIPTION`.

- [ ] **Step 2: Verify it compiles**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS (the enum is exhaustively matched in a few places; if a `match` over `EntityType` errors as non-exhaustive, that's a pre-existing wildcard-free match — add a `_ =>` arm or the specific arm as the compiler points out. Most matches use `_`.)

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/common/types/entity_type.rs
git commit -m "feat(discussion-sub): add SpacePostSubscription EntityType variant"
```

---

### Task 2: Create the `SpacePostSubscription` model

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/discussion/models/space_post_subscription.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/models/mod.rs`

- [ ] **Step 1: Write the model file**

Create `space_post_subscription.rs` with full contents:

```rust
use crate::common::types::{EntityType, Partition};
use crate::common::types::{SpacePartition, SpacePostPartition};
use serde::{Deserialize, Serialize};
#[cfg(feature = "server")]
#[allow(unused_imports)]
use rmcp::schemars;
use by_macros::DynamoEntity;

/// One row per (discussion, subscriber). Stored under the discussion's
/// `SpacePost` partition (same partition its comments use), so listing all
/// subscribers of a discussion is a single partition Query — never a Scan.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[cfg_attr(feature = "server", derive(rmcp::schemars::JsonSchema))]
pub struct SpacePostSubscription {
    pub pk: Partition,  // Partition::SpacePost(post_id) — the discussion partition
    pub sk: EntityType, // EntityType::SpacePostSubscription(user_pk string)

    pub space_pk: Partition,
    pub user_pk: Partition,
    pub created_at: i64,
}

impl SpacePostSubscription {
    /// Derive (pk, sk) for a (discussion, user) pair. `user_pk` is the full
    /// `Partition::User(..)`; its string form is used as the sk suffix so each
    /// user maps to exactly one subscription row per discussion.
    pub fn keys(
        space_post_pk: &SpacePostPartition,
        user_pk: &Partition,
    ) -> (Partition, EntityType) {
        let pk: Partition = space_post_pk.clone().into();
        let sk = EntityType::SpacePostSubscription(user_pk.to_string());
        (pk, sk)
    }

    pub fn new(
        space_post_pk: SpacePostPartition,
        space_pk: SpacePartition,
        user_pk: &Partition,
    ) -> Self {
        let (pk, sk) = Self::keys(&space_post_pk, user_pk);
        Self {
            pk,
            sk,
            space_pk: space_pk.into(),
            user_pk: user_pk.clone(),
            created_at: crate::common::utils::time::get_now_timestamp(),
        }
    }

    /// Sort-key prefix for partition queries that list all subscribers of a
    /// discussion.
    pub fn sk_prefix() -> String {
        "SPACE_POST_SUBSCRIPTION".to_string()
    }
}
```

- [ ] **Step 2: Register the module**

In `models/mod.rs`, add alongside the other model declarations (match the existing `pub mod ...; pub use ...::*;` style — check the file and mirror it). Add:

```rust
mod space_post_subscription;
pub use space_post_subscription::*;
```

- [ ] **Step 3: Verify it compiles**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS. If `get_now_timestamp` is not found, confirm the path `crate::common::utils::time::get_now_timestamp` (used by `SpacePostComment::new`).

- [ ] **Step 4: Lint & format**

```bash
cd app/ratel
dx fmt -f src/features/spaces/pages/actions/actions/discussion/models/space_post_subscription.rs
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/discussion/models/
git commit -m "feat(discussion-sub): add SpacePostSubscription model"
```

---

## Phase 2 — Subscribe / unsubscribe controllers

### Task 3: Write a failing test for subscribe/unsubscribe

**Files:**
- Create: `app/ratel/src/tests/discussion_subscription_tests.rs`
- Modify: `app/ratel/src/tests/mod.rs`

- [ ] **Step 1: Register the test module**

In `app/ratel/src/tests/mod.rs`, add in the alphabetical module list (after `mod discussion_tests;`):

```rust
mod discussion_subscription_tests;
```

- [ ] **Step 2: Write the test file with a shared seed helper + first tests**

Create `app/ratel/src/tests/discussion_subscription_tests.rs`:

```rust
use super::*;
use crate::common::models::space::{SpaceUser};
use crate::common::types::{EntityType, Partition};
use crate::features::spaces::pages::actions::actions::discussion::{
    SpacePost, SpacePostSubscription,
};

/// Seed a published, ongoing space owned by `ctx.test_user` plus one discussion
/// (SpacePost). Returns (space_id, discussion_id) as raw id strings.
async fn seed_space_and_discussion(ctx: &TestContext) -> (String, String) {
    use crate::common::models::space::{SpaceCommon};
    use crate::common::types::{SpaceStatus, SpacePublishState, SpaceVisibility};

    let space_id = uuid::Uuid::new_v4().to_string();
    let post_id = space_id.clone();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let space_pk = Partition::Space(space_id.clone());
    let post_pk = Partition::Feed(post_id.clone());

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

    let post = crate::features::posts::models::Post {
        pk: post_pk.clone(),
        sk: EntityType::Post,
        title: "Sub Test".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    let discussion_id = uuid::Uuid::now_v7().to_string();
    let mut discussion = SpacePost::default();
    discussion.pk = space_pk.clone();
    discussion.sk = EntityType::SpacePost(discussion_id.clone());
    discussion.created_at = now;
    discussion.updated_at = now;
    discussion.title = "Test Discussion".to_string();
    discussion.user_pk = ctx.test_user.0.pk.clone();
    discussion.author_display_name = ctx.test_user.0.display_name.clone();
    discussion.author_username = ctx.test_user.0.username.clone();
    discussion.author_profile_url = ctx.test_user.0.profile_url.clone();
    discussion.create(&ctx.ddb).await.expect("create discussion");

    let _ = SpaceUser::default(); // ensure import used across helpers

    (space_id, discussion_id)
}

async fn subscription_exists(ctx: &TestContext, discussion_id: &str, user_pk: &Partition) -> bool {
    use crate::common::types::SpacePostPartition;
    let (pk, sk) =
        SpacePostSubscription::keys(&SpacePostPartition(discussion_id.to_string()), user_pk);
    SpacePostSubscription::get(&ctx.ddb, &pk, Some(sk))
        .await
        .expect("get subscription")
        .is_some()
}

#[tokio::test]
async fn test_subscribe_creates_row() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "subscribe: {:?}", body);
    assert!(
        subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await,
        "subscription row should exist after subscribe"
    );
}

#[tokio::test]
async fn test_subscribe_is_idempotent() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;
    let path = format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id);

    for _ in 0..2 {
        let (status, _, body) = crate::test_post! {
            app: ctx.app.clone(),
            path: &path,
            headers: ctx.test_user.1.clone(),
        };
        assert_eq!(status, 200, "subscribe twice: {:?}", body);
    }
    assert!(subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await);
}

#[tokio::test]
async fn test_unsubscribe_deletes_row() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };

    let (status, _, body) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "unsubscribe: {:?}", body);
    assert!(
        !subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await,
        "subscription row should be gone after unsubscribe"
    );
}

#[tokio::test]
async fn test_subscribe_requires_auth() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
    };
    assert_ne!(status, 200, "unauthenticated subscribe must fail");
}
```

- [ ] **Step 3: Run the test — expect compile failure**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_subscribe_creates_row`
Expected: FAIL — `subscribe_discussion` route does not exist yet (404 from the router → status != 200), or compile error if imports of `SpacePostSubscription` resolve but the endpoint is missing. This is the expected red state.

- [ ] **Step 4: Commit the failing test**

```bash
git add app/ratel/src/tests/discussion_subscription_tests.rs app/ratel/src/tests/mod.rs
git commit -m "test(discussion-sub): failing subscribe/unsubscribe controller tests"
```

---

### Task 4: Implement subscribe + unsubscribe controllers

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/subscribe_discussion.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/unsubscribe_discussion.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/mod.rs`

- [ ] **Step 1: Write `subscribe_discussion.rs`**

```rust
use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::common::types::SpacePostPartition;
use crate::features::spaces::pages::actions::actions::discussion::*;

#[mcp_tool(
    name = "subscribe_discussion",
    description = "Subscribe the current member to a discussion. While subscribed, they receive a notification and email for every new comment or reply. Idempotent."
)]
#[post("/api/spaces/{space_id}/discussions/{discussion_sk}/subscribe", role: SpaceUserRole, member: SpaceUser, _space: SpaceCommon)]
pub async fn subscribe_discussion(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
) -> Result<()> {
    SpacePost::can_view(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let discussion_sk_entity: EntityType = discussion_sk.into();
    let space_post_pk = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };

    // PutItem overwrites, so a repeat subscribe is naturally idempotent.
    let sub = SpacePostSubscription::new(space_post_pk, space_id, &member.pk);
    sub.create(cli).await.map_err(|e| {
        crate::error!("subscribe_discussion failed: {e}");
        SpaceActionDiscussionError::CreateFailed
    })?;

    Ok(())
}
```

- [ ] **Step 2: Write `unsubscribe_discussion.rs`**

```rust
use crate::common::models::space::{SpaceCommon, SpaceUser};
use crate::common::types::SpacePostPartition;
use crate::features::spaces::pages::actions::actions::discussion::*;

#[mcp_tool(
    name = "unsubscribe_discussion",
    description = "Unsubscribe the current member from a discussion so they stop receiving comment notifications and emails."
)]
#[delete("/api/spaces/{space_id}/discussions/{discussion_sk}/subscribe", role: SpaceUserRole, member: SpaceUser, _space: SpaceCommon)]
pub async fn unsubscribe_discussion(
    #[mcp(description = "Space partition key")] space_id: SpacePartition,
    #[mcp(description = "Discussion sort key (e.g. 'SpacePost#<uuid>')")]
    discussion_sk: SpacePostEntityType,
) -> Result<()> {
    SpacePost::can_view(&role)?;
    let _ = space_id;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let discussion_sk_entity: EntityType = discussion_sk.into();
    let space_post_pk = match &discussion_sk_entity {
        EntityType::SpacePost(id) => SpacePostPartition(id.clone()),
        _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
    };

    let (pk, sk) = SpacePostSubscription::keys(&space_post_pk, &member.pk);
    // Deleting a non-existent row is a no-op, so unsubscribe is idempotent.
    let item = SpacePostSubscription::delete_transact_write_item(&pk, &sk);
    crate::transact_write_items!(cli, vec![item]).map_err(|e| {
        crate::error!("unsubscribe_discussion failed: {e}");
        SpaceActionDiscussionError::CreateFailed
    })?;

    Ok(())
}
```

- [ ] **Step 3: Register both modules**

In `controllers/discussions/mod.rs`, add after the existing entries:

```rust
mod subscribe_discussion;
pub use subscribe_discussion::*;

mod unsubscribe_discussion;
pub use unsubscribe_discussion::*;
```

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS. If `transact_write_items!` macro or `delete_transact_write_item` is not found, confirm the import chain via the `discussion::*` glob (used by `delete_comment.rs`, which calls `SpacePostComment::delete_transact_write_item`).

- [ ] **Step 5: Run the Phase-2 tests — expect PASS**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_subscribe discussion_subscription_tests::test_unsubscribe`
Expected: `test_subscribe_creates_row`, `test_subscribe_is_idempotent`, `test_unsubscribe_deletes_row`, `test_subscribe_requires_auth` all PASS.

- [ ] **Step 6: Lint & format**

```bash
cd app/ratel
dx fmt -f src/features/spaces/pages/actions/actions/discussion/controllers/discussions/subscribe_discussion.rs
dx fmt -f src/features/spaces/pages/actions/actions/discussion/controllers/discussions/unsubscribe_discussion.rs
```

- [ ] **Step 7: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/
git commit -m "feat(discussion-sub): subscribe/unsubscribe controllers"
```

---

## Phase 3 — Author auto-subscribe on discussion creation

### Task 5: Auto-subscribe the author when a discussion is created

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/create_discussion.rs`

- [ ] **Step 1: Write a failing test**

Append to `app/ratel/src/tests/discussion_subscription_tests.rs`:

```rust
#[tokio::test]
async fn test_create_discussion_auto_subscribes_author() {
    let ctx = TestContext::setup().await;

    // Seed a space owned by the test user so create_discussion's SpaceUser
    // extractor resolves to a creator role. Reuse the seed helper's space,
    // but create the discussion through the real endpoint so the auto-subscribe
    // code path runs.
    use crate::common::models::space::SpaceCommon;
    use crate::common::types::{SpaceStatus, SpacePublishState, SpaceVisibility};
    let space_id = uuid::Uuid::new_v4().to_string();
    let post_id = space_id.clone();
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let mut space = SpaceCommon::default();
    space.pk = Partition::Space(space_id.clone());
    space.sk = EntityType::SpaceCommon;
    space.created_at = now;
    space.updated_at = now;
    space.status = Some(SpaceStatus::Ongoing);
    space.publish_state = SpacePublishState::Published;
    space.visibility = SpaceVisibility::Public;
    space.post_pk = Partition::Feed(post_id.clone());
    space.user_pk = ctx.test_user.0.pk.clone();
    space.author_display_name = ctx.test_user.0.display_name.clone();
    space.author_profile_url = ctx.test_user.0.profile_url.clone();
    space.author_username = ctx.test_user.0.username.clone();
    space.create(&ctx.ddb).await.expect("create space");
    let post = crate::features::posts::models::Post {
        pk: Partition::Feed(post_id.clone()),
        sk: EntityType::Post,
        title: "AutoSub".to_string(),
        ..Default::default()
    };
    post.create(&ctx.ddb).await.expect("create post");

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions", space_id),
        headers: ctx.test_user.1.clone(),
        body: {},
    };
    assert_eq!(status, 200, "create discussion: {:?}", body);

    let discussion_sk = body["sk"].as_str().expect("sk in response");
    let discussion_id = discussion_sk
        .strip_prefix("SPACE_POST#")
        .unwrap_or(discussion_sk)
        .to_string();

    assert!(
        subscription_exists(&ctx, &discussion_id, &ctx.test_user.0.pk).await,
        "author should be auto-subscribed to their new discussion"
    );
}
```

> Note: `body["sk"]` is serialized from `EntityType::SpacePost(uuid)`. Its string form is `SPACE_POST#<uuid>` — the test strips that prefix to get the raw id used by `subscription_exists`. If serialization differs, adjust the strip accordingly (run the test once and inspect the printed `body`).

- [ ] **Step 2: Run it — expect FAIL**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_create_discussion_auto_subscribes_author`
Expected: FAIL — no subscription row is created on discussion creation yet.

- [ ] **Step 3: Add auto-subscribe to `create_discussion`**

In `create_discussion.rs`, the handler currently ends with `bump_participant_activity(...)` then `Ok(post)`. Add the subscription write to the existing `items` transaction. Find:

```rust
    let mut items = vec![
        post.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    items.push(
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_posts(
            &space_pk, 1,
        ),
    );
```

Replace with:

```rust
    let mut items = vec![
        post.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    items.push(
        crate::features::spaces::space_common::models::aggregate::DashboardAggregate::inc_posts(
            &space_pk, 1,
        ),
    );

    // Auto-subscribe the author to their own discussion so they receive a
    // notification for every comment/reply. `post.sk` is EntityType::SpacePost.
    let author_subscription = {
        let space_post_id = match &post.sk {
            EntityType::SpacePost(id) => id.clone(),
            _ => unreachable!("SpacePost::new always sets sk = EntityType::SpacePost"),
        };
        SpacePostSubscription::new(
            crate::common::types::SpacePostPartition(space_post_id),
            space_id_for_sub.clone(),
            &member.pk,
        )
    };
    items.push(author_subscription.create_transact_write_item());
```

The handler consumes `space_id` via `let space_pk: Partition = space_id.into();` earlier. To keep a `SpacePartition` for the subscription, capture it before that move. Find:

```rust
    let space_pk: Partition = space_id.into();
```

Replace with:

```rust
    let space_id_for_sub = space_id.clone();
    let space_pk: Partition = space_id.into();
```

> `SpacePostSubscription` is in scope via the `discussion::*` glob already imported at the top of `create_discussion.rs` (`use crate::features::spaces::pages::actions::actions::discussion::*;`).

- [ ] **Step 4: Run the test — expect PASS**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_create_discussion_auto_subscribes_author`
Expected: PASS.

- [ ] **Step 5: Verify build + format + commit**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
dx fmt -f src/features/spaces/pages/actions/actions/discussion/controllers/discussions/create_discussion.rs
git add app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/create_discussion.rs app/ratel/src/tests/discussion_subscription_tests.rs
git commit -m "feat(discussion-sub): auto-subscribe author on discussion creation"
```

---

## Phase 4 — `subscribed` on the discussion detail response

### Task 6: Add `subscribed: bool` to `DiscussionResponse` and compute it

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/types/discussion_response.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/get_discussion_detail.rs`

- [ ] **Step 1: Write a failing test**

Append to `discussion_subscription_tests.rs`:

```rust
#[tokio::test]
async fn test_detail_reports_subscribed_state() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    // Before subscribing → subscribed = false.
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/detail", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "detail: {:?}", body);
    assert_eq!(body["subscribed"], serde_json::json!(false), "detail before: {:?}", body);

    // Subscribe, then detail → subscribed = true.
    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/subscribe", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/{}/discussions/{}/detail", space_id, discussion_id),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "detail: {:?}", body);
    assert_eq!(body["subscribed"], serde_json::json!(true), "detail after: {:?}", body);
}
```

- [ ] **Step 2: Run it — expect FAIL**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_detail_reports_subscribed_state`
Expected: FAIL — `body["subscribed"]` is null (field absent).

- [ ] **Step 3: Add the field to the response struct**

In `discussion_response.rs`, find:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DiscussionResponse {
    pub post: SpacePost,
    pub space_action: crate::features::spaces::pages::actions::models::SpaceAction,
}
```

Replace with:

```rust
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct DiscussionResponse {
    pub post: SpacePost,
    pub space_action: crate::features::spaces::pages::actions::models::SpaceAction,
    #[serde(default)]
    pub subscribed: bool,
}
```

- [ ] **Step 4: Compute `subscribed` in the detail controller**

In `get_discussion_detail.rs`, change the route to also take an optional user, and compute `subscribed`. Find the route attribute + signature:

```rust
#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/detail", role: SpaceUserRole)]
pub async fn get_discussion_detail(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<DiscussionResponse> {
```

Replace with:

```rust
#[get("/api/spaces/{space_id}/discussions/{discussion_sk}/detail", role: SpaceUserRole, user: crate::common::models::OptionalUser)]
pub async fn get_discussion_detail(
    space_id: SpacePartition,
    discussion_sk: SpacePostEntityType,
) -> Result<DiscussionResponse> {
```

Then find the final return:

```rust
    Ok(DiscussionResponse { post, space_action })
}
```

Replace with:

```rust
    let user: Option<crate::features::auth::User> = user.into();
    let subscribed = if let Some(u) = &user {
        let space_post_pk = match &discussion_sk_entity {
            EntityType::SpacePost(id) => crate::common::types::SpacePostPartition(id.clone()),
            _ => return Err(SpaceActionDiscussionError::InvalidDiscussionId.into()),
        };
        let (sub_pk, sub_sk) = SpacePostSubscription::keys(&space_post_pk, &u.pk);
        SpacePostSubscription::get(cli, &sub_pk, Some(sub_sk))
            .await
            .map_err(|e| {
                crate::error!("get_discussion_detail subscribed check failed: {e}");
                SpaceActionDiscussionError::NotFound
            })?
            .is_some()
    } else {
        false
    };

    Ok(DiscussionResponse {
        post,
        space_action,
        subscribed,
    })
}
```

> `discussion_sk_entity` is already bound earlier in the handler (`let discussion_sk_entity: EntityType = discussion_sk.clone().into();`). `SpacePostSubscription` is in scope via the `discussion::*` glob. `OptionalUser` lives at `crate::common::models::OptionalUser` (used by `get_my_score.rs`).

- [ ] **Step 5: Run the test — expect PASS**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_detail_reports_subscribed_state`
Expected: PASS.

- [ ] **Step 6: Verify both builds (the response type crosses the web boundary)**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```
Expected: PASS both.

- [ ] **Step 7: Format + commit**

```bash
cd app/ratel
dx fmt -f src/features/spaces/pages/actions/actions/discussion/types/discussion_response.rs
dx fmt -f src/features/spaces/pages/actions/actions/discussion/controllers/discussions/get_discussion_detail.rs
git add app/ratel/src/features/spaces/pages/actions/actions/discussion/types/discussion_response.rs app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/discussions/get_discussion_detail.rs app/ratel/src/tests/discussion_subscription_tests.rs
git commit -m "feat(discussion-sub): expose subscribed state on discussion detail"
```

---

## Phase 5 — Notification enum variants

### Task 7: Add `DiscussionCommentPosted` to InboxKind / InboxPayload

**Files:**
- Modify: `app/ratel/src/common/types/inbox_kind.rs`

- [ ] **Step 1: Add the `InboxKind` variant**

In `inbox_kind.rs`, find `CrossPostingFailed,` (last variant of `enum InboxKind`) and add after it:

```rust
    CrossPostingFailed,
    /// A new comment/reply was posted on a discussion the recipient subscribes
    /// to (and they are not the author, mentionee, or direct reply target).
    DiscussionCommentPosted,
```

In `as_prefix()`, add the arm before the closing brace of the match:

```rust
            InboxKind::CrossPostingFailed => "XPOST_FAIL",
            InboxKind::DiscussionCommentPosted => "DISC_CMT",
```

- [ ] **Step 2: Add the `InboxPayload` variant**

In `enum InboxPayload`, after the `CrossPostingFailed { .. }` variant, add:

```rust
    DiscussionCommentPosted {
        space_id: SpacePartition,
        discussion_id: String,
        discussion_title: String,
        commenter_name: String,
        commenter_profile_url: String,
        comment_preview: String,
        cta_url: String,
    },
```

In `impl InboxPayload { pub fn url(&self) ... }`, add the arm:

```rust
            InboxPayload::CrossPostingFailed { cta_url, .. } => cta_url,
            InboxPayload::DiscussionCommentPosted { cta_url, .. } => cta_url,
```

In `impl InboxPayload { pub fn kind(&self) ... }`, add the arm:

```rust
            InboxPayload::CrossPostingFailed { .. } => InboxKind::CrossPostingFailed,
            InboxPayload::DiscussionCommentPosted { .. } => InboxKind::DiscussionCommentPosted,
```

- [ ] **Step 3: Verify both builds**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```
Expected: PASS. If other `match` over `InboxKind`/`InboxPayload` exist (e.g. a frontend renderer that maps payload → UI), the compiler will flag them — handle in Task 11 (frontend). For now `cargo check --features web` may surface a non-exhaustive match in the notifications panel; if so, note the file and proceed (Task 11 covers it). If it blocks compilation here, add a minimal arm rendering `comment_preview`/`cta_url` like the `ReplyOnComment` arm in that same renderer.

- [ ] **Step 4: Format + commit**

```bash
cd app/ratel
dx fmt -f src/common/types/inbox_kind.rs
git add app/ratel/src/common/types/inbox_kind.rs
git commit -m "feat(discussion-sub): add DiscussionCommentPosted inbox kind + payload"
```

---

### Task 8: Add `DiscussionCommentNotification` to EmailOperation

**Files:**
- Modify: `app/ratel/src/features/auth/types/email_operation.rs`

- [ ] **Step 1: Add the variant**

In `email_operation.rs`, after the `SpaceActionOngoingNotification { .. }` variant of `enum EmailOperation`, add:

```rust
    DiscussionCommentNotification {
        commenter_name: String,
        discussion_title: String,
        comment_preview: String,
        cta_url: String,
    },
```

- [ ] **Step 2: Add the template_name arm**

In `template_name()`, add before the closing brace:

```rust
            EmailOperation::SpaceActionOngoingNotification { .. } => {
                "space_action_ongoing_notification"
            }
            EmailOperation::DiscussionCommentNotification { .. } => {
                "discussion_comment_notification"
            }
```

- [ ] **Step 3: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS.

- [ ] **Step 4: Format + commit**

```bash
cd app/ratel
dx fmt -f src/features/auth/types/email_operation.rs
git add app/ratel/src/features/auth/types/email_operation.rs
git commit -m "feat(discussion-sub): add DiscussionCommentNotification email operation"
```

> Ops dependency (out of band): the SES template `discussion_comment_notification` must be provisioned in AWS before production emails send. Email is skipped under `test`/`bypass`, so tests are unaffected.

---

## Phase 6 — Unified fan-out

### Task 9: Make the discussion-thread resolver reusable

**Files:**
- Modify: `app/ratel/src/common/utils/reply_notification.rs`

- [ ] **Step 1: Widen visibility of `fetch_space_discussion_thread`**

In `reply_notification.rs`, find:

```rust
#[cfg(feature = "server")]
async fn fetch_space_discussion_thread(
```

Replace `async fn` with `pub(crate) async fn`:

```rust
#[cfg(feature = "server")]
pub(crate) async fn fetch_space_discussion_thread(
```

- [ ] **Step 2: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/common/utils/reply_notification.rs
git commit -m "refactor(discussion-sub): expose fetch_space_discussion_thread to crate"
```

---

### Task 10: Implement the unified fan-out helper + NotificationData variant

**Files:**
- Create: `app/ratel/src/common/utils/discussion_notification.rs`
- Modify: `app/ratel/src/common/utils/mod.rs`
- Modify: `app/ratel/src/common/types/notification_data.rs`

- [ ] **Step 1: Write the helper file**

Create `discussion_notification.rs`:

```rust
//! Unified fan-out for a new comment/reply on a space discussion.
//!
//! Fired as a single `NotificationData::DiscussionCommentPosted` row; recipient
//! resolution happens here at send time so the comment API stays fast. One pass
//! over a shared `seen` set guarantees **one notification per recipient per
//! comment**, with priority: mention > direct reply target > subscriber.
//! Mentions themselves are still created synchronously in the comment
//! controllers; here we only parse the mentioned pks to exclude them.

use crate::common::types::{EntityType, Partition, SpacePartition, SpacePostPartition};

#[cfg(feature = "server")]
#[allow(clippy::too_many_arguments)]
pub async fn send_discussion_comment_posted(
    space_id: &SpacePartition,
    discussion_id: &str,
    discussion_title: &str,
    comment_sk: &str,
    parent_comment_sk: Option<&str>,
    commenter_pk: &str,
    commenter_name: &str,
    comment_content: &str,
    cta_url: &str,
) -> crate::Result<()> {
    use crate::features::auth::models::EmailTemplate;
    use crate::features::auth::types::email_operation::EmailOperation;
    use std::collections::HashSet;
    use std::str::FromStr;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let ses = cfg.ses();

    let space_post_pk = SpacePostPartition(discussion_id.to_string());
    let post_pk: Partition = space_post_pk.clone().into();

    let comment_preview = crate::common::utils::reply_notification::build_preview(comment_content);

    // seen accumulates every pk already handled (higher-priority bucket wins).
    let mut seen: HashSet<String> = HashSet::new();
    seen.insert(commenter_pk.to_string());

    // Priority 1 — mentions: handled synchronously elsewhere, just exclude them.
    for pk in crate::common::utils::mention::extract_mentioned_pks(comment_content) {
        seen.insert(pk);
    }

    // Priority 2 — direct reply targets (replies only).
    let mut reply_emails: Vec<String> = Vec::new();
    if let Some(parent_sk_str) = parent_comment_sk {
        if let Ok(parent_sk) = EntityType::from_str(parent_sk_str) {
            if let Some((_parent_content, parent_author_pk, reply_author_pks)) =
                crate::common::utils::reply_notification::fetch_space_discussion_thread(
                    cli, &post_pk, &parent_sk,
                )
                .await
            {
                let mut targets: Vec<Partition> = Vec::with_capacity(1 + reply_author_pks.len());
                targets.push(parent_author_pk);
                targets.extend(reply_author_pks);

                for pk in targets {
                    let pk_str = pk.to_string();
                    if !seen.insert(pk_str.clone()) {
                        continue;
                    }
                    let Some(email) = lookup_email(cli, &pk).await else {
                        continue;
                    };
                    let payload = crate::common::types::InboxPayload::ReplyOnComment {
                        space_id: Some(space_id.clone()),
                        post_id: None,
                        comment_preview: comment_preview.clone(),
                        replier_name: commenter_name.to_string(),
                        replier_profile_url: String::new(),
                        cta_url: cta_url.to_string(),
                    };
                    if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
                        pk.clone(),
                        payload,
                        comment_sk,
                    )
                    .await
                    {
                        crate::error!("discussion reply inbox row failed: {e}");
                    }
                    reply_emails.push(email);
                }
            }
        }
    }

    // Priority 3 — subscribers (direct partition Query, never a Scan).
    let mut subscriber_emails: Vec<String> = Vec::new();
    let opt = crate::features::spaces::pages::actions::actions::discussion::SpacePostSubscription::opt()
        .sk(crate::features::spaces::pages::actions::actions::discussion::SpacePostSubscription::sk_prefix());
    let subs = match crate::features::spaces::pages::actions::actions::discussion::SpacePostSubscription::query(
        cli,
        post_pk.clone(),
        opt,
    )
    .await
    {
        Ok((items, _)) => items,
        Err(e) => {
            crate::error!("list discussion subscribers failed: {e}");
            Vec::new()
        }
    };

    for sub in subs {
        let pk = sub.user_pk;
        let pk_str = pk.to_string();
        if !seen.insert(pk_str.clone()) {
            continue;
        }
        let Some(email) = lookup_email(cli, &pk).await else {
            continue;
        };
        let payload = crate::common::types::InboxPayload::DiscussionCommentPosted {
            space_id: space_id.clone(),
            discussion_id: discussion_id.to_string(),
            discussion_title: discussion_title.to_string(),
            commenter_name: commenter_name.to_string(),
            commenter_profile_url: String::new(),
            comment_preview: comment_preview.clone(),
            cta_url: cta_url.to_string(),
        };
        if let Err(e) = crate::common::utils::inbox::create_inbox_row_once(
            pk.clone(),
            payload,
            comment_sk,
        )
        .await
        {
            crate::error!("discussion subscriber inbox row failed: {e}");
        }
        subscriber_emails.push(email);
    }

    // Emails — one bulk send per bucket; each recipient is in at most one.
    if !reply_emails.is_empty() {
        let reply_preview = crate::common::utils::reply_notification::build_preview(comment_content);
        let operation = EmailOperation::ReplyOnCommentNotification {
            replier_name: commenter_name.to_string(),
            comment_preview: comment_preview.clone(),
            reply_preview,
            cta_url: cta_url.to_string(),
        };
        let template = EmailTemplate {
            targets: reply_emails,
            operation,
        };
        template.send_email(ses).await?;
    }

    if !subscriber_emails.is_empty() {
        let operation = EmailOperation::DiscussionCommentNotification {
            commenter_name: commenter_name.to_string(),
            discussion_title: discussion_title.to_string(),
            comment_preview,
            cta_url: cta_url.to_string(),
        };
        let template = EmailTemplate {
            targets: subscriber_emails,
            operation,
        };
        template.send_email(ses).await?;
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn lookup_email(cli: &aws_sdk_dynamodb::Client, pk: &Partition) -> Option<String> {
    match crate::common::models::User::get(cli, pk, Some(EntityType::User)).await {
        Ok(Some(u)) if !u.email.is_empty() => Some(u.email),
        Ok(_) => None,
        Err(e) => {
            crate::error!("discussion notification user lookup failed for {pk}: {e}");
            None
        }
    }
}
```

> If `SpacePostSubscription::query` / `::opt` aren't the generated names, mirror the inbox call site exactly: `UserInboxNotification::query(&cli, pk, UserInboxNotification::opt().sk("...".to_string()))` — the DynamoEntity derive generates `query` + `opt` for every entity. The sk prefix string must match the entity's prefix `SPACE_POST_SUBSCRIPTION` (returned by `SpacePostSubscription::sk_prefix()`).

- [ ] **Step 2: Register the module**

In `common/utils/mod.rs`, add alongside `pub mod reply_notification;`:

```rust
pub mod discussion_notification;
```

- [ ] **Step 3: Add the `NotificationData` variant + send arm**

In `notification_data.rs`, after the `ReplyOnComment { .. }` variant, add:

```rust
    DiscussionCommentPosted {
        space_id: SpacePartition,
        discussion_id: String,
        discussion_title: String,
        comment_sk: String,
        parent_comment_sk: Option<String>,
        commenter_pk: String,
        commenter_name: String,
        comment_content: String,
        cta_url: String,
    },
```

In `impl NotificationData { pub async fn send(&self) ... }`, add a match arm before `NotificationData::None =>`:

```rust
            NotificationData::DiscussionCommentPosted {
                space_id,
                discussion_id,
                discussion_title,
                comment_sk,
                parent_comment_sk,
                commenter_pk,
                commenter_name,
                comment_content,
                cta_url,
            } => {
                crate::common::utils::discussion_notification::send_discussion_comment_posted(
                    space_id,
                    discussion_id,
                    discussion_title,
                    comment_sk,
                    parent_comment_sk.as_deref(),
                    commenter_pk,
                    commenter_name,
                    comment_content,
                    cta_url,
                )
                .await?;
            }
```

> `SpacePartition` is already imported in `notification_data.rs` (used by other payloads via `crate::common::*`). If not, add `use crate::common::types::SpacePartition;`.

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS.

- [ ] **Step 5: Format + commit**

```bash
cd app/ratel
dx fmt -f src/common/utils/discussion_notification.rs
dx fmt -f src/common/types/notification_data.rs
git add app/ratel/src/common/utils/discussion_notification.rs app/ratel/src/common/utils/mod.rs app/ratel/src/common/types/notification_data.rs
git commit -m "feat(discussion-sub): unified comment fan-out (mention>reply>subscriber)"
```

---

### Task 11: Fire the unified notification from comment controllers

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/add_comment.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/reply_comment.rs`

- [ ] **Step 1: Write a failing notification test**

Append to `discussion_subscription_tests.rs`:

```rust
use crate::common::models::notification::UserInboxNotification;
use crate::common::types::InboxKind;

async fn inbox_rows_for(ctx: &TestContext, user_pk: Partition) -> Vec<UserInboxNotification> {
    let (rows, _) = UserInboxNotification::query(
        &ctx.ddb,
        user_pk,
        UserInboxNotification::opt().sk("USER_INBOX_NOTIFICATION".to_string()),
    )
    .await
    .expect("query inbox");
    rows
}

#[tokio::test]
async fn test_comment_notifies_subscriber_not_author() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    // A second user subscribes.
    let (subscriber, _headers) = ctx.create_another_user().await;
    {
        use crate::common::types::SpacePostPartition;
        let sub = SpacePostSubscription::new(
            SpacePostPartition(discussion_id.clone()),
            crate::common::types::SpacePartition(space_id.clone()),
            &subscriber.pk,
        );
        sub.create(&ctx.ddb).await.expect("seed subscription");
    }

    // The author (test_user) posts a top-level comment. Drive the fan-out
    // directly (the DynamoDB stream does not run in unit tests).
    crate::common::utils::discussion_notification::send_discussion_comment_posted(
        &crate::common::types::SpacePartition(space_id.clone()),
        &discussion_id,
        "Test Discussion",
        "SPACE_POST_COMMENT#test-comment-1",
        None,
        &ctx.test_user.0.pk.to_string(),
        &ctx.test_user.0.display_name,
        "hello everyone",
        "/spaces/x/discussions/y/comments/z",
    )
    .await
    .expect("fan-out");

    // Subscriber gets exactly one DiscussionCommentPosted row.
    let sub_rows = inbox_rows_for(&ctx, subscriber.pk.clone()).await;
    let disc_rows: Vec<_> = sub_rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .collect();
    assert_eq!(disc_rows.len(), 1, "subscriber should get one row: {:?}", sub_rows);

    // Author (the commenter) gets none.
    let author_rows = inbox_rows_for(&ctx, ctx.test_user.0.pk.clone()).await;
    let author_disc: Vec<_> = author_rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .collect();
    assert!(author_disc.is_empty(), "commenter must not be notified: {:?}", author_rows);
}

#[tokio::test]
async fn test_mentioned_subscriber_gets_only_mention_not_subscription() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    let (mentioned, _h) = ctx.create_another_user().await;
    {
        use crate::common::types::SpacePostPartition;
        let sub = SpacePostSubscription::new(
            SpacePostPartition(discussion_id.clone()),
            crate::common::types::SpacePartition(space_id.clone()),
            &mentioned.pk,
        );
        sub.create(&ctx.ddb).await.expect("seed subscription");
    }

    // Comment content mentions the subscriber via markup.
    let content = format!(
        "hey @[{}](user:{})",
        mentioned.display_name,
        mentioned.pk
    );

    crate::common::utils::discussion_notification::send_discussion_comment_posted(
        &crate::common::types::SpacePartition(space_id.clone()),
        &discussion_id,
        "Test Discussion",
        "SPACE_POST_COMMENT#test-comment-2",
        None,
        &ctx.test_user.0.pk.to_string(),
        &ctx.test_user.0.display_name,
        &content,
        "/spaces/x/discussions/y/comments/z",
    )
    .await
    .expect("fan-out");

    // Because the subscriber was mentioned, the fan-out must NOT add a
    // DiscussionCommentPosted row for them (mention has priority).
    let rows = inbox_rows_for(&ctx, mentioned.pk.clone()).await;
    let disc_rows: Vec<_> = rows
        .iter()
        .filter(|r| r.kind == InboxKind::DiscussionCommentPosted)
        .collect();
    assert!(
        disc_rows.is_empty(),
        "mentioned subscriber must not get a subscription row: {:?}",
        rows
    );
}
```

> These tests call the fan-out helper directly, so they pass once Task 10 is implemented — they do not strictly depend on Task 11's controller wiring. Run them now to confirm the helper's dedup logic. The `UserInboxNotification` import path is `crate::common::models::notification::UserInboxNotification`.

- [ ] **Step 2: Run the fan-out tests — expect PASS**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests::test_comment_notifies_subscriber_not_author discussion_subscription_tests::test_mentioned_subscriber_gets_only_mention`
Expected: PASS (validates the Task 10 helper).

- [ ] **Step 3: Wire `add_comment` to fire the unified notification**

In `add_comment.rs`, the handler binds `let _post = SpacePost::get(...)`. Rename to `post` to read its title. Find:

```rust
    let _post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;
```

Replace with:

```rust
    let post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;
```

Then, at the end of the handler, after the existing mention-notification block (the `crate::common::utils::mention::create_mention_notifications(...)` call) and before `Ok(comment.into())`, add:

```rust
    // Fan out to discussion subscribers (one notification per recipient per
    // comment; mention > reply-target > subscriber priority resolved at send
    // time). Recipient resolution runs off the DynamoDB stream — we only
    // enqueue one notification row here.
    {
        let comment_id_str = match &comment.sk {
            EntityType::SpacePostComment(id) => id.clone(),
            _ => String::new(),
        };
        let cta_url = format!(
            "{}/spaces/{}/discussions/{}/comments/{}",
            crate::common::config::site_base_url(),
            space_id,
            discussion_sk,
            comment_id_str,
        );
        let notification = crate::common::models::notification::Notification::new(
            crate::common::types::NotificationData::DiscussionCommentPosted {
                space_id: space_id.clone(),
                discussion_id: discussion_sk.to_string(),
                discussion_title: post.title.clone(),
                comment_sk: comment.sk.to_string(),
                parent_comment_sk: None,
                commenter_pk: member.pk.to_string(),
                commenter_name: member.display_name.clone(),
                comment_content: comment.content.clone(),
                cta_url,
            },
        );
        if let Err(e) = notification.create(cli).await {
            tracing::error!("Failed to enqueue discussion comment notification: {e}");
        }
    }
```

> `discussion_sk.to_string()` for a `SpacePostEntityType` SubPartition is the raw id (no prefix) — matching what the helper expects for `discussion_id`. `space_id` is `SpacePartition` (Clone). Both are still in scope at the end of `add_comment`.

- [ ] **Step 4: Wire `reply_comment` — replace the old ReplyOnComment fire**

In `reply_comment.rs`, find the existing reply-notification block:

```rust
    // Fire reply-on-comment notification. Recipient resolution (parent author +
    // thread participants → emails) runs at send time, not here — the handler
    // only persists one notification row and returns.
    {
        let notification = crate::common::models::notification::Notification::new(
            crate::common::types::NotificationData::ReplyOnComment {
                source:
                    crate::common::utils::reply_notification::ReplyCommentSource::SpaceDiscussion,
                parent_comment_pk: parent_pk_str,
                parent_comment_sk: parent_sk_str,
                replier_pk: member.pk.to_string(),
                replier_name: member.display_name.clone(),
                reply_content: comment.content.clone(),
                cta_url,
            },
        );
        if let Err(e) = notification.create(cli).await {
            tracing::error!("Failed to enqueue reply-on-comment notification: {}", e);
        }
    }

    Ok(comment.into())
```

Replace with the unified notification (reply targets + subscribers handled in one pass at send time):

```rust
    // Fire the unified discussion-comment notification. The send-time fan-out
    // notifies direct reply targets (parent author + prior repliers) AND
    // subscribers, deduped to one notification per recipient per comment with
    // priority mention > reply-target > subscriber. Replaces the old separate
    // ReplyOnComment fire for discussions.
    {
        let _ = parent_pk_str; // parent pk no longer needed (helper rebuilds it)
        let notification = crate::common::models::notification::Notification::new(
            crate::common::types::NotificationData::DiscussionCommentPosted {
                space_id: space_id.clone(),
                discussion_id: discussion_sk.to_string(),
                discussion_title: post.title.clone(),
                comment_sk: comment.sk.to_string(),
                parent_comment_sk: Some(parent_sk_str),
                commenter_pk: member.pk.to_string(),
                commenter_name: member.display_name.clone(),
                comment_content: comment.content.clone(),
                cta_url,
            },
        );
        if let Err(e) = notification.create(cli).await {
            tracing::error!("Failed to enqueue discussion comment notification: {e}");
        }
    }

    Ok(comment.into())
```

`reply_comment` currently binds `let _post = SpacePost::get(...)`. Rename it to `post` so `post.title` is available. Find:

```rust
    let _post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;
```

Replace with:

```rust
    let post = SpacePost::get(cli, &post_pk, Some(post_sk))
        .await?
        .ok_or(SpaceActionDiscussionError::NotFound)?;
```

> `cta_url`, `parent_sk_str`, `space_id`, `discussion_sk`, `member`, `comment` are all in scope at that point (see existing handler body). `parent_pk_str` becomes unused for the notification but is bound earlier; the `let _ = parent_pk_str;` silences the warning. Alternatively delete the `parent_pk_str` binding — but keeping the `let _ =` is lower-risk.

- [ ] **Step 5: Verify build (warnings are errors)**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: PASS. If `parent_pk_str` (or any now-unused binding) triggers a dead-code warning, ensure the `let _ = parent_pk_str;` is present, or remove the original binding.

- [ ] **Step 6: Add a reply dedup test**

Append to `discussion_subscription_tests.rs`:

```rust
#[tokio::test]
async fn test_reply_target_subscriber_gets_only_reply_row() {
    let ctx = TestContext::setup().await;
    let (space_id, discussion_id) = seed_space_and_discussion(&ctx).await;

    // Parent comment authored by user B, who also subscribes.
    let (parent_author, _h) = ctx.create_another_user().await;
    {
        use crate::common::types::SpacePostPartition;
        let sub = SpacePostSubscription::new(
            SpacePostPartition(discussion_id.clone()),
            crate::common::types::SpacePartition(space_id.clone()),
            &parent_author.pk,
        );
        sub.create(&ctx.ddb).await.expect("seed subscription");
    }
    let parent_uuid = uuid::Uuid::now_v7().to_string();
    let parent_sk = format!("SPACE_POST_COMMENT#{parent_uuid}");
    {
        use crate::features::spaces::pages::actions::actions::discussion::SpacePostComment;
        let mut parent = SpacePostComment::default();
        parent.pk = Partition::SpacePost(discussion_id.clone());
        parent.sk = EntityType::SpacePostComment(parent_uuid.clone());
        parent.content = "parent".to_string();
        parent.author_pk = parent_author.pk.clone();
        parent.author_display_name = parent_author.display_name.clone();
        parent.author_username = parent_author.username.clone();
        parent.author_profile_url = parent_author.profile_url.clone();
        parent.create(&ctx.ddb).await.expect("create parent comment");
    }

    // test_user replies to B's comment.
    crate::common::utils::discussion_notification::send_discussion_comment_posted(
        &crate::common::types::SpacePartition(space_id.clone()),
        &discussion_id,
        "Test Discussion",
        "SPACE_POST_COMMENT_REPLY#reply-1",
        Some(&parent_sk),
        &ctx.test_user.0.pk.to_string(),
        &ctx.test_user.0.display_name,
        "my reply",
        "/spaces/x/discussions/y/comments/z",
    )
    .await
    .expect("fan-out");

    let rows = inbox_rows_for(&ctx, parent_author.pk.clone()).await;
    let reply_rows = rows.iter().filter(|r| r.kind == InboxKind::ReplyOnComment).count();
    let disc_rows = rows.iter().filter(|r| r.kind == InboxKind::DiscussionCommentPosted).count();
    assert_eq!(reply_rows, 1, "parent author should get one reply row: {:?}", rows);
    assert_eq!(disc_rows, 0, "no duplicate subscription row for reply target: {:?}", rows);
}
```

- [ ] **Step 7: Run all discussion subscription tests — expect PASS**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests`
Expected: all tests PASS.

- [ ] **Step 8: Format + commit**

```bash
cd app/ratel
dx fmt -f src/features/spaces/pages/actions/actions/discussion/controllers/comments/add_comment.rs
dx fmt -f src/features/spaces/pages/actions/actions/discussion/controllers/comments/reply_comment.rs
git add app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/ app/ratel/src/tests/discussion_subscription_tests.rs
git commit -m "feat(discussion-sub): fire unified notification on comment + reply"
```

---

## Phase 7 — Frontend

### Task 12: Add subscribe state + toggle action to `UseDiscussionArena`

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/discussion/hooks/use_discussion_arena.rs`

- [ ] **Step 1: Add fields to the struct**

In the `UseDiscussionArena` struct, after `pub disc_loader: Loader<DiscussionResponse>,` add:

```rust
    pub subscribed: Signal<bool>,
    pub handle_toggle_subscription: Action<(), ()>,
```

- [ ] **Step 2: Build the signal + action in the provider body**

After `disc_loader` is constructed (the `let disc_loader = use_loader(...)?;` block), add:

```rust
    let mut subscribed = use_signal(|| false);

    // Seed the toggle state from the loaded detail. Reads disc_loader (tracked)
    // and writes a different signal, so this does not self-trigger.
    use_effect(move || {
        if let Ok(detail) = disc_loader() {
            subscribed.set(detail.subscribed);
        }
    });

    let mut toast = use_toast();
    let handle_toggle_subscription = use_action(move || async move {
        let was = subscribed();
        subscribed.set(!was); // optimistic
        let res = if was {
            unsubscribe_discussion(space_id(), discussion_id()).await
        } else {
            subscribe_discussion(space_id(), discussion_id()).await
        };
        if let Err(e) = res {
            subscribed.set(was); // rollback
            toast.error(e);
        }
        Ok::<(), crate::common::Error>(())
    });
```

> `space_id` / `discussion_id` are the reactive `ReadSignal`s already used by `disc_loader` in this hook (they are read as `space_id()` / `discussion_id()` elsewhere in the file). `subscribe_discussion` / `unsubscribe_discussion` are the server functions from Task 4, reachable via the `discussion::*` glob already imported in this hook. `use_toast` is already imported (used by `like_comment`). If a `toast` binding already exists in scope, reuse it instead of re-declaring.

- [ ] **Step 3: Add both fields to the provider struct literal**

In the `Ok(use_context_provider(|| UseDiscussionArena { ... }))` literal, add:

```rust
        disc_loader,
        subscribed,
        handle_toggle_subscription,
```

(Place `subscribed` and `handle_toggle_subscription` anywhere among the field initializers — field shorthand requires the local names to match the struct fields, which they do.)

- [ ] **Step 4: Verify both builds**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```
Expected: PASS. If a second `let mut toast = use_toast();` conflicts with an existing one, remove the duplicate.

- [ ] **Step 5: Format + commit**

```bash
cd app/ratel
dx fmt -f src/features/spaces/pages/index/action_pages/discussion/hooks/use_discussion_arena.rs
git add app/ratel/src/features/spaces/pages/index/action_pages/discussion/hooks/use_discussion_arena.rs
git commit -m "feat(discussion-sub): subscribe signal + toggle action in UseDiscussionArena"
```

---

### Task 13: Add i18n strings

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/discussion/i18n.rs`

- [ ] **Step 1: Add the translate entries**

In the `translate! { DiscussionArenaTranslate; ... }` block, add:

```rust
    subscribe_btn: {
        en: "Subscribe",
        ko: "구독하기",
    },
    subscribed_btn: {
        en: "Subscribed",
        ko: "구독중",
    },
    subscribe_tooltip: {
        en: "Get notified by alert and email when new comments are posted on this discussion.",
        ko: "이 토론에 새 댓글이 올라오면 알림과 이메일로 받아봅니다.",
    },
```

- [ ] **Step 2: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web`
Expected: PASS.

- [ ] **Step 3: Format + commit**

```bash
cd app/ratel
dx fmt -f src/features/spaces/pages/index/action_pages/discussion/i18n.rs
git add app/ratel/src/features/spaces/pages/index/action_pages/discussion/i18n.rs
git commit -m "feat(discussion-sub): i18n strings for subscribe button"
```

---

### Task 14: Render the toggle button in the top bar

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/discussion/component.rs`
- Modify: `app/ratel/assets/main.css`

- [ ] **Step 1: Pull the new controller fields where the component consumes `arena`**

In `DiscussionArenaPage`, after the existing `let arena = use_discussion_arena(space_id, discussion_id)?;` and the lines that read `arena.<field>`, add:

```rust
    let subscribed = arena.subscribed;
    let mut handle_toggle_subscription = arena.handle_toggle_subscription;
```

- [ ] **Step 2: Replace the `topbar__right` block with the button + status**

Find:

```rust
                div { class: "topbar__right",
                    span { class: "{status_class}", "{status_text}" }
                }
```

Replace with:

```rust
                div { class: "topbar__right",
                    button {
                        class: "topbar__subscribe",
                        "data-testid": "discussion-subscribe-btn",
                        "aria-label": "{tr.subscribe_tooltip}",
                        title: "{tr.subscribe_tooltip}",
                        "aria-pressed": subscribed(),
                        disabled: handle_toggle_subscription.pending(),
                        onclick: move |_| handle_toggle_subscription.call(),
                        if subscribed() {
                            "{tr.subscribed_btn}"
                        } else {
                            "{tr.subscribe_btn}"
                        }
                    }
                    span { class: "{status_class}", "{status_text}" }
                }
```

- [ ] **Step 3: Add button CSS to `main.css`**

Append under the existing discussion section marker. Find the line `/* === src/features/spaces/pages/index/action_pages/discussion/style.css === */` in `app/ratel/assets/main.css` and add these rules right after the `.discussion-arena { ... }` variable block (so the button can use the component variables):

```css
/* discussion subscribe toggle (top bar) */
.topbar__subscribe {
  display: inline-flex;
  align-items: center;
  gap: 6px;
  padding: 6px 14px;
  margin-right: 10px;
  border-radius: 999px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  border: 1px solid var(--dark, rgba(255, 255, 255, 0.14)) var(--light, rgba(0, 0, 0, 0.12));
  background: var(--dark, rgba(255, 255, 255, 0.04)) var(--light, rgba(0, 0, 0, 0.03));
  color: var(--dark, #f0f0f5) var(--light, #12121a);
  transition: background 150ms ease, border-color 150ms ease, opacity 150ms ease;
}
.topbar__subscribe:hover {
  background: var(--dark, rgba(255, 255, 255, 0.08)) var(--light, rgba(0, 0, 0, 0.06));
}
.topbar__subscribe[aria-pressed="true"] {
  border-color: #fcb300;
  color: #fcb300;
  background: rgba(252, 179, 0, 0.12);
}
.topbar__subscribe:disabled {
  opacity: 0.55;
  cursor: default;
}
```

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web`
Expected: PASS.

- [ ] **Step 5: Lint classes + format**

```bash
cd app/ratel
rustywind --custom-regex "class: \"(.*)\"" --write src/features/spaces/pages/index/action_pages/discussion/component.rs
dx fmt -f src/features/spaces/pages/index/action_pages/discussion/component.rs
```

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/action_pages/discussion/component.rs app/ratel/assets/main.css
git commit -m "feat(discussion-sub): subscribe toggle button in discussion top bar"
```

---

### Task 15: Render `DiscussionCommentPosted` in the notification panel (if needed)

**Files:**
- Modify: the notification panel renderer that matches over `InboxPayload` (find it in Step 1)

- [ ] **Step 1: Locate any exhaustive match over `InboxPayload` in the frontend**

Run: `cd app/ratel && grep -rn "InboxPayload::" src/features/notifications/components src/features/notifications/types | grep -v "_ =>"`
If a renderer matches each `InboxPayload` variant (e.g. to produce title/body/icon), it needs a new arm. If everything already routes through `payload.url()` + a generic preview (no per-variant match), skip to Step 4.

- [ ] **Step 2: Add a render arm (only if Step 1 found an exhaustive match)**

Mirror the `ReplyOnComment` arm. Example shape (adapt field names to the actual renderer):

```rust
            InboxPayload::DiscussionCommentPosted {
                discussion_title,
                commenter_name,
                comment_preview,
                cta_url,
                ..
            } => NotificationView {
                title: format!("{commenter_name} commented on \"{discussion_title}\""),
                preview: comment_preview.clone(),
                url: cta_url.clone(),
            },
```

- [ ] **Step 3: Add i18n for the rendered string if the renderer uses `translate!`** (mirror the existing `ReplyOnComment` entry in that component's i18n).

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web`
Expected: PASS.

- [ ] **Step 5: Commit (only if files changed)**

```bash
git add -A
git commit -m "feat(discussion-sub): render DiscussionCommentPosted in inbox panel"
```

---

## Phase 8 — E2E + final verification

### Task 16: Playwright — subscribe toggle persists across reload

**Files:**
- Modify or create: a Playwright spec under `playwright/tests/web/` covering the discussion flow (extend an existing discussion spec if one exists; otherwise create `discussion-subscription.spec.js`)

- [ ] **Step 1: Find an existing discussion spec to extend**

Run: `ls playwright/tests/web | grep -i discuss` (and `grep -rl "discussions/" playwright/tests/web`). If one exists, add a `test()` into its `test.describe.serial(...)`. Otherwise create a new spec.

- [ ] **Step 2: Write the test**

Add a test that navigates to a discussion the logged-in user can access, clicks the subscribe button, asserts the pressed state, reloads, and asserts the state persisted:

```js
import { test, expect } from "@playwright/test";
import { goto, click } from "../utils";

test("discussion subscribe toggle persists", async ({ page }) => {
  // Navigate to a seeded discussion (reuse the flow the existing discussion
  // spec uses to create/open a discussion; capture its URL here).
  await goto(page, DISCUSSION_URL); // replace with the URL from the create step

  const btn = page.getByTestId("discussion-subscribe-btn");
  await expect(btn).toBeVisible();

  // Subscribe.
  await click(page, { testId: "discussion-subscribe-btn" });
  await expect(btn).toHaveAttribute("aria-pressed", "true");

  // Reload — state comes from the detail response.
  await goto(page, DISCUSSION_URL);
  await expect(page.getByTestId("discussion-subscribe-btn")).toHaveAttribute(
    "aria-pressed",
    "true",
  );

  // Unsubscribe.
  await click(page, { testId: "discussion-subscribe-btn" });
  await expect(page.getByTestId("discussion-subscribe-btn")).toHaveAttribute(
    "aria-pressed",
    "false",
  );
});
```

> Replace `DISCUSSION_URL` with the URL produced by the existing discussion-creation flow in the same serial suite (the create step should set a shared `discussionUrl` variable). If creating a fresh spec, first create a space + discussion via the UI as the other discussion specs do, then run these assertions.

- [ ] **Step 3: Run it**

Run: `cd playwright && npx playwright test tests/web/<file>.spec.js --headed`
Expected: PASS (requires local app running with `--features bypass`).

- [ ] **Step 4: Commit**

```bash
git add playwright/tests/web/
git commit -m "test(discussion-sub): playwright subscribe toggle persistence"
```

---

### Task 17: Full verification sweep

- [ ] **Step 1: Compile all targets, warnings as errors**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```
Expected: PASS all.

- [ ] **Step 2: Run the full server test suite for the feature + regressions**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_subscription_tests
DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- discussion_tests
DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- notifications_tests
```
Expected: PASS all (notifications_tests confirms the existing reply/mention paths still work).

- [ ] **Step 3: Update the spec's acceptance state**

Tick the relevant boxes / note any deviations in `docs/superpower/2026-06-08-discussion-subscription.md` (e.g. confirm "no CDK change", note the SES template provisioning still pending).

- [ ] **Step 4: Final commit**

```bash
git add -A
git commit -m "docs(discussion-sub): close the loop on design doc"
```

---

## Self-Review Notes

- **Spec coverage:** model (Task 2) · EntityType (Task 1) · subscribe/unsubscribe API (Task 4) · members-only gating via `role`+`member` extractors (Task 4) · author auto-subscribe (Task 5) · `subscribed` on detail (Task 6) · InboxKind/Payload (Task 7) · EmailOperation (Task 8) · unified fan-out with mention>reply>subscriber dedup (Tasks 9–11) · actor exclusion (Task 10 `seen` seed) · UI toggle (Tasks 12–14) · notification panel rendering (Task 15) · tests (Tasks 3,5,6,11) · Playwright (Task 16). All spec sections map to a task.
- **No-Scan rule:** subscriber listing uses `SpacePostSubscription::query(post_pk, opt().sk(prefix))` — a partition Query (Task 10). Is-subscribed uses GetItem (Task 6). No `Scan`.
- **Type consistency:** `discussion_id` is always the raw SpacePost id string; `comment_sk` is the full sk string used as the dedup `source_id`; `SpacePostSubscription::keys(&SpacePostPartition, &Partition)` signature is used identically in Tasks 2, 4, 6, and tests.
- **Risk:** Task 7 Step 3 and Task 15 both guard against an exhaustive frontend `InboxPayload` match — whichever surfaces first adds the arm. Task 1 Step 2 guards against an exhaustive `EntityType` match.
