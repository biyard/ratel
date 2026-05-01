# Character XP & Skill Tree — Backend Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Spec**: [`roadmap/character-xp-skills.md`](../../../roadmap/character-xp-skills.md)
**Design**: [`docs/superpowers/specs/2026-05-01-character-xp-skills-design.md`](../specs/2026-05-01-character-xp-skills-design.md)

**Goal:** Add an account-level Character XP / Level / Skill-Point system on top of the existing per-space `SpaceScore` pipeline, with two passive skills (Money Tree boosts RatelPoint earnings, Ranker boosts SpaceXP earnings), plus a versioned migration framework gated by `MIGRATE=true`.

**Architecture:** Stream-driven for XP propagation (`SpaceScore` MODIFY → `apply_character_xp_delta` → `CharacterXp.total_xp += delta`, idempotent via per-(user, space) `CharacterXpSource.last_seen_score` marker). Read-time for skill effects (Money Tree wraps `SpaceReward::award`; Ranker wraps `SpaceActivity::new_with_dedup`). `MIGRATE=true` env-gated runner with conditional-update version advancement (replica-safe).

**Tech Stack:** Rust 2024, Dioxus 0.7 fullstack, DynamoDB single-table via `DynamoEntity` macro, Axum 0.8.1 with `#[get]`/`#[post]` route macros, `tower-sessions`, `serde_dynamo`. Tests via `TestContext` + `test_get!`/`test_post!` macros (see `app/ratel/src/tests/`).

**Test layout — feature-local.** Per PO directive (review on plan line ~1780), all tests for this feature live under `app/ratel/src/features/character/tests/{file}.rs` rather than the cross-feature `app/ratel/src/tests/`. The tests are declared via `#[cfg(test)] mod tests;` in `features/character/mod.rs` and split by topic (`leveling_tests`, `character_xp_tests`, `skill_tests`, `migration_tests`) with shared fixtures in `helpers.rs`. This keeps the feature self-contained — moving / deleting / renaming the feature touches only one directory.

**Out of scope for this plan** (separate follow-up plan):
- Stage 2 HTML mockups in `app/ratel/assets/design/character-xp-skills/`
- Stage 3 frontend RSX (`UseCharacter` hook, `CharacterPage`, `SkillTree`/`SkillCard` components, reward-breakdown chip in `user_reward` views)
- Playwright e2e (`playwright/tests/web/character-progression.spec.js`)

---

## Constants reference (the source of truth)

```
xp_required(L→L+1)     = round(C · L²)         where C = 220
total_xp_at_level(L)   = C · (L−1) · L · (2L−1) / 6
sp_granted_at_level(L) = 5 · L
skill_cost(n→n+1)      = 5 + 4·n               (5, 9, 13, 17, 21, 25, 29, 33, 37, 41)
total_to_max(skill)    = 230 SP
max_skill_level        = 10
multiplier(skill_lv)   = 1 + 0.05 · skill_lv   (max 1.50 / +50% at L10)
```

Values referenced repeatedly in tests below. All live as `pub const` in `app/ratel/src/features/character/leveling.rs`.

---

## File structure

**New files:**

```
app/ratel/src/common/models/migration/
├── mod.rs
└── last_backfill_version.rs

app/ratel/src/common/migrations/
├── mod.rs
├── runner.rs
└── m001_backfill_character_xp.rs

app/ratel/src/features/character/
├── mod.rs
├── route.rs
├── leveling.rs
├── i18n.rs
├── models/
│   ├── mod.rs
│   ├── character_xp.rs
│   ├── character_xp_source.rs
│   └── character_skill.rs
├── controllers/
│   ├── mod.rs
│   ├── get_character.rs
│   ├── get_public_character.rs
│   └── level_up.rs
├── services/
│   ├── mod.rs
│   └── apply_character_xp_delta.rs
├── dto/
│   ├── mod.rs
│   └── character_response.rs
└── types/
    ├── mod.rs
    ├── error.rs
    └── skill_id.rs

app/ratel/src/features/character/tests/
├── mod.rs
├── helpers.rs               # shared TestContext re-exports + fixtures (e.g. make_score)
├── leveling_tests.rs        # unit tests for leveling.rs constants/helpers
├── character_xp_tests.rs    # apply_character_xp_delta + handlers
├── skill_tests.rs           # level_up handler + Money Tree + Ranker effects
└── migration_tests.rs       # LastBackfillVersion + m001 backfill + MIGRATE gate
```

**Modified files:**

```
app/ratel/src/common/types/partition.rs       # add Partition::Migration
app/ratel/src/common/types/entity_type.rs     # add CharacterXp, CharacterXpSource(String), CharacterSkill(String), LastBackfillVersion
app/ratel/src/common/models/mod.rs            # pub use migration::*
app/ratel/src/common/mod.rs                   # pub mod migrations
app/ratel/src/common/run.rs                   # call run_migrations() at server bootstrap
app/ratel/src/common/stream_handler.rs        # SPACE_SCORE# branch (INSERT + MODIFY)
app/ratel/src/features/spaces/space_common/models/space_reward.rs   # Money Tree wrap in award()
app/ratel/src/features/activity/models/space_activity.rs            # Ranker wrap in new_with_dedup()
app/ratel/src/features/mod.rs (or app/ratel/src/lib.rs)             # pub mod character
app/ratel/src/route.rs                                              # mount character routes
```

---

## Task 1: Add `Partition::Migration` variant

**Files:**
- Modify: `app/ratel/src/common/types/partition.rs:22-105`

- [ ] **Step 1: Read current Partition enum** to confirm placement of new variant.

Run: `Read app/ratel/src/common/types/partition.rs:90-105`

- [ ] **Step 2: Add `Migration` variant** as a singleton (no inner String, just a discriminant).

Modify the `Partition` enum, inserting after the existing `Category` variant and before the closing `}`:

```rust
    Category, // CATEGORY - shared pk for all categories

    /// Singleton row keyed for migration framework state.
    /// Pairs with `EntityType::LastBackfillVersion` to form a single
    /// (pk, sk) row at `MIGRATION` + `LAST_BACKFILL_VERSION`.
    Migration,
}
```

- [ ] **Step 3: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: compiles with zero warnings.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/partition.rs
git commit -m "feat(character): add Partition::Migration singleton variant"
```

---

## Task 2: Add new `EntityType` variants

**Files:**
- Modify: `app/ratel/src/common/types/entity_type.rs:18-...`

- [ ] **Step 1: Read current EntityType enum** to confirm placement.

Run: `Read app/ratel/src/common/types/entity_type.rs:18-130`

- [ ] **Step 2: Add four new variants** at an appropriate point in the enum (place near other "User" entity types but in a clearly-marked block):

```rust
    // Migration framework
    LastBackfillVersion,

    // Character (account-level progression)
    CharacterXp,
    CharacterXpSource(String),    // space_id (unprefixed; SubPartition wraps SpacePartition)
    CharacterSkill(String),       // skill_id ("money_tree", "ranker", ...)
```

- [ ] **Step 3: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: compiles. The `SubPartition` derive auto-generates `LastBackfillVersionEntityType`, `CharacterXpEntityType`, `CharacterXpSourceEntityType`, `CharacterSkillEntityType` wrapper types.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/types/entity_type.rs
git commit -m "feat(character): add EntityType variants for character & migrations"
```

---

## Task 3: Create `LastBackfillVersion` entity

**Files:**
- Create: `app/ratel/src/common/models/migration/last_backfill_version.rs`
- Create: `app/ratel/src/common/models/migration/mod.rs`
- Modify: `app/ratel/src/common/models/mod.rs`

- [ ] **Step 1: Create `mod.rs`**

Write `app/ratel/src/common/models/migration/mod.rs`:

```rust
mod last_backfill_version;
pub use last_backfill_version::*;
```

- [ ] **Step 2: Create entity file**

Write `app/ratel/src/common/models/migration/last_backfill_version.rs`:

```rust
use crate::common::{utils::time::get_now_timestamp_millis, *};

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct LastBackfillVersion {
    pub pk: Partition,    // Partition::Migration (singleton)
    pub sk: EntityType,   // EntityType::LastBackfillVersion

    pub version: i64,
    pub updated_at: i64,
}

#[cfg(feature = "server")]
impl LastBackfillVersion {
    pub fn singleton_keys() -> (Partition, EntityType) {
        (Partition::Migration, EntityType::LastBackfillVersion)
    }

    /// Atomically advance the stored version from `expected` to `new_version`.
    /// Uses a conditional update so concurrent replicas can't both succeed.
    /// On the very first run (`expected == 0`), permits insert via
    /// "attribute_not_exists OR version == 0".
    pub async fn advance_to(
        cli: &aws_sdk_dynamodb::Client,
        expected: i64,
        new_version: i64,
    ) -> crate::common::Result<()> {
        let (pk, sk) = Self::singleton_keys();
        let now = get_now_timestamp_millis();

        // For the first-ever advance (no row yet), allow attribute_not_exists.
        let mut updater = Self::updater(&pk, &sk)
            .with_version(new_version)
            .with_updated_at(now);

        if expected == 0 {
            updater = updater.condition_expression(
                "attribute_not_exists(version) OR version = :expected",
            );
        } else {
            updater = updater.condition_expression("version = :expected");
        }
        updater = updater.expression_attribute_value(":expected", expected);

        updater.execute(cli).await?;
        Ok(())
    }
}
```

> Note: The exact builder API for `condition_expression` / `expression_attribute_value` may need a small adapter on the `DynamoEntity` `updater()` API. If those builder methods are not yet exposed by `by-macros`, add them in this task — see Task 3a below.

- [ ] **Step 3: Wire into `common::models::mod.rs`**

Open `app/ratel/src/common/models/mod.rs` and add:

```rust
pub mod migration;
pub use migration::*;
```

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean compile. If `condition_expression` is not on the updater, the build fails — proceed to Task 3a.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/common/models/migration/ app/ratel/src/common/models/mod.rs
git commit -m "feat(migrations): add LastBackfillVersion singleton entity"
```

---

## Task 3a: (conditional) Expose conditional-update builder on `DynamoEntity` updater

Only run this task if Task 3 step 4 fails because `.condition_expression(...)` / `.expression_attribute_value(...)` aren't on the generated updater.

**Files:**
- Modify: `packages/by-macros/src/dynamo_entity/updater.rs` (or wherever the updater is generated)

- [ ] **Step 1: Locate the updater generator**

Run: `grep -rn "fn execute\|impl.*Updater\|pub struct.*Updater" packages/by-macros/src/`

- [ ] **Step 2: Add builder methods** to the generated updater struct: `condition_expression(self, expr: impl Into<String>) -> Self` and `expression_attribute_value(self, name: impl Into<String>, value: impl Into<aws_sdk_dynamodb::types::AttributeValue>) -> Self`. The internal storage is `Option<String>` and `HashMap<String, AttributeValue>`. The `execute` impl already uses `update_item()`; add `.condition_expression(self.condition)` and merge `self.expr_values` into the request before sending.

- [ ] **Step 3: Re-run Task 3 step 4 build check.**

- [ ] **Step 4: Commit**

```bash
git add packages/by-macros/
git commit -m "feat(by-macros): expose condition_expression on DynamoEntity updater"
```

---

## Task 4: Scaffold feature-local `tests/` + cover `LastBackfillVersion`

**Files:**
- Create: `app/ratel/src/features/character/tests/mod.rs`
- Create: `app/ratel/src/features/character/tests/helpers.rs`
- Create: `app/ratel/src/features/character/tests/migration_tests.rs`

> The feature module's `mod.rs` (created in Task 7) declares `#[cfg(test)] mod tests;`. This task creates the test sub-tree.

- [ ] **Step 1: Create `tests/mod.rs`**

Write `app/ratel/src/features/character/tests/mod.rs`:

```rust
//! Feature-local tests. Declared from `features/character/mod.rs` under
//! `#[cfg(test)] mod tests;` so they compile only for `cargo test`.
//!
//! Layout:
//! - `helpers`           shared fixtures: TestContext, make_score, award_xp, run_with_env
//! - `leveling_tests`    pure-Rust unit tests for `leveling.rs`
//! - `character_xp_tests` apply_character_xp_delta + GET handlers
//! - `skill_tests`       level_up handler + Money Tree + Ranker effects
//! - `migration_tests`   LastBackfillVersion conditional advance + m001 + MIGRATE gate

mod helpers;
mod leveling_tests;
mod character_xp_tests;
mod skill_tests;
mod migration_tests;
```

- [ ] **Step 2: Create shared `helpers.rs`**

Write `app/ratel/src/features/character/tests/helpers.rs`:

```rust
//! Shared fixtures for the character feature's tests.
//!
//! `TestContext` re-uses the project-wide setup at `crate::tests::TestContext`
//! (it spins up a fresh DynamoDB Local namespace and a router). The helpers
//! below add character-feature-specific factories on top.

pub use crate::tests::TestContext;
pub use crate::common::types::*;
use crate::features::activity::models::SpaceScore;

/// Build a SpaceScore row for `(user, space)` with `total_score = total`.
/// Matches the per-space score the existing aggregation pipeline produces.
pub fn make_score(user_pk: &Partition, space_id: &str, total: i64) -> SpaceScore {
    let space_part = SpacePartition(space_id.to_string());
    let author = AuthorPartition(match user_pk {
        Partition::User(s) => s.clone(),
        _ => panic!("user pk only"),
    });
    let mut s = SpaceScore::new(space_part, author, "u".into(), "".into());
    s.total_score = total;
    s
}

/// Pump enough XP into the user's CharacterXp via the production code path
/// (not direct entity manipulation) so tests exercise `apply_character_xp_delta`.
pub async fn award_xp(ctx: &TestContext, user_pk: &Partition, total: i64) {
    crate::features::character::services::apply_character_xp_delta(
        ctx.ddb,
        make_score(user_pk, "s", total),
    )
    .await
    .unwrap();
}

/// Run a future with an env var temporarily set, then restore the prior value.
/// Used by migration tests that toggle `MIGRATE`. Tests using this MUST run
/// serialized — the project's test runner already serializes by default with
/// `--test-threads=1` for integration tests; if running in parallel, gate
/// with a `static MUTEX: Mutex<()> = Mutex::const_new(());`.
pub async fn run_with_env<F, Fut, T>(key: &str, val: &str, f: F) -> T
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = T>,
{
    let prev = std::env::var(key).ok();
    std::env::set_var(key, val);
    let r = f().await;
    if let Some(p) = prev {
        std::env::set_var(key, p);
    } else {
        std::env::remove_var(key);
    }
    r
}
```

- [ ] **Step 3: Create `migration_tests.rs`**

Write `app/ratel/src/features/character/tests/migration_tests.rs`:

```rust
use super::helpers::*;
use crate::common::models::migration::LastBackfillVersion;

#[tokio::test]
async fn test_last_backfill_version_default_unset() {
    let ctx = TestContext::setup().await;
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk)).await.unwrap();
    assert!(row.is_none(), "no migration row should exist initially");
}

#[tokio::test]
async fn test_advance_to_from_zero_inserts() {
    let ctx = TestContext::setup().await;
    LastBackfillVersion::advance_to(ctx.ddb, 0, 1).await.unwrap();
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("row should exist after advance");
    assert_eq!(row.version, 1);
}

#[tokio::test]
async fn test_advance_to_with_correct_expected_succeeds() {
    let ctx = TestContext::setup().await;
    LastBackfillVersion::advance_to(ctx.ddb, 0, 1).await.unwrap();
    LastBackfillVersion::advance_to(ctx.ddb, 1, 2).await.unwrap();
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("row should exist");
    assert_eq!(row.version, 2);
}

#[tokio::test]
async fn test_advance_to_with_wrong_expected_fails() {
    let ctx = TestContext::setup().await;
    LastBackfillVersion::advance_to(ctx.ddb, 0, 1).await.unwrap();
    let res = LastBackfillVersion::advance_to(ctx.ddb, 0, 2).await;
    assert!(res.is_err(), "advancing with stale expected should be rejected");
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("row should still be at 1");
    assert_eq!(row.version, 1, "version must not advance on conflict");
}
```

- [ ] **Step 4: Create empty placeholder files** for the other test modules so `tests/mod.rs` compiles. Touch (zero-byte content fine):
  - `leveling_tests.rs` — populated in Task 8
  - `character_xp_tests.rs` — populated in Task 14
  - `skill_tests.rs` — populated in Task 22

- [ ] **Step 5: Run the four LBV tests**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::migration_tests`
Expected: 4 passed.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/character/tests/
git commit -m "test(character): scaffold feature-local tests + LastBackfillVersion coverage"
```

---

## Task 5: Create migration framework runner

**Files:**
- Create: `app/ratel/src/common/migrations/mod.rs`
- Create: `app/ratel/src/common/migrations/runner.rs`
- Modify: `app/ratel/src/common/mod.rs`

- [ ] **Step 1: Create `mod.rs` skeleton**

Write `app/ratel/src/common/migrations/mod.rs`:

```rust
#[cfg(feature = "server")]
mod runner;
#[cfg(feature = "server")]
pub use runner::run_migrations;

// Each migration lives in its own module under this directory.
// As migrations are added, register them in `runner.rs`.
```

- [ ] **Step 2: Create runner skeleton (no migrations registered yet)**

Write `app/ratel/src/common/migrations/runner.rs`:

```rust
use crate::common::models::migration::LastBackfillVersion;
use crate::common::types::*;

/// Run all pending migrations in version order. Gated by the `MIGRATE` env
/// var: only executes when `MIGRATE=true` is set. Safe under concurrent
/// replicas — the conditional update on `LastBackfillVersion.version`
/// ensures only one replica wins the version bump.
pub async fn run_migrations(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<()> {
    if std::env::var("MIGRATE").as_deref() != Ok("true") {
        tracing::info!("MIGRATE not set — skipping migrations");
        return Ok(());
    }

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let stored = LastBackfillVersion::get(cli, &pk, Some(&sk))
        .await?
        .map(|r| r.version)
        .unwrap_or(0);

    tracing::info!(stored_version = stored, "migration runner starting");

    // === migration registry (extend here as migrations are added) ===
    // Example pattern (uncommented in Task 26):
    //
    // if stored < 1 {
    //     tracing::info!("running migration 001: backfill_character_xp");
    //     super::m001_backfill_character_xp::run(cli).await?;
    //     LastBackfillVersion::advance_to(cli, stored, 1).await?;
    //     tracing::info!("migration 001 complete; version advanced to 1");
    // }

    tracing::info!("migration runner finished");
    Ok(())
}
```

- [ ] **Step 3: Wire into `common::mod.rs`**

Open `app/ratel/src/common/mod.rs` and add:

```rust
pub mod migrations;
```

(near the other `pub mod` lines).

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/common/migrations/ app/ratel/src/common/mod.rs
git commit -m "feat(migrations): MIGRATE-gated runner skeleton"
```

---

## Task 6: Wire `run_migrations()` at server bootstrap

**Files:**
- Modify: `app/ratel/src/common/run.rs:46-98`

- [ ] **Step 1: Read current bootstrap** to confirm insertion point.

Run: `Read app/ratel/src/common/run.rs:46-98`

- [ ] **Step 2: Insert migration call**

In `serve()`, just after `let cli = cfg.dynamodb();` and before the session-layer setup, add:

```rust
    let cli = cfg.dynamodb();

    // Run pending migrations. No-op unless MIGRATE=true is set.
    // Blocks server startup until done; use a one-shot deploy to set this.
    if let Ok(handle) = tokio::runtime::Handle::try_current() {
        if let Err(e) = handle.block_on(crate::common::migrations::run_migrations(cli)) {
            tracing::error!(error = %e, "migration runner failed; aborting startup");
            std::process::exit(1);
        }
    } else {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime build for migrations");
        if let Err(e) = rt.block_on(crate::common::migrations::run_migrations(cli)) {
            tracing::error!(error = %e, "migration runner failed; aborting startup");
            std::process::exit(1);
        }
    }

    let session_layer = ...
```

- [ ] **Step 3: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/run.rs
git commit -m "feat(migrations): wire run_migrations() at server bootstrap"
```

---

## Task 7: Create `leveling.rs` math module

**Files:**
- Create: `app/ratel/src/features/character/leveling.rs`
- Create: `app/ratel/src/features/character/mod.rs` (skeleton)
- Modify: `app/ratel/src/lib.rs` or `app/ratel/src/features/mod.rs` (add `pub mod character`)

- [ ] **Step 1: Add feature module skeleton**

Write `app/ratel/src/features/character/mod.rs`:

```rust
pub mod leveling;
pub mod models;
pub mod controllers;
pub mod services;
pub mod dto;
pub mod types;
pub mod route;
pub mod i18n;

pub use leveling::*;
pub use models::*;
pub use types::*;
pub use dto::*;

use crate::common::*;

// Feature-local tests (per the test-layout note in the plan header).
// Compiled only for `cargo test`; never shipped to prod binaries.
#[cfg(test)]
mod tests;
```

- [ ] **Step 2: Register in features**

Open `app/ratel/src/features/mod.rs` and append:

```rust
pub mod character;
```

- [ ] **Step 3: Write `leveling.rs`**

```rust
//! Character XP / Level / Skill-Point math. Single source of truth.
//!
//! All formulas locked by the spec at
//! `roadmap/character-xp-skills.md` and the design doc at
//! `docs/superpowers/specs/2026-05-01-character-xp-skills-design.md`.

/// XP curve scale. `xp_required(L→L+1) = round(C · L²)`.
pub const C: i64 = 220;

/// Skill points granted per character level.
pub const SP_PER_LEVEL: i32 = 5;

/// Maximum level any one skill can reach.
pub const MAX_SKILL_LEVEL: i32 = 10;

/// Skill cost from level n to n+1 is `5 + 4·n`.
pub const SKILL_COST_BASE: i32 = 5;
pub const SKILL_COST_SLOPE: i32 = 4;

/// Effect multiplier per skill level (per-mille so we stay in integer math
/// where possible). 5% = 50‰. Cap is 500‰ at L10.
pub const MULTIPLIER_PER_LEVEL_PERMILLE: i32 = 50;

/// Cumulative XP required to reach character level `L` from level 1.
/// Closed form: `C · (L−1) · L · (2L−1) / 6`.
pub fn cumulative_xp_at_level(level: i32) -> i64 {
    if level <= 1 {
        return 0;
    }
    let l = level as i64;
    C * (l - 1) * l * (2 * l - 1) / 6
}

/// Derive character level from cumulative XP. Levels start at 1.
/// Linear search is fine — character level is bounded in practice (<200).
pub fn level_from_xp(total_xp: i64) -> i32 {
    let mut l: i32 = 1;
    while cumulative_xp_at_level(l + 1) <= total_xp {
        l += 1;
        if l > 1_000 {
            // safety bound; should never fire under realistic XP.
            break;
        }
    }
    l
}

/// Total skill points granted at character level `L`.
pub fn total_sp_granted(level: i32) -> i32 {
    SP_PER_LEVEL * level
}

/// Cost to advance a skill from `current_level` (0..MAX) to `current_level + 1`.
/// Returns `None` if already at max.
pub fn skill_cost_next(current_level: i32) -> Option<i32> {
    if current_level >= MAX_SKILL_LEVEL {
        None
    } else {
        Some(SKILL_COST_BASE + SKILL_COST_SLOPE * current_level)
    }
}

/// Effect multiplier as a `(numerator, denominator = 1000)` per-mille pair.
/// Use as `amount * num / 1000`. At skill_level=0 returns (1000, 1000).
pub fn multiplier_permille(skill_level: i32) -> i32 {
    1000 + MULTIPLIER_PER_LEVEL_PERMILLE * skill_level
}

/// Apply a per-mille multiplier to an `i64` amount, rounding to nearest.
pub fn apply_permille(amount: i64, permille: i32) -> i64 {
    // (amount * permille + 500) / 1000 — banker's rounding not required.
    (amount * permille as i64 + 500) / 1000
}
```

- [ ] **Step 4: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/character/ app/ratel/src/features/mod.rs
git commit -m "feat(character): leveling math module (constants + helpers)"
```

---

## Task 8: Test `leveling.rs`

**Files:**
- Modify: `app/ratel/src/features/character/tests/leveling_tests.rs`

> Per the feature-local test convention, leveling unit tests live in the feature's `tests/` tree, not as an inline `#[cfg(test)] mod tests` block. They're plain Rust tests (no DynamoDB) but kept alongside the integration tests so the whole feature's coverage lives in one directory.

- [ ] **Step 1: Replace placeholder with tests**

Write `app/ratel/src/features/character/tests/leveling_tests.rs`:

```rust
use crate::features::character::leveling::*;

#[test]
fn cumulative_xp_known_values() {
    assert_eq!(cumulative_xp_at_level(1), 0);
    // L2: 220 · 1·2·3 / 6 = 220
    assert_eq!(cumulative_xp_at_level(2), 220);
    // L5: 220 · 4·5·9 / 6 = 6_600
    assert_eq!(cumulative_xp_at_level(5), 6_600);
    // L10: 220 · 9·10·19 / 6 = 62_700
    assert_eq!(cumulative_xp_at_level(10), 62_700);
    // L46: 220 · 45·46·91 / 6 = 6_906_900
    assert_eq!(cumulative_xp_at_level(46), 6_906_900);
}

#[test]
fn level_from_xp_boundaries() {
    assert_eq!(level_from_xp(0), 1);
    assert_eq!(level_from_xp(219), 1);
    assert_eq!(level_from_xp(220), 2);
    assert_eq!(level_from_xp(6_599), 4);
    assert_eq!(level_from_xp(6_600), 5);
    assert_eq!(level_from_xp(6_906_900), 46);
}

#[test]
fn sp_granted_linear() {
    assert_eq!(total_sp_granted(1), 5);
    assert_eq!(total_sp_granted(10), 50);
    assert_eq!(total_sp_granted(46), 230);
}

#[test]
fn skill_cost_curve() {
    assert_eq!(skill_cost_next(0), Some(5));
    assert_eq!(skill_cost_next(1), Some(9));
    assert_eq!(skill_cost_next(2), Some(13));
    assert_eq!(skill_cost_next(3), Some(17));
    assert_eq!(skill_cost_next(4), Some(21));
    assert_eq!(skill_cost_next(5), Some(25));
    assert_eq!(skill_cost_next(6), Some(29));
    assert_eq!(skill_cost_next(7), Some(33));
    assert_eq!(skill_cost_next(8), Some(37));
    assert_eq!(skill_cost_next(9), Some(41));
    assert_eq!(skill_cost_next(10), None);

    // Total cost to max: 5+9+13+17+21+25+29+33+37+41 = 230
    let total: i32 = (0..MAX_SKILL_LEVEL).map(|n| skill_cost_next(n).unwrap()).sum();
    assert_eq!(total, 230);
}

#[test]
fn multiplier_curve() {
    assert_eq!(multiplier_permille(0), 1000);
    assert_eq!(multiplier_permille(1), 1050);
    assert_eq!(multiplier_permille(5), 1250);
    assert_eq!(multiplier_permille(10), 1500); // +50% at max = 1.5×
}

#[test]
fn apply_permille_rounding() {
    // 10_000 × 1.20 = 12_000
    assert_eq!(apply_permille(10_000, 1200), 12_000);
    // 10_000 × 1.05 = 10_500
    assert_eq!(apply_permille(10_000, 1050), 10_500);
    // 7 × 1.05 = 7.35 → rounds to 7
    assert_eq!(apply_permille(7, 1050), 7);
    // 9 × 1.05 = 9.45 → rounds to 9
    assert_eq!(apply_permille(9, 1050), 9);
}
```

- [ ] **Step 2: Run tests**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features server -- features::character::tests::leveling_tests`
Expected: all passed.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/character/tests/leveling_tests.rs
git commit -m "test(character): leveling math unit tests (curve + costs + multiplier)"
```

---

## Task 9: Create `CharacterError` and `SkillId` types

**Files:**
- Create: `app/ratel/src/features/character/types/mod.rs`
- Create: `app/ratel/src/features/character/types/error.rs`
- Create: `app/ratel/src/features/character/types/skill_id.rs`
- Modify: `app/ratel/src/common/types/error.rs` — register `CharacterError` variant.

- [ ] **Step 1: Create types/mod.rs**

```rust
mod error;
mod skill_id;
pub use error::*;
pub use skill_id::*;
```

- [ ] **Step 2: Create `skill_id.rs`**

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Hash)]
#[serde(rename_all = "snake_case")]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema))]
pub enum SkillId {
    MoneyTree,
    Ranker,
    // v2 — declared so the data model can store them, but the level-up
    // endpoint rejects any non-MVP id until the v2 spec ships.
    Influencer,
    Sweeper,
}

impl SkillId {
    pub fn as_str(&self) -> &'static str {
        match self {
            SkillId::MoneyTree => "money_tree",
            SkillId::Ranker => "ranker",
            SkillId::Influencer => "influencer",
            SkillId::Sweeper => "sweeper",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "money_tree" => Some(Self::MoneyTree),
            "ranker" => Some(Self::Ranker),
            "influencer" => Some(Self::Influencer),
            "sweeper" => Some(Self::Sweeper),
            _ => None,
        }
    }

    /// MVP skills the level-up endpoint accepts.
    pub fn is_mvp(&self) -> bool {
        matches!(self, SkillId::MoneyTree | SkillId::Ranker)
    }
}
```

- [ ] **Step 3: Create `error.rs`**

```rust
use crate::common::*;
use dioxus_translate::Translate;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum CharacterError {
    #[error("skill not found")]
    #[translate(en = "Skill not found", ko = "스킬을 찾을 수 없습니다")]
    SkillNotFound,

    #[error("skill not yet released")]
    #[translate(en = "This skill is not yet available", ko = "아직 출시되지 않은 스킬입니다")]
    SkillNotReleased,

    #[error("insufficient skill points")]
    #[translate(en = "Insufficient skill points", ko = "스킬 포인트가 부족합니다")]
    InsufficientSp,

    #[error("skill at max level")]
    #[translate(en = "This skill is already at maximum level", ko = "이미 최대 레벨입니다")]
    AlreadyMaxLevel,
}
```

- [ ] **Step 4: Register in `common::Error`**

Open `app/ratel/src/common/types/error.rs`, find the `pub enum Error { ... }` block, and add (preserving alphabetical order with siblings):

```rust
    #[error("{0}")]
    #[translate(from)]
    Character(#[from] crate::features::character::types::CharacterError),
```

- [ ] **Step 5: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/character/types/ app/ratel/src/common/types/error.rs
git commit -m "feat(character): error and SkillId types"
```

---

## Task 10: Create `CharacterXp` entity

**Files:**
- Create: `app/ratel/src/features/character/models/mod.rs`
- Create: `app/ratel/src/features/character/models/character_xp.rs`

- [ ] **Step 1: Create `models/mod.rs`**

```rust
mod character_xp;
mod character_xp_source;
mod character_skill;

pub use character_xp::*;
pub use character_xp_source::*;
pub use character_skill::*;
```

- [ ] **Step 2: Create `character_xp.rs`**

```rust
use crate::common::macros::DynamoEntity;
use crate::common::*;
use crate::features::character::leveling;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct CharacterXp {
    pub pk: Partition,           // Partition::User(user_id)
    pub sk: EntityType,          // EntityType::CharacterXp

    pub created_at: i64,
    pub updated_at: i64,

    pub total_xp: i64,           // monotonic, sum of SpaceScore deltas
    pub level: i32,              // derived from total_xp; denormalized
    pub total_sp_granted: i32,   // = SP_PER_LEVEL * level
    pub total_sp_spent: i32,     // sum of skill_cost paid via level-up endpoint
}

impl CharacterXp {
    pub fn unspent_sp(&self) -> i32 {
        self.total_sp_granted - self.total_sp_spent
    }

    pub fn user_keys(user_pk: &Partition) -> (Partition, EntityType) {
        (user_pk.clone(), EntityType::CharacterXp)
    }
}

#[cfg(feature = "server")]
impl CharacterXp {
    pub fn new(user_pk: Partition) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: user_pk,
            sk: EntityType::CharacterXp,
            created_at: now,
            updated_at: now,
            total_xp: 0,
            level: 1,
            total_sp_granted: leveling::total_sp_granted(1), // 5 SP at L1 from day 0
            total_sp_spent: 0,
        }
    }
}
```

- [ ] **Step 3: Verify build (skipping the other two model files which mod.rs references — keep them as empty placeholders for now)**

Create empty stub files to satisfy `mod.rs`:
- `character_xp_source.rs` with `// placeholder, see Task 11`
- `character_skill.rs` with `// placeholder, see Task 13`

Then: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`

(`mod.rs` re-exports `pub use *` from each, so empty files are fine.)

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/character/models/
git commit -m "feat(character): CharacterXp entity"
```

---

## Task 11: Create `CharacterXpSource` entity (per-(user,space) last-seen marker)

**Files:**
- Modify: `app/ratel/src/features/character/models/character_xp_source.rs`

- [ ] **Step 1: Write the entity**

```rust
use crate::common::macros::DynamoEntity;
use crate::common::*;

/// Per-(user, space) marker recording the last `SpaceScore.total_score`
/// applied to the user's CharacterXp. Used to compute the delta on each
/// SpaceScore MODIFY event so XP is idempotent under stream replay.
#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct CharacterXpSource {
    pub pk: Partition,             // Partition::User(user_id)
    pub sk: EntityType,            // EntityType::CharacterXpSource(space_id)

    pub last_seen_score: i64,
    pub updated_at: i64,
}

impl CharacterXpSource {
    pub fn keys(user_pk: &Partition, space_id: &str) -> (Partition, EntityType) {
        (
            user_pk.clone(),
            EntityType::CharacterXpSource(space_id.to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl CharacterXpSource {
    pub fn new(user_pk: Partition, space_id: String, last_seen_score: i64) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: user_pk,
            sk: EntityType::CharacterXpSource(space_id),
            last_seen_score,
            updated_at: now,
        }
    }
}
```

- [ ] **Step 2: Build check + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
git add app/ratel/src/features/character/models/character_xp_source.rs
git commit -m "feat(character): CharacterXpSource per-(user,space) marker"
```

---

## Task 12: Create `CharacterSkill` entity

**Files:**
- Modify: `app/ratel/src/features/character/models/character_skill.rs`

- [ ] **Step 1: Write the entity**

```rust
use crate::common::macros::DynamoEntity;
use crate::common::*;
use crate::features::character::types::SkillId;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity, PartialEq)]
pub struct CharacterSkill {
    pub pk: Partition,            // Partition::User(user_id)
    pub sk: EntityType,           // EntityType::CharacterSkill(skill_id)

    pub level: i32,               // 0..MAX_SKILL_LEVEL
    pub created_at: i64,
    pub updated_at: i64,
}

impl CharacterSkill {
    pub fn keys(user_pk: &Partition, skill_id: SkillId) -> (Partition, EntityType) {
        (
            user_pk.clone(),
            EntityType::CharacterSkill(skill_id.as_str().to_string()),
        )
    }
}

#[cfg(feature = "server")]
impl CharacterSkill {
    pub fn new(user_pk: Partition, skill_id: SkillId) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        Self {
            pk: user_pk,
            sk: EntityType::CharacterSkill(skill_id.as_str().to_string()),
            level: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Read a single skill row's level, treating "row absent" as level 0.
    /// Use only when you need exactly one skill (e.g. inside the level-up
    /// handler before mutating). For "show me every skill the user has",
    /// use `list_for_user` instead — it's a single query.
    pub async fn level_or_zero(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
        skill_id: SkillId,
    ) -> crate::common::Result<i32> {
        let (pk, sk) = Self::keys(user_pk, skill_id);
        let row = Self::get(cli, &pk, Some(&sk)).await?;
        Ok(row.map(|r| r.level).unwrap_or(0))
    }

    /// Read every skill row for a user in **a single DynamoDB Query** —
    /// `pk = user_pk AND begins_with(sk, "CHARACTER_SKILL#")`. Returns the
    /// raw rows; the caller maps them to `(SkillId, level)` pairs and fills
    /// in level=0 for missing entries. Per-user skill set is bounded
    /// (≤4 in MVP, ≤4 in v2), so the result fits in a single response page.
    ///
    /// Prefer this over four sequential `level_or_zero` calls or four
    /// parallel `tokio::join!` calls — the round-trip count drops from
    /// 4-or-more to 1, which matters at request-handler latency budget.
    ///
    /// Implementation note: `find_by_pk` is generated by the `DynamoEntity`
    /// derive on the main table (no GSI involved). The sk-prefix filter is
    /// applied via the macro's `begins_with` helper if exposed; otherwise
    /// the equivalent `query_by_pk_and_sk_prefix` helper or a direct
    /// `aws_sdk_dynamodb::Client::query()` with
    /// `key_condition_expression = "pk = :pk AND begins_with(sk, :prefix)"`
    /// is used (the implementer should pick whichever the by-macros API
    /// already exposes — both reduce to one Query under the hood).
    pub async fn list_for_user(
        cli: &aws_sdk_dynamodb::Client,
        user_pk: &Partition,
    ) -> crate::common::Result<Vec<Self>> {
        let opts = Self::opt().limit(50);
        // Try the macro-generated prefix helper first; fall back to
        // pk-only with client-side filter if the helper isn't available.
        // (Result set per user is ≤4, so client-side filtering is cheap.)
        let (rows, _) = Self::find_by_pk_with_sk_prefix(
            cli,
            user_pk.clone(),
            "CHARACTER_SKILL#",
            opts,
        ).await?;
        Ok(rows)
    }

    /// Convenience: turn a `list_for_user` result into a complete
    /// `(SkillId, level)` map for every known SkillId, defaulting absent
    /// rows to level 0. Caller-side because `SkillId` is in the feature
    /// crate and may add variants without breaking this model.
    pub fn levels_by_id(rows: &[Self]) -> Vec<(SkillId, i32)> {
        let level_for = |id: SkillId| -> i32 {
            rows.iter()
                .find(|r| matches!(&r.sk, EntityType::CharacterSkill(s) if s == id.as_str()))
                .map(|r| r.level)
                .unwrap_or(0)
        };
        [SkillId::MoneyTree, SkillId::Ranker, SkillId::Influencer, SkillId::Sweeper]
            .into_iter()
            .map(|id| (id, level_for(id)))
            .collect()
    }
}
```

> **Implementer note**: if `Self::find_by_pk_with_sk_prefix` is not yet exposed by `packages/by-macros`, expose it as a small wrapper that emits a `Query` with `KeyConditionExpression = "pk = :pk AND begins_with(sk, :prefix)"`. This is a one-time addition that benefits every entity in the codebase doing per-user collection reads (compare to the existing `UserMetadata` pattern referenced in the design doc, which keeps multiple per-user fields in one row precisely to avoid this anti-pattern).

- [ ] **Step 2: Build check + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
git add app/ratel/src/features/character/models/character_skill.rs
git commit -m "feat(character): CharacterSkill entity + level_or_zero helper"
```

---

## Task 13: Create `apply_character_xp_delta` service

**Files:**
- Create: `app/ratel/src/features/character/services/mod.rs`
- Create: `app/ratel/src/features/character/services/apply_character_xp_delta.rs`

- [ ] **Step 1: services/mod.rs**

```rust
#[cfg(feature = "server")]
mod apply_character_xp_delta;
#[cfg(feature = "server")]
pub use apply_character_xp_delta::*;
```

- [ ] **Step 2: Write the service**

```rust
use crate::common::*;
use crate::features::activity::models::SpaceScore;
use crate::features::character::leveling;
use crate::features::character::models::{CharacterXp, CharacterXpSource};

/// Apply the change in `SpaceScore.total_score` for a (user, space) into the
/// user's CharacterXp. Idempotent under stream replay: a re-delivered MODIFY
/// event with the same `score.total_score` produces zero delta.
///
/// `score`: the *new* SpaceScore (post-MODIFY image, or post-INSERT image).
pub async fn apply_character_xp_delta(
    cli: &aws_sdk_dynamodb::Client,
    score: SpaceScore,
) -> crate::common::Result<()> {
    let user_pk: Partition = score.user_pk.clone().into();
    let space_pk_str = match &score.space_pk {
        Partition::Space(s) => s.clone(),
        _ => {
            tracing::warn!(
                user_pk = %user_pk,
                space_pk = ?score.space_pk,
                "apply_character_xp_delta: unexpected space_pk variant; skipping"
            );
            return Ok(());
        }
    };

    let (src_pk, src_sk) = CharacterXpSource::keys(&user_pk, &space_pk_str);
    let last_seen = CharacterXpSource::get(cli, &src_pk, Some(&src_sk))
        .await?
        .map(|r| r.last_seen_score)
        .unwrap_or(0);

    let new_total = score.total_score;
    let delta = new_total - last_seen;

    if delta == 0 {
        // Replay; nothing to do.
        return Ok(());
    }
    if delta < 0 {
        // Score decreased — spec Q3 says XP is monotonic. Don't debit, but
        // do advance last_seen so we don't re-apply the same negative delta.
        tracing::warn!(
            user_pk = %user_pk,
            space = %space_pk_str,
            last_seen,
            new_total,
            "negative SpaceScore delta — last_seen advanced, CharacterXp unchanged"
        );
        let new_src = CharacterXpSource::new(user_pk.clone(), space_pk_str.clone(), new_total);
        let _ = new_src.create(cli).await;
        return Ok(());
    }

    // Read current CharacterXp, default to a fresh row.
    let (xp_pk, xp_sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(cli, &xp_pk, Some(&xp_sk)).await?;
    let xp = xp.unwrap_or_else(|| CharacterXp::new(user_pk.clone()));

    let new_total_xp = xp.total_xp + delta;
    let new_level = leveling::level_from_xp(new_total_xp);
    let new_sp_granted = leveling::total_sp_granted(new_level);
    let now = crate::common::utils::time::get_now_timestamp_millis();

    CharacterXp::updater(&xp_pk, &xp_sk)
        .with_total_xp(new_total_xp)
        .with_level(new_level)
        .with_total_sp_granted(new_sp_granted)
        .with_updated_at(now)
        .execute(cli)
        .await?;

    // Record last_seen so future deltas are correct.
    let new_src = CharacterXpSource::new(user_pk.clone(), space_pk_str, new_total);
    new_src.create(cli).await?;

    if new_level != xp.level {
        tracing::info!(
            user_pk = %user_pk,
            old_level = xp.level,
            new_level,
            new_sp = new_sp_granted - xp.total_sp_granted,
            "character level up"
        );
        // Future hook: enqueue InboxNotification here.
    }

    Ok(())
}
```

- [ ] **Step 3: Build check**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/character/services/
git commit -m "feat(character): apply_character_xp_delta service (idempotent)"
```

---

## Task 14: Test `apply_character_xp_delta`

**Files:**
- Modify: `app/ratel/src/features/character/tests/character_xp_tests.rs`

- [ ] **Step 1: Replace placeholder with tests**

Write `app/ratel/src/features/character/tests/character_xp_tests.rs`:

```rust
use super::helpers::*;
use crate::features::character::models::{CharacterXp, CharacterXpSource};
use crate::features::character::services::apply_character_xp_delta;

#[tokio::test]
async fn test_apply_xp_first_score_inserts_xp_row() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    let score = make_score(&user_pk, "space-a", 5_000);

    apply_character_xp_delta(ctx.ddb, score).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &pk, Some(&sk))
        .await
        .unwrap()
        .expect("xp row created");
    assert_eq!(xp.total_xp, 5_000);
    assert_eq!(xp.level, 4);  // cumulative_xp(4) = 220·3·4·7/6 = 3_080 < 5_000 < 6_600 = L5
    assert_eq!(xp.total_sp_granted, 5 * 4);
    assert_eq!(xp.total_sp_spent, 0);
}

#[tokio::test]
async fn test_apply_xp_replay_idempotent() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    let score = make_score(&user_pk, "space-a", 5_000);

    apply_character_xp_delta(ctx.ddb, score.clone()).await.unwrap();
    apply_character_xp_delta(ctx.ddb, score).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000, "replay must not double-count");
}

#[tokio::test]
async fn test_apply_xp_increment_uses_delta() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 1_000)).await.unwrap();
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 1_500)).await.unwrap();
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 5_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000);
}

#[tokio::test]
async fn test_apply_xp_negative_delta_does_not_debit() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 5_000)).await.unwrap();
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 4_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000, "monotonic — negative deltas dropped");
}

#[tokio::test]
async fn test_apply_xp_level_up_grants_sp() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    // First: small score, ends at L1 (220 needed for L2).
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 100)).await.unwrap();
    // Then: enough to cross many levels.
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "s", 100_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert!(xp.level >= 12);
    assert_eq!(xp.total_sp_granted, 5 * xp.level);
}

#[tokio::test]
async fn test_apply_xp_per_space_independent() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "space-a", 1_000)).await.unwrap();
    apply_character_xp_delta(ctx.ddb, make_score(&user_pk, "space-b", 2_000)).await.unwrap();

    let (pk, sk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 3_000, "delta from each space accumulates");
}
```

- [ ] **Step 2: Run tests**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::character_xp_tests::test_apply_xp`
Expected: 6 passed.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/character/tests/character_xp_tests.rs
git commit -m "test(character): apply_character_xp_delta — insert/replay/delta/negative/levelup/multi-space"
```

---

## Task 15: Add `SPACE_SCORE#` branch in stream handler

**Files:**
- Modify: `app/ratel/src/common/stream_handler.rs`

- [ ] **Step 1: Locate the INSERT and MODIFY arms**

Run: `grep -n '"INSERT"\|"MODIFY"\|sk.starts_with' app/ratel/src/common/stream_handler.rs | head -40`

- [ ] **Step 2: Add new dispatch branches**

In the INSERT arm, append a branch (after the existing `SPACE_ACTIVITY#` branch):

```rust
            } else if sk.starts_with("SPACE_SCORE#") {
                // Newly-created SpaceScore row (first activity in a space).
                // Treat it as `delta = total_score - 0 = total_score`.
                let score: crate::features::activity::models::SpaceScore = deserialize(image)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                if let Err(e) = crate::features::character::services::apply_character_xp_delta(
                    cli, score,
                ).await {
                    tracing::error!(error = %e, "stream: CharacterXpDelta (INSERT) failed");
                }
            }
```

In the MODIFY arm, add the same logic:

```rust
        "MODIFY" => {
            let image = new_image.ok_or(Error::from(InfraError::StreamMissingImage))?;
            let sk = get_sk(image).unwrap_or_default();

            if sk.starts_with("SPACE_SCORE#") {
                let score: crate::features::activity::models::SpaceScore = deserialize(image)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                if let Err(e) = crate::features::character::services::apply_character_xp_delta(
                    cli, score,
                ).await {
                    tracing::error!(error = %e, "stream: CharacterXpDelta (MODIFY) failed");
                }
            }
            // ... existing MODIFY branches preserved below ...
        }
```

If the existing handler has no MODIFY arm yet, add one with this branch.

- [ ] **Step 3: Verify build**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features "server,lambda"`
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/stream_handler.rs
git commit -m "feat(character): SPACE_SCORE# stream branch dispatches XP delta"
```

---

## Task 16: Wire EventBridge envelope dispatch

**Files:**
- Modify: `app/ratel/src/common/types/event_bridge_envelope.rs`

- [ ] **Step 1: Read current `DetailType` enum**

Run: `grep -n "pub enum DetailType\|impl EventBridgeEnvelope\|fn proc" app/ratel/src/common/types/event_bridge_envelope.rs | head`

- [ ] **Step 2: Add a `CharacterXpDelta` variant** before `Unknown`:

```rust
pub enum DetailType {
    // ... existing ...
    CharacterXpDelta,
    #[serde(other)]
    Unknown,
}
```

- [ ] **Step 3: Add proc() arm**

In `proc()` match, before the `_ => Unknown` fallback:

```rust
            DetailType::CharacterXpDelta => {
                let score: crate::features::activity::models::SpaceScore =
                    DetailType::parse_detail(&self.detail)?;
                let cfg = crate::common::CommonConfig::default();
                let cli = cfg.dynamodb();
                crate::features::character::services::apply_character_xp_delta(cli, score).await
            }
```

- [ ] **Step 4: Build check + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features "server,lambda"
git add app/ratel/src/common/types/event_bridge_envelope.rs
git commit -m "feat(character): DetailType::CharacterXpDelta dispatches to service"
```

> **CDK note (out-of-scope for this plan):** A new EventBridge Pipe + Rule for `SPACE_SCORE#` updates will be added to `cdk/lib/dynamo-stream-event.ts` per `conventions/implementing-event-bridge.md`. Track this as a separate ticket; the local-dev poller branch from Task 15 is sufficient for non-Lambda environments.

---

## Task 17: Create `CharacterResponse` DTO

**Files:**
- Create: `app/ratel/src/features/character/dto/mod.rs`
- Create: `app/ratel/src/features/character/dto/character_response.rs`

- [ ] **Step 1: dto/mod.rs**

```rust
mod character_response;
pub use character_response::*;
```

- [ ] **Step 2: Write the DTO**

```rust
use crate::common::*;
use crate::features::character::leveling;
use crate::features::character::models::{CharacterSkill, CharacterXp};
use crate::features::character::types::SkillId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CharacterResponse {
    pub total_xp: i64,
    pub level: i32,
    /// XP needed at the threshold of the *next* character level (cumulative).
    pub xp_to_next_level: i64,
    /// Current XP minus the threshold of the current level (progress in current level).
    pub xp_progress_in_level: i64,
    /// Total span of the current level (xp_to_next_level - cumulative_at_current).
    pub xp_span_of_level: i64,
    pub unspent_sp: i32,
    pub total_sp_granted: i32,
    pub total_sp_spent: i32,
    pub skills: Vec<CharacterSkillResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct CharacterSkillResponse {
    pub skill_id: SkillId,
    pub level: i32,
    pub max_level: i32,
    /// `None` when at max.
    pub next_level_cost: Option<i32>,
    /// Per-mille multiplier (1000 = 1.0×, 1500 = 1.5×).
    pub multiplier_permille: i32,
    /// Whether this skill is part of the MVP set; non-MVP skills appear in
    /// the response with level=0 and `next_level_cost=None` so the UI can
    /// render a "coming soon" tile if it wants to.
    pub is_released: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "server", derive(schemars::JsonSchema, aide::OperationIo))]
pub struct PublicCharacterResponse {
    /// Only level is exposed publicly (per spec Q5).
    pub level: i32,
}

impl CharacterResponse {
    pub fn from_parts(xp: &CharacterXp, skills: Vec<(SkillId, i32)>) -> Self {
        let cur_threshold = leveling::cumulative_xp_at_level(xp.level);
        let next_threshold = leveling::cumulative_xp_at_level(xp.level + 1);
        let unspent = xp.unspent_sp();

        let mvp = [SkillId::MoneyTree, SkillId::Ranker];
        let v2 = [SkillId::Influencer, SkillId::Sweeper];

        let level_for = |id: SkillId| -> i32 {
            skills.iter().find(|(s, _)| *s == id).map(|(_, l)| *l).unwrap_or(0)
        };

        let mut response_skills = Vec::with_capacity(4);
        for id in mvp.iter().copied() {
            let lv = level_for(id);
            response_skills.push(CharacterSkillResponse {
                skill_id: id,
                level: lv,
                max_level: leveling::MAX_SKILL_LEVEL,
                next_level_cost: leveling::skill_cost_next(lv),
                multiplier_permille: leveling::multiplier_permille(lv),
                is_released: true,
            });
        }
        for id in v2.iter().copied() {
            response_skills.push(CharacterSkillResponse {
                skill_id: id,
                level: 0,
                max_level: leveling::MAX_SKILL_LEVEL,
                next_level_cost: None,
                multiplier_permille: 1000,
                is_released: false,
            });
        }

        Self {
            total_xp: xp.total_xp,
            level: xp.level,
            xp_to_next_level: next_threshold,
            xp_progress_in_level: xp.total_xp - cur_threshold,
            xp_span_of_level: next_threshold - cur_threshold,
            unspent_sp: unspent,
            total_sp_granted: xp.total_sp_granted,
            total_sp_spent: xp.total_sp_spent,
            skills: response_skills,
        }
    }
}
```

- [ ] **Step 3: Build check + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
git add app/ratel/src/features/character/dto/
git commit -m "feat(character): CharacterResponse + PublicCharacterResponse DTOs"
```

---

## Task 18: Create `get_character_handler`

**Files:**
- Create: `app/ratel/src/features/character/controllers/mod.rs`
- Create: `app/ratel/src/features/character/controllers/get_character.rs`

- [ ] **Step 1: controllers/mod.rs**

```rust
mod get_character;
mod get_public_character;
mod level_up;

pub use get_character::*;
pub use get_public_character::*;
pub use level_up::*;
```

(Create empty placeholder files for `get_public_character.rs` and `level_up.rs` for now; populate in later tasks.)

- [ ] **Step 2: Write `get_character.rs`**

```rust
use crate::common::*;
use crate::features::auth::User;
use crate::features::character::dto::CharacterResponse;
use crate::features::character::models::{CharacterSkill, CharacterXp};
use crate::features::character::types::SkillId;

#[get("/api/me/character", user: User)]
pub async fn get_character_handler() -> Result<CharacterResponse> {
    #[cfg(feature = "server")]
    {
        let cfg = crate::common::CommonConfig::default();
        let cli = cfg.dynamodb();

        // One DynamoDB call per logical resource — CharacterXp + the full
        // skill collection — issued in parallel for ~1 RTT total. Avoids
        // the N+1 anti-pattern of looping `level_or_zero` per skill.
        let (xp_pk, xp_sk) = CharacterXp::user_keys(&user.pk);
        let (xp_res, skill_rows) = tokio::try_join!(
            CharacterXp::get(cli, &xp_pk, Some(&xp_sk)),
            CharacterSkill::list_for_user(cli, &user.pk),
        )?;
        let xp = xp_res.unwrap_or_else(|| CharacterXp::new(user.pk.clone()));
        let skills = CharacterSkill::levels_by_id(&skill_rows);

        Ok(CharacterResponse::from_parts(&xp, skills))
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}
```

- [ ] **Step 3: Build check + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
git add app/ratel/src/features/character/controllers/
git commit -m "feat(character): GET /api/me/character handler"
```

---

## Task 19: Test `get_character_handler`

**Files:**
- Modify: `app/ratel/src/features/character/tests/character_xp_tests.rs`

- [ ] **Step 1: Append tests** (top of file already has `use super::helpers::*;`)

```rust
#[tokio::test]
async fn test_get_character_unauthenticated_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/me/character",
    };
    assert_ne!(status, 200);
}

#[tokio::test]
async fn test_get_character_brand_new_user_returns_default() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/me/character",
        headers: ctx.test_user.1.clone(),
        response_type: crate::features::character::dto::CharacterResponse,
    };
    assert_eq!(status, 200, "brand new user: {:?}", body);
    assert_eq!(body.total_xp, 0);
    assert_eq!(body.level, 1);
    assert_eq!(body.unspent_sp, 5);
    assert_eq!(body.skills.len(), 4);
    let mt = body.skills.iter().find(|s| matches!(s.skill_id, crate::features::character::types::SkillId::MoneyTree)).unwrap();
    assert_eq!(mt.level, 0);
    assert_eq!(mt.next_level_cost, Some(5));
}

#[tokio::test]
async fn test_get_character_after_xp_delta() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    let score = make_score(&user_pk, "space-a", 5_000);
    crate::features::character::services::apply_character_xp_delta(ctx.ddb, score).await.unwrap();

    let (_, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/me/character",
        headers: ctx.test_user.1.clone(),
        response_type: crate::features::character::dto::CharacterResponse,
    };
    assert_eq!(body.total_xp, 5_000);
    assert_eq!(body.level, 4);
    assert_eq!(body.unspent_sp, 20);
}
```

- [ ] **Step 2: Run + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::character_xp_tests::test_get_character
git add app/ratel/src/features/character/tests/character_xp_tests.rs
git commit -m "test(character): get_character handler — unauth/default/post-XP"
```

---

## Task 20: Create `get_public_character_handler`

**Files:**
- Modify: `app/ratel/src/features/character/controllers/get_public_character.rs`

- [ ] **Step 1: Write the handler**

```rust
use crate::common::*;
use crate::common::models::auth::User;
use crate::features::character::dto::PublicCharacterResponse;
use crate::features::character::models::CharacterXp;

#[get("/api/users/{username}/character")]
pub async fn get_public_character_handler(username: String) -> Result<PublicCharacterResponse> {
    #[cfg(feature = "server")]
    {
        let cfg = crate::common::CommonConfig::default();
        let cli = cfg.dynamodb();

        // Look up user by username. Returns 404 (NotFound) on miss.
        let opt = User::opt().limit(1);
        let (users, _) = User::find_by_username(cli, &username, opt).await?;
        let target = users.into_iter().next().ok_or(Error::NotFound)?;

        let (xp_pk, xp_sk) = CharacterXp::user_keys(&target.pk);
        let xp = CharacterXp::get(cli, &xp_pk, Some(&xp_sk)).await?;
        let level = xp.map(|x| x.level).unwrap_or(1);

        Ok(PublicCharacterResponse { level })
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}
```

- [ ] **Step 2: Build + tests**

Append to `app/ratel/src/features/character/tests/character_xp_tests.rs`:

```rust
#[tokio::test]
async fn test_get_public_character_returns_level_only() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    crate::features::character::services::apply_character_xp_delta(
        ctx.ddb, make_score(&user_pk, "s", 5_000),
    ).await.unwrap();

    let username = ctx.test_user.0.username.clone();
    let (status, _, body) = crate::test_get! {
        app: ctx.app.clone(),
        path: &format!("/api/users/{}/character", username),
        response_type: crate::features::character::dto::PublicCharacterResponse,
    };
    assert_eq!(status, 200, "{:?}", body);
    assert_eq!(body.level, 4);
}

#[tokio::test]
async fn test_get_public_character_unknown_user_404() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_get! {
        app: ctx.app.clone(),
        path: "/api/users/no-such-user-asdf/character",
    };
    assert_eq!(status, 404);
}
```

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::character_xp_tests::test_get_public_character`

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/character/controllers/get_public_character.rs app/ratel/src/features/character/tests/character_xp_tests.rs
git commit -m "feat(character): GET /api/users/:username/character (public, level-only)"
```

---

## Task 21: Create `level_up_handler`

**Files:**
- Modify: `app/ratel/src/features/character/controllers/level_up.rs`

- [ ] **Step 1: Write the handler**

```rust
use crate::common::*;
use crate::features::auth::User;
use crate::features::character::dto::CharacterResponse;
use crate::features::character::leveling;
use crate::features::character::models::{CharacterSkill, CharacterXp};
use crate::features::character::types::{CharacterError, SkillId};

#[post("/api/me/skills/{skill_id}/level-up", user: User)]
pub async fn level_up_handler(skill_id: String) -> Result<CharacterResponse> {
    #[cfg(feature = "server")]
    {
        let id = SkillId::from_str(&skill_id).ok_or(CharacterError::SkillNotFound)?;
        if !id.is_mvp() {
            return Err(CharacterError::SkillNotReleased.into());
        }

        let cfg = crate::common::CommonConfig::default();
        let cli = cfg.dynamodb();

        let (xp_pk, xp_sk) = CharacterXp::user_keys(&user.pk);
        let mut xp = CharacterXp::get(cli, &xp_pk, Some(&xp_sk))
            .await?
            .unwrap_or_else(|| CharacterXp::new(user.pk.clone()));

        let cur_level = CharacterSkill::level_or_zero(cli, &user.pk, id).await?;
        let cost = leveling::skill_cost_next(cur_level)
            .ok_or(CharacterError::AlreadyMaxLevel)?;
        if xp.unspent_sp() < cost {
            return Err(CharacterError::InsufficientSp.into());
        }

        let now = crate::common::utils::time::get_now_timestamp_millis();

        // Persist new skill level (upsert).
        let (sk_pk, sk_sk) = CharacterSkill::keys(&user.pk, id);
        let new_skill_level = cur_level + 1;
        if cur_level == 0 {
            CharacterSkill {
                pk: sk_pk.clone(),
                sk: sk_sk.clone(),
                level: new_skill_level,
                created_at: now,
                updated_at: now,
            }
            .create(cli)
            .await?;
        } else {
            CharacterSkill::updater(&sk_pk, &sk_sk)
                .with_level(new_skill_level)
                .with_updated_at(now)
                .execute(cli)
                .await?;
        }

        // Bump total_sp_spent.
        xp.total_sp_spent += cost;
        if xp.created_at == 0 {
            // First-time row — must insert.
            xp.create(cli).await?;
        } else {
            CharacterXp::updater(&xp_pk, &xp_sk)
                .with_total_sp_spent(xp.total_sp_spent)
                .with_updated_at(now)
                .execute(cli)
                .await?;
        }

        // Re-read assembled state for the response. Fan out the two
        // reads in parallel (CharacterXp single-row + CharacterSkill
        // collection) so the post-mutation latency stays at ~1 RTT.
        let (xp_res, skill_rows) = tokio::try_join!(
            CharacterXp::get(cli, &xp_pk, Some(&xp_sk)),
            CharacterSkill::list_for_user(cli, &user.pk),
        )?;
        let xp = xp_res.ok_or(Error::NotFound)?;
        let skills = CharacterSkill::levels_by_id(&skill_rows);

        Ok(CharacterResponse::from_parts(&xp, skills))
    }
    #[cfg(not(feature = "server"))]
    unreachable!()
}
```

- [ ] **Step 2: Build check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/character/controllers/level_up.rs
git commit -m "feat(character): POST /api/me/skills/:skill_id/level-up handler"
```

---

## Task 22: Test `level_up_handler`

**Files:**
- Modify: `app/ratel/src/features/character/tests/skill_tests.rs`

- [ ] **Step 1: Replace placeholder with tests** (`award_xp` lives in `helpers.rs`, no need to redefine):

Write `app/ratel/src/features/character/tests/skill_tests.rs`:

```rust
use super::helpers::*;

#[tokio::test]
async fn test_level_up_money_tree_l1_success() {
    let ctx = TestContext::setup().await;
    let (status, _, body) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
        response_type: crate::features::character::dto::CharacterResponse,
    };
    assert_eq!(status, 200, "{:?}", body);
    let mt = body.skills.iter().find(|s| matches!(s.skill_id, crate::features::character::types::SkillId::MoneyTree)).unwrap();
    assert_eq!(mt.level, 1);
    assert_eq!(mt.multiplier_permille, 1050);
    assert_eq!(body.unspent_sp, 0); // L1 char = 5 SP, spent 5
    assert_eq!(body.total_sp_spent, 5);
}

#[tokio::test]
async fn test_level_up_insufficient_sp_rejected() {
    let ctx = TestContext::setup().await;
    // Brand new user has 5 SP. Buying MoneyTree L1 (5) is fine; trying L2 (cost 9) without more XP should fail.
    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };
    assert_eq!(status, 400, "second level-up without XP should be rejected");
}

#[tokio::test]
async fn test_level_up_unknown_skill_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/no_such_skill/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };
    assert_eq!(status, 400);
}

#[tokio::test]
async fn test_level_up_v2_skill_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/influencer/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };
    assert_eq!(status, 400, "v2 skill must be gated");
}

#[tokio::test]
async fn test_level_up_max_level_rejected() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();
    // Pump in enough XP to easily afford max-out: 230 SP needs char L46, so 7M XP is plenty.
    award_xp(&ctx, &user_pk, 8_000_000).await;
    for _ in 0..10 {
        let (status, _, _) = crate::test_post! {
            app: ctx.app.clone(),
            path: "/api/me/skills/money_tree/level-up",
            headers: ctx.test_user.1.clone(),
            body: {},
        };
        assert_eq!(status, 200);
    }
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };
    assert_eq!(status, 400, "11th level-up must be rejected");
}

#[tokio::test]
async fn test_level_up_unauthenticated_rejected() {
    let ctx = TestContext::setup().await;
    let (status, _, _) = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        body: {},
    };
    assert_ne!(status, 200);
}
```

- [ ] **Step 2: Run + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::skill_tests::test_level_up
git add app/ratel/src/features/character/tests/skill_tests.rs
git commit -m "test(character): level_up — success/insufficient/unknown/v2/max/unauth"
```

---

## Task 23: Wire Money Tree multiplier in `SpaceReward::award`

**Files:**
- Modify: `app/ratel/src/features/spaces/space_common/models/space_reward.rs:167-...`

- [ ] **Step 1: Read current `award()` implementation** (lines 167–230) to confirm where `space_reward.get_amount()` is computed and where it's added to `UserReward.total_points` / `User.points`.

- [ ] **Step 2: Add helper near top of impl block**

```rust
#[cfg(feature = "server")]
impl SpaceReward {
    async fn money_tree_multiplier_permille(
        cli: &aws_sdk_dynamodb::Client,
        target_pk: &Partition,
    ) -> i32 {
        use crate::features::character::leveling::multiplier_permille;
        use crate::features::character::models::CharacterSkill;
        use crate::features::character::types::SkillId;

        match CharacterSkill::level_or_zero(cli, target_pk, SkillId::MoneyTree).await {
            Ok(lv) => multiplier_permille(lv),
            Err(e) => {
                tracing::warn!(target_pk = %target_pk, error = %e, "money_tree lookup failed; defaulting to 1.0×");
                1000
            }
        }
    }
}
```

- [ ] **Step 3: Wrap the amount calculation**

Find: `let amount = space_reward.get_amount();`

Replace with:

```rust
        let raw_amount = space_reward.get_amount();
        let multiplier_permille = Self::money_tree_multiplier_permille(cli, &target_pk).await;
        let amount = crate::features::character::leveling::apply_permille(raw_amount, multiplier_permille);
        let money_tree_bonus = amount - raw_amount;
        if money_tree_bonus > 0 {
            tracing::info!(
                target_pk = %target_pk,
                raw_amount,
                multiplier_permille,
                bonus = money_tree_bonus,
                "money tree bonus applied"
            );
        }
```

The `amount` variable downstream is unchanged in usage; only its computation differs. The bonus is logged for observability now; the UI breakdown is a separate frontend task.

> **Important:** the owner-bonus payout (the creator's 10% cut) must NOT be boosted by Money Tree. Inspect the existing `award` body for any "owner_pk" branch that also computes an amount; that branch must continue to use `raw_amount * 0.10` (or whatever the existing fraction is), not `amount`.

- [ ] **Step 4: Build + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
git add app/ratel/src/features/spaces/space_common/models/space_reward.rs
git commit -m "feat(character): Money Tree boosts participant payout in SpaceReward::award"
```

---

## Task 24: Test Money Tree end-to-end

**Files:**
- Modify: `app/ratel/src/features/character/tests/skill_tests.rs`

- [ ] **Step 1: Append test**

```rust
#[tokio::test]
async fn test_money_tree_boosts_user_reward_amount() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    // Buy MoneyTree L1.
    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/money_tree/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };

    // Fabricate a SpaceReward and call award directly.
    use crate::common::types::*;
    use crate::features::spaces::space_common::models::space_reward::SpaceReward;

    let space_id = SpacePartition("space-fixture".to_string());
    let reward = SpaceReward::new(
        space_id.clone(),
        "action-1".into(),
        RewardUserBehavior::PollAnswer,
        "test reward".into(),
        1, // credits
        10_000, // point
        RewardPeriod::Forever,
        RewardCondition::None,
    );
    reward.create(ctx.ddb).await.unwrap();

    let user_reward = SpaceReward::award(ctx.ddb, &reward, user_pk.clone(), None)
        .await
        .unwrap();

    // L1 = +5%, so 10_000 → 10_500
    assert_eq!(user_reward.total_points, 10_500, "+5% boost expected");
}
```

- [ ] **Step 2: Run + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::skill_tests::test_money_tree_boosts
git add app/ratel/src/features/character/tests/skill_tests.rs
git commit -m "test(character): Money Tree L1 boosts UserReward by 5%"
```

---

## Task 25: Wire Ranker multiplier in `SpaceActivity::new_with_dedup`

**Files:**
- Modify: `app/ratel/src/features/activity/models/space_activity.rs`

- [ ] **Step 1: Read current `new_with_dedup`** to find `let total_score = base_score + additional_score;`

- [ ] **Step 2: Make the function async** so it can read the user's Ranker level. Update the signature:

```rust
    pub async fn new_with_dedup(
        cli: &aws_sdk_dynamodb::Client,
        space_id: SpacePartition,
        author: AuthorPartition,
        action_id: String,
        action_type: SpaceActionType,
        data: SpaceActivityData,
        base_score: i64,
        additional_score: i64,
        user_name: String,
        user_avatar: String,
        dedup_key: String,
    ) -> Self {
        let now = crate::common::utils::time::get_now_timestamp_millis();
        let space_pk: Partition = space_id.clone().into();
        let user_pk: Partition = author.clone().into();

        // Apply Ranker boost to additional_score only (spec FR17).
        let mult_permille = {
            use crate::features::character::leveling::multiplier_permille;
            use crate::features::character::models::CharacterSkill;
            use crate::features::character::types::SkillId;
            CharacterSkill::level_or_zero(cli, &user_pk, SkillId::Ranker)
                .await
                .map(multiplier_permille)
                .unwrap_or(1000)
        };
        let boosted_additional = crate::features::character::leveling::apply_permille(additional_score, mult_permille);
        let total_score = base_score + boosted_additional;

        let sk = EntityType::SpaceActivity(format!("{}#{}", dedup_key, now));

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
            additional_score: boosted_additional,
            total_score,
        }
    }
```

Update `new` (the non-dedup wrapper) similarly to take `cli` and become `async`.

- [ ] **Step 3: Update all callers** of `SpaceActivity::new` and `new_with_dedup` to pass `cli` and `.await`. Find them with:

```bash
grep -rn "SpaceActivity::new\|SpaceActivity::new_with_dedup" app/ratel/src --include="*.rs"
```

Each caller needs to add the `cli` argument. Most will already have a `cli` in scope (e.g., the `handle_xp_event.rs` services).

- [ ] **Step 4: Build + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
git add app/ratel/src/features/activity/ app/ratel/src/features/spaces/
git commit -m "feat(character): Ranker boosts SpaceActivity.additional_score at creation"
```

---

## Task 26: Test Ranker end-to-end

**Files:**
- Modify: `app/ratel/src/features/character/tests/skill_tests.rs`

- [ ] **Step 1: Append test**

```rust
#[tokio::test]
async fn test_ranker_boosts_additional_score() {
    let ctx = TestContext::setup().await;
    let user_pk = ctx.test_user.0.pk.clone();

    let _ = crate::test_post! {
        app: ctx.app.clone(),
        path: "/api/me/skills/ranker/level-up",
        headers: ctx.test_user.1.clone(),
        body: {},
    };

    use crate::common::types::*;
    use crate::features::activity::models::SpaceActivity;
    use crate::features::activity::types::SpaceActivityData;
    use crate::features::spaces::pages::actions::types::SpaceActionType;

    let author = AuthorPartition(match &user_pk {
        Partition::User(s) => s.clone(),
        _ => unreachable!(),
    });

    let activity = SpaceActivity::new_with_dedup(
        ctx.ddb,
        SpacePartition("space-fixture".into()),
        author,
        "action-1".into(),
        SpaceActionType::Poll,
        SpaceActivityData::default(),
        100, // base
        50,  // additional, boosted
        "u".into(),
        "".into(),
        "dedup-1".into(),
    ).await;

    // Ranker L1 = +5% → 50 × 1.05 = 53 (rounded). total_score = 100 + 53 = 153.
    assert_eq!(activity.additional_score, 53);
    assert_eq!(activity.total_score, 153);
    assert_eq!(activity.base_score, 100, "base unchanged");
}
```

- [ ] **Step 2: Run + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::skill_tests::test_ranker_boosts
git add app/ratel/src/features/character/tests/skill_tests.rs
git commit -m "test(character): Ranker L1 boosts additional_score by 5%"
```

---

## Task 27: Create migration `m001_backfill_character_xp`

**Files:**
- Create: `app/ratel/src/common/migrations/m001_backfill_character_xp.rs`
- Modify: `app/ratel/src/common/migrations/runner.rs`

- [ ] **Step 1: Write the migration**

```rust
//! Migration 001 — backfill CharacterXp from existing SpaceScore rows.
//! Idempotent: re-running computes the same end state.

use crate::common::*;
use crate::features::activity::models::SpaceScore;
use crate::features::character::leveling;
use crate::features::character::models::{CharacterXp, CharacterXpSource};
use std::collections::HashMap;

pub async fn run(cli: &aws_sdk_dynamodb::Client) -> crate::common::Result<()> {
    tracing::info!("m001: scanning SpaceScore rows");

    // Aggregate per-user totals and collect per-(user, space) last_seen.
    let mut totals: HashMap<Partition, i64> = HashMap::new();
    let mut sources: Vec<(Partition, String, i64)> = Vec::new();

    let mut bookmark: Option<String> = None;
    let mut pages = 0;
    loop {
        pages += 1;
        if pages > 1_000 {
            return Err(Error::Internal(
                "m001 exceeded 1000 pages; aborting".into(),
            ));
        }

        let opts = SpaceScore::opt_with_bookmark(bookmark.clone()).limit(500);
        // Scan via the existing find_by_space_rank GSI (no per-row filter; we want all).
        // Fall back to a full table scan if the GSI scan API isn't surfaced.
        let (rows, next) = SpaceScore::scan_all(cli, opts).await?;
        for row in rows {
            let user_pk: Partition = row.user_pk.into();
            *totals.entry(user_pk.clone()).or_insert(0) += row.total_score;

            if let Partition::Space(space_id) = row.space_pk {
                sources.push((user_pk, space_id, row.total_score));
            }
        }
        if next.is_none() {
            break;
        }
        bookmark = next;
    }

    tracing::info!(users = totals.len(), sources = sources.len(), "m001: aggregation done; writing");

    // Upsert CharacterXp per user. Fan out with bounded concurrency
    // (16 in flight) so a 10k-user backfill finishes in seconds, not
    // tens of minutes. `try_join_all` collects errors; `buffer_unordered`
    // gives us back-pressure without unbounded spawn.
    use futures::stream::{self, StreamExt};

    let now = crate::common::utils::time::get_now_timestamp_millis();
    let xp_writes = stream::iter(totals.into_iter().map(|(user_pk, total_xp)| {
        let level = leveling::level_from_xp(total_xp);
        let row = CharacterXp {
            pk: user_pk,
            sk: EntityType::CharacterXp,
            created_at: now,
            updated_at: now,
            total_xp,
            level,
            total_sp_granted: leveling::total_sp_granted(level),
            total_sp_spent: 0,
        };
        async move { row.put(cli).await }
    }))
    .buffer_unordered(16);
    xp_writes
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<crate::common::Result<Vec<_>>>()?;

    let src_writes = stream::iter(sources.into_iter().map(|(user_pk, space_id, last_seen)| {
        let row = CharacterXpSource {
            pk: user_pk,
            sk: EntityType::CharacterXpSource(space_id),
            last_seen_score: last_seen,
            updated_at: now,
        };
        async move { row.put(cli).await }
    }))
    .buffer_unordered(16);
    src_writes
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<crate::common::Result<Vec<_>>>()?;

    tracing::info!("m001: complete");
    Ok(())
}
```

> Note: `SpaceScore::scan_all` and `Self::put` are assumed on the entity API. If not present, the existing `find_by_space_rank` GSI iteration via `find_by_*` paginated calls is the equivalent — adapt accordingly. `put` may need to be `create` with a "replace if exists" overload; if `create` errors on duplicate, switch to `Self::updater(...).execute()` after a get.

- [ ] **Step 2: Register in `runner.rs`**

In `app/ratel/src/common/migrations/runner.rs` uncomment / add:

```rust
mod m001_backfill_character_xp;

// inside run_migrations, replacing the example comment:
    if stored < 1 {
        tracing::info!("running migration 001: backfill_character_xp");
        m001_backfill_character_xp::run(cli).await?;
        LastBackfillVersion::advance_to(cli, stored, 1).await?;
        tracing::info!("migration 001 complete; version advanced to 1");
    }
```

- [ ] **Step 3: Build check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/migrations/
git commit -m "feat(migrations): m001 — backfill CharacterXp from SpaceScore"
```

---

## Task 28: Test migration framework end-to-end

**Files:**
- Modify: `app/ratel/src/features/character/tests/migration_tests.rs`

- [ ] **Step 1: Append tests** (`run_with_env` already lives in `helpers.rs`)

```rust
#[tokio::test]
async fn test_run_migrations_skips_when_migrate_unset() {
    let ctx = TestContext::setup().await;
    std::env::remove_var("MIGRATE");
    crate::common::migrations::run_migrations(ctx.ddb).await.unwrap();

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk)).await.unwrap();
    assert!(row.is_none(), "MIGRATE unset must not advance version");
}

#[tokio::test]
async fn test_run_migrations_runs_m001() {
    let ctx = TestContext::setup().await;
    // Seed a SpaceScore so the backfill has work to do.
    use crate::features::activity::models::SpaceScore;
    let user_pk = ctx.test_user.0.pk.clone();
    let space_part = SpacePartition("seed".into());
    let author = AuthorPartition(match &user_pk { Partition::User(s)=>s.clone(), _=>unreachable!() });
    let mut s = SpaceScore::new(space_part, author, "u".into(), "".into());
    s.total_score = 5_000;
    s.create(ctx.ddb).await.unwrap();

    run_with_env("MIGRATE", "true", || async {
        crate::common::migrations::run_migrations(ctx.ddb).await.unwrap();
    }).await;

    // Verify version advanced.
    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(row.version, 1);

    // Verify CharacterXp seeded.
    use crate::features::character::models::CharacterXp;
    let (xpk, xsk) = CharacterXp::user_keys(&user_pk);
    let xp = CharacterXp::get(ctx.ddb, &xpk, Some(&xsk)).await.unwrap().unwrap();
    assert_eq!(xp.total_xp, 5_000);
    assert_eq!(xp.level, 4);
}

#[tokio::test]
async fn test_run_migrations_idempotent_at_version() {
    let ctx = TestContext::setup().await;
    LastBackfillVersion::advance_to(ctx.ddb, 0, 1).await.unwrap();

    run_with_env("MIGRATE", "true", || async {
        crate::common::migrations::run_migrations(ctx.ddb).await.unwrap();
    }).await;

    let (pk, sk) = LastBackfillVersion::singleton_keys();
    let row = LastBackfillVersion::get(ctx.ddb, &pk, Some(&sk)).await.unwrap().unwrap();
    assert_eq!(row.version, 1, "no further migrations to run");
}
```

- [ ] **Step 2: Run + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests::migration_tests
git add app/ratel/src/features/character/tests/migration_tests.rs
git commit -m "test(migrations): MIGRATE gate + m001 + idempotent re-run"
```

---

## Task 29: Wire `character` routes into the dioxus router

**Files:**
- Create: `app/ratel/src/features/character/route.rs` (placeholder for now — Stage 3 frontend will fill)
- Modify: `app/ratel/src/features/character/mod.rs` (re-export)
- Confirm controllers are auto-mounted (the `#[get]`/`#[post]` macros in this codebase usually attach to the dioxus server router automatically; verify via `dx serve` health).

- [ ] **Step 1: Stub `route.rs`**

```rust
// Frontend routes are added in the Stage 3 frontend follow-up plan.
// This file exists so module hierarchy is consistent.
```

- [ ] **Step 2: Verify endpoints respond**

Run dev server in one terminal: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web`
In another: `curl -s http://localhost:8000/api/users/somebody/character` — expect 404 (or auth error if the route is gated, which it shouldn't be — `get_public_character_handler` has no `user:` arg, so it's open).

- [ ] **Step 3: Commit (whatever stub was added)**

```bash
git add app/ratel/src/features/character/
git commit -m "chore(character): scaffold route.rs placeholder for Stage 3"
```

---

## Task 30: Lint & format all touched files

- [ ] **Step 1: Find all `.rs` files modified in the branch**

```bash
git diff --name-only origin/dev | grep -E '\.rs$'
```

- [ ] **Step 2: Apply rustywind + dx fmt**

For each `.rs` file from Step 1:

```bash
rustywind --custom-regex 'class: "(.*)"' --write <file>
dx fmt -f <file>
```

- [ ] **Step 3: Verify final builds**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
```

All three must pass with zero warnings.

- [ ] **Step 4: Run full test suite**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- features::character::tests
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/
git commit -m "style: rustywind + dx fmt on character feature"
```

---

## Task 31: Update spec acceptance checklist

**Files:**
- Modify: `roadmap/character-xp-skills.md` — flip implemented criteria from `- [ ]` to `- [x]`.

- [ ] **Step 1: Mark backend acceptance criteria** that are now passing tests:
  - Earning XP in a space → CharacterXp delta
  - Stream replay idempotent
  - Crossing a level threshold grants `5L` SP
  - Spending 5 SP on Money Tree raises to L1
  - Spending 5 SP on Ranker raises to L1
  - Skill > L10 rejected
  - Spending more SP than user has rejected
  - Backfill produces same `CharacterXp.total_xp` whether run once or three times
  - `MIGRATE` unset doesn't run backfill
  - `MIGRATE=true` after backfill is no-op
  - Skill cost ramp `5,9,13,...,41` cumulative `5,14,...,230`

- [ ] **Step 2: Leave frontend criteria unchecked** — those flip in the Stage 3 frontend plan.

- [ ] **Step 3: Commit**

```bash
git add roadmap/character-xp-skills.md
git commit -m "docs(roadmap): mark backend acceptance criteria as shipped"
```

---

## Self-review checklist (run at end of plan execution)

Before marking the backend plan complete, verify:

- [ ] All 31 tasks committed (`git log --oneline origin/dev..HEAD` shows 31+ commits).
- [ ] `cargo check --features server` passes with `-D warnings`.
- [ ] `dx check --features web` passes with `-D warnings`.
- [ ] All test files under `app/ratel/src/features/character/tests/` pass under `--features "full,bypass"` via `cargo test -- features::character::tests`.
- [ ] No test code from this feature lives outside `features/character/tests/` (the project-wide `app/ratel/src/tests/<feature>_tests.rs` convention is intentionally NOT followed for this feature; see plan header note).
- [ ] No `TODO`, `FIXME`, or `unimplemented!()` left in `features/character/` or `common/migrations/`.
- [ ] Owner-bonus payout in `SpaceReward::award` is **not** boosted by Money Tree (Task 23 gotcha).
- [ ] `Ranker` multiplier is applied to `additional_score` only, not `base_score` (Task 25).
- [ ] `Self::put` and `Self::scan_all` calls in m001 resolve against the actual `DynamoEntity` API; if they don't, adapt to `create` + `updater` and the existing `find_by_*` pagination.
- [ ] `CharacterSkill::find_by_pk_with_sk_prefix` is exposed by `packages/by-macros`. If not, add it as a small helper that issues `Query` with `KeyConditionExpression = "pk = :pk AND begins_with(sk, :prefix)"`. **Do not** fall back to the per-skill loop pattern — the whole point of `list_for_user` is to keep `/api/me/character` at one Query (vs. four GetItems). `tokio::try_join!` of four GetItems is the *worst* fallback (still 4 reads, just parallelized — same DynamoDB cost, more connection overhead).
- [ ] `m001` backfill writes use `buffer_unordered(16)` not a sequential loop. Verify by inspecting wall-clock time on a seeded test (10 users × 3 spaces should finish in <1s).

## Stage 3 frontend (in this plan — Tasks 32–43)

Stage 2 mockups are committed at `app/ratel/assets/design/character-xp-skills/`:

- `character-page.html` — `/me/character` page (uses the same ArenaTopbar as Home Arena; Character hud-btn carries `aria-current="page"`).
- `reward-breakdown.html` — Money Tree bonus surfaced in user_reward views (3 variants; we ship Variant A "inline breakdown row").
- `public-profile-badge.html` — visitor view (Variant A "header chip" + Variant B "inline mini badge"; we ship the large header chip on profile pages).

Class names + element IDs from the mockups are the **contract** — they stay identical through RSX conversion.

**Entry point.** Per PO directive: the Character page is reached from the Home Arena (`/`) topbar, by clicking a new "Character" hud-btn (Lucide `award` icon). The Character page itself reuses the same ArenaTopbar layout — there are NO Posts/Spaces/Rewards tabs above the page content; the hud-btn for Character carries `aria-current="page"` to indicate the active section.

---

## Task 32: Extract `ArenaTopbar` shared layout component

The current Home Arena topbar lives inline in `app/ratel/src/views/index/component.rs:207-376`. It needs to be extracted into a reusable component so the Character page can render the same bar with the Character hud-btn marked active.

**Files:**
- Create: `app/ratel/src/components/arena_topbar/mod.rs`
- Create: `app/ratel/src/components/arena_topbar/component.rs`
- Modify: `app/ratel/src/views/index/component.rs` — replace inline markup with `<ArenaTopbar active={None} />`
- Modify: `app/ratel/src/components/mod.rs` — `pub mod arena_topbar; pub use arena_topbar::*;`

> Note: the existing `app/ratel/src/features/social/pages/team_arena/topbar/` `ArenaTopbar` is a different component (team-arena-specific). We're extracting the **Home/Character** topbar; do not collide. Place this one under `crate::components::arena_topbar` (root-level, like `notification_bell`).

- [ ] **Step 1: Create `mod.rs`**

```rust
mod component;
pub use component::*;
```

- [ ] **Step 2: Define active-section enum**

In `component.rs`:

```rust
use crate::common::*;
use crate::route::Route;
use dioxus::prelude::*;

/// Which top-level section is currently active. Drives `aria-current="page"`
/// on the matching hud-btn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArenaTopbarSection {
    Home,
    Character,
    // Drafts / Rewards / Credentials / MyAi / Essence / Settings can be
    // added later as those pages adopt this layout.
}
```

- [ ] **Step 3: Move the topbar markup verbatim** from `views/index/component.rs:207-376` into the new component, parameterised by `active: Option<ArenaTopbarSection>`. Each hud-btn gets `aria_current: matches_active(active, Section::Foo).then_some("page")`.

For brevity here, the new component re-uses every existing hud-btn (Create, Drafts, Rewards, Credentials, MyAi, Essence, Teams, Settings, Notifications) and adds the **Character** hud-btn between Credentials and MyAi:

```rust
button {
    class: "hud-btn",
    aria_label: "{t.character}",
    aria_current: (active == Some(ArenaTopbarSection::Character)).then_some("page"),
    "data-testid": "home-btn-character",
    onclick: move |_| nav.push(Route::CharacterPage {}),
    svg {
        fill: "none", stroke: "currentColor",
        stroke_linecap: "round", stroke_linejoin: "round",
        stroke_width: "1.6", view_box: "0 0 24 24",
        xmlns: "http://www.w3.org/2000/svg",
        circle { cx: "12", cy: "8", r: "6" }
        path { d: "M15.477 12.89 17 22l-5-3-5 3 1.523-9.11" }
    }
    span { class: "hud-btn__label", "{t.character}" }
}
```

The component owns its own translations (i18n.rs) for the labels — including the new `character` key — and accepts `#[props] active: Option<ArenaTopbarSection>` plus the click-handler dependencies (popup for login modal, nav for routes, signals for popovers like `notifications_open`/`teams_open`). All of those are read from contexts inside the component, not threaded as props, so the call site stays clean.

- [ ] **Step 4: Replace the inline markup in `views/index/component.rs:207-376`** with `ArenaTopbar { active: None }`. The existing `t` translations file shrinks accordingly (the labels now live in `arena_topbar/i18n.rs`).

- [ ] **Step 5: Verify build**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 6: Visual smoke**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```

Open `/`. Topbar must be visually identical to before (no regression). Hover the new Character hud-btn — label "Character" appears.

- [ ] **Step 7: Commit**

```bash
git add app/ratel/src/components/arena_topbar/ app/ratel/src/views/index/component.rs app/ratel/src/components/mod.rs
git commit -m "refactor(home): extract ArenaTopbar; add Character hud-btn"
```

---

## Task 33: Append Character page CSS to `main.css`

Per `conventions/styling.md`, all component CSS lives in `app/ratel/assets/main.css`. Copy the styles from `app/ratel/assets/design/character-xp-skills/character-page.html` (everything inside `<style>...</style>` *except* the topbar/hud-btn rules — those already exist for Home Arena and we don't redefine them) into a new section in `main.css`.

**Files:**
- Modify: `app/ratel/assets/main.css` — append section.

- [ ] **Step 1: Append section marker + styles**

Append to `app/ratel/assets/main.css`:

```css
/* === src/features/character/pages/character_page === */

.hud-btn[aria-current="page"]{background:rgba(252,179,0,0.10);border-color:rgba(252,179,0,0.45);box-shadow:0 0 18px rgba(252,179,0,0.18)}
.hud-btn[aria-current="page"] svg{color:var(--accent-gold)}
.hud-btn[aria-current="page"]::after{content:'';position:absolute;left:50%;bottom:-8px;transform:translateX(-50%);width:24px;height:2px;border-radius:2px;background:var(--accent-gold);box-shadow:0 0 8px rgba(252,179,0,0.6)}
.hud-btn[aria-current="page"] .hud-btn__label{opacity:1;color:var(--accent-gold)}

/* (paste the rest of the rules from character-page.html — .character-arena,
   .character-page, .character-hero{,*}, .skill-grid, .skill-card{,*},
   .levelup-toast{,*}, .section-header{,*}, and the @media (max-width: 720px)
   block. Use the exact selectors from the mockup; do not rename.) */
```

- [ ] **Step 2: Build check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

CSS-only changes don't affect the type checker; this just confirms no build regression.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/assets/main.css
git commit -m "style(character): append character-page CSS to main.css"
```

---

## Task 34: Build `UseCharacter` controller hook

**Files:**
- Create: `app/ratel/src/features/character/hooks/mod.rs`
- Create: `app/ratel/src/features/character/hooks/use_character.rs`
- Modify: `app/ratel/src/features/character/mod.rs` — add `pub mod hooks;`

- [ ] **Step 1: hooks/mod.rs**

```rust
mod use_character;
pub use use_character::*;
```

- [ ] **Step 2: Write the hook** (`async fn` method shape per `conventions/hooks-and-actions.md` — components await `ctx.level_up(id).await` and decide UX; we also expose `level_up_action` as a `use_action` so the button can disable on `.pending()`):

```rust
use crate::common::*;
use crate::features::character::controllers::{
    get_character_handler, level_up_handler,
};
use crate::features::character::dto::CharacterResponse;
use crate::features::character::types::SkillId;
use dioxus::prelude::*;

#[derive(Clone, Copy, DioxusController)]
pub struct UseCharacter {
    pub character: Loader<CharacterResponse>,
    /// `Action<(SkillId,), ()>` — UI binds to `.pending()` to disable the
    /// Level Up button mid-spend; success/failure UX (toast) is owned by
    /// the component via `await ctx.level_up(id)`.
    pub level_up_action: Action<(SkillId,), ()>,
}

impl UseCharacter {
    pub async fn level_up(&mut self, skill_id: SkillId) -> Result<()> {
        let _ = level_up_handler(skill_id.as_str().to_string()).await?;
        self.character.refresh();
        Ok(())
    }
}

#[track_caller]
pub fn use_character() -> UseCharacter {
    use_context::<UseCharacter>()
}

pub fn use_character_provider() -> std::result::Result<UseCharacter, RenderError> {
    if let Some(ctx) = try_use_context::<UseCharacter>() {
        return Ok(ctx);
    }

    let character = use_loader(|| async move { get_character_handler().await })?;

    let mut character_loader = character;
    let level_up_action = use_action(move |id: SkillId| async move {
        let _ = level_up_handler(id.as_str().to_string()).await?;
        character_loader.refresh();
        Ok::<(), crate::common::Error>(())
    });

    Ok(use_context_provider(UseCharacter { character, level_up_action }))
}
```

- [ ] **Step 3: Build check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/character/hooks/ app/ratel/src/features/character/mod.rs
git commit -m "feat(character): UseCharacter controller hook (Loader + level_up Action)"
```

---

## Task 35: Build `CharacterPage` component (RSX conversion)

**Files:**
- Create: `app/ratel/src/features/character/pages/character_page/mod.rs`
- Create: `app/ratel/src/features/character/pages/character_page/component.rs`
- Create: `app/ratel/src/features/character/pages/character_page/i18n.rs`
- Modify: `app/ratel/src/features/character/mod.rs` — add `pub mod pages;`
- Create: `app/ratel/src/features/character/pages/mod.rs`

- [ ] **Step 1: pages scaffolding**

```rust
// pages/mod.rs
mod character_page;
pub use character_page::*;
```

```rust
// pages/character_page/mod.rs
mod component;
mod i18n;
pub use component::*;
pub use i18n::*;
```

- [ ] **Step 2: i18n.rs**

```rust
use dioxus_translate::translate;

translate! {
    CharacterPageTranslate;

    page_title:        { en: "Character", ko: "캐릭터" },
    skill_tree_title:  { en: "Skill Tree", ko: "스킬 트리" },
    skill_tree_hint:   { en: "+5% per level · Max +50% at L10", ko: "레벨당 +5% · L10에서 최대 +50%" },
    level_label:       { en: "Level", ko: "레벨" },
    xp_title:          { en: "Character XP", ko: "캐릭터 XP" },
    xp_to_next:        { en: "{remaining} XP to Level {next_level}", ko: "다음 레벨까지 {remaining} XP" },
    xp_total_earned:   { en: "Total XP earned: {total}", ko: "누적 XP: {total}" },
    sp_label:          { en: "Skill Points", ko: "스킬 포인트" },
    sp_hint_ready:     { en: "{n} points ready to spend", ko: "사용 가능한 포인트 {n}개" },
    sp_hint_empty:     { en: "Earn XP to grant more", ko: "XP를 획득하여 추가 포인트를 받으세요" },
    money_tree_name:   { en: "Money Tree", ko: "머니트리" },
    money_tree_sub:    { en: "RatelPoint earning boost", ko: "RatelPoint 추가 보상" },
    money_tree_desc:   { en: "Boosts every RatelPoint payout you receive from any space's reward, applied multiplicatively before the amount is credited to your balance.", ko: "스페이스에서 받는 모든 RatelPoint 보상에 곱셈으로 적용되어 잔액에 반영되기 전에 추가 지급됩니다." },
    ranker_name:       { en: "Ranker", ko: "랭커" },
    ranker_sub:        { en: "SpaceXP & Character XP boost", ko: "스페이스XP와 캐릭터XP 추가 보상" },
    ranker_desc:       { en: "Boosts the bonus portion of every SpaceActivity you record. Compounds: more XP per action → faster character leveling → more SP for future skills.", ko: "기록되는 SpaceActivity의 추가 점수 부분에 적용됩니다. 행동당 XP가 늘어 → 레벨업이 빨라지고 → 더 많은 SP를 얻습니다." },
    influencer_name:   { en: "Influencer", ko: "인플루언서" },
    influencer_sub:    { en: "Lower Hot threshold for your spaces", ko: "내 스페이스의 Hot 진입 기준 완화" },
    influencer_desc:   { en: "Lowers the participants-required-for-Hot threshold for spaces you own — at L6 your space surfaces with just 4 participants instead of the global 10.", ko: "내가 만든 스페이스의 Hot 진입 기준을 낮춥니다. L6에서 기본 10명 대신 4명만 있어도 Hot에 노출됩니다." },
    sweeper_name:      { en: "Sweeper", ko: "싹쓸이" },
    sweeper_sub:       { en: "Higher owner bonus on your spaces", ko: "내 스페이스의 오너 보너스 증가" },
    sweeper_desc:      { en: "When a participant claims a reward in a space you own, the owner-bonus you receive goes up by +5% per level. At L6 you take 40% of every payout instead of the default 10%.", ko: "내 스페이스에서 참여자가 보상을 받을 때마다 오너 보너스가 레벨당 +5% 증가합니다. L6에서는 기본 10% 대신 40%를 받습니다." },

    levelup_label:     { en: "Level Up", ko: "레벨 업" },
    maxed_label:       { en: "Maxed", ko: "만렙" },
    locked_label:      { en: "Locked", ko: "잠김" },
    coming_soon:       { en: "v2 · Coming soon", ko: "v2 · 출시 예정" },
    next_boost:        { en: "Next: +{pct}% boost", ko: "다음: +{pct}% 부스트" },
    not_released:      { en: "Not yet released", ko: "출시 예정" },
    levelup_toast_title: { en: "{skill} leveled up", ko: "{skill} 레벨업" },
    levelup_toast_sub:   { en: "Now +{pct}% on every {target}", ko: "이제 모든 {target}에 +{pct}%" },
}
```

- [ ] **Step 3: Convert `character-page.html` body → RSX**

Run `dx translate -f app/ratel/assets/design/character-xp-skills/character-page.html` to seed `component.rs`. Then post-process:

1. Drop the `<style>` import (CSS is in `main.css`).
2. Replace the inline mockup ArenaTopbar with `crate::components::ArenaTopbar { active: ArenaTopbarSection::Character }`.
3. Drop the mockup-only `.state-switcher` (production page does not have it).
4. Drop the demo `<script>` body — wire RSX directly to `UseCharacter`.
5. Replace static numbers with values from `use_character()?.character.read()`.
6. Replace static text with `translate!` references via `CharacterPageTranslate`.
7. Wire each Level Up `button` to `ctx.level_up_action.call(skill_id)` with `disabled: ctx.level_up_action.pending() || unspent_sp < cost`.

Final structure (excerpt):

```rust
use crate::common::*;
use crate::components::{ArenaTopbar, ArenaTopbarSection};
use crate::features::character::dto::{CharacterResponse, CharacterSkillResponse};
use crate::features::character::hooks::{use_character_provider, UseCharacter};
use crate::features::character::pages::character_page::CharacterPageTranslate;
use crate::features::character::types::SkillId;
use dioxus::prelude::*;

#[component]
pub fn CharacterPage() -> Element {
    let ctx = use_character_provider()?;
    let tr = CharacterPageTranslate::new(use_locale());
    let resource = ctx.character;
    let response = resource()?;

    rsx! {
        SeoMeta { title: "{tr.page_title}" }

        div { class: "character-arena",
            ArenaTopbar { active: ArenaTopbarSection::Character }

            main { class: "character-page", id: "character-page",
                CharacterHero { response: response.clone(), tr: tr.clone() }
                section_header_row { tr: tr.clone() }

                div { class: "skill-grid",
                    for s in response.skills.iter() {
                        SkillCard {
                            response: s.clone(),
                            unspent_sp: response.unspent_sp,
                            on_levelup: move |id| {
                                let mut ctx = ctx;
                                spawn(async move { let _ = ctx.level_up(id).await; });
                            },
                        }
                    }
                }
            }
        }
    }
}
```

> Component decomposition: `CharacterHero` (XP bar + level + SP pill) and `SkillCard` (one skill card) are extracted under `pages/character_page/character_hero/` and `pages/character_page/skill_card/` per `conventions/feature-module-structure.md`'s "Extract a sub-component when a section is self-contained and > ~50 lines of RSX" rule.

- [ ] **Step 4: Build check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/character/pages/
git commit -m "feat(character): CharacterPage + CharacterHero + SkillCard RSX conversion"
```

---

## Task 36: Add `Route::CharacterPage` route

**Files:**
- Modify: `app/ratel/src/route.rs` — add the route variant.
- Modify: `app/ratel/src/features/character/route.rs` — drop placeholder, re-export.

- [ ] **Step 1: Add route variant**

In `app/ratel/src/route.rs`, add:

```rust
#[layout(RootLayout)]
    // ... existing routes ...
    #[route("/me/character")]
    CharacterPage {},
```

> Public-profile route variant `/users/:username/character` is mounted by Task 38 once the public chip exists.

- [ ] **Step 2: Build + visual smoke**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx serve --port 8000 --web
```

Open `/`. Click the new Character hud-btn → navigates to `/me/character`. The page shows the hero with default values (since the user has no XP yet); the Character hud-btn is highlighted with the gold underline.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/route.rs app/ratel/src/features/character/route.rs
git commit -m "feat(character): mount /me/character route"
```

---

## Task 37: Add Money Tree breakdown chip in `user_reward` views (Variant A)

Per `app/ratel/assets/design/character-xp-skills/reward-breakdown.html` Variant A: an inline breakdown row appears under each reward transaction whenever the user's Money Tree level > 0 *and* the row's `bonus > 0`.

**Files:**
- Modify: `app/ratel/src/features/social/pages/user_reward/views/mod.rs` — add `RewardBreakdownChip` next to each reward transaction row.
- Modify: `app/ratel/src/features/social/pages/user_reward/dto/...` — surface `money_tree_bonus: i64` and `money_tree_level: i32` from the existing `UserRewardHistory` row (these come from the `metadata` field that Task 23 populates).
- Modify: `app/ratel/assets/main.css` — append the breakdown row styles from `reward-breakdown.html`.

- [ ] **Step 1: DTO**

Add to `RewardTransactionResponse` (or whatever the existing list-row DTO is named):

```rust
pub money_tree_bonus: i64,    // amount in RatelPoint added by Money Tree (0 if no skill)
pub money_tree_level: i32,    // 0..10
```

The server fn populates these from `UserRewardHistory.metadata` — Task 23 already records both `money_tree_bonus` and the multiplier per claim.

- [ ] **Step 2: RSX chip**

Inside the existing reward-row component, append (preserving the exact class names from the mockup):

```rust
if response.money_tree_level > 0 && response.money_tree_bonus > 0 {
    div { class: "reward-tx__breakdown", role: "note", "aria-label": "Reward breakdown",
        span { class: "breakdown__base",
            "Base "
            em { "{format_with_commas(response.amount - response.money_tree_bonus, None)}" }
        }
        span { class: "breakdown__plus", "+" }
        span {
            class: "mt-chip",
            title: "Money Tree skill at level {response.money_tree_level}",
            // award icon — same SVG as character-page.html
            svg { /* ... */ }
            "Money Tree L{response.money_tree_level} +{response.money_tree_level * 5}%"
        }
        span { class: "breakdown__equals", "= " }
        span { class: "breakdown__total", "{format_with_commas(response.amount, None)}" }
    }
}
```

- [ ] **Step 3: Append breakdown CSS** to `main.css` under `/* === src/features/social/pages/user_reward/views (money-tree breakdown) === */`. Copy `.reward-tx__breakdown`, `.breakdown__base`, `.breakdown__plus`, `.mt-chip`, `.breakdown__equals`, `.breakdown__total` from the mockup verbatim.

- [ ] **Step 4: Build + visual smoke**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
```

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/social/pages/user_reward/ app/ratel/assets/main.css
git commit -m "feat(character): Money Tree breakdown chip in user_reward views"
```

---

## Task 38: Public profile level chip (visitor view)

Per `public-profile-badge.html` Variant A: a "Level 32" chip on the right side of the profile header.

**Files:**
- Create: `app/ratel/src/features/character/components/level_chip/mod.rs`
- Create: `app/ratel/src/features/character/components/level_chip/component.rs`
- Modify: the existing public-profile header component (find via `grep -rn "profile-header\|profile_header" app/ratel/src --include="*.rs"`) — render `<LevelChip username={...} />`.
- Modify: `app/ratel/assets/main.css` — append `.character-level-chip{,*}` and `.level-badge-mini{,*}` styles from the mockup.

- [ ] **Step 1: Component**

```rust
#[component]
pub fn LevelChip(username: ReadSignal<String>) -> Element {
    let resource = use_loader(move || async move {
        crate::features::character::controllers::get_public_character_handler(username()).await
    })?;

    let public = resource()?;
    let level = public.level;
    let tr = LevelChipTranslate::new(use_locale());

    rsx! {
        span {
            class: "character-level-chip",
            "data-level": "{level}",
            title: "{tr.tooltip}",
            span { class: "character-level-chip__label", "{tr.level_label}" }
            span { class: "character-level-chip__num", "{level}" }
            span { class: "character-level-chip__sub",
                if level <= 1 { "{tr.just_joined}" } else { "{tr.ratel_character}" }
            }
        }
    }
}
```

- [ ] **Step 2: Add `Route::PublicCharacterPage`** for the dedicated `/users/:username/character` URL (referenced from `get_public_character_handler` in Task 20). For now this can route to a thin wrapper that just renders `LevelChip` with no breakdown — or we punt this view to a future iteration if profile-page integration is enough.

- [ ] **Step 3: Append CSS**

Append `.character-level-chip{,*}` and `.level-badge-mini{,*}` from `public-profile-badge.html` to `main.css`.

- [ ] **Step 4: Build + commit**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
git add app/ratel/src/features/character/components/level_chip/ app/ratel/assets/main.css <profile-header-file>
git commit -m "feat(character): public profile level chip"
```

---

## Task 39: Lint + format frontend changes

- [ ] For each `.rs` file modified in Tasks 32–38: `rustywind --custom-regex 'class: "(.*)"' --write <file>` then `dx fmt -f <file>`.

- [ ] Build:

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```

- [ ] Commit:

```bash
git add app/ratel/
git commit -m "style: rustywind + dx fmt on character feature frontend"
```

---

## Task 40: Playwright e2e — character progression

**Files:**
- Create: `playwright/tests/web/character-progression.spec.js`

- [ ] **Step 1: Write the scenario** (per `conventions/playwright-tests.md`, prefer extending existing scenarios — but this is a wholly new flow, so a new spec file is justified):

```js
import { test, expect } from "@playwright/test";
import { click, goto } from "../utils";

test.describe.serial("Character progression", () => {
  test("brand-new user lands at Level 1 with 5 SP", async ({ page }) => {
    await goto(page, "/");
    await click(page, { testId: "home-btn-character" });
    await page.waitForURL(/\/me\/character/, { waitUntil: "load" });
    await expect(page.getByTestId("hero-level")).toHaveText("1");
    await expect(page.getByTestId("hero-sp-value")).toHaveText("5");
  });

  test("buying Money Tree L1 disables button when SP runs out", async ({ page }) => {
    await goto(page, "/me/character");
    await click(page, { testId: "skill-levelup-money_tree" });
    // After spending 5 SP, the next-level cost (9) > unspent (0)
    await expect(page.getByTestId("skill-levelup-money_tree")).toBeDisabled();
  });

  test("voting in a poll grants XP visible on /me/character", async ({ page }) => {
    // Drive an XP-earning action via an existing space-flow utility if available;
    // otherwise call the test API endpoint that seeds a SpaceActivity. This test
    // is the integration boundary between backend XP propagation and frontend
    // display — the backend tests in features::character::tests already cover
    // the propagation itself.
    // ... seed activity ...
    await goto(page, "/me/character");
    const xp = await page.getByTestId("hero-xp-total").innerText();
    expect(parseInt(xp.replace(/,/g, ""), 10)).toBeGreaterThan(0);
  });
});
```

> The `data-testid` values referenced (`home-btn-character`, `skill-levelup-money_tree`, `hero-level`, `hero-sp-value`, `hero-xp-total`) are added in Tasks 32, 35, and the mockup template — keep them in sync.

- [ ] **Step 2: Add `data-testid` attributes** in `CharacterHero` and `SkillCard` components for the elements the test queries (some are already in the mockup; mirror them in RSX).

- [ ] **Step 3: Run the spec**

```bash
cd playwright && npx playwright test tests/web/character-progression.spec.js --headed
```

All three tests must pass.

- [ ] **Step 4: Commit**

```bash
git add playwright/tests/web/character-progression.spec.js
git commit -m "test(character): Playwright e2e — landing, level-up, XP propagation"
```

---

## Task 41: Update spec acceptance checklist (frontend criteria)

- [ ] In `roadmap/character-xp-skills.md`, flip these from `- [ ]` to `- [x]`:
  - `/me/character` page shows total XP, level, XP to next level, and unspent SP, all updating live as new activities post.
  - A user with no past activity who is brand new sees Level 1 and 0 unspent SP after the level-up bookkeeping (i.e., they get their level-1 SP grant on first appearance).
  - A user can see their Character Level on another user's public profile.

- [ ] Commit:

```bash
git add roadmap/character-xp-skills.md
git commit -m "docs(roadmap): mark frontend acceptance criteria as shipped"
```

---

## Self-review additions (Stage 3 frontend)

- [ ] `app/ratel/src/components/arena_topbar/` is the only place the home/character topbar markup lives. The previous inline copy at `views/index/component.rs:207-376` has been replaced with `<ArenaTopbar />`.
- [ ] No `document::Stylesheet { href: asset!("./style.css") }` was introduced anywhere; all character/CSS is in `app/ratel/assets/main.css` under named section markers.
- [ ] Class names from the mockup (`.character-hero__level`, `.skill-card__pip`, `.character-level-chip`, `.mt-chip`, etc.) are preserved verbatim in RSX.
- [ ] Money Tree breakdown chip renders only when both `money_tree_level > 0` AND `money_tree_bonus > 0` — never on a flat row.
- [ ] Profile tabs are NOT present on `/me/character`; the ArenaTopbar is the only nav.
- [ ] Character hud-btn carries `aria-current="page"` only on `/me/character`; on Home it's just a normal hud-btn.
