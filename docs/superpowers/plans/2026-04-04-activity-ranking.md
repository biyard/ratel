# Activity Ranking Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a stream-aggregated activity scoring and ranking system for spaces, with creator-configurable scores and real-time ranking UI in dashboard and sidebar.

**Architecture:** Action controllers write `SpaceActivity` records to DynamoDB. The DynamoDB Stream handler picks up INSERTs and atomically increments `SpaceScore` counters. Rankings are read via GSI sorted by score. `AuthorPartition` type provides type-safe user/team identity for composite keys.

**Tech Stack:** Rust, Dioxus 0.7, DynamoDB (single-table), DynamoDB Streams, TailwindCSS v4

**Spec:** `docs/superpowers/specs/2026-04-04-activity-ranking-design.md`

---

## File Structure

### New files (features/activity/)
| File | Responsibility |
|------|---------------|
| `app/ratel/src/features/activity/mod.rs` | Feature module root, re-exports |
| `app/ratel/src/features/activity/models/mod.rs` | Model re-exports |
| `app/ratel/src/features/activity/models/space_activity.rs` | SpaceActivity DynamoEntity |
| `app/ratel/src/features/activity/models/space_score.rs` | SpaceScore DynamoEntity |
| `app/ratel/src/features/activity/types/mod.rs` | Type re-exports |
| `app/ratel/src/features/activity/types/author_partition.rs` | AuthorPartition enum type |
| `app/ratel/src/features/activity/types/space_activity_data.rs` | SpaceActivityData enum |
| `app/ratel/src/features/activity/types/error.rs` | ActivityError enum |
| `app/ratel/src/features/activity/controllers/mod.rs` | Controller re-exports |
| `app/ratel/src/features/activity/controllers/get_ranking.rs` | GET ranking endpoint |
| `app/ratel/src/features/activity/controllers/get_my_score.rs` | GET my-score endpoint |
| `app/ratel/src/features/activity/controllers/record_activity.rs` | Internal activity recording helper |
| `app/ratel/src/features/activity/services/mod.rs` | Service re-exports |
| `app/ratel/src/features/activity/services/aggregate_score.rs` | Stream handler score aggregation |
| `app/ratel/src/features/activity/components/mod.rs` | Component re-exports |
| `app/ratel/src/features/activity/components/activity_score_setting.rs` | Creator score config UI |
| `app/ratel/src/features/activity/components/ranking_widget.rs` | Sidebar ranking widget |
| `app/ratel/src/features/activity/i18n.rs` | Translations |

### Modified files
| File | Change |
|------|--------|
| `app/ratel/src/common/types/mod.rs` | Add `author_partition` module |
| `app/ratel/src/common/types/entity_type.rs` | Add `SpaceActivity(String)`, `SpaceScore` variants |
| `app/ratel/src/common/types/error.rs` | Add `Activity(#[from] ActivityError)` variant |
| `app/ratel/src/features/mod.rs` | Add `pub mod activity;` |
| `app/ratel/src/features/spaces/pages/actions/models/space_action.rs` | Add `activity_score`, `additional_score` fields |
| `app/ratel/src/features/spaces/pages/actions/controllers/update_space_action.rs` | Add `ActivityScore` variant |
| `app/ratel/src/features/spaces/pages/actions/components/action_common_settings/mod.rs` | Add `ActivityScoreSetting` |
| `app/ratel/src/common/stream_handler.rs` | Add SpaceActivity INSERT handler |
| `app/ratel/src/features/spaces/pages/actions/actions/poll/controllers/respond_poll.rs` | Call record_activity |
| `app/ratel/src/features/spaces/pages/actions/actions/quiz/controllers/respond_quiz.rs` | Call record_activity |
| `app/ratel/src/features/spaces/pages/actions/actions/follow/controllers/follow_user.rs` | Call record_activity |
| `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/reply_comment.rs` | Call record_activity |
| `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/add_comment.rs` | Call record_activity |
| `app/ratel/src/features/spaces/pages/dashboard/controllers/list_ranking.rs` | Implement ranking handler |
| `app/ratel/src/features/spaces/pages/dashboard/controllers/list_dashboard_data.rs` | Wire RankingTable data |
| `app/ratel/src/features/spaces/space_common/components/space_nav/mod.rs` | Add RankingWidget |
| `app/ratel/Cargo.toml` | Add `activity` feature flag |

---

## Task 1: AuthorPartition Type

**Files:**
- Create: `app/ratel/src/features/activity/types/author_partition.rs`
- Modify: `app/ratel/src/common/types/mod.rs`

- [ ] **Step 1: Create AuthorPartition type**

Create `app/ratel/src/features/activity/types/author_partition.rs`:

```rust
use std::fmt::Display;
use std::str::FromStr;

use crate::common::*;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum AuthorPartition {
    #[default]
    Unknown,
    User(String),
    Team(String),
}

impl Display for AuthorPartition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthorPartition::Unknown => write!(f, "UNKNOWN"),
            AuthorPartition::User(id) => write!(f, "USER#{id}"),
            AuthorPartition::Team(id) => write!(f, "TEAM#{id}"),
        }
    }
}

impl FromStr for AuthorPartition {
    type Err = crate::common::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if let Some(id) = s.strip_prefix("USER#") {
            Ok(AuthorPartition::User(id.to_string()))
        } else if let Some(id) = s.strip_prefix("TEAM#") {
            Ok(AuthorPartition::Team(id.to_string()))
        } else if s == "UNKNOWN" {
            Ok(AuthorPartition::Unknown)
        } else {
            Err(crate::common::Error::InvalidPartitionKey(format!(
                "invalid author partition: {s}"
            )))
        }
    }
}

impl From<UserPartition> for AuthorPartition {
    fn from(u: UserPartition) -> Self {
        AuthorPartition::User(u.0)
    }
}

impl From<TeamPartition> for AuthorPartition {
    fn from(t: TeamPartition) -> Self {
        AuthorPartition::Team(t.0)
    }
}

impl From<Partition> for AuthorPartition {
    fn from(p: Partition) -> Self {
        match p {
            Partition::User(id) => AuthorPartition::User(id),
            Partition::Team(id) => AuthorPartition::Team(id),
            _ => AuthorPartition::Unknown,
        }
    }
}
```

- [ ] **Step 2: Run dx check to verify compilation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS (AuthorPartition is not yet imported anywhere, but the file should compile)

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/activity/types/author_partition.rs
git commit -m "feat(activity): add AuthorPartition type for user/team identity"
```

---

## Task 2: EntityType Additions + Feature Flag

**Files:**
- Modify: `app/ratel/src/common/types/entity_type.rs:206`
- Modify: `app/ratel/Cargo.toml:114,166`

- [ ] **Step 1: Add EntityType variants**

In `app/ratel/src/common/types/entity_type.rs`, add before the closing `}` of the enum (after `McpClientSecret`):

```rust
    // Activity
    SpaceActivity(String), // SPACE_ACTIVITY#action_id#timestamp
    SpaceScore,
```

- [ ] **Step 2: Add activity feature flag to Cargo.toml**

In `app/ratel/Cargo.toml`:

Change line 114:
```toml
full = ["membership", "social", "spaces_full", "activity"]
```

Add after line 166 (`spaces_full = ["spaces"]`):
```toml
activity = []
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/entity_type.rs app/ratel/Cargo.toml
git commit -m "feat(activity): add SpaceActivity/SpaceScore entity types and feature flag"
```

---

## Task 3: ActivityError Type

**Files:**
- Create: `app/ratel/src/features/activity/types/error.rs`
- Modify: `app/ratel/src/common/types/error.rs`

- [ ] **Step 1: Create ActivityError**

Create `app/ratel/src/features/activity/types/error.rs`:

```rust
use crate::common::*;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ActivityError {
    #[error("activity already recorded for this action")]
    #[translate(en = "You have already completed this action", ko = "이미 완료한 활동입니다")]
    AlreadyRecorded,

    #[error("score aggregation failed")]
    #[translate(en = "Score update failed, please try again", ko = "점수 업데이트에 실패했습니다")]
    AggregationFailed,

    #[error("invalid activity data")]
    #[translate(en = "Invalid activity data", ko = "잘못된 활동 데이터입니다")]
    InvalidData,
}

#[cfg(feature = "server")]
impl dioxus::prelude::IntoResponse for ActivityError {
    fn into_response(self) -> axum::response::Response {
        crate::common::Error::from(self).into_response()
    }
}

impl AsStatusCode for ActivityError {
    fn as_status_code(&self) -> u16 {
        match self {
            ActivityError::AlreadyRecorded => 409,
            ActivityError::AggregationFailed => 500,
            ActivityError::InvalidData => 400,
        }
    }
}
```

- [ ] **Step 2: Register in common::Error**

In `app/ratel/src/common/types/error.rs`, add a new variant in the `Error` enum (after the last `#[translate(from)]` block):

```rust
    #[error("{0}")]
    #[translate(from)]
    Activity(#[from] crate::features::activity::types::ActivityError),
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/types/error.rs app/ratel/src/common/types/error.rs
git commit -m "feat(activity): add ActivityError type with translations"
```

---

## Task 4: SpaceActivityData Type

**Files:**
- Create: `app/ratel/src/features/activity/types/space_activity_data.rs`
- Create: `app/ratel/src/features/activity/types/mod.rs`

- [ ] **Step 1: Create SpaceActivityData enum**

Create `app/ratel/src/features/activity/types/space_activity_data.rs`:

```rust
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpaceActivityData {
    #[default]
    Unknown,
    Poll {
        poll_id: String,
        answered_optional_count: u32,
    },
    Quiz {
        quiz_id: String,
        passed: bool,
        correct_count: u32,
        pass_threshold: u32,
    },
    Follow {
        follow_id: String,
    },
    Discussion {
        discussion_id: String,
        is_first_contribution: bool,
    },
}
```

- [ ] **Step 2: Create types/mod.rs**

Create `app/ratel/src/features/activity/types/mod.rs`:

```rust
mod author_partition;
mod error;
mod space_activity_data;

pub use author_partition::*;
pub use error::*;
pub use space_activity_data::*;
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/types/
git commit -m "feat(activity): add SpaceActivityData enum and types module"
```

---

## Task 5: SpaceActivity Model

**Files:**
- Create: `app/ratel/src/features/activity/models/space_activity.rs`
- Create: `app/ratel/src/features/activity/models/mod.rs`

- [ ] **Step 1: Create SpaceActivity DynamoEntity**

Create `app/ratel/src/features/activity/models/space_activity.rs`:

```rust
use crate::common::macros::DynamoEntity;
use crate::features::activity::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceActivity {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,
    pub sk: EntityType,

    #[dynamo(prefix = "SACT", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    pub user_pk: AuthorPartition,
    pub user_name: String,
    pub user_avatar: String,
    pub action_id: String,
    pub action_type: SpaceActionType,
    pub data: SpaceActivityData,

    pub base_score: i64,
    pub additional_score: i64,
    pub total_score: i64,
}

#[cfg(feature = "server")]
impl SpaceActivity {
    pub fn new(
        space_id: SpacePartition,
        author: AuthorPartition,
        action_id: String,
        action_type: SpaceActionType,
        data: SpaceActivityData,
        base_score: i64,
        additional_score: i64,
        user_name: String,
        user_avatar: String,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let space_pk: Partition = space_id.clone().into();
        let total_score = base_score + additional_score;
        let sk = EntityType::SpaceActivity(format!("{}#{}", action_id, now));

        Self {
            pk: CompositePartition(space_id, author.clone()),
            sk,
            space_pk,
            created_at: now,
            user_pk: author,
            user_name,
            user_avatar,
            action_id,
            action_type,
            data,
            base_score,
            additional_score,
            total_score,
        }
    }
}
```

- [ ] **Step 2: Create models/mod.rs**

Create `app/ratel/src/features/activity/models/mod.rs`:

```rust
mod space_activity;
mod space_score;

pub use space_activity::*;
pub use space_score::*;
```

Note: `space_score.rs` will be created in the next task. For now, comment out the `space_score` lines to compile.

Temporary `models/mod.rs`:

```rust
mod space_activity;
// mod space_score;

pub use space_activity::*;
// pub use space_score::*;
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/models/
git commit -m "feat(activity): add SpaceActivity DynamoEntity model"
```

---

## Task 6: SpaceScore Model

**Files:**
- Create: `app/ratel/src/features/activity/models/space_score.rs`
- Modify: `app/ratel/src/features/activity/models/mod.rs`

- [ ] **Step 1: Create SpaceScore DynamoEntity**

Create `app/ratel/src/features/activity/models/space_score.rs`:

```rust
use crate::common::macros::DynamoEntity;
use crate::features::activity::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceScore {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,
    pub sk: EntityType,

    #[dynamo(prefix = "SCSP", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "SCR", index = "gsi1", sk)]
    pub total_score: i64,

    pub user_pk: AuthorPartition,
    pub user_name: String,
    pub user_avatar: String,

    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,

    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl SpaceScore {
    pub fn keys(
        space_id: &SpacePartition,
        author: &AuthorPartition,
    ) -> (CompositePartition<SpacePartition, AuthorPartition>, EntityType) {
        (
            CompositePartition(space_id.clone(), author.clone()),
            EntityType::SpaceScore,
        )
    }
}
```

- [ ] **Step 2: Uncomment space_score in models/mod.rs**

Update `app/ratel/src/features/activity/models/mod.rs`:

```rust
mod space_activity;
mod space_score;

pub use space_activity::*;
pub use space_score::*;
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/models/
git commit -m "feat(activity): add SpaceScore DynamoEntity model"
```

---

## Task 7: Feature Module Scaffold + Registration

**Files:**
- Create: `app/ratel/src/features/activity/mod.rs`
- Create: `app/ratel/src/features/activity/controllers/mod.rs`
- Create: `app/ratel/src/features/activity/services/mod.rs`
- Create: `app/ratel/src/features/activity/components/mod.rs`
- Create: `app/ratel/src/features/activity/i18n.rs`
- Modify: `app/ratel/src/features/mod.rs`

- [ ] **Step 1: Create feature module root**

Create `app/ratel/src/features/activity/mod.rs`:

```rust
pub mod controllers;
pub mod models;
pub mod services;
pub mod types;

#[cfg(not(feature = "server"))]
pub mod components;

pub mod i18n;

pub use crate::common::*;
pub use types::*;
```

- [ ] **Step 2: Create controller scaffold**

Create `app/ratel/src/features/activity/controllers/mod.rs`:

```rust
mod get_ranking;
mod get_my_score;
mod record_activity;

pub use get_ranking::*;
pub use get_my_score::*;
pub(crate) use record_activity::*;
```

Create placeholder `app/ratel/src/features/activity/controllers/get_ranking.rs`:

```rust
#![allow(dead_code, unused_imports)]
use crate::features::activity::*;
```

Create placeholder `app/ratel/src/features/activity/controllers/get_my_score.rs`:

```rust
#![allow(dead_code, unused_imports)]
use crate::features::activity::*;
```

Create placeholder `app/ratel/src/features/activity/controllers/record_activity.rs`:

```rust
#![allow(dead_code, unused_imports)]
use crate::features::activity::*;
```

- [ ] **Step 3: Create services scaffold**

Create `app/ratel/src/features/activity/services/mod.rs`:

```rust
mod aggregate_score;

pub use aggregate_score::*;
```

Create placeholder `app/ratel/src/features/activity/services/aggregate_score.rs`:

```rust
#![allow(dead_code, unused_imports)]
use crate::features::activity::*;
```

- [ ] **Step 4: Create components scaffold**

Create `app/ratel/src/features/activity/components/mod.rs`:

```rust
mod activity_score_setting;
mod ranking_widget;

pub use activity_score_setting::*;
pub use ranking_widget::*;
```

Create placeholder `app/ratel/src/features/activity/components/activity_score_setting.rs`:

```rust
use crate::features::activity::*;

#[component]
pub fn ActivityScoreSetting() -> Element {
    rsx! { div {} }
}
```

Create placeholder `app/ratel/src/features/activity/components/ranking_widget.rs`:

```rust
use crate::features::activity::*;

#[component]
pub fn RankingWidget() -> Element {
    rsx! { div {} }
}
```

- [ ] **Step 5: Create i18n**

Create `app/ratel/src/features/activity/i18n.rs`:

```rust
use crate::common::*;

translate! {
    ActivityTranslate;

    activity_score: { en: "Activity Score", ko: "활동 점수" },
    additional_score: { en: "Additional Score", ko: "추가 점수" },
    additional_score_desc: { en: "Score per additional item", ko: "추가 항목당 점수" },
    ranking: { en: "Ranking", ko: "랭킹" },
    my_rank: { en: "My Rank", ko: "내 순위" },
    score_label: { en: "Score", ko: "점수" },
    rank_label: { en: "Rank", ko: "순위" },
    participant: { en: "Participant", ko: "참여자" },
    no_ranking_data: { en: "No ranking data yet", ko: "아직 랭킹 데이터가 없습니다" },
    activity_score_updated: { en: "Activity score updated.", ko: "활동 점수가 업데이트되었습니다." },
    you: { en: "You", ko: "나" },
}
```

- [ ] **Step 6: Register in features/mod.rs**

In `app/ratel/src/features/mod.rs`, add after `pub mod ai_moderator;`:

```rust
#[cfg(feature = "activity")]
pub mod activity;
```

- [ ] **Step 7: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 8: Commit**

```bash
git add app/ratel/src/features/activity/ app/ratel/src/features/mod.rs
git commit -m "feat(activity): scaffold activity feature module with controllers, services, components"
```

---

## Task 8: SpaceAction Model Changes (activity_score, additional_score)

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/models/space_action.rs`

- [ ] **Step 1: Add fields to SpaceAction**

In `app/ratel/src/features/spaces/pages/actions/models/space_action.rs`, add after `pub total_points: u64,`:

```rust
    #[serde(default)]
    pub activity_score: i64,
    #[serde(default)]
    pub additional_score: i64,
```

In the `SpaceAction::new()` method, add to the `Self { ... }` block before the closing `}`:

```rust
            activity_score: 0,
            additional_score: 0,
```

- [ ] **Step 2: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/models/space_action.rs
git commit -m "feat(activity): add activity_score and additional_score fields to SpaceAction"
```

---

## Task 9: UpdateSpaceActionRequest — ActivityScore Variant

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/controllers/update_space_action.rs`

- [ ] **Step 1: Add ActivityScore variant to request enum**

In `app/ratel/src/features/spaces/pages/actions/controllers/update_space_action.rs`, add to the `UpdateSpaceActionRequest` enum:

```rust
pub enum UpdateSpaceActionRequest {
    Credits { credits: u64 },
    Time { started_at: i64, ended_at: i64 },
    Prerequisite { prerequisite: bool },
    ActivityScore { activity_score: i64, additional_score: i64 },
}
```

- [ ] **Step 2: Handle the new variant in update_space_action**

In the `match req { ... }` block of `update_space_action`, add after the `Prerequisite` arm:

```rust
        UpdateSpaceActionRequest::ActivityScore {
            activity_score,
            additional_score,
        } => {
            space_action.activity_score = activity_score;
            space_action.additional_score = additional_score;
            SpaceAction::updater(&pk, &EntityType::SpaceAction)
                .with_activity_score(activity_score)
                .with_additional_score(additional_score)
                .with_updated_at(now)
                .execute(cli)
                .await
                .map_err(|e| {
                    Error::InternalServerError(format!("Failed to update activity score: {e:?}"))
                })?;
        }
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/controllers/update_space_action.rs
git commit -m "feat(activity): add ActivityScore variant to UpdateSpaceActionRequest"
```

---

## Task 10: record_activity Helper

**Files:**
- Modify: `app/ratel/src/features/activity/controllers/record_activity.rs`

- [ ] **Step 1: Implement record_activity**

Replace `app/ratel/src/features/activity/controllers/record_activity.rs`:

```rust
use crate::features::activity::models::SpaceActivity;
use crate::features::activity::*;

/// Calculate base score based on action type and data.
#[cfg(feature = "server")]
fn calculate_base_score(
    data: &SpaceActivityData,
    activity_score: i64,
) -> i64 {
    match data {
        SpaceActivityData::Poll { .. } => activity_score,
        SpaceActivityData::Follow { .. } => activity_score,
        SpaceActivityData::Quiz { passed, .. } => {
            if *passed { activity_score } else { 0 }
        }
        SpaceActivityData::Discussion { is_first_contribution, .. } => {
            if *is_first_contribution { activity_score } else { 0 }
        }
        SpaceActivityData::Unknown => 0,
    }
}

/// Calculate additional score based on action type and data.
#[cfg(feature = "server")]
fn calculate_additional_score(
    data: &SpaceActivityData,
    additional_score_per_item: i64,
) -> i64 {
    match data {
        SpaceActivityData::Poll { answered_optional_count, .. } => {
            additional_score_per_item * (*answered_optional_count as i64)
        }
        SpaceActivityData::Quiz { correct_count, .. } => {
            additional_score_per_item * (*correct_count as i64)
        }
        SpaceActivityData::Discussion { .. } => additional_score_per_item,
        SpaceActivityData::Follow { .. } => 0,
        SpaceActivityData::Unknown => 0,
    }
}

/// Record an activity event for a user in a space.
/// This writes a SpaceActivity record; the stream handler aggregates the score.
#[cfg(feature = "server")]
pub(crate) async fn record_activity(
    cli: &aws_sdk_dynamodb::Client,
    space_id: SpacePartition,
    author: AuthorPartition,
    action_id: String,
    action_type: SpaceActionType,
    activity_score: i64,
    additional_score_per_item: i64,
    data: SpaceActivityData,
    user_name: String,
    user_avatar: String,
) -> crate::common::Result<()> {
    let base = calculate_base_score(&data, activity_score);
    let additional = calculate_additional_score(&data, additional_score_per_item);

    let activity = SpaceActivity::new(
        space_id,
        author,
        action_id,
        action_type,
        data,
        base,
        additional,
        user_name,
        user_avatar,
    );
    activity.create(cli).await?;
    Ok(())
}
```

- [ ] **Step 2: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/activity/controllers/record_activity.rs
git commit -m "feat(activity): implement record_activity helper with score calculation"
```

---

## Task 11: aggregate_score Service (Stream Handler)

**Files:**
- Modify: `app/ratel/src/features/activity/services/aggregate_score.rs`

- [ ] **Step 1: Implement aggregate_score**

Replace `app/ratel/src/features/activity/services/aggregate_score.rs`:

```rust
use crate::features::activity::models::{SpaceActivity, SpaceScore};
use crate::features::activity::*;

/// Atomically increment a user's SpaceScore from a SpaceActivity event.
/// Called by the DynamoDB stream handler on SpaceActivity INSERT.
#[cfg(feature = "server")]
pub async fn aggregate_score(activity: SpaceActivity) -> crate::common::Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let now = crate::common::utils::time::get_now_timestamp_millis();

    let (score_pk, score_sk) = SpaceScore::keys(
        &CompositePartition::<SpacePartition, AuthorPartition>::into_first(&activity.pk),
        &activity.user_pk,
    );

    // Build updater with atomic ADD on total_score and the matching breakdown field
    let mut updater = SpaceScore::updater(&score_pk, &score_sk);
    updater = updater
        .increase_total_score(activity.total_score)
        .with_user_pk(activity.user_pk.clone())
        .with_user_name(activity.user_name.clone())
        .with_user_avatar(activity.user_avatar.clone())
        .with_space_pk(activity.space_pk.clone())
        .with_updated_at(now);

    match activity.action_type {
        SpaceActionType::Poll => {
            updater = updater.increase_poll_score(activity.total_score);
        }
        SpaceActionType::Quiz => {
            updater = updater.increase_quiz_score(activity.total_score);
        }
        SpaceActionType::Follow => {
            updater = updater.increase_follow_score(activity.total_score);
        }
        SpaceActionType::TopicDiscussion => {
            updater = updater.increase_discussion_score(activity.total_score);
        }
    }

    updater.execute(cli).await?;
    Ok(())
}
```

Note: The `CompositePartition::into_first` helper may not exist. If not, extract the SpacePartition from the composite pk directly:

```rust
    let space_id = activity.pk.0.clone();
    let author = activity.user_pk.clone();
    let (score_pk, score_sk) = SpaceScore::keys(&space_id, &author);
```

Use whichever compiles. The key point is to reconstruct the SpaceScore pk from the SpaceActivity's pk components.

- [ ] **Step 2: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/activity/services/aggregate_score.rs
git commit -m "feat(activity): implement aggregate_score stream handler service"
```

---

## Task 12: Wire Stream Handler

**Files:**
- Modify: `app/ratel/src/common/stream_handler.rs`

- [ ] **Step 1: Add SpaceActivity match arm**

In `app/ratel/src/common/stream_handler.rs`, in the `"INSERT"` match arm, add after the `NOTIFICATION#` block (before the closing `}` of the INSERT arm):

```rust
            } else if sk.starts_with("SPACE_ACTIVITY#") {
                let activity = deserialize(image)?;
                if let Err(e) =
                    crate::features::activity::services::aggregate_score(activity).await
                {
                    tracing::error!(error = %e, "stream: ActivityScoreAggregate failed");
                }
            }
```

- [ ] **Step 2: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/common/stream_handler.rs
git commit -m "feat(activity): wire SpaceActivity stream handler for score aggregation"
```

---

## Task 13: Integrate record_activity into Action Controllers

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/poll/controllers/respond_poll.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/quiz/controllers/respond_quiz.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/follow/controllers/follow_user.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/add_comment.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/actions/discussion/controllers/comments/reply_comment.rs`

- [ ] **Step 1: Integrate into respond_poll**

In `respond_poll.rs`, after the `SpaceReward::award` block (after line ~282, inside the `else` branch for new responses), add:

```rust
        // Record activity for scoring
        if let Err(e) = crate::features::activity::controllers::record_activity(
            cli,
            space_partition.clone(),
            crate::features::activity::types::AuthorPartition::from(user.pk.clone()),
            poll_action_id.clone(),
            crate::features::spaces::pages::actions::types::SpaceActionType::Poll,
            space_action.activity_score,
            space_action.additional_score,
            crate::features::activity::types::SpaceActivityData::Poll {
                poll_id: poll_sk.to_string(),
                answered_optional_count: req.answers.iter().enumerate().filter(|(i, _)| {
                    poll.questions.get(*i).map_or(false, |q| {
                        match q {
                            Question::SingleChoice(cq) => cq.is_required != Some(true),
                            Question::MultipleChoice(cq) => cq.is_required != Some(true),
                            Question::ShortAnswer(sq) => sq.is_required != Some(true),
                            Question::Subjective(sq) => sq.is_required != Some(true),
                            Question::Checkbox(cq) => cq.is_required != Some(true),
                            Question::Dropdown(dq) => dq.is_required != Some(true),
                            Question::LinearScale(lq) => lq.is_required != Some(true),
                        }
                    })
                }).count() as u32,
            },
            user.display_name.clone(),
            user.profile_url.clone(),
        ).await {
            tracing::error!(error = %e, "Failed to record poll activity");
        }
```

- [ ] **Step 2: Integrate into respond_quiz**

In `respond_quiz.rs`, after the `SpaceReward::award` block (inside the `if score >= quiz.pass_score && !already_passed` block), add:

```rust
        // Record activity for scoring
        if let Err(e) = crate::features::activity::controllers::record_activity(
            cli,
            space_id.clone(),
            crate::features::activity::types::AuthorPartition::from(user.pk.clone()),
            quiz_action_id.clone(),
            crate::features::spaces::pages::actions::types::SpaceActionType::Quiz,
            space_action.activity_score,
            space_action.additional_score,
            crate::features::activity::types::SpaceActivityData::Quiz {
                quiz_id: quiz_id.to_string(),
                passed: true,
                correct_count: score as u32,
                pass_threshold: quiz.pass_score as u32,
            },
            user.display_name.clone(),
            user.profile_url.clone(),
        ).await {
            tracing::error!(error = %e, "Failed to record quiz activity");
        }
```

- [ ] **Step 3: Integrate into follow_user**

In `follow_user.rs`, after the `SpaceReward::award` block (before the final `Ok(())`), add:

```rust
    // Record activity for scoring
    let space_action = crate::features::spaces::pages::actions::models::SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), follow_id.to_string()),
        Some(EntityType::SpaceAction),
    ).await.ok().flatten();
    if let Some(ref sa) = space_action {
        if let Err(e) = crate::features::activity::controllers::record_activity(
            cli,
            space_id.clone(),
            crate::features::activity::types::AuthorPartition::from(user.pk.clone()),
            follow_id.to_string(),
            crate::features::spaces::pages::actions::types::SpaceActionType::Follow,
            sa.activity_score,
            sa.additional_score,
            crate::features::activity::types::SpaceActivityData::Follow {
                follow_id: follow_id.to_string(),
            },
            user.display_name.clone(),
            user.profile_url.clone(),
        ).await {
            tracing::error!(error = %e, "Failed to record follow activity");
        }
    }
```

- [ ] **Step 4: Integrate into add_comment (first contribution)**

In `add_comment.rs`, after the `SpaceReward::award` block (before the final `Ok(comment.into())`), add:

```rust
    // Record activity for scoring (first contribution check)
    let space_action = SpaceAction::get(
        cli,
        &CompositePartition(space_id.clone(), discussion_sk.to_string()),
        Some(EntityType::SpaceAction),
    ).await.ok().flatten();
    if let Some(ref sa) = space_action {
        // Check if this is the user's first contribution to this discussion
        let author_partition = crate::features::activity::types::AuthorPartition::from(author.pk.clone());
        let activity_pk = CompositePartition(space_id.clone(), author_partition.clone());
        let prefix = format!("SPACE_ACTIVITY#{}#", discussion_sk.to_string());
        let (existing_activities, _) = crate::features::activity::models::SpaceActivity::query_by_pk(
            cli, &activity_pk, Some(prefix), None,
        ).await.unwrap_or_default();
        let is_first = existing_activities.is_empty();

        if let Err(e) = crate::features::activity::controllers::record_activity(
            cli,
            space_id.clone(),
            author_partition,
            discussion_sk.to_string(),
            crate::features::spaces::pages::actions::types::SpaceActionType::TopicDiscussion,
            sa.activity_score,
            sa.additional_score,
            crate::features::activity::types::SpaceActivityData::Discussion {
                discussion_id: discussion_sk.to_string(),
                is_first_contribution: is_first,
            },
            author.display_name.clone(),
            author.profile_url.clone(),
        ).await {
            tracing::error!(error = %e, "Failed to record discussion activity");
        }
    }
```

- [ ] **Step 5: Integrate into reply_comment**

In `reply_comment.rs`, after the `SpaceReward::award` block (before the final `Ok(comment.into())`), add the same pattern as add_comment (Step 4 above), but use `author.pk` and `author.display_name`/`author.profile_url` as available in that controller's scope.

- [ ] **Step 6: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS (may require adjusting imports — follow compiler errors)

- [ ] **Step 7: Commit**

```bash
git add app/ratel/src/features/spaces/pages/actions/actions/
git commit -m "feat(activity): integrate record_activity into poll, quiz, follow, discussion controllers"
```

---

## Task 14: get_ranking Endpoint

**Files:**
- Modify: `app/ratel/src/features/activity/controllers/get_ranking.rs`
- Modify: `app/ratel/src/features/spaces/pages/dashboard/controllers/list_ranking.rs`

- [ ] **Step 1: Implement get_ranking controller**

Replace `app/ratel/src/features/activity/controllers/get_ranking.rs`:

```rust
use crate::features::activity::models::SpaceScore;
use crate::features::activity::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RankingResponse {
    pub entries: Vec<RankingEntryResponse>,
    pub bookmark: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct RankingEntryResponse {
    pub rank: u32,
    pub user_pk: String,
    pub name: String,
    pub avatar: String,
    pub total_score: i64,
}

#[get("/api/spaces/:space_id/ranking", space: crate::common::models::space::SpaceCommon)]
pub async fn get_ranking_handler(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> Result<RankingResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut opts = SpaceScore::opt();
    opts.scan_forward = Some(false); // descending by total_score
    opts.limit = Some(50);
    if let Some(ref bm) = bookmark {
        opts.bookmark = Some(bm.clone());
    }

    let (scores, next_bookmark) = SpaceScore::find_by_space_pk(cli, &space_pk, opts).await?;

    let base_rank = if bookmark.is_some() {
        // For paginated requests, we'd need to know the offset.
        // For simplicity, return 0-based ranks; client adjusts.
        0u32
    } else {
        0u32
    };

    let entries: Vec<RankingEntryResponse> = scores
        .iter()
        .enumerate()
        .map(|(i, score)| RankingEntryResponse {
            rank: base_rank + (i as u32) + 1,
            user_pk: score.user_pk.to_string(),
            name: score.user_name.clone(),
            avatar: score.user_avatar.clone(),
            total_score: score.total_score,
        })
        .collect();

    Ok(RankingResponse {
        entries,
        bookmark: next_bookmark,
    })
}
```

- [ ] **Step 2: Implement list_ranking_handler for dashboard**

Replace `app/ratel/src/features/spaces/pages/dashboard/controllers/list_ranking.rs`:

```rust
use crate::features::spaces::pages::dashboard::*;

#[get("/api/spaces/:space_id/incentive/ranking", _space: crate::common::models::space::SpaceCommon)]
pub async fn list_ranking_handler(
    space_id: SpacePartition,
    bookmark: Option<String>,
) -> crate::common::Result<crate::features::spaces::space_common::types::dashboard::RankingTableData> {
    use crate::features::spaces::space_common::types::dashboard::*;
    use crate::features::activity::models::SpaceScore;

    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();
    let space_pk: Partition = space_id.into();

    let mut opts = SpaceScore::opt();
    opts.scan_forward = Some(false);
    opts.limit = Some(50);

    let (scores, _) = SpaceScore::find_by_space_pk(cli, &space_pk, opts).await?;

    let entries: Vec<RankingEntry> = scores
        .iter()
        .enumerate()
        .map(|(i, score)| RankingEntry {
            rank: (i as u32) + 1,
            name: score.user_name.clone(),
            avatar: score.user_avatar.clone(),
            score: score.total_score as f64,
            change: 0,
        })
        .collect();

    Ok(RankingTableData {
        entries,
        page_size: 10,
    })
}
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/controllers/get_ranking.rs app/ratel/src/features/spaces/pages/dashboard/controllers/list_ranking.rs
git commit -m "feat(activity): implement get_ranking and list_ranking_handler endpoints"
```

---

## Task 15: get_my_score Endpoint

**Files:**
- Modify: `app/ratel/src/features/activity/controllers/get_my_score.rs`

- [ ] **Step 1: Implement get_my_score**

Replace `app/ratel/src/features/activity/controllers/get_my_score.rs`:

```rust
use crate::features::activity::models::SpaceScore;
use crate::features::activity::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct MyScoreResponse {
    pub total_score: i64,
    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,
    pub rank: u32,
}

#[get("/api/spaces/:space_id/my-score", space: crate::common::models::space::SpaceCommon, user: crate::features::auth::User)]
pub async fn get_my_score_handler(
    space_id: SpacePartition,
) -> Result<MyScoreResponse> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let author = AuthorPartition::from(user.pk.clone());
    let (pk, sk) = SpaceScore::keys(&space_id, &author);

    let score = SpaceScore::get(cli, &pk, Some(sk)).await?.unwrap_or_default();

    // Calculate rank by counting entries with higher score
    let space_pk: Partition = space_id.into();
    let mut opts = SpaceScore::opt();
    opts.scan_forward = Some(false);
    opts.limit = Some(1000); // reasonable cap

    let (all_scores, _) = SpaceScore::find_by_space_pk(cli, &space_pk, opts).await?;
    let rank = all_scores
        .iter()
        .position(|s| s.total_score <= score.total_score)
        .map(|pos| (pos as u32) + 1)
        .unwrap_or(0);

    Ok(MyScoreResponse {
        total_score: score.total_score,
        poll_score: score.poll_score,
        quiz_score: score.quiz_score,
        follow_score: score.follow_score,
        discussion_score: score.discussion_score,
        rank,
    })
}
```

- [ ] **Step 2: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/activity/controllers/get_my_score.rs
git commit -m "feat(activity): implement get_my_score endpoint with rank calculation"
```

---

## Task 16: ActivityScoreSetting Component

**Files:**
- Modify: `app/ratel/src/features/activity/components/activity_score_setting.rs`
- Modify: `app/ratel/src/features/spaces/pages/actions/components/action_common_settings/mod.rs`

- [ ] **Step 1: Implement ActivityScoreSetting**

Replace `app/ratel/src/features/activity/components/activity_score_setting.rs`:

```rust
use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;
use crate::features::spaces::pages::actions::controllers::update_space_action;
use crate::features::spaces::pages::actions::controllers::UpdateSpaceActionRequest;
use crate::features::spaces::pages::actions::models::SpaceAction;

#[component]
pub fn ActivityScoreSetting(
    space_id: ReadSignal<SpacePartition>,
    action_id: ReadSignal<String>,
    action_setting: ReadSignal<SpaceAction>,
) -> Element {
    let tr: ActivityTranslate = use_translate();
    let mut toast = crate::common::use_toast();
    let setting = action_setting();
    let mut current_activity_score = use_signal(move || setting.activity_score);
    let mut current_additional_score = use_signal(move || setting.additional_score);
    let show_additional = setting.space_action_type != SpaceActionType::Follow;

    rsx! {
        Collapsible {
            CollapsibleTrigger {
                r#as: move |attrs: Vec<Attribute>| {
                    rsx! {
                        Card {
                            variant: CardVariant::Outlined,
                            class: "cursor-pointer",
                            ..attrs,
                            Row {
                                class: "w-full p-4",
                                main_axis_align: MainAxisAlign::Between,
                                cross_axis_align: CrossAxisAlign::Center,
                                span { class: "text-sm font-semibold text-text-primary",
                                    "{tr.activity_score}"
                                }
                                lucide_dioxus::ChevronDown { class: "w-4 h-4 text-foreground-muted" }
                            }
                        }
                    }
                },
            }
            CollapsibleContent {
                Card {
                    variant: CardVariant::Outlined,
                    class: "rounded-t-none border-t-0 p-4",
                    Col {
                        class: "gap-4 w-full",

                        // Activity Score input
                        Col {
                            class: "gap-2",
                            Label { "{tr.activity_score}" }
                            Input {
                                r#type: InputType::Number,
                                value: "{current_activity_score()}",
                                oninput: move |e: FormEvent| {
                                    if let Ok(v) = e.value().parse::<i64>() {
                                        current_activity_score.set(v);
                                    }
                                },
                                onconfirm: {
                                    let space_id = space_id.clone();
                                    let action_id = action_id.clone();
                                    move |_| async move {
                                        let req = UpdateSpaceActionRequest::ActivityScore {
                                            activity_score: current_activity_score(),
                                            additional_score: current_additional_score(),
                                        };
                                        match update_space_action(space_id(), action_id(), req).await {
                                            Ok(_) => toast.info(tr.activity_score_updated.to_string()),
                                            Err(e) => toast.error(e),
                                        }
                                    }
                                },
                            }
                        }

                        // Additional Score input (hidden for Follow)
                        if show_additional {
                            Col {
                                class: "gap-2",
                                Label { "{tr.additional_score}" }
                                p { class: "text-xs text-foreground-muted", "{tr.additional_score_desc}" }
                                Input {
                                    r#type: InputType::Number,
                                    value: "{current_additional_score()}",
                                    oninput: move |e: FormEvent| {
                                        if let Ok(v) = e.value().parse::<i64>() {
                                            current_additional_score.set(v);
                                        }
                                    },
                                    onconfirm: {
                                        let space_id = space_id.clone();
                                        let action_id = action_id.clone();
                                        move |_| async move {
                                            let req = UpdateSpaceActionRequest::ActivityScore {
                                                activity_score: current_activity_score(),
                                                additional_score: current_additional_score(),
                                            };
                                            match update_space_action(space_id(), action_id(), req).await {
                                                Ok(_) => toast.info(tr.activity_score_updated.to_string()),
                                                Err(e) => toast.error(e),
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Add ActivityScoreSetting to ActionCommonSettings**

In `app/ratel/src/features/spaces/pages/actions/components/action_common_settings/mod.rs`, after the `RewardSetting { ... }` block (around line 227), add:

```rust
            #[cfg(feature = "activity")]
            crate::features::activity::components::ActivityScoreSetting {
                space_id: space_id.into(),
                action_id: action_id.into(),
                action_setting: action_setting.into(),
            }
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/components/activity_score_setting.rs app/ratel/src/features/spaces/pages/actions/components/action_common_settings/mod.rs
git commit -m "feat(activity): add ActivityScoreSetting component to ActionCommonSettings"
```

---

## Task 17: RankingWidget (Sidebar)

**Files:**
- Modify: `app/ratel/src/features/activity/components/ranking_widget.rs`
- Modify: `app/ratel/src/features/spaces/space_common/components/space_nav/mod.rs`

- [ ] **Step 1: Implement RankingWidget**

Replace `app/ratel/src/features/activity/components/ranking_widget.rs`:

```rust
use crate::features::activity::controllers::{get_my_score_handler, get_ranking_handler};
use crate::features::activity::i18n::ActivityTranslate;
use crate::features::activity::*;

#[component]
pub fn RankingWidget(space_id: SpacePartition) -> Element {
    let tr: ActivityTranslate = use_translate();

    let ranking_loader = use_server_future(use_reactive(
        (&space_id,),
        |(sid,)| async move { get_ranking_handler(sid.clone(), None).await },
    ))?;

    let my_score_loader = use_server_future(use_reactive(
        (&space_id,),
        |(sid,)| async move { get_my_score_handler(sid.clone()).await },
    ))?;

    let ranking = ranking_loader.read();
    let my_score = my_score_loader.read();

    let top3 = ranking
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .map(|r| r.entries.iter().take(3).cloned().collect::<Vec<_>>())
        .unwrap_or_default();

    let my = my_score
        .as_ref()
        .and_then(|r| r.as_ref().ok())
        .cloned();

    if top3.is_empty() {
        return rsx! {};
    }

    rsx! {
        Card {
            variant: CardVariant::Outlined,
            class: "mx-4 mb-2",
            Col {
                class: "w-full",

                // Header
                Row {
                    class: "px-3 py-2 border-b border-separator",
                    main_axis_align: MainAxisAlign::Between,
                    cross_axis_align: CrossAxisAlign::Center,
                    span { class: "text-xs font-semibold text-text-primary", "{tr.ranking}" }
                }

                // Top 3 entries
                for entry in top3.iter() {
                    Row {
                        class: "px-3 py-1.5",
                        main_axis_align: MainAxisAlign::Between,
                        cross_axis_align: CrossAxisAlign::Center,
                        Row {
                            class: "gap-2",
                            cross_axis_align: CrossAxisAlign::Center,
                            span { class: "text-xs font-medium text-foreground-muted w-4 text-center",
                                "{entry.rank}"
                            }
                            if !entry.avatar.is_empty() {
                                img {
                                    class: "w-5 h-5 rounded-full",
                                    src: "{entry.avatar}",
                                    alt: "{entry.name}",
                                }
                            } else {
                                div { class: "flex items-center justify-center w-5 h-5 rounded-full bg-primary",
                                    span { class: "text-[10px] font-medium text-btn-primary-text",
                                        "{entry.name.chars().next().unwrap_or('?')}"
                                    }
                                }
                            }
                            span { class: "text-xs text-text-primary truncate max-w-[80px]",
                                "{entry.name}"
                            }
                        }
                        span { class: "text-xs font-medium text-text-primary",
                            "{entry.total_score}"
                        }
                    }
                }

                // Current user row
                if let Some(ref score) = my {
                    if score.rank > 0 {
                        div { class: "border-t border-separator" }
                        Row {
                            class: "px-3 py-1.5 bg-primary/10 rounded-b-lg",
                            main_axis_align: MainAxisAlign::Between,
                            cross_axis_align: CrossAxisAlign::Center,
                            Row {
                                class: "gap-2",
                                cross_axis_align: CrossAxisAlign::Center,
                                span { class: "text-xs font-medium text-foreground-muted w-4 text-center",
                                    "{score.rank}"
                                }
                                span { class: "text-xs font-semibold text-primary",
                                    "{tr.you}"
                                }
                            }
                            span { class: "text-xs font-medium text-primary",
                                "{score.total_score}"
                            }
                        }
                    }
                }
            }
        }
    }
}
```

- [ ] **Step 2: Add RankingWidget to SpaceNav**

In `app/ratel/src/features/spaces/space_common/components/space_nav/mod.rs`, before the `Row { class: "max-tablet:hidden", ... }` block that contains `SpaceUserProfile` (around line 76), add:

```rust
            // Ranking widget (desktop only)
            #[cfg(feature = "activity")]
            div { class: "max-tablet:hidden px-2",
                crate::features::activity::components::RankingWidget {
                    space_id: space_id.clone(),
                }
            }
```

- [ ] **Step 3: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/activity/components/ranking_widget.rs app/ratel/src/features/spaces/space_common/components/space_nav/mod.rs
git commit -m "feat(activity): add RankingWidget to space sidebar"
```

---

## Task 18: Wire Dashboard RankingTable to Real Data

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/dashboard/controllers/list_dashboard_data.rs`

- [ ] **Step 1: Add RankingTable to dashboard data**

In `app/ratel/src/features/spaces/pages/dashboard/controllers/list_dashboard_data.rs`, after the InfoCard block (before the final `Ok(components)`), add:

```rust
    // RankingTable: populated from activity scores
    #[cfg(feature = "activity")]
    {
        use crate::features::activity::models::SpaceScore;
        use crate::features::spaces::space_common::types::dashboard::*;

        let mut opts = SpaceScore::opt();
        opts.scan_forward = Some(false);
        opts.limit = Some(50);

        if let Ok((scores, _)) = SpaceScore::find_by_space_pk(cli, &space_pk, opts).await {
            if !scores.is_empty() {
                let entries: Vec<RankingEntry> = scores
                    .iter()
                    .enumerate()
                    .map(|(i, score)| RankingEntry {
                        rank: (i as u32) + 1,
                        name: score.user_name.clone(),
                        avatar: score.user_avatar.clone(),
                        score: score.total_score as f64,
                        change: 0,
                    })
                    .collect();

                components.push(DashboardComponentData::RankingTable(RankingTableData {
                    entries,
                    page_size: 10,
                }));
            }
        }
    }
```

- [ ] **Step 2: Run dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/dashboard/controllers/list_dashboard_data.rs
git commit -m "feat(activity): wire RankingTable in dashboard to SpaceScore data"
```

---

## Task 19: Final Verification

- [ ] **Step 1: Full dx check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web
```

Expected: PASS with no errors

- [ ] **Step 2: Check feature flag isolation**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web --no-default-features --features "web,membership,social,spaces_full"
```

Expected: PASS (activity feature disabled, no compilation errors from cfg-gated code)

- [ ] **Step 3: Final commit if any fixups needed**

```bash
git add -A && git commit -m "fix(activity): compilation fixes from final verification"
```

Only if Step 1 or 2 required changes.
