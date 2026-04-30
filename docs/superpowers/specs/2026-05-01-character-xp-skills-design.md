# Character XP & Skill Tree — Brainstorming Design

**Roadmap**: [`roadmap/character-xp-skills.md`](../../../roadmap/character-xp-skills.md)
**Date**: 2026-05-01
**Author**: hackartist
**Status**: Pending PO review (Stage 1 brainstorming output, pre-Stage 2)

---

## 1. Summary

Add an account-level XP/Level/Skill-Point progression layer on top of the existing per-space `SpaceActivity` → `SpaceScore` pipeline. Users gain Character XP whenever a `SpaceScore` row updates (delta-based, idempotent), level up on a fixed cubic curve, and spend skill points on a small skill tree of passive economic boosts.

MVP ships **two skills** (Money Tree, Ranker — both participant-side multipliers). Two more (Influencer, Sweeper — creator-side) are designed but explicitly deferred so the data model accommodates them without rework.

## 2. Architecture overview

```
SpaceActivity INSERT (existing)
    └─> stream_handler::SPACE_ACTIVITY#  (existing)
        └─> aggregate_score()           (existing — writes/updates SpaceScore)

SpaceScore MODIFY (new dispatch branch)
    └─> stream_handler::SPACE_SCORE#    (new branch in same file)
        └─> apply_character_xp_delta()  (new service)
              ├─ read CharacterXp.last_seen_score[(user, space)]
              ├─ compute delta = new_total_score − last_seen
              ├─ if delta == 0: skip (idempotent under stream replay)
              ├─ if delta > 0:
              │    ├─ CharacterXp.total_xp += delta
              │    ├─ recompute Level, total SP grant
              │    └─ on level-up: emit InboxNotification
              └─ persist last_seen_score[(user, space)] = new_total_score

SpaceReward::award (existing)
    └─ before recording amount → multiply by user's Money Tree skill level
       boost (1 + 0.05 × level)

SpaceActivity::new (existing — called from XP event handlers)
    └─ before computing total_score → multiply additional_score by user's
       Ranker skill level boost (1 + 0.05 × level)
```

Key shape: **stream-driven for XP, read-time for skill effects**. XP is materialized on every score change so the `/me/character` page is a single point read. Skill effects are applied at the *write site of the affected economic event* (reward payout, activity insert), not retrofitted at read time, so existing `SpaceScore` / `User.points` numbers stay authoritative.

## 3. Data model

### 3.1 New entities

```rust
// app/ratel/src/features/character/models/character_xp.rs
#[derive(DynamoEntity, ...)]
pub struct CharacterXp {
    pub pk: Partition,           // Partition::User(user_id)
    pub sk: EntityType,          // EntityType::CharacterXp

    pub created_at: i64,
    pub updated_at: i64,

    pub total_xp: i64,           // monotonic
    pub level: i32,              // derived, denormalized for fast read
    pub total_sp_granted: i32,   // 5 + (level-1), monotonic
    pub total_sp_spent: i32,     // sum of skill costs paid
    // unspent_sp = total_sp_granted - total_sp_spent (computed, not stored)
}
```

```rust
// app/ratel/src/features/character/models/character_xp_source.rs
//
// Per-(user, space) last-seen marker for idempotent delta computation.
// One row per space the user has activity in.
#[derive(DynamoEntity, ...)]
pub struct CharacterXpSource {
    pub pk: Partition,                          // Partition::User(user_id)
    pub sk: EntityType,                         // EntityType::CharacterXpSource(space_id)

    pub last_seen_score: i64,                   // SpaceScore.total_score at last apply
    pub updated_at: i64,
}
```

```rust
// app/ratel/src/features/character/models/character_skill.rs
//
// One row per (user, skill_id) — only created the first time a user spends
// a point on the skill. Absence = level 0.
#[derive(DynamoEntity, ...)]
pub struct CharacterSkill {
    pub pk: Partition,                          // Partition::User(user_id)
    pub sk: EntityType,                         // EntityType::CharacterSkill(skill_id)

    pub level: i32,                             // 0..10
    pub created_at: i64,
    pub updated_at: i64,
}

// Stable string IDs — adding Influencer/Sweeper later is purely additive.
pub enum SkillId {
    MoneyTree,    // "money_tree"
    Ranker,       // "ranker"
    Influencer,   // "influencer"  (v2)
    Sweeper,      // "sweeper"     (v2)
}
```

### 3.2 New `EntityType` variants

```rust
// app/ratel/src/common/types/entity_type.rs
CharacterXp,
CharacterXpSource(String),    // space_id (no prefix; SubPartition wraps SpacePartition)
CharacterSkill(String),       // skill_id ("money_tree", "ranker", ...)
```

### 3.3 GSI considerations

Character XP / Skills are always read by `(user_pk, *)` — pure pk-prefix lookups on the main table. **No new GSIs needed.** Profile-view lookups for "show me everyone's level" are explicitly out of scope (no leaderboard). If we later want a "top N by level" page, we can add a GSI then.

## 4. Leveling math

```
xp_required(L→L+1)     = round(C · L³)               where C = 100
total_xp_at_level(L)   = C · (L−1)² · L² / 4         (closed form)
sp_granted_at_level(L) = L                           (1 SP per level)

skill_cost(n→n+1)      = 5 + n                       (5, 6, 7, ..., 14)
total_to_max(skill)    = sum of skill_cost(0..9)     = 5+6+...+14 = 95
```

`C = 100` is calibrated against the observed 10-day activity window (avg participant ≈ 360k SpaceXP, top participants ≈ 650k). The original draft used `C = 50`, which put the avg participant at L20 in ~7 weeks — too fast for the new SP=1/level grant rate. See Q7 / Q8 in the roadmap spec.

**Worked numbers** (`C = 100`, SP = `L`, avg participant earns ≈ 36k XP/day):

| Char Level | Cumulative XP | Time (avg) | Total SP | Affordable skill build |
|---|---|---|---|---|
| 1  | 0          | day 0    | 1   | — |
| 5  | 10,000     | ~7h      | 5   | one skill at L1 (5 SP) |
| 10 | 202,500    | ~5d      | 10  | both skills at L1 (10 SP) |
| 16 | 843,750    | ~3.5w    | 16  | MoneyTree L2 + Ranker L1 (5+6+5 = 16 SP) |
| 23 | 3,025,000  | ~3 mo    | 23  | both skills at L2 (5+6+5+6 = 22 SP, 1 spare) |
| 33 | 14,191,875 | ~13 mo   | 33  | one skill at L5 (5+6+7+8+9 = 35 SP — close, needs L34) |
| 95 | 1,989,061k | many yr  | 95  | one skill maxed at L10 (95 SP) |

Tuning levers (single constants in `app/ratel/src/features/character/leveling.rs`):
- `C` (XP curve steepness) — dial after first-week telemetry. `C = 75` is a faster early game, `C = 150` is a slower one.
- SP-per-level (currently `1`) — dial if early gating feels too tight.
- Per-skill `max_level` (currently `10`) and triangular base (`5`) — dial if Money Tree at +50% caps overpowered.

## 5. Event flow detail

### 5.1 XP delta on stream

`stream_handler.rs` already dispatches on sk prefix. Add a branch for `SPACE_SCORE#` MODIFY events:

```rust
} else if sk.starts_with("SPACE_SCORE#") {
    let score: SpaceScore = deserialize(image)?;
    let old_total = old_image.and_then(|img| /* extract total_score */).unwrap_or(0);
    if let Err(e) = crate::features::character::services::apply_character_xp_delta(score, old_total).await {
        tracing::error!(error = %e, "stream: CharacterXpDelta failed");
    }
}
```

Inside `apply_character_xp_delta`:

1. Read `CharacterXpSource` for `(user_pk, space_pk)`. If absent, treat `last_seen = 0`.
2. Compute `delta = new_total − last_seen`. If `delta == 0`, return early (replay safety).
3. If `delta > 0`, transactional update:
   - `CharacterXp.total_xp += delta` (or insert with `total_xp = delta` if first ever).
   - Recompute `level`, `total_sp_granted` from new total_xp.
   - Persist new `level` / `total_sp_granted`.
   - `CharacterXpSource.last_seen_score = new_total`.
4. If level changed, enqueue an `InboxNotification` ("You leveled up to L7! +1 SP").
5. If `delta < 0`: log warning and skip (Open Question 3 → monotonic). Still update `last_seen_score` to the new value to avoid getting stuck.

**Idempotency contract**: re-processing the same MODIFY event sees `last_seen == new_total` and short-circuits. EventBridge's at-least-once delivery is safe.

### 5.2 Money Tree at reward payout

`SpaceReward::award` (`app/ratel/src/features/spaces/space_common/models/space_reward.rs:167`) currently computes `let amount = space_reward.get_amount();`. Wrap that:

```rust
let raw_amount = space_reward.get_amount();
let multiplier = character_skill::money_tree_multiplier(cli, &target_pk).await;
let amount = (raw_amount as f64 * multiplier).round() as i64;
let bonus  = amount - raw_amount;
// record `amount` against UserReward.total_points and User.points
// record `bonus` in UserRewardHistory.metadata for the breakdown UI
```

`money_tree_multiplier(user_pk)` = `1.0 + 0.05 × CharacterSkill::level_or_zero(user_pk, MoneyTree)`. Single point read, cached per request via `tokio::sync::OnceCell` if needed.

Owner bonus payout (creator's 10% cut) is **not** boosted — Money Tree only multiplies the participant's primary payout (req 14).

### 5.3 Ranker at activity insert

`SpaceActivity::new_with_dedup` currently does `let total_score = base_score + additional_score;`. Wrap:

```rust
let multiplier = character_skill::ranker_multiplier(cli, &author).await;
let boosted_additional = (additional_score as f64 * multiplier).round() as i64;
let total_score = base_score + boosted_additional;
```

Stored `additional_score` reflects the boosted value so audit trails show what was actually credited.

### 5.4 Skill point spend

New endpoint:

```
POST /api/me/skills/:skill_id/level-up
```

Handler logic:
1. Read `CharacterXp` for user; compute `unspent = total_sp_granted - total_sp_spent`.
2. Read `CharacterSkill` for `(user, skill_id)`; treat absent as level 0.
3. Compute `cost = 5 + current_level`.
4. Reject if `unspent < cost` or `current_level >= 10`.
5. Transactional update:
   - `CharacterSkill.level = current_level + 1`
   - `CharacterXp.total_sp_spent += cost`
6. Return new state.

### 5.5 Backfill

Admin migration at `app/ratel/src/features/admin/controllers/migrations/backfill_character_xp.rs`:

1. Scan GSI on `SpaceScore` (existing `find_by_space_rank` index).
2. Group by `user_pk`; sum `total_score`.
3. For each user: upsert `CharacterXp { total_xp = sum, level = derive(sum), total_sp_granted = derive(level), total_sp_spent = 0 }` and write a `CharacterXpSource` row per space the user appeared in with `last_seen_score = SpaceScore.total_score`.
4. Idempotent: re-running computes the same XP and the same `last_seen_score`, so no further deltas accumulate post-backfill.

## 6. API surface

| Method | Path | Auth | Purpose |
|---|---|---|---|
| GET | `/api/me/character` | User session | Returns `CharacterXp` + all `CharacterSkill` rows + computed `unspent_sp` and `xp_to_next_level` |
| GET | `/api/users/:username/character` | Public (or User session) | Public view: returns level only (Q5 = yes) |
| POST | `/api/me/skills/:skill_id/level-up` | User session | Spends SP to advance skill |
| POST | `/api/admin/migrations/backfill-character-xp` | Admin | One-time backfill |

All path params use SubPartition types per `conventions/server-functions.md`.

Response DTO sketch:

```rust
pub struct CharacterResponse {
    pub total_xp: i64,
    pub level: i32,
    pub xp_to_next_level: i64,    // computed
    pub xp_progress_in_level: i64, // computed
    pub unspent_sp: i32,
    pub skills: Vec<CharacterSkillResponse>,
}

pub struct CharacterSkillResponse {
    pub skill_id: SkillId,
    pub level: i32,
    pub max_level: i32,           // 10
    pub next_level_cost: Option<i32>,  // None if maxed
    pub multiplier_pct: i32,      // for display: "+25%" at level 5
}
```

## 7. Frontend architecture

### 7.1 Route + page

Per Q2 (recommended: tab on existing profile), add:
- Route: `/me/character` (self) + `/<username>/character` (visitor view) under existing `features/social/pages/character/`.
- Tab on existing profile page next to "Posts" / "Spaces" / "Rewards".

### 7.2 Hook

`UseCharacter` controller in `features/social/pages/character/hooks/use_character.rs`:

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct UseCharacter {
    pub character: Loader<CharacterResponse>,
    pub level_up: Action<(SkillId,), CharacterResponse>,
}
```

`level_up` is the `use_action` shape because the UI binds to `.pending()` to disable the button mid-spend.

### 7.3 Components

- `CharacterHeader` — XP bar, level badge, unspent SP pill.
- `SkillTree` — grid of `SkillCard`.
- `SkillCard` — name, description, level pips (10 dots), "Next +5%" cost, "Level Up" button.
- `RewardBreakdownChip` — shown inside existing reward claim UI (`features/social/pages/user_reward/`) when Money Tree level > 0.

UI mockups in HTML/CSS are produced in **Stage 2** (`app/ratel/assets/design/character-xp-skills/`) — out of scope for this brainstorming doc.

## 8. Test plan

### 8.1 Server-function integration tests (`app/ratel/src/tests/character_tests.rs`)

| Case | Asserts |
|---|---|
| `test_xp_increments_on_space_score_modify` | After insert + modify of `SpaceActivity`, `CharacterXp.total_xp == new SpaceScore.total_score` |
| `test_xp_replay_idempotent` | Re-applying same `SpaceScore` MODIFY does not double-count |
| `test_level_up_grants_sp` | Crossing level 2 threshold sets `total_sp_granted = 6` |
| `test_skill_level_up_success` | Spending 5 SP on Money Tree raises level to 1, decrements unspent |
| `test_skill_level_up_insufficient_sp` | Reject when unspent < cost |
| `test_skill_level_up_max_level` | Reject at level 10 |
| `test_money_tree_boost_applied_to_payout` | After level 1, `UserReward.total_points` reflects +5% |
| `test_ranker_boost_applied_to_activity` | After level 1, new `SpaceActivity.total_score` = `base + additional × 1.05` |
| `test_get_character_unauth_self_route_rejected` | `/api/me/character` requires session |
| `test_get_character_public_route_returns_level_only` | `/api/users/:username/character` omits SP and skill build |
| `test_backfill_idempotent` | Running backfill twice produces same `CharacterXp.total_xp` |

### 8.2 Playwright E2E (`playwright/tests/web/character-progression.spec.js`)

Extend existing space-flow scenario:
- After voting in a poll, navigate to `/me/character`, assert XP > 0 and level ≥ 1.
- Spend SP on Money Tree, return to space, vote in another poll, claim reward, assert claim shows boosted amount.

## 9. Risks & open questions

(All Q1–Q6 in `roadmap/character-xp-skills.md` are restated there with recommendations. Below are implementation-side risks.)

- **Stream replay window.** EventBridge can deliver up to ~24h late. The `last_seen_score` marker handles correctness but log "stale delta applied" if `now - SpaceScore.updated_at > 1h` for visibility.
- **Backfill cost.** A scan of `SpaceScore` is bounded by the number of (user, space) pairs. At current volume this is O(thousands), trivially safe — but the migration must page through GSI (`find_by_space`) instead of full table scan.
- **Race between level-up SP grant and SP spend.** If a user is at level 1 with 5 SP, spends 5 SP, then a stream event grants level 2 (+1 SP), they should see 1 unspent. Solution: `total_sp_granted` and `total_sp_spent` are both monotonic counters, computed independently; `unspent = granted - spent` is the read-time derivation, race-free.
- **Reward breakdown UI churn.** Existing user_reward views already render `UserRewardHistory.metadata`; adding a `money_tree_bonus` field is additive.

## 10. Build sequence (Stage 3 preview)

Roughly the order Stage 3 would execute, per `.claude/rules/workflows/develop-a-new-feature.md`:

1. EntityType variants + `CharacterXp` / `CharacterXpSource` / `CharacterSkill` models.
2. `apply_character_xp_delta` service + `stream_handler.rs` SPACE_SCORE# branch.
3. `level_up_handler` + `get_character_handler` + DTO.
4. Money Tree wrapping in `SpaceReward::award`.
5. Ranker wrapping in `SpaceActivity::new_with_dedup`.
6. Backfill admin migration.
7. `UseCharacter` controller hook.
8. RSX components from Stage 2 mockups.
9. Tests (server fn + Playwright).

## Decisions needing PO sign-off (summary)

| # | Question | Recommended | Spec already reflects? |
|---|----------|-------------|------------------------|
| Q1 | Ranker boost scope | `additional_score` only | yes |
| Q2 | Page route | profile tab + `/me/character` | yes |
| Q3 | XP monotonic | yes (no debit) | yes |
| Q4 | Skill respec | not in MVP | yes |
| Q5 | Public level visibility | yes, level only | yes |
| Q6 | Sweeper cap (v2) | +50% / 60% total | yes (v2) |
| Q7 | SP grant rate per level | **1 SP/level** (PO directive — long endgame) | yes |
| Q8 | XP curve `C` | **`C = 100`** (calibrated against 10-day data: avg 360k, top 650k) | yes |

If you override any of these, the spec change is one-line in `roadmap/character-xp-skills.md` and (for Q1, Q2, Q5) one paragraph in this doc.
