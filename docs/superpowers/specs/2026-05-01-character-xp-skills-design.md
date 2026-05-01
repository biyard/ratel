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
    pub total_sp_granted: i32,   // = level (1 SP per character level)
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
LastBackfillVersion,          // singleton, paired with Partition::Migration
```

### 3.3 Migration framework entity

```rust
// app/ratel/src/common/models/migration/last_backfill_version.rs
#[derive(DynamoEntity, ...)]
pub struct LastBackfillVersion {
    pub pk: Partition,           // Partition::Migration  (singleton)
    pub sk: EntityType,          // EntityType::LastBackfillVersion

    pub version: i64,            // last successfully completed migration's required_version
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl LastBackfillVersion {
    /// Atomic increment via conditional update — `version` only advances if the
    /// caller's `expected` matches the stored value. Prevents two replicas from
    /// double-running the same migration.
    pub async fn advance_to(
        cli: &aws_sdk_dynamodb::Client,
        expected: i64,
        new_version: i64,
    ) -> Result<()> { /* … conditional update on version == expected … */ }
}
```

### 3.4 GSI considerations

Character XP / Skills are always read by `(user_pk, *)` — pure pk-prefix lookups on the main table. **No new GSIs needed.** Profile-view lookups for "show me everyone's level" are explicitly out of scope (no leaderboard). If we later want a "top N by level" page, we can add a GSI then.

## 4. Leveling math

```
xp_required(L→L+1)     = round(C · L²)               where C = 220
total_xp_at_level(L)   = C · (L−1) · L · (2L−1) / 6  (closed form)
sp_granted_at_level(L) = L                           (1 SP per level)

skill_cost(n→n+1)      = 5 + n                       (5, 6, 7, 8, 9, 10)
total_to_max(skill)    = sum of skill_cost(0..5)     = 5+6+7+8+9+10 = 45
max_skill_level        = 6                           (cap multiplier +30% at L6)
```

The shape is **quadratic** (was cubic in earlier drafts). `C = 220` is calibrated to hit the PO target *"one skill maxable in 6 months for an avg participant"*: with max-skill at L45 and avg activity ≈ 36k XP/day, `cumulative_xp(45) ≈ 36k × 180 → C ≈ 220`. See Q8/Q9 in the roadmap spec.

**Worked numbers** (`C = 220`, quadratic curve, SP = `L`, avg ≈ 36k XP/day, top ≈ 65k XP/day):

| Char Level | Cumulative XP | Time (avg / top) | Total SP | Affordable skill build |
|---|---|---|---|---|
| L1  | 0          | day 0              | 1   | — (need 5 SP for first skill) |
| L5  | 6,600      | ~4 h / ~2 h        | 5   | one skill at L1 (5 SP) |
| L10 | 62,700     | ~1.7 d / ~1 d      | 10  | both skills at L1 (10 SP) |
| L11 | 84,700     | ~2.4 d / ~1.3 d    | 11  | one skill at L2 (5+6 = 11 SP) |
| L16 | 272,800    | ~7.6 d / ~4.2 d    | 16  | MoneyTree L2 + Ranker L1 (5+6+5 = 16 SP) |
| L18 | 392,700    | ~11 d / ~6 d       | 18  | one skill at L3 (5+6+7 = 18 SP) |
| L22 | 728,420    | ~20 d / ~11 d      | 22  | both skills at L2 (22 SP) |
| L26 | 1,215,500  | ~34 d / ~19 d      | 26  | one skill at L4 (26 SP) |
| L30 | 1,882,100  | ~52 d (~1.7 mo) / ~29 d | 30 | both at L3 + spare (36 SP needed — close, hits at L36) |
| L35 | 3,010,700  | ~84 d (~2.8 mo) / ~46 d | 35 | one skill at L5 (35 SP) |
| L36 | 3,280,200  | ~91 d (~3 mo) / ~50 d   | 36 | both skills at L3 (36 SP) ✓ |
| **L45** | **6,461,400** | **~180 d (6 mo) / ~99 d (~3.3 mo)** | **45** | **one skill maxed at L6 (45 SP)** ← MVP endgame |
| L50 | 8,893,500  | ~247 d (~8 mo) / ~137 d | 50  | one maxed + the other at L1 (45+5 = 50 SP) |
| L75 | 30,321,500 | ~2.3 yr / ~1.3 yr      | 75  | one maxed + other at L4 (45+26 = 71 SP), 4 spare |
| **L90** | **52,572,300** | **~4 yr / ~2.2 yr**   | **90** | **both skills maxed (45+45 = 90 SP)** ← true endgame |

Reading the table: an avg participant gets their first skill at L1 in ~4 hours, both skills at L1 by day 2, both at L2 in ~3 weeks, both at L3 in ~3 months, **one skill maxed at L6 in 6 months**. A top participant hits each milestone in roughly 55% of the avg-participant time, so they max one skill in ~3.3 months.

Tuning levers (single constants in `app/ratel/src/features/character/leveling.rs`):
- `C` (XP curve scale) — dial after first-week telemetry. `C = 150` makes one-skill-max ~4 months for avg; `C = 300` makes it ~8 months.
- Curve **shape** (quadratic vs. cubic) — bigger lever than `C`; cubic shuts off the late game.
- SP-per-level (currently `1`) — dial if early gating feels too tight.
- Per-skill `max_level` (currently `6`) and triangular base (`5`) — `max_level = 10` brings cap to +50% and cost-to-max to 95 SP, requiring `C` to drop to ~105 to keep the 6-month goal.

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

### 5.5 Backfill via versioned migration framework

Backfills are no longer admin endpoints; they're **versioned migrations** run automatically on server startup, gated by an `MIGRATE=true` env var.

**Layout**:

```
app/ratel/src/common/migrations/
├── mod.rs                            // run_migrations() entry point
├── runner.rs                         // version-gated dispatch
└── m001_backfill_character_xp.rs     // first migration (required_version = 1)
```

**Runner** (`runner.rs`):

```rust
pub async fn run_migrations(cli: &aws_sdk_dynamodb::Client) -> Result<()> {
    if std::env::var("MIGRATE").as_deref() != Ok("true") {
        tracing::info!("MIGRATE not set — skipping migrations");
        return Ok(());
    }

    let doc = LastBackfillVersion::get(cli, &Partition::Migration, Some(EntityType::LastBackfillVersion))
        .await?
        .unwrap_or_default();

    if doc.version < 1 {
        tracing::info!("running migration 001: backfill_character_xp");
        m001_backfill_character_xp::run(cli).await?;
        LastBackfillVersion::advance_to(cli, doc.version, 1).await?;
    }

    // Future migrations stack additively:
    // if doc.version < 2 { m002_xxx::run(cli).await?; LastBackfillVersion::advance_to(cli, 1, 2).await?; }

    Ok(())
}
```

**Wired** at server bootstrap in `app/ratel/src/main.rs` (or wherever `axum::serve` is set up), before starting the HTTP listener.

**Migration 001 — backfill_character_xp** (`m001_backfill_character_xp.rs`):

1. Scan `SpaceScore` via the existing `find_by_space_rank` GSI, paginated.
2. Group rows by `user_pk`; sum `total_score`.
3. For each user: upsert `CharacterXp { total_xp = sum, level = derive(sum), total_sp_granted = level, total_sp_spent = 0 }`. **Upsert, not increment** — re-running converges, never accumulates.
4. For each `(user, space)` pair seen: upsert `CharacterXpSource { last_seen_score = score.total_score }`.
5. `total_sp_spent = 0` is correct for backfilled users — no skill points have been spent yet because the spend endpoint didn't exist before this deploy.

**Idempotency contracts**:

- Within a single run that crashes mid-way: re-running re-reads `SpaceScore` and writes the same target state. No partial-progress markers.
- Across deploys: `LastBackfillVersion.version` advances only after the migration completes successfully. A crash mid-migration leaves `version` unchanged, so the next `MIGRATE=true` deploy re-runs from the start.
- Across replicas in the same release: the conditional `advance_to(expected, new)` ensures only one replica wins the version bump. The losing replica skips because `doc.version == 1` on its read after losing the race.

**Operational rule**: Set `MIGRATE=true` on **exactly one** instance per release (typically a one-shot Lambda or a single ECS task) to avoid contention on the scan. The conditional version bump is a safety net, not the primary contention guard.

## 6. API surface

| Method | Path | Auth | Purpose |
|---|---|---|---|
| GET | `/api/me/character` | User session | Returns `CharacterXp` + all `CharacterSkill` rows + computed `unspent_sp` and `xp_to_next_level` |
| GET | `/api/users/:username/character` | Public (or User session) | Public view: returns level only (Q5 = yes) |
| POST | `/api/me/skills/:skill_id/level-up` | User session | Spends SP to advance skill |

Backfill is **not** an HTTP endpoint — it runs at server startup under `MIGRATE=true`. See §5.5.

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
| `test_backfill_idempotent` | Running migration 001 twice produces same `CharacterXp.total_xp` |
| `test_migrate_unset_skips` | Server start with `MIGRATE` unset does not advance `LastBackfillVersion.version` |
| `test_migrate_already_at_version` | Server start with `MIGRATE=true` and `version >= 1` does not re-run migration 001 |
| `test_migrate_replica_race` | Two concurrent `advance_to(0, 1)` calls — exactly one succeeds, the other returns the conditional-update conflict |

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

1. EntityType variants + `CharacterXp` / `CharacterXpSource` / `CharacterSkill` / `LastBackfillVersion` models.
2. Migration framework: `common/migrations/runner.rs` + `m001_backfill_character_xp.rs`, wired at startup with `MIGRATE=true` gate.
3. `apply_character_xp_delta` service + `stream_handler.rs` SPACE_SCORE# branch.
4. `level_up_handler` + `get_character_handler` + DTO.
5. Money Tree wrapping in `SpaceReward::award`.
6. Ranker wrapping in `SpaceActivity::new_with_dedup`.
7. `UseCharacter` controller hook.
8. RSX components from Stage 2 mockups.
9. Tests (server fn + Playwright + migration replica race).

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
| Q8 | XP curve shape + `C` | **quadratic, `C = 220`** (PO directive — one-skill-max in 6 months for avg participant) | yes |
| Q9 | Max skill level | **6** (cost-to-max = 45 SP, multiplier cap +30%) | yes |

If you override any of these, the spec change is one-line in `roadmap/character-xp-skills.md` and (for Q1, Q2, Q5) one paragraph in this doc.
