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
    pub total_sp_granted: i32,   // = 5 · level (5 SP per character level)
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
sp_granted_at_level(L) = 5 · L                       (5 SP per char level)

skill_cost(n→n+1)      = 5 + 4·n                     (5, 9, 13, 17, 21, 25, 29, 33, 37, 41)
total_to_max(skill)    = sum of skill_cost(0..9)     = 5+9+13+...+41 = 230
max_skill_level        = 10                          (cap multiplier +50% / 1.5× at L10)
```

The shape is **quadratic** (was cubic in earlier drafts). The 6-month one-skill-max target is preserved by **the cost curve, not the level cap**: SP grant is 5/lv (Q7), max skill level is 10 (Q9), and `cost(n→n+1) = 5 + 4n` (Q9b). Total 230 SP → char L46 → ~6.4 mo avg / ~3.5 mo top under `C = 220`.

**Worked numbers** (`C = 220`, quadratic curve, SP = `5L`, avg ≈ 36k XP/day, top ≈ 65k XP/day):

| Char Level | Cumulative XP | Time (avg / top) | Total SP | Affordable skill build |
|---|---|---|---|---|
| L1  | 0          | day 0                  | 5    | one skill at L1 (5 SP) ← from day 0 |
| L2  | 220        | ~9 min / ~5 min        | 10   | both skills at L1 (5+5 = 10 SP) |
| L3  | 1,100      | ~45 min / ~25 min      | 15   | one skill at L2 (5+9 = 14 SP), 1 spare |
| L6  | 6,050      | ~4 h / ~2 h            | 30   | one skill at L3 (27 SP), 3 spare |
| L9  | 22,440     | ~15 h / ~8 h           | 45   | one skill at L4 (44 SP), 1 spare |
| L13 | 71,500     | ~2 d / ~1.1 d          | 65   | one skill at L5 (65 SP) ✓ |
| L18 | 196,350    | ~5.5 d / ~3 d          | 90   | one skill at L6 (90 SP), or both at L4 (88 SP) |
| L24 | 475,640    | ~13 d / ~7 d           | 120  | one skill at L7 (119 SP), 1 spare |
| L31 | 1,039,500  | ~29 d (~1 mo) / ~16 d  | 155  | one skill at L8 (152 SP), 3 spare |
| L38 | 1,933,250  | ~54 d (~1.8 mo) / ~30 d | 190 | one skill at L9 (189 SP), 1 spare |
| **L46** | **6,906,900** | **~192 d (~6.4 mo) / ~106 d (~3.5 mo)** | **230** | **one skill maxed at L10** ← MVP endgame |
| L50 | 8,893,500  | ~247 d (~8 mo) / ~137 d | 250  | one maxed + other at L4 (230+44 = 274) — needs L55 |
| L55 | 11,870,100 | ~330 d (~11 mo) / ~183 d | 275 | one maxed + other at L4 (274 SP), 1 spare |
| L75 | 30,321,500 | ~2.3 yr / ~1.3 yr      | 375  | one maxed + other at L7 (230+119 = 349), 26 spare |
| **L92** | **56,221,300** | **~4.3 yr / ~2.4 yr**   | **460** | **both skills maxed (230+230 = 460 SP)** ← true endgame |

Reading the table: an avg participant gets their **first skill at L1 immediately on account creation** (5 SP at char L1), both skills at L1 in ~9 min, one skill at L5 in ~2 days, one skill at L7 in ~2 weeks, **one skill maxed (L10, +50%) in ~6.4 months**, both maxed in ~4.3 years. A top participant hits each milestone in roughly 55% of avg-participant time.

Tuning levers (single constants in `app/ratel/src/features/character/leveling.rs`):
- `C` (XP curve scale, currently 220) — dial after first-week telemetry. `C = 150` makes one-skill-max ~4 mo for avg; `C = 300` makes it ~8 mo.
- Curve **shape** (quadratic vs. cubic) — bigger lever than `C`.
- SP-per-level (currently `5`) — dial if early game feels too generous.
- Skill cost slope (currently `+4` per level after L1 entry) — flatter (`+3`) shortens endgame to ~3.3 mo; steeper (`+5`) extends to ~11 mo.

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
| Q7 | SP grant rate per level | **5 SP/level** (PO directive — fast early game, steep cost preserves endgame) | yes |
| Q8 | XP curve shape + `C` | **quadratic, `C = 220`** (PO directive — one-skill-max in 6 months for avg participant) | yes |
| Q9 | Max skill level | **10** (cap multiplier +50% / 1.5×, level pips feel substantial) | yes |
| Q9b | Skill cost curve | **`5 + 4n`** (L1=5 entry, L2+ ramps faster, total 230 SP) | yes |

If you override any of these, the spec change is one-line in `roadmap/character-xp-skills.md` and (for Q1, Q2, Q5) one paragraph in this doc.
