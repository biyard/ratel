# Activity Ranking System Design

## Overview

An activity scoring and ranking system for spaces. Users earn points by performing actions (poll, follow, quiz, discussion/reply). Creators can customize scores per action. Rankings are displayed in the space dashboard (full table) and sidebar (top 3 + current user).

Activity scores coexist independently with the existing credits/rewards system.

## Architecture

**Write path**: Action controllers write a `SpaceActivity` record to DynamoDB after each action. The DynamoDB Stream handler picks up the INSERT and atomically increments a `SpaceScore` record using `UpdateItem` with `ADD`.

**Read path**: Ranking queries use a GSI on `SpaceScore` sorted by `total_score` descending. Individual user scores use direct GetItem on the composite partition key.

```
[Action Controller] → INSERT SpaceActivity → [DynamoDB Stream] → UpdateItem SpaceScore
                                                                         ↓
[Dashboard/Sidebar] ← GSI Query (space_pk, score DESC) ← SpaceScore table
```

## Data Model

### AuthorPartition (new type in `common/types/`)

Represents the actor — either a user or team. Used as the second component of CompositePartition for activity/score entities.

```rust
#[derive(Debug, Clone, SerializeDisplay, DeserializeFromStr, Default, PartialEq, Eq)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub enum AuthorPartition {
    #[default]
    Unknown,
    User(String),   // Display: "USER#user_id"
    Team(String),   // Display: "TEAM#team_id"
}

impl From<UserPartition> for AuthorPartition { ... }
impl From<TeamPartition> for AuthorPartition { ... }
```

Display/FromStr serialization:
- `AuthorPartition::User("abc")` → `"USER#abc"`
- `AuthorPartition::Team("xyz")` → `"TEAM#xyz"`

### SpaceActivity (immutable event log)

Records each user activity event. One record per action execution (except discussion replies, which create one record per reply).

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceActivity {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,  // SPACE#id##USER#id
    pub sk: EntityType,  // SpaceActivity#action_id#timestamp

    #[dynamo(prefix = "SACT", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "TS", index = "gsi1", sk)]
    pub created_at: i64,

    pub user_pk: AuthorPartition,
    pub action_id: String,
    pub action_type: SpaceActionType,
    pub data: SpaceActivityData,

    pub user_name: String,
    pub user_avatar: String,

    pub base_score: i64,       // action score
    pub additional_score: i64, // additional score
    pub total_score: i64,      // base + additional (pre-computed for stream handler)
}
```

### SpaceActivityData

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SpaceActivityData {
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

### SpaceScore (aggregated counter, updated by stream)

Single record per user per space. Updated atomically by the stream handler.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceScore {
    pub pk: CompositePartition<SpacePartition, AuthorPartition>,  // SPACE#id##USER#id
    pub sk: EntityType,  // SpaceScore

    #[dynamo(prefix = "SCSP", index = "gsi1", pk)]
    pub space_pk: Partition,
    #[dynamo(prefix = "SCR", index = "gsi1", sk)]
    pub total_score: i64,

    pub user_pk: AuthorPartition,
    pub user_name: String,
    pub user_avatar: String,

    // Score breakdown by action type
    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,

    pub updated_at: i64,
}
```

**Access patterns**:
- My score: `GetItem(pk=CompositePartition(space, author), sk=SpaceScore)`
- Leaderboard: `GSI1 Query(gsi1_pk=space_pk, ScanIndexForward=false, Limit=N)`
- My rank: Query GSI1 with `total_score >= my_score`, count results

**Hot partition mitigation**: CompositePartition distributes writes across partitions. GSI1 concentrates reads per space but reads are 3x cheaper (3,000 RCU vs 1,000 WCU per partition) and ranking queries are infrequent compared to score writes.

### SpaceAction changes (2 new fields)

Add to existing `SpaceAction` struct:

```rust
pub activity_score: i64,       // default auto-calculated per action type
pub additional_score: i64,     // default 5 (0 for Follow)
```

Auto-default calculation when action is created:

| Action | `activity_score` default | `additional_score` default |
|--------|-------------------------|---------------------------|
| Poll | 10 × question_count | 5 |
| Follow | 10 | 0 |
| Quiz | 30 × question_count | 5 |
| Discussion | 50 | 5 |

For Quiz: awarded `activity_score` = `pass_threshold × (activity_score / question_count)` when user passes. If creator overrides `activity_score` to a flat number, that flat number is awarded on pass.

### EntityType additions

```rust
// In common/types/entity_type.rs
SpaceActivity(String),  // SPACE_ACTIVITY#action_id#timestamp
SpaceScore,             // SPACE_SCORE
```

### Partition addition (if needed)

No new Partition variants needed — uses existing `Partition::Space`.

## Score Calculation Rules

### Poll (one-per-action)
- **Base**: `activity_score` from SpaceAction (default: 10 × question_count)
- **Additional**: `additional_score × answered_optional_count` (default: 5 per optional question answered)
- Questions with `is_required: Some(false)` or `is_required: None` are optional

### Follow (one-per-action)
- **Base**: `activity_score` from SpaceAction (default: 10)
- **Additional**: 0 (not applicable)

### Quiz (one-per-action)
- **Base**: `activity_score` from SpaceAction if user passes (default: 30 × pass_threshold). 0 if user fails.
- **Additional**: `additional_score × correct_count` (default: 5 per correct answer)
- Score is awarded only on first passing attempt (not on retries if already passed)

### Discussion — first contribution (one-per-action)
- **Base**: `activity_score` from SpaceAction (default: 50)
- **Additional**: `additional_score` (default: 5 for the reply itself)

### Discussion — subsequent replies (repeatable)
- **Base**: 0
- **Additional**: `additional_score` (default: 5 per reply)

## Feature Module Structure

New feature: `app/ratel/src/features/activity/`

```
features/activity/
├── mod.rs
├── models/
│   ├── mod.rs
│   ├── space_activity.rs
│   └── space_score.rs
├── types/
│   ├── mod.rs
│   ├── author_partition.rs
│   ├── space_activity_data.rs
│   └── error.rs
├── controllers/
│   ├── mod.rs
│   ├── get_ranking.rs           # GET /api/spaces/:space_id/ranking
│   ├── get_my_score.rs          # GET /api/spaces/:space_id/my-score
│   └── record_activity.rs       # internal helper (pub(crate)), not an endpoint
├── services/
│   ├── mod.rs
│   └── aggregate_score.rs       # stream handler logic
├── components/
│   ├── mod.rs
│   ├── ranking_widget.rs        # sidebar: top 3 + current user
│   └── activity_score_setting.rs # for ActionCommonSettings
├── hooks/
│   └── mod.rs
└── i18n.rs
```

Feature flag: `activity` (new), added to `full` and `spaces_full` feature sets.

Registration: add `pub mod activity;` to `features/mod.rs`.

## Stream Handler Changes

In `stream_handler.rs`, add a new match arm in the `INSERT` branch:

```rust
} else if sk.starts_with("SPACE_ACTIVITY#") {
    let activity: SpaceActivity = deserialize(image)?;
    if let Err(e) = crate::features::activity::services::aggregate_score(activity).await {
        tracing::error!(error = %e, "stream: ActivityScoreAggregate failed");
    }
}
```

### `aggregate_score` implementation

```rust
pub async fn aggregate_score(activity: SpaceActivity) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let score_pk = activity.pk.clone();
    let score_sk = EntityType::SpaceScore;

    // Atomic increment using UpdateItem with ADD
    // If item doesn't exist, DynamoDB creates it with ADD initializing from 0
    SpaceScore::updater(&score_pk, &score_sk)
        .increase_total_score(activity.total_score)
        .increase_{action_type}_score(activity.total_score)  // dynamic per type
        .with_user_pk(activity.user_pk)
        .with_user_name(...)  // from activity or lookup
        .with_space_pk(activity.space_pk)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    Ok(())
}
```

Note: The updater's `increase_*` methods map to DynamoDB `ADD` operations, which are atomic and create the attribute if it doesn't exist (initializing to 0 + value). The `with_*` methods use `SET` for idempotent metadata updates.

For `user_name` and `user_avatar`: the stream handler can look up from the User entity, or the `record_activity` helper can include them in the SpaceActivity record directly (preferred — avoids extra read in hot path).

## Action Controller Integration

Each action controller calls `record_activity()` after the main action succeeds. This is a non-blocking best-effort write (log error but don't fail the action).

### `record_activity` helper

```rust
pub(crate) async fn record_activity(
    cli: &aws_sdk_dynamodb::Client,
    space_id: SpacePartition,
    author: AuthorPartition,
    action_id: String,
    action_type: SpaceActionType,
    activity_score: i64,
    additional_score: i64,
    data: SpaceActivityData,
    user_name: String,
    user_avatar: String,
) -> Result<()> {
    let base_score = calculate_base_score(&action_type, &data, activity_score);
    let add_score = calculate_additional_score(&action_type, &data, additional_score);
    let total = base_score + add_score;

    let activity = SpaceActivity::new(
        space_id, author, action_id, action_type,
        data, base_score, add_score, total,
        user_name, user_avatar,
    );
    activity.create(cli).await?;
    Ok(())
}
```

### Integration points (4 controllers)

1. **`respond_poll`** — after `answer_record.create()` succeeds (new response only, not edits):
   - Count optional questions answered
   - Call `record_activity` with `SpaceActivityData::Poll { poll_id, answered_optional_count }`

2. **`respond_quiz`** — after `attempt.create()` succeeds, only on first passing attempt:
   - Call `record_activity` with `SpaceActivityData::Quiz { quiz_id, passed, correct_count, pass_threshold }`

3. **`follow_user`** — after the transact_write succeeds:
   - Call `record_activity` with `SpaceActivityData::Follow { follow_id }`

4. **`reply_comment`** — after `SpacePostComment::reply()` succeeds:
   - Check if user has prior SpaceActivity records for this discussion to determine `is_first_contribution`
   - Call `record_activity` with `SpaceActivityData::Discussion { discussion_id, is_first_contribution }`

## API Endpoints

### GET `/api/spaces/:space_id/ranking`

Returns paginated ranking for a space.

**Query params**: `bookmark: Option<String>`, `limit: Option<i32>` (default 20, max 100)

**Response**:
```rust
pub struct RankingResponse {
    pub entries: Vec<RankingEntry>,
    pub bookmark: Option<String>,
    pub my_rank: Option<MyRankEntry>,
}

pub struct MyRankEntry {
    pub rank: u32,
    pub name: String,
    pub avatar: String,
    pub total_score: i64,
}
```

**Implementation**: GSI1 query on `space_pk` sorted by `total_score` desc. For `my_rank`, separate query counting items with `total_score > my_score` + 1.

### GET `/api/spaces/:space_id/my-score`

Returns the current user's score breakdown.

**Response**:
```rust
pub struct MyScoreResponse {
    pub total_score: i64,
    pub poll_score: i64,
    pub quiz_score: i64,
    pub follow_score: i64,
    pub discussion_score: i64,
    pub rank: u32,
}
```

**Implementation**: GetItem on `CompositePartition(space, author)` + `SpaceScore` sk. Rank computed by counting GSI1 entries with higher score.

## UI Changes

### 1. ActionCommonSettings — Activity Score Section

New `ActivityScoreSetting` component added to `ActionCommonSettings` after `RewardSetting`.

**Props**: `space_id`, `action_id`, `action_setting: SpaceAction`

**UI**:
- "Activity Score" label + numeric Input (shows auto-calculated default, creator-editable)
- "Additional Score" label + numeric Input (shows 5 default; hidden for Follow actions)
- On change: calls `update_space_action` with new `UpdateSpaceActionRequest::ActivityScore` variant

**UpdateSpaceActionRequest addition**:
```rust
pub enum UpdateSpaceActionRequest {
    Credits { credits: u64 },
    Time { started_at: i64, ended_at: i64 },
    Prerequisite { prerequisite: bool },
    ActivityScore { activity_score: i64, additional_score: i64 },  // NEW
}
```

### 2. Space Dashboard — Ranking Table (wired to real data)

Currently `list_ranking_handler` is commented out. Wire it up:

- Uncomment and implement `list_ranking_handler` using `get_ranking` controller
- `DashboardComponentData::RankingTable` already exists in the dashboard grid
- Add `my_rank` display at the bottom of the RankingTable component (sticky footer row)
- Current user's row highlighted with `bg-primary/10` background

### 3. Space Sidebar — Ranking Widget

New `RankingWidget` component placed in `SpaceNav` between nav items and user profile (desktop only, hidden on mobile via `max-tablet:hidden`).

**Layout**:
```
┌─────────────────────┐
│ 🏆 Ranking          │
├─────────────────────┤
│ 1  Alice    1,250   │
│ 2  Bob        980   │
│ 3  Carol      870   │
├─────────────────────┤
│ 15 You        340   │ ← current user (if not in top 3)
└─────────────────────┘
```

**Data source**: Uses same `get_ranking` endpoint with `limit=3`. Current user score from `get_my_score`.

**Props**: `space_id: SpacePartition`

**Behavior**:
- Shows top 3 entries with rank, name, score
- Shows current user's rank at the bottom (always, even if in top 3)
- If user is not logged in, hides "You" row
- Compact design: fits within sidebar width

## Error Types

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum ActivityError {
    #[error("activity already recorded for this action")]
    #[translate(en = "You have already completed this action", ko = "이미 완료한 활동입니다")]
    AlreadyRecorded,

    #[error("score aggregation failed")]
    #[translate(en = "Score update failed, please try again", ko = "점수 업데이트에 실패했습니다")]
    AggregationFailed,
}
```

Registered in `common::Error` with `#[from]` + `#[translate(from)]`.

## i18n Strings

```rust
translate! {
    ActivityTranslate;

    activity_score: { en: "Activity Score", ko: "활동 점수" },
    additional_score: { en: "Additional Score", ko: "추가 점수" },
    ranking: { en: "Ranking", ko: "랭킹" },
    my_rank: { en: "My Rank", ko: "내 순위" },
    score: { en: "Score", ko: "점수" },
    rank: { en: "Rank", ko: "순위" },
    participant: { en: "Participant", ko: "참여자" },
    no_ranking_data: { en: "No ranking data yet", ko: "아직 랭킹 데이터가 없습니다" },
    activity_score_updated: { en: "Activity score updated.", ko: "활동 점수가 업데이트되었습니다." },
}
```

## Testing Strategy

- **Unit**: Score calculation logic (base + additional for each action type)
- **Integration**: `record_activity` writes SpaceActivity, `aggregate_score` increments SpaceScore
- **Controller tests**: `get_ranking` and `get_my_score` endpoints with test data
- **E2E (Playwright)**: Creator sets activity scores → participant performs action → ranking updates

## Migration / Rollout

- No data migration needed — new entities, new feature flag
- Existing spaces get ranking functionality when `activity` feature is enabled
- Existing actions without `activity_score`/`additional_score` fields default to 0 (serde default)
- Auto-calculation of defaults happens only on new action creation, not retroactively
