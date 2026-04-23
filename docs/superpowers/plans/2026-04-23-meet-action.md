# Meet Action (Phase 1) Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship Meet as the 5th space action so a space admin can add one from the existing type picker and configure its mode / title / description / start time / duration / reward on a dedicated editor page.

**Architecture:** Mirror the Poll pattern end-to-end. A `SpaceMeet` entity carries Meet-specific fields (`mode`, `start_time`, `duration_min`); a companion `SpaceAction` row carries the shared fields (title, description, credits, prerequisite, status). A single `Route::MeetActionPage` branches internally on `use_space_role` — admin gets the editor, participant gets a minimal viewer. Live and Ended phases render Coming Soon placeholders because the framework needs *something* to draw when `SpaceActionStatus` transitions to `Ongoing` / `Finish`, but the actual live meeting, recording, transcription, calendar, notifications, and Essence ingestion are explicit non-goals of this phase.

**Tech Stack:** Rust (edition 2024), Dioxus 0.7 fullstack + TailwindCSS v4, Axum 0.8, DynamoDB single-table design via `DynamoEntity` derive, Playwright for e2e.

**Spec:** [docs/superpowers/specs/2026-04-23-meet-action-design.md](../specs/2026-04-23-meet-action-design.md)
**Roadmap:** [roadmap/meet-action.md](../../../roadmap/meet-action.md)
**Mockups:** [app/ratel/assets/design/meet-action/](../../../app/ratel/assets/design/meet-action/)

---

## File Structure

### Server-side (new)

| Path | Purpose |
|------|---------|
| `app/ratel/src/features/spaces/pages/actions/actions/meet/mod.rs` | Module root, re-exports |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/models/mod.rs` | Model re-exports |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/models/space_meet.rs` | `SpaceMeet`, `MeetMode` |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/types/mod.rs` | Types re-exports |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/types/error.rs` | `MeetActionError` |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/types/response.rs` | `MeetResponse` |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs` | Controller re-exports |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/create_meet.rs` | `POST /api/spaces/{pk}/meets` |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/get_meet.rs` | `GET /api/spaces/{pk}/meets/{sk}` |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/update_meet.rs` | `POST /api/spaces/{pk}/meets/{sk}` (field-level) |
| `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/delete_meet.rs` | `DELETE /api/spaces/{pk}/meets/{sk}` |
| `app/ratel/src/tests/meet_action_tests.rs` | Integration tests |

### Client-side (new)

| Path | Purpose |
|------|---------|
| `.../meet/components/mod.rs` | Components re-exports |
| `.../meet/components/meet_page/mod.rs` | Page re-exports |
| `.../meet/components/meet_page/component.rs` | `MeetActionPage` (role branch) |
| `.../meet/components/meet_page/editor_view.rs` | `MeetEditorView` (admin) |
| `.../meet/components/meet_page/viewer_view.rs` | `MeetViewerView` (participant + Coming Soon for Live/Ended) |
| `.../meet/components/meet_page/mode_toggle.rs` | `MeetModeToggle` |
| `.../meet/components/meet_page/details_card.rs` | `MeetDetailsCard` |
| `.../meet/components/meet_page/when_card.rs` | `MeetWhenCard` |
| `.../meet/components/meet_page/config_card.rs` | `MeetConfigCard` (wraps common settings) |
| `.../meet/components/meet_page/submit_bar.rs` | `MeetSubmitBar` |
| `.../meet/components/meet_page/style.css` | Editor styles (ported from create-meet.html) |
| `.../meet/components/meet_page/script.js` | Any JS helpers (duration stepper optional) |
| `.../meet/components/meet_page/i18n.rs` | `MeetActionTranslate` |
| `.../meet/components/meet_page/page.html` | Reference mockup (copy of create-meet.html) |
| `.../meet/components/meet_page/hooks/mod.rs` | Hooks re-exports |
| `.../meet/components/meet_page/hooks/use_meet.rs` | `UseMeet` controller |
| `.../meet/components/meet_card/mod.rs` | Card re-exports |
| `.../meet/components/meet_card/component.rs` | `MeetActionCard` (carousel) |
| `.../meet/components/meet_card/style.css` | Card styles |

Where `.../` = `app/ratel/src/features/spaces/pages/actions/actions/meet/`

### Existing files modified

| Path | What changes |
|------|---------|
| `app/ratel/src/common/types/entity_type.rs` | Add `SpaceMeet(String)` variant + `EntityType → Partition` match arm |
| `app/ratel/src/common/types/reward/reward_action.rs` | Add `SpaceMeet` variant |
| `app/ratel/src/common/types/reward/reward_user_behavior.rs` | Add `AttendMeet` variant + update `action()` / `list_behaviors()` |
| `app/ratel/src/common/types/error.rs` | Register `MeetActionError` via `#[from]` + `#[translate(from)]` |
| `app/ratel/src/features/spaces/pages/actions/types/space_action_type.rs` | Add `Meet` variant + `to_behavior()` + `create()` arms |
| `app/ratel/src/features/spaces/pages/actions/actions/mod.rs` | Add `pub mod meet;` |
| `app/ratel/src/features/spaces/space_common/models/dashboard/aggregate.rs` | Add `inc_meets(pk, delta)` |
| `app/ratel/src/features/spaces/models/space.rs` | Add `include_meetings_in_essence: bool` |
| `app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs` | Add `SpaceActionType::Meet` match arm rendering `MeetActionCard` |
| `app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/component.rs` | Add 5th Meet option + i18n keys |
| `app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/style.css` | Meet `[data-type="meet"]` palette |
| `app/ratel/src/route.rs` | Add `Route::MeetActionPage { space_id, meet_id }` + import |
| `app/ratel/src/features/spaces/layout.rs` | Register `MeetActionPage` layout if required |
| `app/ratel/src/common/mcp/server.rs` | Register `create_meet`, `get_meet`, `update_meet`, `delete_meet` MCP tools |
| `app/ratel/src/tests/mod.rs` | Add `mod meet_action_tests;` |
| `playwright/tests/web/space-actions.spec.js` (or equivalent) | Add Meet scenario blocks |

---

## Task 1: Add `EntityType::SpaceMeet` variant

**Files:**
- Modify: `app/ratel/src/common/types/entity_type.rs`

- [ ] **Step 1: Add the enum variant**

Open `app/ratel/src/common/types/entity_type.rs`. Find the `SpaceQuiz(String)` line (around line 92) and add the Meet variant immediately after the Quiz block:

```rust
    SpaceQuiz(String),        // SpaceQuiz#{uuid}
    SpaceQuizAnswer(String),  // SpaceQuizAnswer#{quiz_id}
    SpaceQuizAttempt(String), // SpaceQuizAttempt#{quiz_id}#{attempt_id}

    // Meet action entity types
    SpaceMeet(String),        // SpaceMeet#{uuid}
```

- [ ] **Step 2: Handle the `EntityType → Partition` match arm (if applicable)**

Search the same file for `EntityType::SpacePoll(v) => Partition::Poll(v)` (around line 240). If there is a `Partition::Meet` variant it would need mapping; the Meet action does not need a partition alias, so **leave the match as-is**. The `#[derive(SubPartition)]` will auto-generate `SpaceMeetEntityType` with no additional code.

- [ ] **Step 3: Verify it compiles**

Run:
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```
Expected: compiles clean.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/entity_type.rs
git commit -m "feat(meet-action): add SpaceMeet EntityType variant"
```

---

## Task 2: Add `RewardAction::SpaceMeet` + `RewardUserBehavior::AttendMeet`

**Files:**
- Modify: `app/ratel/src/common/types/reward/reward_action.rs`
- Modify: `app/ratel/src/common/types/reward/reward_user_behavior.rs`

- [ ] **Step 1: Add `RewardAction::SpaceMeet`**

Open `app/ratel/src/common/types/reward/reward_action.rs`, add the variant:

```rust
pub enum RewardAction {
    #[default]
    SpacePoll,
    SpaceStudyAndQuiz,
    SpaceDiscussion,
    SpaceFollow,
    SpaceMeet,
}
```

- [ ] **Step 2: Add `RewardUserBehavior::AttendMeet`**

Open `app/ratel/src/common/types/reward/reward_user_behavior.rs`. Add the variant and update both `action()` and `list_behaviors()`:

```rust
pub enum RewardUserBehavior {
    #[default]
    #[translate(en = "Poll Response", ko = "투표 응답")]
    RespondPoll,
    #[translate(en = "Discussion Comment", ko = "토론 댓글")]
    DiscussionComment,
    #[translate(en = "Quiz Answer", ko = "퀴즈 답변")]
    QuizAnswer,
    #[translate(en = "Follow", ko = "팔로우")]
    Follow,
    #[translate(en = "Attend Meet", ko = "회의 참석")]
    AttendMeet,
}

impl RewardUserBehavior {
    pub fn action(&self) -> RewardAction {
        match self {
            Self::RespondPoll => RewardAction::SpacePoll,
            Self::DiscussionComment => RewardAction::SpaceDiscussion,
            Self::QuizAnswer => RewardAction::SpaceStudyAndQuiz,
            Self::Follow => RewardAction::SpaceFollow,
            Self::AttendMeet => RewardAction::SpaceMeet,
        }
    }

    pub fn list_behaviors(action: RewardAction) -> Vec<Self> {
        match action {
            RewardAction::SpacePoll => vec![Self::RespondPoll],
            RewardAction::SpaceDiscussion => vec![Self::DiscussionComment],
            RewardAction::SpaceStudyAndQuiz => vec![Self::QuizAnswer],
            RewardAction::SpaceFollow => vec![Self::Follow],
            RewardAction::SpaceMeet => vec![Self::AttendMeet],
        }
    }
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/reward/
git commit -m "feat(meet-action): add RewardAction::SpaceMeet + AttendMeet behavior"
```

---

## Task 3: Scaffold Meet module directory

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/mod.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/mod.rs`

- [ ] **Step 1: Create the module root**

Write to `app/ratel/src/features/spaces/pages/actions/actions/meet/mod.rs`:

```rust
pub mod controllers;
pub mod models;
pub mod types;

#[cfg(not(feature = "server"))]
pub mod components;

pub use controllers::*;
pub use models::*;
pub use types::*;

#[cfg(not(feature = "server"))]
pub use components::*;

use crate::features::spaces::pages::actions::*;
```

- [ ] **Step 2: Register the module in the parent**

Open `app/ratel/src/features/spaces/pages/actions/actions/mod.rs`, add:

```rust
pub mod meet;
```

- [ ] **Step 3: Create empty sub-module shells so the root compiles**

Create four placeholder files:

`app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs`:
```rust
```

`app/ratel/src/features/spaces/pages/actions/actions/meet/models/mod.rs`:
```rust
```

`app/ratel/src/features/spaces/pages/actions/actions/meet/types/mod.rs`:
```rust
```

(Files left empty; subsequent tasks fill them in.)

- [ ] **Step 4: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/ app/ratel/src/features/spaces/pages/actions/actions/mod.rs
git commit -m "feat(meet-action): scaffold meet module directory"
```

---

## Task 4: `SpaceMeet` model + `MeetMode` enum

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/models/space_meet.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/models/mod.rs`

- [ ] **Step 1: Define the model**

Write `space_meet.rs`:

```rust
use crate::common::utils::time::get_now_timestamp_millis;
use crate::features::spaces::pages::actions::*;

use crate::common::macros::DynamoEntity;

#[derive(
    Debug, Clone, Serialize, Deserialize, Default, PartialEq, Translate,
)]
pub enum MeetMode {
    #[default]
    #[translate(ko = "예약", en = "Scheduled")]
    Scheduled,
    #[translate(ko = "즉시 시작", en = "Instant")]
    Instant,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
#[dynamo(prefix = "SM")]
pub struct SpaceMeet {
    pub pk: Partition,
    pub sk: EntityType,

    pub created_at: i64,
    pub updated_at: i64,

    pub mode: MeetMode,
    pub start_time: i64,
    pub duration_min: i32,
}

#[cfg(feature = "server")]
impl SpaceMeet {
    pub fn new(space_pk: SpacePartition) -> Result<Self> {
        let now = get_now_timestamp_millis();
        let pk: Partition = space_pk.into();
        let sk = EntityType::SpaceMeet(uuid::Uuid::new_v4().to_string());
        Ok(Self {
            pk,
            sk,
            created_at: now,
            updated_at: now,
            mode: MeetMode::Scheduled,
            start_time: now,
            duration_min: 60,
        })
    }

    pub fn can_edit(role: &SpaceUserRole) -> Result<()> {
        if role.is_admin() {
            Ok(())
        } else {
            Err(crate::common::Error::Unauthorized)
        }
    }
}
```

- [ ] **Step 2: Re-export from `models/mod.rs`**

Write `models/mod.rs`:

```rust
mod space_meet;
pub use space_meet::*;
```

- [ ] **Step 3: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

If `Error::Unauthorized` does not exist, use the closest equivalent already defined on `common::Error` (e.g., `Error::NoPermission`, `Error::BadRequest` as a last resort). Inspect `app/ratel/src/common/types/error.rs` to confirm.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/models/
git commit -m "feat(meet-action): add SpaceMeet entity + MeetMode enum"
```

---

## Task 5: `MeetActionError` typed error enum

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/types/error.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/types/mod.rs`
- Modify: `app/ratel/src/common/types/error.rs`

- [ ] **Step 1: Write the error enum**

Write `meet/types/error.rs`:

```rust
use crate::features::spaces::pages::actions::*;
use dioxus_translate::Translate;
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone, PartialEq)]
pub enum MeetActionError {
    #[error("create meet failed")]
    #[translate(en = "Could not create the meet", ko = "회의를 생성할 수 없습니다")]
    CreateFailed,

    #[error("update meet failed")]
    #[translate(en = "Could not save changes", ko = "변경 사항을 저장할 수 없습니다")]
    UpdateFailed,

    #[error("meet not found")]
    #[translate(en = "Meet not found", ko = "회의를 찾을 수 없습니다")]
    NotFound,

    #[error("invalid duration {0}")]
    #[translate(en = "Duration must be between 15 and 1440 minutes", ko = "지속 시간은 15~1440분 사이여야 합니다")]
    InvalidDuration(i32),

    #[error("delete meet failed")]
    #[translate(en = "Could not delete the meet", ko = "회의를 삭제할 수 없습니다")]
    DeleteFailed,
}
```

- [ ] **Step 2: Re-export from `types/mod.rs`**

Write `types/mod.rs`:

```rust
mod error;
pub use error::*;

mod response;
pub use response::*;
```

(Note: `response.rs` is created in Task 6; we register the mod now so later edits are one-line.)

Create an empty placeholder `types/response.rs`:

```rust
```

- [ ] **Step 3: Register on `common::Error`**

Open `app/ratel/src/common/types/error.rs`. Find the existing `#[from]` / `#[translate(from)]` registrations (search for an existing `#[from] XxxError`). Add:

```rust
#[error("{0}")]
#[translate(from)]
MeetAction(#[from] crate::features::spaces::pages::actions::actions::meet::MeetActionError),
```

Pattern to follow: `SpacePollError` registration if present, or `SpaceRewardError` registration (see `app/ratel/src/common/types/reward/error.rs`).

- [ ] **Step 4: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/types/ app/ratel/src/common/types/error.rs
git commit -m "feat(meet-action): add MeetActionError typed enum"
```

---

## Task 6: `MeetResponse` DTO

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/types/response.rs`

- [ ] **Step 1: Write the response DTO**

```rust
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MeetResponse {
    pub pk: SpacePartition,
    pub sk: SpaceMeetEntityType,
    pub mode: MeetMode,
    pub start_time: i64,
    pub duration_min: i32,
    pub created_at: i64,
    pub updated_at: i64,

    #[serde(default)]
    pub space_action: SpaceAction,
}

#[cfg(feature = "server")]
impl From<SpaceMeet> for MeetResponse {
    fn from(m: SpaceMeet) -> Self {
        let pk: SpacePartition = m.pk.into();
        let sk: SpaceMeetEntityType = m.sk.into();
        Self {
            pk,
            sk,
            mode: m.mode,
            start_time: m.start_time,
            duration_min: m.duration_min,
            created_at: m.created_at,
            updated_at: m.updated_at,
            space_action: SpaceAction::default(),
        }
    }
}
```

- [ ] **Step 2: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

If `Partition::into(SpacePartition)` fails, look at how `PollResponse` does the same conversion in `app/ratel/src/features/spaces/pages/actions/actions/poll/types/response.rs` and mirror it.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/types/response.rs
git commit -m "feat(meet-action): add MeetResponse DTO"
```

---

## Task 7: `DashboardAggregate::inc_meets`

**Files:**
- Modify: `app/ratel/src/features/spaces/space_common/models/dashboard/aggregate.rs`

- [ ] **Step 1: Locate `inc_polls`**

Open the file and find the `inc_polls` function (around line 67 per the earlier grep). Use it as the exact template for `inc_meets`.

- [ ] **Step 2: Add `inc_meets`**

After `inc_polls`, add:

```rust
    pub fn inc_meets(
        space_pk: &Partition,
        delta: i64,
    ) -> aws_sdk_dynamodb::types::TransactWriteItem {
        Self::inc_field(space_pk, "meets", delta)
    }
```

If `inc_polls` uses a different helper name (e.g., `inc_field` vs explicit transact item construction), match its exact code path.

- [ ] **Step 3: Add a `meets: i64` field to the `DashboardAggregate` struct**

In the same file, locate the struct and add:

```rust
    #[serde(default)]
    pub meets: i64,
```

- [ ] **Step 4: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/space_common/models/dashboard/aggregate.rs
git commit -m "feat(meet-action): add DashboardAggregate::inc_meets counter"
```

---

## Task 8: `create_meet` controller — write the failing test first

**Files:**
- Create: `app/ratel/src/tests/meet_action_tests.rs`
- Modify: `app/ratel/src/tests/mod.rs`

- [ ] **Step 1: Register the test module**

Open `app/ratel/src/tests/mod.rs`. Add:

```rust
mod meet_action_tests;
```

- [ ] **Step 2: Write the initial failing test**

Write to `app/ratel/src/tests/meet_action_tests.rs`:

```rust
use super::*;

#[tokio::test]
async fn test_create_meet_admin_success() {
    let ctx = TestContext::setup().await;

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/test-space-id/meets",
        headers: ctx.test_user.1.clone(),
        body: { },
    };
    assert_eq!(status, 200, "create_meet should succeed: {:?}", body);
    assert!(body["sk"].as_str().is_some(), "response must include sk");
}

#[tokio::test]
async fn test_create_meet_unauthenticated() {
    let ctx = TestContext::setup().await;

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: "/api/spaces/test-space-id/meets",
        body: { },
    };
    assert_ne!(status, 200, "unauthenticated create_meet should fail");
}
```

- [ ] **Step 3: Run tests — expect compile failure (handler does not exist yet)**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests
```
Expected: test module compiles but test endpoint returns 404 or method not found → the tests fail (or the crate fails to build because the route is missing; this is fine for the TDD red step).

- [ ] **Step 4: Commit the failing tests**

```bash
git add app/ratel/src/tests/meet_action_tests.rs app/ratel/src/tests/mod.rs
git commit -m "test(meet-action): failing integration tests for create_meet"
```

---

## Task 9: `create_meet` controller — implementation

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/create_meet.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs`

- [ ] **Step 1: Implement the controller**

Write `create_meet.rs`:

```rust
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::models::SpaceAction;
use crate::features::spaces::space_common::models::aggregate::DashboardAggregate;

#[mcp_tool(
    name = "create_meet",
    description = "Create a new meet action in a space. Requires creator role."
)]
#[post("/api/spaces/{space_pk}/meets", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn create_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
) -> Result<MeetResponse> {
    SpaceMeet::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let meet = SpaceMeet::new(space_pk.clone())?;

    let space_action = SpaceAction::new(
        space_pk.clone(),
        SpaceMeetEntityType::from(meet.sk.clone()).to_string(),
        crate::features::spaces::pages::actions::types::SpaceActionType::Meet,
    );

    let space_pk_partition: Partition = space_pk.into();
    let _ = DashboardAggregate::get_or_create(cli, &space_pk_partition).await?;

    let mut items = vec![
        meet.create_transact_write_item(),
        space_action.create_transact_write_item(),
    ];
    items.push(DashboardAggregate::inc_meets(&space_pk_partition, 1));
    crate::transact_write_items!(cli, items).map_err(|e| {
        crate::error!("Failed to create meet: {e}");
        MeetActionError::CreateFailed
    })?;

    let mut ret: MeetResponse = meet.into();
    ret.space_action = space_action;
    Ok(ret)
}
```

- [ ] **Step 2: Register in controllers mod**

Write `controllers/mod.rs`:

```rust
mod create_meet;
pub use create_meet::*;
```

- [ ] **Step 3: Add `SpaceActionType::Meet` enum variant (temporary — no `create()` arm yet)**

Open `app/ratel/src/features/spaces/pages/actions/types/space_action_type.rs`. Add the variant:

```rust
pub enum SpaceActionType {
    #[default]
    #[translate(ko = "투표", en = "Poll")]
    Poll,
    #[translate(ko = "토론", en = "Discussion")]
    TopicDiscussion,
    #[translate(ko = "팔로우", en = "Follow")]
    Follow,
    #[translate(ko = "퀴즈", en = "Quiz")]
    Quiz,
    #[translate(ko = "미팅", en = "Meet")]
    Meet,
}
```

Add the `to_behavior()` arm:

```rust
SpaceActionType::Meet => RewardUserBehavior::AttendMeet,
```

Leave `create()` without a Meet arm for now — it will be completed in Task 14 once the route exists. Match must still be exhaustive, so add a placeholder:

```rust
SpaceActionType::Meet => Err(crate::common::Error::BadRequest("meet routing not wired yet".into())),
```

- [ ] **Step 4: Run the `create_meet` tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests::test_create_meet_admin_success meet_action_tests::test_create_meet_unauthenticated
```
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/ app/ratel/src/features/spaces/pages/actions/types/space_action_type.rs
git commit -m "feat(meet-action): implement create_meet controller"
```

---

## Task 10: `get_meet` controller + test

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/get_meet.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs`
- Modify: `app/ratel/src/tests/meet_action_tests.rs`

- [ ] **Step 1: Add the failing test**

Append to `meet_action_tests.rs`:

```rust
#[tokio::test]
async fn test_get_meet_returns_response_with_space_action() {
    let ctx = TestContext::setup().await;

    let (_, _, create_body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/test-space-id/meets",
        headers: ctx.test_user.1.clone(),
        body: { },
    };
    let meet_sk = create_body["sk"].as_str().unwrap();

    let (status, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/spaces/test-space-id/meets/{}", meet_sk),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "get_meet: {:?}", body);
    assert_eq!(body["sk"].as_str().unwrap(), meet_sk);
    assert!(body["space_action"].is_object(), "space_action must be populated");
}
```

- [ ] **Step 2: Run — expect FAIL (handler missing)**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests::test_get_meet_returns_response_with_space_action
```

- [ ] **Step 3: Implement**

Write `get_meet.rs`:

```rust
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[mcp_tool(
    name = "get_meet",
    description = "Fetch a meet action with its companion SpaceAction row."
)]
#[get("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn get_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
    #[mcp(description = "Meet sort key")] meet_sk: SpaceMeetEntityType,
) -> Result<MeetResponse> {
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();

    let pk: Partition = space_pk.clone().into();
    let sk: EntityType = meet_sk.clone().into();

    let meet = SpaceMeet::get(cli, &pk, &sk).await.map_err(|e| {
        crate::error!("get meet failed: {e}");
        MeetActionError::NotFound
    })?;

    let action_id = SpaceMeetEntityType::from(meet.sk.clone()).to_string();
    let space_action = SpaceAction::get(cli, &pk, &EntityType::SpaceAction, Some(action_id))
        .await
        .unwrap_or_default();

    let mut ret: MeetResponse = meet.into();
    ret.space_action = space_action;
    Ok(ret)
}
```

If `SpaceAction::get(...)` signature does not match (third positional arg for action_id), inspect `features/spaces/pages/actions/actions/poll/controllers/get_poll.rs` for the correct pattern and mirror it.

- [ ] **Step 4: Register in mod.rs**

Append to `controllers/mod.rs`:

```rust
mod get_meet;
pub use get_meet::*;
```

- [ ] **Step 5: Run tests — expect PASS**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests
```

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/ app/ratel/src/tests/meet_action_tests.rs
git commit -m "feat(meet-action): implement get_meet + test"
```

---

## Task 11: `update_meet` controller + tests

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/update_meet.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs`
- Modify: `app/ratel/src/tests/meet_action_tests.rs`

- [ ] **Step 1: Add failing tests**

Append to `meet_action_tests.rs`:

```rust
#[tokio::test]
async fn test_update_meet_mode() {
    let ctx = TestContext::setup().await;
    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/test-space-id/meets",
        headers: ctx.test_user.1.clone(),
        body: { },
    };
    let meet_sk = body["sk"].as_str().unwrap().to_string();

    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/test-space-id/meets/{}", meet_sk),
        headers: ctx.test_user.1.clone(),
        body: { "Mode": { "mode": "Instant" } },
    };
    assert_eq!(status, 200, "update_meet mode: {:?}", body);

    let (_, _, body) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/spaces/test-space-id/meets/{}", meet_sk),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(body["mode"], "Instant", "mode should be updated");
}

#[tokio::test]
async fn test_update_meet_duration_invalid_zero() {
    let ctx = TestContext::setup().await;
    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/test-space-id/meets",
        headers: ctx.test_user.1.clone(),
        body: { },
    };
    let meet_sk = body["sk"].as_str().unwrap().to_string();

    let (status, _, _) = crate::test_post! {
        app: ctx.app,
        path: &format!("/api/spaces/test-space-id/meets/{}", meet_sk),
        headers: ctx.test_user.1.clone(),
        body: { "DurationMin": { "duration_min": 0 } },
    };
    assert_ne!(status, 200, "duration 0 should fail");
}
```

- [ ] **Step 2: Implement `update_meet`**

Write `update_meet.rs`:

```rust
use crate::features::spaces::pages::actions::actions::meet::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UpdateMeetRequest {
    Mode { mode: MeetMode },
    StartTime { start_time: i64 },
    DurationMin { duration_min: i32 },
}

#[mcp_tool(
    name = "update_meet",
    description = "Update meet-specific fields (mode, start_time, duration_min)."
)]
#[post("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole, space: crate::common::models::space::SpaceCommon)]
pub async fn update_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
    #[mcp(description = "Meet sort key")] meet_sk: SpaceMeetEntityType,
    #[mcp(description = "Field-level update request as JSON")] req: UpdateMeetRequest,
) -> Result<()> {
    SpaceMeet::can_edit(&role)?;
    let common_config = crate::common::CommonConfig::default();
    let cli = common_config.dynamodb();
    let pk: Partition = space_pk.into();
    let sk: EntityType = meet_sk.into();

    match req {
        UpdateMeetRequest::Mode { mode } => {
            SpaceMeet::update()
                .mode(mode)
                .updated_at(crate::common::utils::time::get_now_timestamp_millis())
                .execute(cli, &pk, &sk)
                .await
        }
        UpdateMeetRequest::StartTime { start_time } => {
            SpaceMeet::update()
                .start_time(start_time)
                .updated_at(crate::common::utils::time::get_now_timestamp_millis())
                .execute(cli, &pk, &sk)
                .await
        }
        UpdateMeetRequest::DurationMin { duration_min } => {
            if !(15..=1440).contains(&duration_min) {
                return Err(MeetActionError::InvalidDuration(duration_min).into());
            }
            SpaceMeet::update()
                .duration_min(duration_min)
                .updated_at(crate::common::utils::time::get_now_timestamp_millis())
                .execute(cli, &pk, &sk)
                .await
        }
    }
    .map_err(|e| {
        crate::error!("update meet failed: {e}");
        MeetActionError::UpdateFailed
    })?;
    Ok(())
}
```

If the generated `SpaceMeet::update()` fluent builder signature differs, inspect `SpacePoll::update()` usage in `update_poll.rs` for the correct pattern.

- [ ] **Step 3: Register + run tests**

Append to `controllers/mod.rs`:

```rust
mod update_meet;
pub use update_meet::*;
```

Run:
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/ app/ratel/src/tests/meet_action_tests.rs
git commit -m "feat(meet-action): implement update_meet with duration validation"
```

---

## Task 12: `delete_meet` controller + test

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/delete_meet.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs`
- Modify: `app/ratel/src/tests/meet_action_tests.rs`

- [ ] **Step 1: Add failing test**

Append:

```rust
#[tokio::test]
async fn test_delete_meet_removes_row() {
    let ctx = TestContext::setup().await;
    let (_, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/test-space-id/meets",
        headers: ctx.test_user.1.clone(),
        body: { },
    };
    let meet_sk = body["sk"].as_str().unwrap().to_string();

    let (status, _, _) = crate::test_delete! {
        app: ctx.app.clone(),
        path: &format!("/api/spaces/test-space-id/meets/{}", meet_sk),
        headers: ctx.test_user.1.clone(),
    };
    assert_eq!(status, 200, "delete_meet should succeed");

    let (status, _, _) = crate::test_get! {
        app: ctx.app,
        path: &format!("/api/spaces/test-space-id/meets/{}", meet_sk),
        headers: ctx.test_user.1.clone(),
    };
    assert_ne!(status, 200, "get after delete should fail");
}
```

- [ ] **Step 2: Implement — mirror `delete_poll`**

Write `delete_meet.rs`, mirroring `app/ratel/src/features/spaces/pages/actions/actions/poll/controllers/delete_poll.rs` exactly, substituting `SpaceMeet`, `SpaceMeetEntityType`, `DashboardAggregate::inc_meets(..., -1)`, and `MeetActionError::DeleteFailed`.

- [ ] **Step 3: Register + run**

Append to `controllers/mod.rs`:

```rust
mod delete_meet;
pub use delete_meet::*;
```

Run:
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/delete_meet.rs app/ratel/src/features/spaces/pages/actions/actions/meet/controllers/mod.rs app/ratel/src/tests/meet_action_tests.rs
git commit -m "feat(meet-action): implement delete_meet"
```

---

## Task 13: MCP tool registration + test

**Files:**
- Modify: `app/ratel/src/common/mcp/server.rs`
- Modify: `app/ratel/src/tests/mcp_tests.rs`

- [ ] **Step 1: Register each tool on `RatelMcpServer`**

In the `#[tool_router] impl RatelMcpServer` block in `server.rs`, add:

```rust
    #[rmcp::tool(name = "create_meet", description = "Create a new meet action in a space.")]
    async fn create_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::CreateMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::create_meet_mcp_handler(
            &self.mcp_secret, req,
        )
        .await
    }

    #[rmcp::tool(name = "get_meet", description = "Fetch a meet action.")]
    async fn get_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::GetMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::get_meet_mcp_handler(
            &self.mcp_secret, req,
        )
        .await
    }

    #[rmcp::tool(name = "update_meet", description = "Update a meet action field.")]
    async fn update_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::UpdateMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::update_meet_mcp_handler(
            &self.mcp_secret, req,
        )
        .await
    }

    #[rmcp::tool(name = "delete_meet", description = "Delete a meet action.")]
    async fn delete_meet(
        &self,
        Parameters(req): Parameters<crate::features::spaces::pages::actions::actions::meet::DeleteMeetMcpRequest>,
    ) -> McpResult {
        crate::features::spaces::pages::actions::actions::meet::delete_meet_mcp_handler(
            &self.mcp_secret, req,
        )
        .await
    }
```

(The `*McpRequest` structs are auto-generated by `#[mcp_tool]`. If names differ, search the generated code via `grep -rn "CreateMeetMcpRequest" target/` or inspect the corresponding Poll code.)

- [ ] **Step 2: Add MCP test**

Append to `app/ratel/src/tests/mcp_tests.rs`:

```rust
#[tokio::test]
async fn test_mcp_tool_create_meet() {
    let (ctx, token) = setup_mcp_test().await;

    let (status, body) = mcp_tool_call(
        ctx.app,
        &token,
        "create_meet",
        serde_json::json!({ "space_pk": "test-space-id" }),
    )
    .await;

    assert_eq!(status, 200, "mcp create_meet: {:?}", body);
    let content = extract_tool_content(&body);
    assert!(content["sk"].as_str().is_some(), "content must include sk");
}
```

- [ ] **Step 3: Run tests**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- mcp_tests::test_mcp_tool_create_meet
```
Expected: PASS.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/mcp/server.rs app/ratel/src/tests/mcp_tests.rs
git commit -m "feat(meet-action): register Meet MCP tools + tests"
```

---

## Task 14: Register `Route::MeetActionPage` + complete `SpaceActionType::Meet::create()`

**Files:**
- Modify: `app/ratel/src/route.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/types/space_action_type.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/mod.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/mod.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/component.rs`

- [ ] **Step 1: Create the placeholder `MeetActionPage` component**

Write `meet/components/mod.rs`:

```rust
pub mod meet_page;
pub use meet_page::*;
```

Write `meet/components/meet_page/mod.rs`:

```rust
mod component;
pub use component::*;
```

Write `meet/components/meet_page/component.rs` (placeholder — real content lands in Task 16+):

```rust
use crate::features::spaces::pages::actions::*;

#[component]
pub fn MeetActionPage(
    space_id: ReadSignal<SpacePartition>,
    meet_id: ReadSignal<SpaceMeetEntityType>,
) -> Element {
    let _ = (space_id, meet_id);
    rsx! { div { class: "meet-action-page placeholder",
        SeoMeta { title: "Meet" }
        "Meet action page (Phase 1 scaffold)"
    } }
}
```

- [ ] **Step 2: Register the route**

Open `app/ratel/src/route.rs`. Add import near the top (alongside other action page imports, ~line 32):

```rust
use crate::features::spaces::pages::actions::actions::meet::MeetActionPage;
```

Add the route after `FollowActionPage` (around line 196):

```rust
    #[route("/meets/:meet_id")]
    MeetActionPage { space_id: SpacePartition, meet_id: SpaceMeetEntityType },
```

- [ ] **Step 3: Wire up `SpaceActionType::Meet::create()`**

Open `app/ratel/src/features/spaces/pages/actions/types/space_action_type.rs`. Replace the placeholder arm with:

```rust
SpaceActionType::Meet => {
    let response = crate::features::spaces::pages::actions::actions::meet::create_meet(space_id.clone()).await?;
    let meet_id = SpaceMeetEntityType::from(response.sk);
    Ok(Route::MeetActionPage {
        space_id: space_id.clone(),
        meet_id,
    })
}
```

If `response.sk` type is already `SpaceMeetEntityType`, drop the `::from()` and assign directly.

- [ ] **Step 4: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- [ ] **Step 5: Add handler test that `SpaceActionType::Meet::create()` returns the right route**

Append to `meet_action_tests.rs`:

```rust
#[tokio::test]
async fn test_space_action_type_meet_create_returns_route() {
    let ctx = TestContext::setup().await;
    // create must succeed so the route has a valid meet_id
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/spaces/test-space-id/meets",
        headers: ctx.test_user.1.clone(),
        body: { },
    };
    assert_eq!(status, 200, "{:?}", body);
    assert!(body["sk"].as_str().unwrap().starts_with(""), "sk present");
}
```

Run:
```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests
```

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/ app/ratel/src/route.rs app/ratel/src/tests/meet_action_tests.rs
git commit -m "feat(meet-action): register Route::MeetActionPage + wire SpaceActionType::Meet.create()"
```

---

## Task 15: Add `Space.include_meetings_in_essence` field

**Files:**
- Modify: `app/ratel/src/features/spaces/models/space.rs`

- [ ] **Step 1: Add the field**

Open `space.rs`. Inside `pub struct Space { ... }`, add:

```rust
    #[serde(default)]
    pub include_meetings_in_essence: bool,
```

- [ ] **Step 2: Update any `Space::new`, `Default`, test fixtures that build a `Space` literally**

Run `grep -n "Space {" app/ratel/src --include="*.rs" -r` to find literal constructions; add `include_meetings_in_essence: false` where required (or rely on `#[derive(Default)]`).

- [ ] **Step 3: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/models/space.rs
git commit -m "feat(meet-action): add Space.include_meetings_in_essence field (persistence only)"
```

---

## Task 16: `UseMeet` controller hook

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/hooks/mod.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/hooks/use_meet.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/mod.rs`

- [ ] **Step 1: Write `hooks/mod.rs`**

```rust
mod use_meet;
pub use use_meet::*;
```

- [ ] **Step 2: Write `use_meet.rs`**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

#[derive(Clone, Copy)]
pub struct UseMeet {
    pub space_id: ReadSignal<SpacePartition>,
    pub meet_id: ReadSignal<SpaceMeetEntityType>,
    pub meet: Loader<MeetResponse>,
    pub update_mode: Action<(MeetMode,), ()>,
    pub update_start_time: Action<(i64,), ()>,
    pub update_duration: Action<(i32,), ()>,
    pub publish: Action<(), ()>,
}

#[track_caller]
pub fn use_meet(
    space_id: ReadSignal<SpacePartition>,
    meet_id: ReadSignal<SpaceMeetEntityType>,
) -> std::result::Result<UseMeet, RenderError> {
    if let Some(ctx) = try_use_context::<UseMeet>() {
        return Ok(ctx);
    }

    let mut meet = use_loader(move || async move {
        get_meet(space_id(), meet_id()).await
    })?;

    let update_mode = use_action(move |mode: MeetMode| async move {
        update_meet(space_id(), meet_id(), UpdateMeetRequest::Mode { mode }).await?;
        meet.refresh();
        Ok::<(), crate::common::Error>(())
    });

    let update_start_time = use_action(move |start_time: i64| async move {
        update_meet(
            space_id(),
            meet_id(),
            UpdateMeetRequest::StartTime { start_time },
        )
        .await?;
        meet.refresh();
        Ok::<(), crate::common::Error>(())
    });

    let update_duration = use_action(move |duration_min: i32| async move {
        update_meet(
            space_id(),
            meet_id(),
            UpdateMeetRequest::DurationMin { duration_min },
        )
        .await?;
        meet.refresh();
        Ok::<(), crate::common::Error>(())
    });

    let publish = use_action(move || async move {
        let current = meet();
        let mode = current.as_ref().ok().map(|m| m.mode.clone()).unwrap_or_default();
        if mode == MeetMode::Instant {
            let now = crate::common::utils::time::get_now_timestamp_millis();
            update_meet(
                space_id(),
                meet_id(),
                UpdateMeetRequest::StartTime { start_time: now },
            )
            .await?;
        }
        let action_id = meet_id().to_string();
        update_space_action(
            space_id(),
            action_id,
            UpdateSpaceActionRequest::Status {
                status: SpaceActionStatus::Ongoing,
            },
        )
        .await?;
        meet.refresh();
        Ok::<(), crate::common::Error>(())
    });

    Ok(provide_root_context(UseMeet {
        space_id,
        meet_id,
        meet,
        update_mode,
        update_start_time,
        update_duration,
        publish,
    }))
}
```

- [ ] **Step 3: Register hooks in `meet_page/mod.rs`**

```rust
mod component;
pub use component::*;

pub mod hooks;
pub use hooks::*;
```

- [ ] **Step 4: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

If `use_loader` or `Loader<T>` don't resolve, check the imports used in `features/notifications/hooks/use_inbox.rs` and mirror.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/
git commit -m "feat(meet-action): add UseMeet controller hook"
```

---

## Task 17: Add Meet option to `TypePickerModal`

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/component.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/i18n.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/style.css`

- [ ] **Step 1: Add translation keys**

Open `pages/index/i18n.rs`. Inside `SpaceViewerTranslate`:

```rust
    meet_name: {
        en: "Meet",
        ko: "미팅",
    },
    meet_desc: {
        en: "Live meeting with recording & transcript",
        ko: "녹화·transcript 자동 저장 실시간 회의",
    },
```

- [ ] **Step 2: Add the 5th button in the type grid**

Open `type_picker_modal/component.rs`. After the Follow button, insert:

```rust
                    // Meet
                    button {
                        class: "type-option",
                        "data-testid": "type-option-meet",
                        "data-type": "meet",
                        onclick: move |_| {
                            on_pick.call(SpaceActionType::Meet);
                            on_close.call(());
                        },
                        div { class: "type-option__icon",
                            svg {
                                fill: "none",
                                stroke: "currentColor",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "2",
                                view_box: "0 0 24 24",
                                xmlns: "http://www.w3.org/2000/svg",
                                polygon { points: "23 7 16 12 23 17 23 7" }
                                rect {
                                    x: "1",
                                    y: "5",
                                    width: "15",
                                    height: "14",
                                    rx: "2",
                                    ry: "2",
                                }
                            }
                        }
                        div { class: "type-option__name", "{tr.meet_name}" }
                        div { class: "type-option__desc", "{tr.meet_desc}" }
                    }
```

- [ ] **Step 3: Add CSS palette for `[data-type="meet"]`**

Open `type_picker_modal/style.css`. Mirror the Poll/Quiz/Follow blocks; palette coral `#fb7185`:

```css
.type-option[data-type="meet"]:hover {
  border-color: rgba(251, 113, 133, 0.35);
  color: #fb7185;
}
.type-option[data-type="meet"]:hover .type-option__icon {
  background: rgba(251, 113, 133, 0.12);
  color: #fb7185;
}
```

- [ ] **Step 4: Verify compile + lint**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
rustywind --custom-regex 'class: "(.*)"' --write app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/component.rs
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/
git commit -m "feat(meet-action): add Meet option to TypePickerModal"
```

---

## Task 18: `MeetEditorView` skeleton + ArenaTopbar + ActionEditTopbar

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/editor_view.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/i18n.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/style.css` (empty initially)
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/mod.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/component.rs`

- [ ] **Step 1: Write `i18n.rs`**

```rust
use dioxus_translate::translate;

translate! {
    MeetActionTranslate;

    page_title: { en: "Create Meet", ko: "회의 생성" },
    mode_label: { en: "Start mode", ko: "시작 방식" },
    mode_scheduled: { en: "Scheduled", ko: "예약" },
    mode_instant: { en: "Instant · Start now", ko: "즉시 시작" },
    mode_scheduled_desc: { en: "Opens at a set time. Participants get reminders.", ko: "정해진 시간에 열립니다. 참가자는 리마인더를 받습니다." },
    mode_instant_desc: { en: "Goes Live immediately. One notification to participants.", ko: "즉시 Live로 전환됩니다. participant에게 한 번 알림 전송." },

    details_label: { en: "Details", ko: "세부 정보" },
    details_title_label: { en: "Title", ko: "제목" },
    details_title_placeholder: { en: "e.g. Mid-semester retrospective call", ko: "예: 학기 중간 회고 회의" },
    details_description_label: { en: "Description (optional)", ko: "설명 (선택)" },
    details_description_placeholder: { en: "Agenda, materials to prepare…", ko: "안건, 준비할 자료…" },

    when_label: { en: "When", ko: "일정" },
    when_start_label: { en: "Start time", ko: "시작 시간" },
    when_duration_label: { en: "Estimated duration", ko: "예상 지속 시간" },
    when_duration_unit_min: { en: "min", ko: "분" },

    submit_schedule: { en: "Schedule Meet", ko: "Meet 예약" },
    submit_start_now: { en: "Start now", ko: "지금 시작" },

    coming_soon_badge: { en: "Coming Soon", ko: "준비 중" },
    live_label: { en: "Live now", ko: "진행 중" },
    ended_label: { en: "Meeting ended", ko: "종료됨" },
}
```

- [ ] **Step 2: Write an empty `style.css` (to be filled in Task 24)**

```css
/* Meet editor styles — populated in Task 24 */
```

- [ ] **Step 3: Write `editor_view.rs` skeleton**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::components::*;

#[component]
pub fn MeetEditorView() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet { space_id, meet_id, meet, .. } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let _ = (space_id, meet_id);

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        SeoMeta { title: "{tr.page_title}" }

        div { class: "meet-editor",
            // ArenaTopbar and ActionEditTopbar are rendered by the layout;
            // this view owns only the content column.
            MeetModeToggle {}
            MeetDetailsCard {}
            MeetWhenCard {}
            MeetConfigCard {}
            MeetSubmitBar {}
        }
    }
}
```

Child components are added in subsequent tasks; until then, stub them to return `rsx! {}` in Task 19–23 before filling in UI.

- [ ] **Step 4: Register components + update page-level `component.rs` to call `MeetEditorView` for admins**

Update `meet_page/mod.rs`:

```rust
mod component;
pub use component::*;

mod editor_view;
pub use editor_view::*;

mod viewer_view;
pub use viewer_view::*;

mod mode_toggle;
pub use mode_toggle::*;

mod details_card;
pub use details_card::*;

mod when_card;
pub use when_card::*;

mod config_card;
pub use config_card::*;

mod submit_bar;
pub use submit_bar::*;

mod i18n;
pub use i18n::*;

pub mod hooks;
pub use hooks::*;
```

Update `component.rs`:

```rust
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::*;
use crate::features::spaces::space_common::hooks::use_space_role;

#[component]
pub fn MeetActionPage(
    space_id: ReadSignal<SpacePartition>,
    meet_id: ReadSignal<SpaceMeetEntityType>,
) -> Element {
    let role = use_space_role()();
    use_context_provider(|| space_id);
    use_context_provider(|| meet_id);

    rsx! {
        if role.is_admin() {
            MeetEditorView {}
        } else {
            MeetViewerView {}
        }
    }
}
```

Create empty stubs for each of `mode_toggle.rs`, `details_card.rs`, `when_card.rs`, `config_card.rs`, `submit_bar.rs`, `viewer_view.rs`:

```rust
use dioxus::prelude::*;

#[component]
pub fn MeetModeToggle() -> Element { rsx! {} }  // Task 19
// (same pattern for each stub with its own component name)
```

- [ ] **Step 5: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/
git commit -m "feat(meet-action): scaffold MeetActionPage with role-based branching"
```

---

## Task 19: `MeetModeToggle` — Scheduled / Instant segmented control

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/mode_toggle.rs`

- [ ] **Step 1: Implement**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;

#[component]
pub fn MeetModeToggle() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet { meet, mut update_mode, .. } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let current = meet().ok().map(|m| m.mode.clone()).unwrap_or_default();

    let pick = move |mode: MeetMode| update_mode.call(mode);

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.mode_label}" }
            }
            div { class: "mode-toggle", role: "tablist", "data-testid": "meet-mode-toggle",
                div {
                    class: "mode-option",
                    "role": "tab",
                    "aria-selected": current == MeetMode::Scheduled,
                    "data-testid": "meet-mode-scheduled",
                    onclick: {
                        let pick = pick.clone();
                        move |_| pick(MeetMode::Scheduled)
                    },
                    span { class: "mode-option__title", "{tr.mode_scheduled}" }
                    p { class: "mode-option__desc", "{tr.mode_scheduled_desc}" }
                }
                div {
                    class: "mode-option",
                    "role": "tab",
                    "aria-selected": current == MeetMode::Instant,
                    "data-testid": "meet-mode-instant",
                    onclick: move |_| pick(MeetMode::Instant),
                    span { class: "mode-option__title", "{tr.mode_instant}" }
                    p { class: "mode-option__desc", "{tr.mode_instant_desc}" }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/mode_toggle.rs
git commit -m "feat(meet-action): MeetModeToggle Scheduled/Instant segmented control"
```

---

## Task 20: `MeetDetailsCard` — title + description via `update_space_action`

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/details_card.rs`

- [ ] **Step 1: Implement**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::controllers::{
    UpdateSpaceActionRequest, update_space_action,
};

#[component]
pub fn MeetDetailsCard() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let mut toast = use_toast();
    let UseMeet { space_id, meet_id, meet, .. } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let current = meet().ok().map(|m| m.space_action.clone()).unwrap_or_default();
    let initial_title = current.title.clone();
    let initial_desc = current.description.clone();
    let mut title = use_signal(move || initial_title.clone());
    let mut desc = use_signal(move || initial_desc.clone());

    let save_title = move |_| {
        let value = title();
        spawn(async move {
            let action_id = meet_id().to_string();
            if let Err(e) = update_space_action(
                space_id(),
                action_id,
                UpdateSpaceActionRequest::Title { title: value },
            )
            .await
            {
                toast.error(e);
            }
        });
    };
    let save_desc = move |_| {
        let value = desc();
        spawn(async move {
            let action_id = meet_id().to_string();
            if let Err(e) = update_space_action(
                space_id(),
                action_id,
                UpdateSpaceActionRequest::Description { description: value },
            )
            .await
            {
                toast.error(e);
            }
        });
    };

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.details_label}" }
            }
            div { class: "field",
                label { class: "field__label", "{tr.details_title_label}" }
                input {
                    class: "field__input",
                    "data-testid": "meet-title-input",
                    placeholder: "{tr.details_title_placeholder}",
                    value: "{title}",
                    oninput: move |e| title.set(e.value()),
                    onfocusout: save_title,
                }
            }
            div { class: "field",
                label { class: "field__label", "{tr.details_description_label}" }
                textarea {
                    class: "field__textarea",
                    "data-testid": "meet-description-input",
                    placeholder: "{tr.details_description_placeholder}",
                    value: "{desc}",
                    oninput: move |e| desc.set(e.value()),
                    onfocusout: save_desc,
                }
            }
        }
    }
}
```

If `UpdateSpaceActionRequest` variants `Title` / `Description` have slightly different names (check the enum in `features/spaces/pages/actions/controllers/`), use the actual variant names.

- [ ] **Step 2: Verify compile + lint**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
rustywind --custom-regex 'class: "(.*)"' --write .../details_card.rs
dx fmt -f .../details_card.rs
```

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/details_card.rs
git commit -m "feat(meet-action): MeetDetailsCard title + description"
```

---

## Task 21: `MeetWhenCard` — start time + duration stepper

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/when_card.rs`

- [ ] **Step 1: Implement**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;

fn format_datetime_local(ts_ms: i64) -> String {
    use chrono::{TimeZone, Utc};
    Utc.timestamp_millis_opt(ts_ms)
        .single()
        .map(|dt| dt.format("%Y-%m-%dT%H:%M").to_string())
        .unwrap_or_default()
}

fn parse_datetime_local(s: &str) -> Option<i64> {
    use chrono::{NaiveDateTime, Utc, TimeZone};
    NaiveDateTime::parse_from_str(s, "%Y-%m-%dT%H:%M")
        .ok()
        .and_then(|ndt| Utc.from_local_datetime(&ndt).single())
        .map(|dt| dt.timestamp_millis())
}

#[component]
pub fn MeetWhenCard() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet {
        meet,
        mut update_start_time,
        mut update_duration,
        ..
    } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let current = meet().ok().unwrap_or_default();
    let mode = current.mode.clone();
    let start_value = format_datetime_local(current.start_time);
    let duration = current.duration_min;

    let start_disabled = mode == MeetMode::Instant;

    let on_start_change = move |e: FormEvent| {
        if let Some(ts) = parse_datetime_local(&e.value()) {
            update_start_time.call(ts);
        }
    };
    let dec = move |_| {
        let next = (duration - 15).max(15);
        update_duration.call(next);
    };
    let inc = move |_| {
        let next = (duration + 15).min(1440);
        update_duration.call(next);
    };

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "{tr.when_label}" }
            }
            div { class: "when-row",
                div { class: "field",
                    label { class: "field__label", "{tr.when_start_label}" }
                    input {
                        class: "field__input",
                        r#type: "datetime-local",
                        "data-testid": "meet-start-time",
                        disabled: start_disabled,
                        value: "{start_value}",
                        onchange: on_start_change,
                    }
                }
                div { class: "field",
                    label { class: "field__label", "{tr.when_duration_label}" }
                    div { class: "dur",
                        button {
                            class: "dur__step",
                            "data-testid": "meet-duration-dec",
                            onclick: dec,
                            "−"
                        }
                        span { class: "dur__value", "data-testid": "meet-duration-value",
                            "{duration}"
                            small { " {tr.when_duration_unit_min}" }
                        }
                        button {
                            class: "dur__step",
                            "data-testid": "meet-duration-inc",
                            onclick: inc,
                            "+"
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Verify compile + lint**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
rustywind --custom-regex 'class: "(.*)"' --write .../when_card.rs
dx fmt -f .../when_card.rs
```

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/when_card.rs
git commit -m "feat(meet-action): MeetWhenCard start time + duration stepper"
```

---

## Task 22: `MeetConfigCard` — reuse common settings

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/config_card.rs`

- [ ] **Step 1: Implement**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::components::{
    ActionDeleteButton, ActionDependencySelector, ActionRewardSetting, ActionStatusControl,
    PrerequisiteTile,
};

#[component]
pub fn MeetConfigCard() -> Element {
    let UseMeet { space_id, meet_id, meet, .. } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let current = meet().ok().map(|m| m.space_action.clone()).unwrap_or_default();
    let saved_credits = current.credits;
    let action_status = current.status.clone();
    let initial_prereq = current.prerequisite;
    let initial_depends = current.depends_on.clone();

    let action_id_str = meet_id().to_string();
    let action_id_signal: ReadSignal<String> = use_signal(move || action_id_str.clone()).into();

    rsx! {
        section { class: "meet-card",
            header { class: "meet-card__head",
                h2 { class: "meet-card__title meet-card__title--meet", "Settings" }
            }

            ActionDependencySelector {
                space_id,
                action_id: action_id_signal,
                initial_depends_on: initial_depends,
            }

            ActionRewardSetting {
                space_id,
                action_id: action_id_signal,
                saved_credits,
                action_status: action_status.clone(),
            }

            PrerequisiteTile {
                space_id,
                action_id: action_id_signal,
                initial_prerequisite: initial_prereq,
                on_changed: move |_| {},
            }

            ActionStatusControl {
                space_id,
                action_id: action_id_signal,
                initial_status: action_status.clone(),
                on_changed: move |_| {},
            }

            ActionDeleteButton {
                space_id: space_id(),
                action_id: meet_id().to_string(),
            }
        }
    }
}
```

- [ ] **Step 2: Verify compile + lint**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/config_card.rs
git commit -m "feat(meet-action): MeetConfigCard reusing common action settings"
```

---

## Task 23: `MeetSubmitBar` + `MeetViewerView`

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/submit_bar.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/viewer_view.rs`

- [ ] **Step 1: Implement `MeetSubmitBar`**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::actions::meet::*;
use crate::features::spaces::pages::actions::SpaceActionStatus;

#[component]
pub fn MeetSubmitBar() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet { meet, mut publish, .. } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let current = meet().ok();
    let mode = current.as_ref().map(|m| m.mode.clone()).unwrap_or_default();
    let status = current
        .as_ref()
        .and_then(|m| m.space_action.status.clone())
        .unwrap_or(SpaceActionStatus::Designing);
    let is_published = !matches!(status, SpaceActionStatus::Designing);
    let label = if mode == MeetMode::Instant {
        tr.submit_start_now.to_string()
    } else {
        tr.submit_schedule.to_string()
    };

    rsx! {
        div { class: "create-bar",
            Button {
                "data-testid": "meet-submit-button",
                disabled: is_published,
                onclick: move |_| publish.call(),
                "{label}"
            }
        }
    }
}
```

- [ ] **Step 2: Implement `MeetViewerView` with Coming Soon placeholders**

```rust
use crate::common::*;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::*;
use crate::features::spaces::pages::actions::SpaceActionStatus;

#[component]
pub fn MeetViewerView() -> Element {
    let tr: MeetActionTranslate = use_translate();
    let UseMeet { meet, .. } = use_meet(
        use_context::<ReadSignal<SpacePartition>>(),
        use_context::<ReadSignal<SpaceMeetEntityType>>(),
    )?;
    let current = meet().ok();
    let title = current.as_ref().map(|m| m.space_action.title.clone()).unwrap_or_default();
    let description = current.as_ref().map(|m| m.space_action.description.clone()).unwrap_or_default();
    let status = current
        .as_ref()
        .and_then(|m| m.space_action.status.clone())
        .unwrap_or(SpaceActionStatus::Designing);
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let start_time = current.as_ref().map(|m| m.start_time).unwrap_or(0);
    let duration = current.as_ref().map(|m| m.duration_min as i64).unwrap_or(0);
    let is_live = matches!(status, SpaceActionStatus::Ongoing)
        && start_time <= now
        && now < start_time + duration * 60_000;
    let is_scheduled = matches!(status, SpaceActionStatus::Ongoing) && now < start_time;
    let is_ended = matches!(status, SpaceActionStatus::Finish);

    rsx! {
        SeoMeta { title: "{title}" }
        div { class: "meet-viewer", "data-testid": "meet-viewer-view",
            h1 { "{title}" }
            p { "{description}" }

            if is_scheduled {
                div { class: "meet-viewer__scheduled",
                    "Starts at: {start_time} ms" // simple placeholder, countdown UI comes with Live phase
                }
            } else if is_live {
                div { class: "meet-viewer__live",
                    span { "{tr.live_label}" }
                    Button {
                        disabled: true,
                        "data-testid": "meet-live-join",
                        "{tr.coming_soon_badge}"
                    }
                }
            } else if is_ended {
                div { class: "meet-viewer__ended",
                    span { "{tr.ended_label}" }
                    Button {
                        disabled: true,
                        "data-testid": "meet-ended-archive",
                        "{tr.coming_soon_badge}"
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 3: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/submit_bar.rs app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/viewer_view.rs
git commit -m "feat(meet-action): MeetSubmitBar + MeetViewerView with Coming Soon placeholders"
```

---

## Task 24: Port `create-meet.html` styles → `meet_page/style.css`

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/style.css`
- Optional: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/page.html` (reference copy)

- [ ] **Step 1: Copy `app/ratel/assets/design/meet-action/create-meet.html` into `meet_page/page.html`** for reference.

- [ ] **Step 2: Extract CSS from the mockup's `<style>` blocks + shared.css**

Paste into `style.css`, then:
- Replace all hard-coded colors with the space toggle pattern `var(--dark, ...) var(--light, ...)` where appropriate (see `conventions/styling.md`).
- Preserve mockup class names exactly: `mode-toggle`, `mode-option`, `mode-option__icon`, `mode-option__title`, `mode-option__desc`, `dur`, `dur__step`, `dur__value`, `when-row`, `field`, `field__input`, `field__textarea`, `field__label`, `meet-card`, `meet-card__head`, `meet-card__title`, `meet-card__title--meet`, `create-bar`.

- [ ] **Step 3: Verify via dev server**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```

Open http://localhost:8000/, log in as admin, create a Meet, confirm the editor visually matches `create-meet.html`.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_page/
git commit -m "feat(meet-action): port editor styles from mockup"
```

---

## Task 25: `MeetActionCard` for carousel

**Files:**
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_card/mod.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_card/component.rs`
- Create: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/meet_card/style.css`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/meet/components/mod.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs`

- [ ] **Step 1: Implement the card**

`meet_card/mod.rs`:
```rust
mod component;
pub use component::*;
```

`meet_card/component.rs`:
```rust
use crate::common::*;
use crate::features::spaces::pages::actions::types::SpaceActionSummary;
use crate::features::spaces::pages::actions::actions::meet::components::meet_page::MeetActionTranslate;
use crate::features::spaces::pages::actions::SpaceActionStatus;

#[derive(Clone, Copy, PartialEq)]
enum MeetPhase { Draft, Scheduled, Live, Ended }

fn derive_phase(action: &SpaceActionSummary) -> MeetPhase {
    let status = action.status.clone().unwrap_or(SpaceActionStatus::Designing);
    let now = crate::common::utils::time::get_now_timestamp_millis();
    let start = action.started_at.unwrap_or(0);
    let end = action.ended_at.unwrap_or(start);
    match status {
        SpaceActionStatus::Designing => MeetPhase::Draft,
        SpaceActionStatus::Ongoing if now < start => MeetPhase::Scheduled,
        SpaceActionStatus::Ongoing if now < end => MeetPhase::Live,
        SpaceActionStatus::Ongoing => MeetPhase::Ended,
        SpaceActionStatus::Finish => MeetPhase::Ended,
    }
}

#[component]
pub fn MeetActionCard(
    action: SpaceActionSummary,
    space_id: ReadSignal<SpacePartition>,
    #[props(default)] is_admin: bool,
) -> Element {
    let tr: MeetActionTranslate = use_translate();
    let nav = use_navigator();
    let phase = derive_phase(&action);
    let meet_id: SpaceMeetEntityType = action.action_id.clone().into();
    let title_display = if action.title.is_empty() { "새 회의".to_string() } else { action.title.clone() };
    let _ = is_admin;

    let go = move |_| {
        nav.push(crate::route::Route::MeetActionPage {
            space_id: space_id(),
            meet_id: meet_id.clone(),
        });
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "quest-card",
            "data-testid": "action-card-meet",
            "data-kind": "meet",
            "data-phase": match phase {
                MeetPhase::Draft => "draft",
                MeetPhase::Scheduled => "scheduled",
                MeetPhase::Live => "live",
                MeetPhase::Ended => "ended",
            },
            onclick: go,

            header { class: "quest-card__head",
                span { class: "quest-card__kind", "MEET" }
                match phase {
                    MeetPhase::Draft => rsx! { span { class: "badge badge--muted", "설정 중" } },
                    MeetPhase::Scheduled => rsx! { span { class: "badge badge--meet", "Scheduled" } },
                    MeetPhase::Live => rsx! { span { class: "badge badge--live", "{tr.live_label}" } },
                    MeetPhase::Ended => rsx! { span { class: "badge badge--ended", "{tr.ended_label}" } },
                }
            }

            h3 { class: "quest-card__title", "{title_display}" }

            footer { class: "quest-card__foot",
                match phase {
                    MeetPhase::Draft => rsx! { span { class: "cta", "재진입" } },
                    MeetPhase::Scheduled => rsx! { span { class: "cta", "자세히 보기" } },
                    MeetPhase::Live => rsx! {
                        Button { disabled: true, "data-testid": "meet-card-join",
                            span { "입장 · " }
                            span { class: "badge badge--coming-soon", "{tr.coming_soon_badge}" }
                        }
                    },
                    MeetPhase::Ended => rsx! {
                        Button { disabled: true, "data-testid": "meet-card-archive",
                            span { "아카이브 · " }
                            span { class: "badge badge--coming-soon", "{tr.coming_soon_badge}" }
                        }
                    },
                }
            }
        }
    }
}
```

`meet_card/style.css`: empty initially (styles piggyback on quest-card system). Fill in during Task 26 if card needs Meet-specific tweaks.

- [ ] **Step 2: Export from components/mod.rs**

```rust
pub mod meet_card;
pub use meet_card::*;

pub mod meet_page;
pub use meet_page::*;
```

- [ ] **Step 3: Render `MeetActionCard` from the action dashboard carousel**

Open `pages/index/action_dashboard/component.rs`. Find the `match action.action_type { ... }` block inside the carousel loop. Add:

```rust
                                SpaceActionType::Meet => rsx! {
                                    MeetActionCard {
                                        key: "{key}",
                                        action,
                                        space_id,
                                        is_admin,
                                    }
                                },
```

Add import at the top:

```rust
use crate::features::spaces::pages::actions::actions::meet::MeetActionCard;
```

- [ ] **Step 4: Verify compile**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/meet/components/ app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs
git commit -m "feat(meet-action): MeetActionCard + carousel integration"
```

---

## Task 26: Final Rust-side lint / format pass

**Files:** all files created or modified above.

- [ ] **Step 1: Apply `rustywind` + `dx fmt` to every modified `.rs` file**

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel
find app/ratel/src/features/spaces/pages/actions/actions/meet -name "*.rs" \
    -exec rustywind --custom-regex 'class: "(.*)"' --write {} \; \
    -exec dx fmt -f {} \;
rustywind --custom-regex 'class: "(.*)"' --write app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/component.rs app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/action_dashboard/type_picker_modal/component.rs app/ratel/src/features/spaces/pages/index/action_dashboard/component.rs
```

- [ ] **Step 2: Re-run all checks**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests mcp_tests::test_mcp_tool_create_meet
```
Expected: all pass.

- [ ] **Step 3: Commit**

```bash
git add -u
git commit -m "chore(meet-action): apply lint + format pass"
```

---

## Task 27: Playwright e2e

**Files:**
- Modify (or create if missing): `playwright/tests/web/spaces-actions.spec.js` (the existing space-actions scenario spec; if a different file covers the carousel, extend that one instead)

- [ ] **Step 1: Extend the serial suite with Meet flow**

Add inside the existing `test.describe.serial(...)`:

```js
test("Admin adds a Meet via TypePicker", async ({ page }) => {
  await goto(page, "/spaces/" + sharedSpaceId + "/");
  await click(page, { testId: "add-action-card" });
  await expect(page.getByTestId("type-option-meet")).toBeVisible();
  await click(page, { testId: "type-option-meet" });
  await page.waitForURL(/\/meets\/[a-z0-9-]+/, { waitUntil: "load" });
  await expect(page.getByTestId("meet-editor-view")).toBeVisible();
});

test("Admin toggles mode Scheduled ↔ Instant", async ({ page }) => {
  await click(page, { testId: "meet-mode-instant" });
  await expect(page.getByTestId("meet-mode-instant")).toHaveAttribute("aria-selected", "true");
  await click(page, { testId: "meet-mode-scheduled" });
  await expect(page.getByTestId("meet-mode-scheduled")).toHaveAttribute("aria-selected", "true");
});

test("Admin sets title and duration", async ({ page }) => {
  await fill(page, { testId: "meet-title-input" }, "E2E Meet");
  await page.keyboard.press("Tab");
  await click(page, { testId: "meet-duration-inc" });
  await expect(page.getByTestId("meet-duration-value")).toContainText("75");
});

test("Admin publishes the Meet (Scheduled)", async ({ page }) => {
  await click(page, { testId: "meet-submit-button" });
  await page.waitForURL(/\/$|\/actions/, { waitUntil: "load" });
  await expect(page.getByTestId("action-card-meet")).toBeVisible();
});
```

`sharedSpaceId` is assumed to exist in the serial suite's shared state; wire it from whatever earlier test creates the space. If the suite has no such helper, copy the setup preamble from `spaces-poll.spec.js` or equivalent.

- [ ] **Step 2: Run locally**

```bash
make infra
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local dx serve --port 8000 --web &
cd playwright && npx playwright test tests/web/spaces-actions.spec.js --headed
```
Expected: PASS.

- [ ] **Step 3: Add `data-testid="add-action-card"`** on the admin's AddActionCard component if missing. Search `add_action_card/component.rs`; if no testid, add `"data-testid": "add-action-card"`.

- [ ] **Step 4: Commit**

```bash
git add playwright/ app/ratel/src/features/spaces/pages/index/action_dashboard/add_action_card/
git commit -m "test(meet-action): Playwright e2e for Meet add + configure flow"
```

---

## Task 28: Full verification

- [ ] **Step 1: Run the full build matrix**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass"
```

- [ ] **Step 2: Run Playwright against the prod-built docker image**

```bash
cd /home/hackartist/data/devel/github.com/biyard/ratel
make infra
# Build + bring up the testing image per conventions/build-commands.md
COMMIT=local-meet make testing
cd playwright && CI=true make test
```

All tests must pass. Only then is Phase 1 ready for PR.

- [ ] **Step 3: Open a pull request**

```bash
git push hackartists <branch>
gh pr create --title "feat(meet-action): Phase 1 — admin add + configure" \
  --base dev \
  --body "$(cat <<'EOF'
## Summary
- Adds Meet as the 5th space action (alongside Poll / Quiz / Discussion / Follow).
- Admin can add a Meet from the existing type picker and configure mode / title / description / start time / duration / reward on a dedicated editor page.
- Live / Ended phases render Coming Soon placeholders; recording, transcription, calendar, notifications, and Essence ingestion are explicit follow-up work.

## Test plan
- [x] `cargo test --features full,bypass -- meet_action_tests`
- [x] `cargo test --features full,bypass -- mcp_tests::test_mcp_tool_create_meet`
- [x] `dx check --web`
- [x] `cargo check --features server` with `RUSTFLAGS='-D warnings'`
- [x] Playwright `spaces-actions.spec.js` (Meet flow) — green against the testing Docker image with `CI=true`

🤖 Generated with [Claude Code](https://claude.com/claude-code)
EOF
)"
```

---

## Spec coverage check

| Spec section | Covered by |
|--------------|-----------|
| Scope (admin add + configure) | Tasks 3–25 end-to-end |
| `SpaceMeet` data model | Task 4 |
| `EntityType::SpaceMeet` | Task 1 |
| `SpaceActionType::Meet` | Tasks 9 (stub) + 14 (full `create()`) |
| `RewardUserBehavior::AttendMeet` | Task 2 |
| `DashboardAggregate::inc_meets` | Task 7 |
| `MeetActionError` | Task 5 |
| `MeetResponse` | Task 6 |
| `create_meet` / `get_meet` / `update_meet` / `delete_meet` controllers | Tasks 9, 10, 11, 12 |
| MCP tools for each controller | Task 13 |
| `Route::MeetActionPage` + role branch | Task 14 |
| `Space.include_meetings_in_essence` field | Task 15 |
| `UseMeet` controller hook | Task 16 |
| TypePickerModal integration | Task 17 |
| `MeetEditorView` + subcomponents | Tasks 18–23 |
| `MeetActionCard` in carousel | Task 25 |
| Coming Soon placeholders (Live/Ended) | Tasks 23 (viewer) + 25 (card) |
| Server integration tests | Tasks 8, 10, 11, 12 |
| MCP test | Task 13 |
| Playwright e2e | Task 27 |
| Build verification | Tasks 26 + 28 |

## Non-goals (explicit deferrals — NOT in this plan)

- Live meeting UI (video grid, controls, chat, reactions, raise-hand).
- Recording, transcription, speaker diarization.
- Ended / Archive page (recording player, transcript, chat log, moderation log).
- Cancelled / Expired terminal states + cancellation reason.
- Google Calendar OAuth + ICS.
- AWS Chime SDK integration.
- In-app + email notification delivery for the four Meet events.
- Essence ingestion of Meet transcripts (field is persisted; ingestion is not).
- Host / attendee reward split with ≥1-min presence gating.
