# Badge System · System Design

**Roadmap**: [roadmap/badge.md](../../../roadmap/badge.md)
**Design**: [/app/ratel/assets/design/badge/](../../../app/ratel/assets/design/badge/)
**Author / Date**: doyooon · 2026-04-24

## Summary

Ship a non-transferable badge system that recognises civic participation across the whole platform. The catalog is code-resident (Rust enum + static array). Award detection runs asynchronously over DynamoDB Streams → EventBridge into a single `BadgeEvaluator` service. The Trophy Vault page (`/badges`) shows the entire catalog with progress; the Trophy Case strip on each profile shows up to 6 most-recently earned badges. Badge awards emit one inbox notification to the earner via the existing `UserInboxNotification` flow.

## Scope

### In scope (Phase 1)

- New feature module `app/ratel/src/features/badges/` (top-level, sibling to `notifications`).
- Two new entities: `UserBadge` (awarded ledger) and `UserBadgeProgress` (denormalised counters).
- Static catalog with **all 22 badges from the design**, of which 3 (Marathoner, Signal Boost, Beta Pilot) ship with a `BadgeTrigger::ComingSoon` placeholder — they render in the vault as permanently locked with a "Coming Soon" affordance, no progress bar (see [Coming Soon affordance](#coming-soon-affordance)).
- 4 new server functions: `list_catalog`, `list_my_earned`, `get_my_progress`, `list_user_earned`.
- 2 new routes: `BadgeVaultPage { }` and `UserBadgeVaultPage { username }`.
- `UseBadges` controller hook with category filter signal.
- Trophy Case strip component embedded into the existing profile page.
- New `InboxKind::BadgeAwarded` variant + corresponding `InboxPayload` branch.
- `BadgeEvaluator` service that listens to existing entity streams and awards badges atomically.

### Out of scope (deferred to Phase 2)

- **Marathoner data layer** — daily-activity entity + streak reset semantics. Phase 1 ships the visual placeholder only.
- **Signal Boost data layer** — Hot-list event emission. Phase 1 ships the visual placeholder only.
- **Beta Pilot data layer** — `Feedback` entity + submission UI. Phase 1 ships the visual placeholder only.
- **Backfill** of historical activity for existing users at launch — separate one-shot migration tracked outside this doc.
- User-defined badges, leaderboards, per-team scopes, secondary market — all explicit non-goals in the spec.

## Data model

### New entity: `UserBadge`

`features/badges/models/user_badge.rs`

```rust
#[derive(DynamoEntity, Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[dynamo(prefix = "UB")]
pub struct UserBadge {
    pub pk: Partition,           // USER#{user_id}
    pub sk: EntityType,          // USER_BADGE#{badge_id}     ← idempotency key

    pub badge_id: BadgeId,

    // GSI1: list a user's awards in reverse chronological order.
    // Backs the 6-up Trophy Case strip and "View all" earned-only listing.
    #[dynamo(index = "gsi1", sk)]
    pub awarded_at: i64,
    #[dynamo(prefix = "UB", name = "find_by_user_recent", index = "gsi1", pk)]
    pub user_pk: Partition,
}
```

- Main-table read `UserBadge::get(user_pk, USER_BADGE#{badge_id})` is the **idempotency check** — `attribute_not_exists(sk)` conditional write guarantees `BadgeEvaluator` cannot award the same badge twice even under concurrent stream events.
- GSI1 query `find_by_user_recent` powers profile / Trophy Vault listings.

### New entity: `UserBadgeProgress`

`features/badges/models/user_badge_progress.rs`

```rust
#[derive(DynamoEntity, Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
#[dynamo(prefix = "UBP")]
pub struct UserBadgeProgress {
    pub pk: Partition,           // USER#{user_id}
    pub sk: EntityType,          // USER_BADGE_PROGRESS         ← singleton per user

    pub spaces_joined: i64,
    pub spaces_created_ongoing: i64,
    pub poll_votes_cast: i64,
    pub discussion_comments: i64,
    pub quizzes_perfect: i64,
    pub follow_quests_completed: i64,
    pub prereq_passed_within_1h: i64,
    pub fast_joins_within_1h: i64,
    pub rewards_accumulated_cr: i64,

    pub updated_at: i64,
}
```

- One row per user, named-field counters (atomic `ADD` updates per field — no map-nested attribute paths).
- Adding a new badge that needs a new counter is a schema change to this struct + a new EventBridge rule. Acceptable churn for Phase 1; revisit if the catalog explodes.
- `followers_count` and `followings_count` already exist on `User` — those badges read from `User`, not `UserBadgeProgress`.

### `EntityType` additions

```rust
// common/types/entity_type.rs
pub enum EntityType {
    ...,
    UserBadge(String),         // sk = USER_BADGE#{badge_id}
    UserBadgeProgress,         // sk = USER_BADGE_PROGRESS (singleton)
}
```

`pk` for both entities is the existing `Partition::User(user_id)` variant — co-located with `User`, `UserInboxNotification`, `UserReward`, etc. No new `Partition` variants are introduced; if a future phase needs cross-user lookup we add them then.

### `InboxKind` / `InboxPayload` additions

```rust
// common/types/inbox_kind.rs
pub enum InboxKind {
    ...,
    BadgeAwarded,
}

pub enum InboxPayload {
    ...,
    BadgeAwarded {
        badge_id: BadgeId,
        badge_name: String,
        badge_rarity: BadgeRarity,
        badge_category: BadgeCategory,
        criterion_text: String,
    },
}

impl InboxPayload {
    pub fn url(&self) -> &str {
        match self {
            ...,
            Self::BadgeAwarded { .. } => "/badges",
        }
    }
}
```

## Catalog representation

`features/badges/types/catalog.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, strum::Display, strum::EnumString)]
pub enum BadgeId {
    // Participation
    FirstSteps, ActiveVoice, Kingmaker, Legislator, PrereqMaster, Deliberator,
    // Creator
    Curator, Architect, QuestDesigner, Foundation,
    // Social
    Connector, CommunityPillar, VoiceOfReason, MindMeld,
    // Achievement
    Scholar, RewardHunter, TrailBlazer, Marathoner,
    // Special
    FoundingVoice, EarlyBird, SignalBoost, BetaPilot,
}

pub enum BadgeCategory { Participation, Creator, Social, Achievement, Special }
pub enum BadgeRarity { Legendary, Rare, Common }

pub enum BadgeTrigger {
    SpacesJoined { count: i64 },
    PollVotesCast { count: i64 },
    SpacesCreatedOngoing { count: i64 },
    SpaceParticipantsAtLeast { count: i64 },     // Foundation: any one space ≥ N
    QuestActionsAllFour,                         // QuestDesigner
    Followers { count: i64 },                    // reads User.followers_count
    Followings { count: i64 },                   // reads User.followings_count
    DiscussionComments { count: i64 },
    QuizzesPerfect { count: i64 },
    FollowQuestsCompleted { count: i64 },
    PrereqPassedWithin1h { count: i64 },
    JoinedWithinWindow { from: i64, to: i64 },   // FoundingVoice
    FastJoinsWithin1h { count: i64 },            // EarlyBird
    RewardsAccumulatedCr { cr: i64 },            // RewardHunter

    /// Placeholder for badges whose data source is not yet available.
    /// `BadgeEvaluator` skips these; the UI renders them as locked with a
    /// "Coming Soon" affordance. Used by Marathoner / SignalBoost / BetaPilot in Phase 1.
    ComingSoon,
}

pub struct BadgeDef {
    pub id: BadgeId,
    pub name: &'static str,
    pub category: BadgeCategory,
    pub rarity: BadgeRarity,
    pub criterion_text: &'static str,
    pub icon: BadgeIcon,           // enum mapping to a lucide / custom icon
    pub trigger: BadgeTrigger,
}

pub static BADGE_CATALOG: &[BadgeDef] = &[
    BadgeDef { id: BadgeId::FirstSteps, name: "First Steps", category: BadgeCategory::Participation, rarity: BadgeRarity::Common,
        criterion_text: "Join your first space and complete a prerequisite.",
        icon: BadgeIcon::CheckCircle, trigger: BadgeTrigger::SpacesJoined { count: 1 } },
    // ... 18 more active entries, names & criteria copied verbatim from badges.html

    // Coming Soon — visual only in Phase 1 (data layer deferred to Phase 2)
    BadgeDef { id: BadgeId::Marathoner, name: "Marathoner", category: BadgeCategory::Achievement, rarity: BadgeRarity::Legendary,
        criterion_text: "Participate in at least one space every day for 30 days.",
        icon: BadgeIcon::Lightning, trigger: BadgeTrigger::ComingSoon },
    BadgeDef { id: BadgeId::SignalBoost, name: "Signal Boost", category: BadgeCategory::Special, rarity: BadgeRarity::Legendary,
        criterion_text: "Have one of your posts featured on the home arena Hot list.",
        icon: BadgeIcon::Megaphone, trigger: BadgeTrigger::ComingSoon },
    BadgeDef { id: BadgeId::BetaPilot, name: "Beta Pilot", category: BadgeCategory::Special, rarity: BadgeRarity::Rare,
        criterion_text: "Submit 3 bug reports or feedback items during mainnet beta.",
        icon: BadgeIcon::CheckCircle, trigger: BadgeTrigger::ComingSoon },
];
```

### Coming Soon affordance

Badges with `BadgeTrigger::ComingSoon` are skipped by `BadgeEvaluator` (no progress tracked, no awards possible). The Trophy Vault renders them in the locked style with these differences from a normal locked badge:

- The progress bar is replaced by a small `COMING SOON` chip in the meta line position.
- The hover tooltip appends `· Available in a future release.` to the criterion text.
- They participate in their category count (so Special still shows `5 / 5` in catalog totals) but are excluded from any "earned by N users" stat.
- They never appear in the Trophy Case strip on the profile (which lists only earned badges).

The catalog is the **single source of truth**. The frontend fetches it via `GET /api/badges/catalog` (server-rendered from `BADGE_CATALOG`) so we never duplicate badge metadata in TypeScript-style fixtures.

## Award flow (DynamoDB Stream → EventBridge → Lambda)

### Architecture

```
Entity INSERT/MODIFY (e.g. SpaceParticipant)
  → CDK Pipe filter (sk prefix)
  → EventBridge with DetailType::BadgeProgress{Source}
  → Rule routes to app-shell Lambda
  → EventBridgeEnvelope::proc()
  → BadgeEvaluator::on_progress(user_pk, counter_kind, delta)
      ├ atomic UPDATE UserBadgeProgress ADD counter delta  (returns ALL_NEW)
      ├ for each badge with this trigger:
      │   if old < threshold && new >= threshold:
      │     UserBadge.create_if_absent(user_pk, badge_id)   ← conditional write
      │     UserInboxNotification.send(user_pk, BadgeAwarded {...})
      └ done
```

### Stream sources (Phase 1)

| Source entity (sk prefix) | Counter incremented | Badges that listen |
|---|---|---|
| `SP#` SpaceParticipant INSERT | `spaces_joined` | FirstSteps, ActiveVoice, Legislator, Deliberator |
| `SPA#` SpacePollUserAnswer INSERT | `poll_votes_cast` | Kingmaker |
| `DC#` Discussion comment INSERT | `discussion_comments` | VoiceOfReason, MindMeld |
| `SQA#` SpaceQuizAttempt MODIFY (score=100) | `quizzes_perfect` | Scholar |
| `UF#` UserFollow INSERT | (read `User.followers_count` directly) | Connector, CommunityPillar |
| Space MODIFY (status → Ongoing) | `spaces_created_ongoing` | Curator, Architect |
| Space MODIFY (status → Finish) | (paginated `SpaceParticipant::find_by_space` COUNT for `Foundation`) | Foundation |
| `UR#` UserReward INSERT | `rewards_accumulated_cr` | RewardHunter |
| `SP#` INSERT with `informed_agreed=true` within 1h of join | `prereq_passed_within_1h` | PrereqMaster |
| `SP#` INSERT with `created_at - space.published_at < 1h` | `fast_joins_within_1h` | EarlyBird |

Time-window badges (FoundingVoice) and the QuestDesigner all-four-types check evaluate at the same trigger points but read additional context (User.created_at; SpaceAction count of types per space).

### `DetailType` additions

```rust
pub enum DetailType {
    ...,
    BadgeProgressSpaceJoined,
    BadgeProgressPollVoted,
    BadgeProgressDiscussionComment,
    BadgeProgressQuizPerfect,
    BadgeProgressUserFollow,
    BadgeProgressSpaceStatusChange,
    BadgeProgressSpaceFinish,
    BadgeProgressUserReward,
}
```

Each rule routes to the same Lambda. `EventBridgeEnvelope::proc()` dispatches into `BadgeEvaluator::on_event(...)` with the parsed entity.

### Idempotency

- `UserBadge` write uses `condition_expression = "attribute_not_exists(sk)"`. Concurrent stream events for the same threshold crossing → exactly one write succeeds, others get `ConditionalCheckFailed` and silently no-op.
- `UserInboxNotification` is only emitted on the successful path, so we never spam.
- `UserBadgeProgress` updates use atomic `ADD` — duplicate-delivery (EventBridge at-least-once) inflates counters. Mitigation: use the source entity's `pk + sk` as a deduplication key in a small TTL table, OR accept the rare over-count for Phase 1 (a user can earn one extra "10 spaces joined" if a stream record duplicates — they'd just earn the badge a few entries earlier than reality). **Decision: accept for Phase 1**, revisit if user reports inflated counters.

### Local-dev parity

Each new branch is mirrored in `common/stream_handler.rs` so Docker/local-dev runs the same evaluation without EventBridge. Pattern matches existing `NOTIFICATION#` branch.

## API surface

All routes are server functions in `features/badges/controllers/`.

| Route | Method | Auth | Returns |
|---|---|---|---|
| `/api/badges/catalog` | GET | none (public catalog) | `Vec<BadgeCatalogResponse>` |
| `/api/badges/me/earned` | GET | logged-in | `ListResponse<UserBadgeResponse>` (recent first) |
| `/api/badges/me/progress` | GET | logged-in | `UserBadgeProgressResponse` |
| `/api/users/{user_id}/badges/earned` | GET | none (public) | `ListResponse<UserBadgeResponse>` |

Path param `user_id: UserPartition` (SubPartition — clients pass just the id, no `USER#` prefix).

`UserBadgeProgressResponse` is intentionally **only** exposed on `/me/...` — never for other users (privacy constraint in spec).

`BadgeCatalogResponse` includes the full `BadgeDef` data plus a server-side `current_count: i64` of how many users have earned it (computed once at boot; cached). Phase 1 may ship without `current_count` and add it later if useful for UI.

## Frontend architecture

### Module layout

```
features/badges/
├── mod.rs
├── i18n.rs
├── controllers/      ← server functions
├── models/           ← UserBadge, UserBadgeProgress
├── types/
│   ├── catalog.rs    ← BadgeId, BadgeDef, BADGE_CATALOG, BadgeTrigger
│   ├── icon.rs       ← BadgeIcon → lucide_dioxus mapping
│   ├── error.rs
│   └── response.rs
├── services/
│   └── badge_evaluator.rs   ← runs in Lambda + stream_handler
├── hooks/
│   └── use_badges.rs        ← UseBadges controller
└── views/
    ├── trophy_vault/        ← Route::BadgeVaultPage page
    │   ├── component.rs, style.css, page.html, i18n.rs
    │   ├── filter_bar/      ← sub-component
    │   ├── badge_medallion/ ← reused for own-vault and other-user-vault
    │   └── progress_ring/
    └── trophy_case_strip/   ← embedded in profile page
        ├── component.rs, style.css
        └── badge_mini/      ← compact medallion
```

### `UseBadges` controller hook

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct UseBadges {
    pub catalog: Loader<Vec<BadgeCatalogResponse>>,
    pub my_earned: Loader<Vec<UserBadgeResponse>>,
    pub my_progress: Loader<UserBadgeProgressResponse>,
    pub category_filter: Signal<Option<BadgeCategory>>,
}

#[track_caller]
pub fn use_badges() -> Result<UseBadges, RenderError> { /* try_use_context + provide_root_context */ }
```

For viewing **another user's** vault we use a separate, simpler hook `use_user_badges(user_id: ReadSignal<UserPartition>)` returning only `earned` (no progress, no catalog rebuild needed — the catalog hook is shared).

### Routes

```rust
// route.rs
#[layout(RootLayout)]
    #[route("/badges")]
    BadgeVaultPage { },

#[nest("/:username")]
    #[route("/badges")]
    UserBadgeVaultPage { username: String },
```

### Profile integration

`features/users/views/profile/component.rs` (or wherever the profile page lives — to be confirmed during scaffolding) renders `TrophyCaseStrip { user_id }` between the stats grid and the activity feed. `TrophyCaseStrip` consumes the appropriate hook based on whether `user_id == current_user`.

### Activity feed integration

The user-facing activity feed already renders inbox notifications — no new code needed. The new `BadgeAwarded` `InboxPayload` variant is rendered with the rose-coloured icon + the headline / subtitle defined in [Functional requirement 18](../../../roadmap/badge.md#notification--activity-feed) (see notification panel component).

## Test plan

### Server function integration tests — `app/ratel/src/tests/badge_tests.rs`

- `test_catalog_lists_all_active_badges`
- `test_my_earned_empty_for_new_user`
- `test_my_progress_zero_for_new_user`
- `test_other_user_earned_excludes_progress` — `/api/users/{id}/badges/earned` responds 200; no endpoint exists at `/api/users/{id}/badges/progress`
- `test_evaluator_awards_first_steps_on_first_join` — direct call to `BadgeEvaluator::on_event(SpaceParticipant INSERT)`, assert `UserBadge` row + 1 inbox notification
- `test_evaluator_idempotent_on_duplicate_event` — calling the evaluator twice for the same event yields one badge, one notification
- `test_evaluator_does_not_revoke_on_decrement` — even if `UserBadgeProgress.spaces_joined` is manually decremented below threshold, the badge remains
- `test_founding_voice_within_window` and `test_founding_voice_outside_window`

### Playwright — extend or create

- New spec `playwright/tests/web/badges.spec.js`:
  - `Step 1: profile shows empty Trophy Case for a brand-new user`
  - `Step 2: vote in a poll, verify Trophy Case shows the awarded badge after navigation back to profile`
  - `Step 3: navigate to /badges, verify category filter narrows the grid`
  - `Step 4: hover a locked badge, verify criterion tooltip text`
  - `Step 5: visit another user's /:username/badges page, verify only earned badges render and no progress bar appears`

## Open questions / risks

### Carried over from spec

- **Streak counting (Marathoner)** — deferred from Phase 1. Need a `UserActivityDay` entity or a daily heartbeat job before this badge is implementable.
- **Hot-list event (Signal Boost)** — deferred. Awaiting Hot-list to expose `DetailType::PostFeaturedOnHot`.
- **Feedback entity (Beta Pilot)** — deferred. Requires either a `Feedback` model + submission UI, or a manual ingest from Slack / GitHub.
- **Backfill at launch** — handled by separate one-shot migration outside this doc; the migration calls `BadgeEvaluator::backfill(user)` for each existing user, which reads existing entity counts (one-time COUNT) into `UserBadgeProgress` and triggers any badges already earned.

### New (Stage 3)

- **Foundation badge cost** — counting `SP#{space_pk}` rows on space-finish is O(participants). Phase 1 ships the simple paginated COUNT (a 500-participant space resolves in one page; even 50k pages in <5s on Lambda). **Refactor trigger**: when a space exceeds ~5k participants in production, snapshot `participant_count` onto `Space` and read from there.
- **Counter inflation on at-least-once delivery** — accepted for Phase 1 (see [Idempotency](#idempotency)). EventBridge / DynamoDB Streams guarantee at-least-once, so duplicate events occasionally inflate `UserBadgeProgress` counters. Worst-case effect: a user earns a counter-based badge a few activities earlier than reality. No incorrect badges are awarded; thresholds simply trigger slightly early. **Refactor trigger**: if users report visibly inflated counts, add a small dedup table keyed by source-entity pk+sk with TTL = 24h.
- **`BadgeIcon` mapping** — the design uses ~20 distinct lucide icons. Catalog stores a `BadgeIcon` enum; the frontend maps each variant to a `lucide_dioxus::*` component. Adding a new badge that needs a new icon requires adding both an enum variant and the render mapping — flag this in code review if it becomes a friction point.

## References

- [roadmap/badge.md](../../../roadmap/badge.md) — Stage 1 spec
- [/app/ratel/assets/design/badge/badges.html](../../../app/ratel/assets/design/badge/badges.html) — Trophy Vault visual contract
- [/app/ratel/assets/design/badge/profile.html](../../../app/ratel/assets/design/badge/profile.html) — Trophy Case strip visual contract
- [.claude/rules/conventions/implementing-event-bridge.md](../../../.claude/rules/conventions/implementing-event-bridge.md) — Pipe + Rule + handler pattern
- [.claude/rules/conventions/hooks-and-actions.md](../../../.claude/rules/conventions/hooks-and-actions.md) — `UseBadges` controller pattern
- Notification reference implementation: `app/ratel/src/features/notifications/hooks/use_inbox.rs`
