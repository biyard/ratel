# Meet Action (Phase 1 — admin add + configure) · System Design

**Roadmap**: [roadmap/meet-action.md](../../../roadmap/meet-action.md)
**Design**: [/app/ratel/assets/design/meet-action/](../../../app/ratel/assets/design/meet-action/)
**Author / Date**: hackartists · 2026-04-23

## Summary

Ship Meet as the 5th space action alongside Poll / Quiz / Discussion / Follow. Phase 1 scope is narrow: a space admin can add a Meet from the existing action type picker and configure it (title, description, mode, start time, duration, rewards) on a dedicated editor page. Live meeting UI, recording, transcription, calendar sync, notifications, and Essence ingestion are explicit non-goals of this phase and are sequenced for follow-up phases.

## Scope

### In scope
- `SpaceActionType::Meet` variant on the existing action enum.
- `TypePickerModal` gains a 5th "Meet" option with the `NEW` affordance.
- `SpaceMeet` entity + companion `SpaceAction` row created in one transaction (mirrors the Poll pattern).
- Dioxus page at `Route::MeetActionPage { space_id, meet_id }` with role-based branching (admin → editor, participant → viewer stub).
- Field-level update endpoint `update_meet` for Meet-specific fields (mode, start_time, duration_min).
- Common action settings reused: `ActionDependencySelector`, `ActionRewardSetting`, `PrerequisiteTile`, `ActionStatusControl`, `ActionDeleteButton`.
- `MeetActionCard` for the carousel with Scheduled / Live / Ended visual states (derived from `SpaceActionStatus` + timestamps).
- `DashboardAggregate::inc_meets` counter.

### Stubbed in this phase (UI only)
- Essence toggle on editor — reads `Space.include_meetings_in_essence` but does not ingest.
- Notifications preview card — static T-0 / T-10min / Live / Rec-ready chips, no delivery.
- Rewards — single common `ActionRewardSetting` instead of the mockup's host/attendee split. Actual reward distribution piggybacks on the existing pipeline in a later phase.

### Out of scope (follow-up phases)
- Live meeting UI (`live.html`): video grid, chat, raise-hand, reactions, host controls.
- Archive page (`ended.html`): recording player, diarized transcript, chat log, moderation log, participant presence aggregate.
- Terminal states (Cancelled / Expired) with cancellation reason persistence.
- Google Calendar OAuth + ICS download.
- AWS Chime SDK integration, recording, transcription.
- Inbox + email notification delivery for the four Meet events.
- Space Essence ingestion of Meet transcripts.
- Host / attendee reward split with ≥1-minute presence gating.

## Data model

### New entity: `SpaceMeet`

`features/spaces/pages/actions/actions/meet/models/space_meet.rs`

```rust
#[derive(DynamoEntity, Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[dynamo(prefix = "SM")]
pub struct SpaceMeet {
    pub pk: Partition,            // SPACE#{id} — matches SpacePoll
    pub sk: EntityType,           // SPACE_MEET#{meet_id}

    pub created_at: i64,
    pub updated_at: i64,

    pub mode: MeetMode,           // Scheduled | Instant
    pub start_time: i64,          // ms timestamp
    pub duration_min: i32,        // 15..=1440
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum MeetMode {
    #[default]
    Scheduled,
    Instant,
}
```

Meet state is **not** stored as a separate enum. It is derived from the companion `SpaceAction.status` plus timestamps:

| `SpaceAction.status` | Time | Derived phase |
|----------------------|------|---------------|
| `Designing` | — | Draft (editor only, hidden from carousel for non-admins) |
| `Ongoing` | `now < start_time` | Scheduled |
| `Ongoing` | `start_time ≤ now < start_time + duration_min` | Live (Phase 1: label only, no Chime session) |
| `Ongoing` | `now ≥ start_time + duration_min` | (auto-stale — follow-up phase will transition to Finish) |
| `Finish` | — | Ended |

Cancelled / Expired distinction is deferred until a later phase introduces `cancelled_at`, `cancel_reason`, `live_started_at`, `ended_at` fields.

### Companion `SpaceAction` row

`SpaceAction` is written in the same transaction. It carries:

- `title`, `description`, `credits`, `activity_score`, `prerequisite`, `depends_on`, `status` — all owned by the common action row.
- `space_action_type = SpaceActionType::Meet`.

Mirrors `create_poll.rs` exactly. Meet-specific fields stay on `SpaceMeet`; everything shared across action types stays on `SpaceAction`.

### `EntityType` additions

```rust
#[derive(SubPartition, ...)]
pub enum EntityType {
    ...,
    SpaceMeet(String),  // auto-generates SpaceMeetEntityType
}
```

### `SpaceActionType` additions

```rust
pub enum SpaceActionType {
    Poll, TopicDiscussion, Follow, Quiz,
    #[translate(ko = "미팅", en = "Meet")]
    Meet,
}

impl SpaceActionType {
    pub fn to_behavior(&self) -> RewardUserBehavior {
        match self {
            ...,
            SpaceActionType::Meet => RewardUserBehavior::AttendMeet,
        }
    }

    pub async fn create(&self, space_id: SpacePartition) -> Result<Route> {
        match self {
            ...,
            SpaceActionType::Meet => {
                let response = create_meet(space_id.clone()).await?;
                let meet_id = SpaceMeetEntityType::from(response.sk);
                Ok(Route::MeetActionPage { space_id, meet_id })
            }
        }
    }
}
```

`RewardUserBehavior::AttendMeet` is a new variant on the existing reward behavior enum.

### Aggregate

`DashboardAggregate::inc_meets(space_pk, delta)` — transact item mirroring `inc_polls`, `inc_posts`. Used by `create_meet` (+1) and `delete_meet` (−1).

## API surface

All endpoints follow the Ratel naming convention (SubPartition types in path, `id` naming, typed error enums, `#[mcp_tool]` on every endpoint to expose via MCP).

### `POST /api/spaces/{space_pk}/meets`

Create an empty Meet + companion SpaceAction row. Admin only.

```rust
#[mcp_tool(name = "create_meet", description = "Create a new meet action in a space. Requires creator role.")]
#[post("/api/spaces/{space_pk}/meets", role: SpaceUserRole, space: SpaceCommon)]
pub async fn create_meet(
    #[mcp(description = "Space partition key")] space_pk: SpacePartition,
) -> Result<MeetResponse>;
```

Transaction writes: `SpaceMeet`, `SpaceAction`, `DashboardAggregate::inc_meets(+1)`. Returns `MeetResponse { pk, sk, mode, start_time, duration_min, space_action: SpaceAction }`.

### `GET /api/spaces/{space_pk}/meets/{meet_sk}`

Fetch a Meet + companion SpaceAction. Space participants (role-gated).

```rust
#[mcp_tool(name = "get_meet", description = "Fetch a meet action.")]
#[get("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole, space: SpaceCommon)]
pub async fn get_meet(
    space_pk: SpacePartition,
    meet_sk: SpaceMeetEntityType,
) -> Result<MeetResponse>;
```

### `POST /api/spaces/{space_pk}/meets/{meet_sk}`

Field-level update of Meet-specific fields. Admin only.

```rust
#[derive(Serialize, Deserialize)]
pub enum UpdateMeetRequest {
    Mode { mode: MeetMode },
    StartTime { start_time: i64 },
    DurationMin { duration_min: i32 },
}

#[mcp_tool(name = "update_meet", description = "Update meet-specific fields.")]
#[post("/api/spaces/{space_pk}/meets/{meet_sk}", role: SpaceUserRole, space: SpaceCommon)]
pub async fn update_meet(
    space_pk: SpacePartition,
    meet_sk: SpaceMeetEntityType,
    req: UpdateMeetRequest,
) -> Result<()>;
```

Common fields (title, description, credits, prerequisite, depends_on, status) are updated through the existing `update_space_action` endpoint — no Meet-specific wrappers.

### `DELETE /api/spaces/{space_pk}/meets/{meet_sk}`

Delete the Meet + companion SpaceAction + decrement aggregate. Admin only. Uses the existing `delete_space_action` controller with a Meet-specific wrapper if needed, or a dedicated `delete_meet` endpoint mirroring `delete_poll`.

### Typed error enum

```rust
#[derive(Error, Serialize, Deserialize, Translate, Clone)]
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
}
```

Registered on `common::Error` with `#[from]` + `#[translate(from)]`.

## Event flow

Not applicable in Phase 1. The Meet entity does not trigger DynamoDB Stream → EventBridge pipes in this phase — no recording, no transcription, no notification fan-out, no Essence ingestion. Follow-up phases will add pipes for recording-ready, transcript-ready, meeting-lifecycle events.

## External integrations

None in Phase 1. AWS Chime SDK, Google Calendar OAuth, SMTP email — all follow-up.

## Frontend architecture

### Routes

```rust
Route::MeetActionPage { space_id: SpacePartition, meet_id: SpaceMeetEntityType }
```

Single route for both admin editor and participant viewer. Internal branching uses `use_space_role()`.

### Component tree

```
MeetActionPage (component.rs, role-based branch)
├─ MeetEditorView         (admin — editor_view.rs)
│  ├─ ArenaTopbar          (common)
│  ├─ ActionEditTopbar     (common)
│  ├─ MeetModeToggle       (meet-specific: Scheduled / Instant segmented toggle)
│  ├─ MeetDetailsCard      (title + description via update_space_action)
│  ├─ MeetWhenCard         (start_time datetime-local + duration stepper via update_meet)
│  ├─ MeetConfigCard       (config_card.rs — wraps all common action settings)
│  │  ├─ ActionDependencySelector
│  │  ├─ ActionRewardSetting
│  │  ├─ PrerequisiteTile
│  │  ├─ ActionStatusControl
│  │  └─ ActionDeleteButton
│  ├─ MeetEssenceToggleCard  (stub: reads Space.include_meetings_in_essence, paid-tier gating UI, no ingestion)
│  ├─ MeetNotificationsPreviewCard  (static info panel — 4 chips)
│  └─ MeetSubmitBar        (sticky primary "Meet 예약" / "지금 시작" — wraps update_meet(StartTime) + ActionStatusControl Ongoing)
└─ MeetViewerView         (participant — viewer_view.rs, minimal stub)
   ├─ Title + description
   ├─ Scheduled → start-time countdown
   ├─ Live → "Live now" placeholder (no Chime)
   └─ Ended → "Meeting ended" placeholder (no archive yet)
```

### Carousel integration

`MeetActionCard` (features/spaces/pages/actions/actions/meet/components/meet_card/) rendered inside `action_dashboard/component.rs`'s match arm for `SpaceActionType::Meet`. Accepts `SpaceActionSummary` and derives display from `status` + `start_time` + `duration_min`:

| Derived phase | Card appearance |
|---------------|-----------------|
| Draft (Designing) | visible in the carousel (same as Poll / Quiz in Designing), muted styling, placeholder title "새 회의" if empty, "설정 중" badge, click → re-enters editor |
| Scheduled | title, D-day countdown, coral accent (`#fb7185`), "자세히 보기" CTA |
| Live | pulse-dot animation, "LIVE 진행 중", "입장" CTA (stub click) |
| Ended | muted style, "종료됨", "아카이브 보기" CTA (stub click) |

Click routes to `Route::MeetActionPage` where role-based branching renders the right view.

### Controller hook

`features/spaces/pages/actions/actions/meet/components/meet_page/hooks/use_meet.rs` (new):

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct UseMeet {
    pub meet: Loader<MeetResponse>,
    pub update_mode: Action<(MeetMode,), ()>,
    pub update_start_time: Action<(i64,), ()>,
    pub update_duration: Action<(i32,), ()>,
    pub publish: Action<(), ()>,  // Instant/Scheduled submit: patches start_time (if Instant) then status=Ongoing
}

pub fn use_meet(space_id: ReadSignal<SpacePartition>, meet_id: ReadSignal<SpaceMeetEntityType>)
    -> Result<UseMeet, RenderError>;
```

Components consume `UseMeet` — they never import `_handler` functions directly. See `conventions/hooks-and-actions.md`.

### Space entity additive field

`Space` gains `include_meetings_in_essence: bool` (default `false`). Purely persisted in Phase 1; the actual Essence ingestion wiring is a later phase. Added here so the `MeetEssenceToggleCard` has a real field to bind to rather than introducing a runtime-only signal.

## Test plan

### Server function tests — `app/ratel/src/tests/meet_action_tests.rs`

| Test | Verifies |
|------|----------|
| `test_create_meet_admin_success` | POST `/api/spaces/{pk}/meets` → 200; `SpaceMeet` row exists; companion `SpaceAction` row exists; `DashboardAggregate.meets` = 1 |
| `test_create_meet_unauthorized` | Participant role → 403 |
| `test_create_meet_unauthenticated` | No session → 401 |
| `test_update_meet_mode` | `UpdateMeetRequest::Mode { Instant }` persists |
| `test_update_meet_start_time` | `UpdateMeetRequest::StartTime { ts }` persists |
| `test_update_meet_duration_valid` | 15, 60, 1440 all succeed |
| `test_update_meet_duration_invalid` | 0, -10, 1441 → 400 (`InvalidDuration`) |
| `test_get_meet_returns_space_action` | Response carries `space_action: SpaceAction` |
| `test_delete_meet_removes_both_rows` | `SpaceMeet` + `SpaceAction` gone; aggregate = 0 |
| `test_action_type_create_meet_returns_route` | `SpaceActionType::Meet.create(pk)` → `Route::MeetActionPage` |

### MCP tool tests — `app/ratel/src/tests/mcp_tests.rs`

`test_mcp_tool_create_meet` — MCP bridge invokes the handler and returns the expected MeetResponse JSON.

### Playwright e2e — extend `spaces` scenario

Add new `test()` blocks inside the existing `test.describe.serial()` in `playwright/tests/web/spaces-*.spec.js`:

1. Admin opens space arena → AddActionCard → TypePickerModal shows Meet as 5th option (`data-testid="type-option-meet"`).
2. Click Meet → URL updates to `/spaces/{id}/meets/{id}` and MeetEditorView renders (`data-testid="meet-editor-view"`).
3. Toggle Mode → attribute `aria-selected` flips on `meet-mode-scheduled` / `meet-mode-instant`.
4. Fill title → autosave via `update_space_action`; reload reveals the saved value.
5. Click duration stepper + and − → value updates and persists.
6. Click "Meet 예약" → navigate back to action list; `MeetActionCard` appears in the carousel with `[data-kind="meet"]` and status `Scheduled`.
7. Click the card → MeetActionPage re-renders the editor (admin) or viewer stub (participant).

Required new `data-testid` attributes: `type-option-meet`, `meet-editor-view`, `meet-viewer-view`, `meet-mode-toggle`, `meet-mode-scheduled`, `meet-mode-instant`, `meet-title-input`, `meet-description-input`, `meet-start-time`, `meet-duration-value`, `meet-duration-inc`, `meet-duration-dec`, `meet-submit-button`, `action-card-meet`.

### Build verification

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- meet_action_tests
```

## Open questions / risks

- **OQ-A**: The mockup's `create-meet.html` shows two separate reward inputs (host / attendee). This design collapses them into a single `ActionRewardSetting`. When the host / attendee split is introduced in a later phase, the reward model will need a second credit field on either `SpaceAction` or `SpaceMeet`. Current bet: add it on `SpaceMeet` to keep `SpaceAction` generic.
- **OQ-B**: Draft Meets are shown in the carousel with a "설정 중" badge (mirrors Poll / Quiz in Designing). Alternative: hide Draft Meets from the carousel entirely so the space action list stays publish-clean. Decision: follow Poll's convention in Phase 1 for consistency; revisit if user feedback shows Draft cards are noisy.
- **OQ-C**: `RewardUserBehavior::AttendMeet` has no pipeline yet (distribution comes in a later phase). It is introduced now so `SpaceActionType::Meet::to_behavior()` compiles; unused downstream until the Live phase ships.
- **Risk**: Phase 1 creates a visible "Meet" action with a functional Publish button but no actual meeting. Users reaching the Ongoing state will see a "Live now" placeholder with no way to join. Mitigation: ship Phase 1 as admin-only preview (e.g., gated behind a feature flag or the `bypass` flag) until the Live phase lands.

## References

- [roadmap/meet-action.md](../../../roadmap/meet-action.md) — Stage 1 requirements.
- [app/ratel/assets/design/meet-action/](../../../app/ratel/assets/design/meet-action/) — Stage 2 HTML mockups.
- `features/spaces/pages/actions/actions/poll/controllers/create_poll.rs` — the pattern this design mirrors for the create path.
- `features/spaces/pages/actions/actions/poll/views/main/creator/config_card/component.rs` — the config card pattern this design reuses.
- `.claude/rules/conventions/server-functions.md`, `hooks-and-actions.md`, `server-function-tests.md`, `mcp-tools.md` — conventions followed.
