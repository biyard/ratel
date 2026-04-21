# Hot Spaces Ranking — Implementation Plan

> 한국어 번역: [2026-04-21-hot-spaces-ranking.ko.md](./2026-04-21-hot-spaces-ranking.ko.md)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the current in-request server-side activity scoring in `list_hot_spaces_handler` with a pre-computed, event-driven ranking so the home carousel surfaces globally hot spaces in O(1) DynamoDB calls regardless of total public space count.

**Architecture:** Extend `by-macros` to support Number-typed GSI sort keys, then introduce two new entities (`SpaceHotScore`, `SpaceActionCount`) maintained by existing + new EventBridge pipelines. Reads become a single GSI7 query plus two parallel `batch_get` calls. Writes are atomic increments on event (participant join, action create/delete) via the existing `increase_*`/`decrease_*` setters. The legacy `count_actions` N+1 scan and in-request `activity_score` sort are retired.

**Tech Stack:** Rust, DynamoEntity macro (Number sort-key extension), DynamoDB GSI with native Number sort key, EventBridge Pipes + Rules, CDK.

**Precondition:** Current PR `feature/home-sort` has already (a) fixed the tab-switch blur, (b) fixed My Spaces ordering, (c) introduced an in-request `activity_score` sort over a 50-space fetch window. This plan supersedes (c) with a scalable design.

---

## Known Limitations of the Current PR (what this plan fixes)

| Problem | Current PR | After this plan |
|---|---|---|
| Fetch window cap (50) | Miss hot spaces ranked beyond index 50 of GSI6 | GSI7 ranks every public space globally |
| N+1 `count_actions` | 50 sequential `SpaceAction::find_by_space` scans per home load | Zero scans at read time — counts come from `SpaceActionCount.batch_get` |
| Read-time sort cost | CPU + DynamoDB quota grows with fetch window | Ranking is pre-computed; read path is O(page size) |
| Page bookmark semantics | Re-sort on every page → pagination inconsistent | DynamoDB-backed GSI7 bookmark is stable |
| Feedback loop risk if `SpaceCommon.hot_score` were added | — | Scores live on separate entities, no `SpaceCommon` MODIFY trigger |

---

## File Structure

### New files

| File | Purpose |
|------|---------|
| `app/ratel/src/features/activity/models/space_hot_score.rs` | `SpaceHotScore` entity + GSI7 declaration |
| `app/ratel/src/features/activity/models/space_action_count.rs` | `SpaceActionCount` entity (denormalized counts) |
| `app/ratel/src/features/activity/services/hot_score.rs` | `bump_hot_score()` helper + `SCORE_DELTA_*` constants |
| `app/ratel/src/features/activity/services/action_count.rs` | Increment/decrement helpers for action counts |
| `app/ratel/src/features/admin/controllers/migrations/backfill_hot_scores.rs` | One-shot admin endpoint to seed `SpaceHotScore` + `SpaceActionCount` for existing spaces |
| `app/ratel/src/tests/hot_score_tests.rs` | Integration tests for pipeline + handler |

### Modified files

| File | Change |
|------|--------|
| `scripts/create-indexes.sh` | Add `gsi7` (String PK / **Number** SK) and parameterize the `AttributeType` per index |
| `packages/by-macros/src/dynamo_entity/mod.rs` | Support `#[dynamo(index = "...", sk, as_number)]` → emit `AttributeValue::N` for that sort key |
| `packages/by-macros/src/query_builder_functions.rs` or equivalent | Ensure `increase_*` / `decrease_*` setters are generated for `i64` fields that are also `as_number` sort keys |
| `app/ratel/src/common/types/error.rs` or equivalent | (optional) Add `EntityType::SpaceHotScore`, `EntityType::SpaceActionCount` |
| `cdk/lib/dynamo-stream-event.ts` | Add `SpaceParticipantJoinPipe`, `SpaceActionCountPipe` + rules; reuse `ActivityScorePipe` |
| `app/ratel/src/common/types/event_bridge_envelope.rs` | Add `DetailType::SpaceParticipantJoin`, `DetailType::SpaceActionCountUpdate` variants + `proc()` match arms |
| `app/ratel/src/common/stream_handler.rs` | Local-dev parity: mirror the new EventBridge handlers for `SpaceParticipant` INSERT and `SpaceAction` INSERT/REMOVE |
| `app/ratel/src/features/timeline/services/fan_out/popular_space.rs` | Append `SpaceHotScore` upsert after fan-out |
| `app/ratel/src/features/activity/services/aggregate_score.rs` | Append `SpaceHotScore` upsert after per-user score aggregation |
| `app/ratel/src/features/spaces/space_common/controllers/list_hot_spaces.rs` | Replace body with GSI7 query + `SpaceCommon.batch_get` + `SpaceActionCount.batch_get`; delete `activity_score()` and `count_actions()` |

### Retired (to be removed in Phase 5)

- `fn activity_score(...)` in `list_hot_spaces.rs`
- `fn count_actions(...)` in `list_hot_spaces.rs` and `list_my_home_spaces.rs` (replace with `SpaceActionCount.batch_get`)
- `limit(50)` / `items.truncate(10)` in `list_hot_spaces.rs`

---

## Entity Design

### A. `SpaceHotScore` — ranking entity

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceHotScore {
    pub pk: Partition,        // "SPACE#{space_id}"
    pub sk: EntityType,       // EntityType::SpaceHotScore

    /// Ranking bucket.
    /// - "PUB_PUB"  : Published + Public (visible in home Hot carousel)
    /// - "HIDDEN"   : Private / Draft (kept for rebucket on visibility change)
    #[dynamo(prefix = "HOT", name = "find_hot_by_bucket", index = "gsi7", pk)]
    pub bucket: String,

    /// Accumulated hot score. Serves as GSI7 sort key (Number type).
    /// Updated atomically via `increase_hot_score(delta)` /
    /// `decrease_hot_score(delta)` — no read-modify-write cycle.
    #[dynamo(index = "gsi7", sk, as_number)]
    pub hot_score: i64,

    /// Timestamp of the most recent event (participant/action) that moved the
    /// score. Used by the periodic decay job to identify idle rows.
    pub last_activity_at: i64,

    /// Denormalized lookup key for read-time batch_get.
    pub space_pk: Partition,

    pub updated_at: i64,
}
```

#### Why Number SK

Accumulated counters combined with atomic `ADD` are the standard DynamoDB
leaderboard pattern. The sort key stays in sync because it *is* the counter —
a single `ADD hot_score :delta` update adjusts both the main attribute and the
GSI projection. This is the same pattern used by `Post.likes` +
`increase_likes(1)` elsewhere in the codebase, extended to a ranking GSI.

With a String sort key we would need to either (a) maintain a second `hot_score_sk: String` field that lags behind the counter and risks lost-update races when resynchronized, or (b) perform a read-modify-write for every event. Both add complexity and eliminate the atomicity guarantee that makes the `ADD` approach safe under concurrent writes. The one-time cost of a tiny `by-macros` extension (Phase 0) buys a much cleaner system that any future ranking entity can reuse.

#### Score strategy — accumulated increments

Each event contributes a fixed weighted delta. No read-modify-write, no race:

| Event | Delta applied to `hot_score` |
|---|---|
| `SpaceParticipant` INSERT | `+10` |
| `SpaceAction` INSERT      | `+20` |
| `SpaceAction` REMOVE      | `-20` |
| (future: comment INSERT)  | `+3`  |

```rust
// Constants live in services/hot_score.rs
pub const SCORE_DELTA_PARTICIPANT: i64 = 10;
pub const SCORE_DELTA_ACTION: i64      = 20;
```

#### Update contract — atomic increment

```rust
pub async fn bump_hot_score(
    cli: &aws_sdk_dynamodb::Client,
    space: &SpaceCommon,
    delta: i64,
    now: i64,
) -> Result<()> {
    let (pk, sk) = SpaceHotScore::keys(&space.pk);
    let mut updater = SpaceHotScore::updater(&pk, &sk)
        .with_bucket(bucket_for(space))
        .with_space_pk(space.pk.clone())
        .with_last_activity_at(now)
        .with_updated_at(now);

    // by-macros generates increase_*/decrease_* for i64 fields; use the signed
    // variant so REMOVE events can pass a negative delta without branching.
    if delta >= 0 {
        updater = updater.increase_hot_score(delta);
    } else {
        updater = updater.decrease_hot_score(-delta);
    }
    updater.execute(cli).await
}
```

Because every write uses `ADD` on a Number attribute, **concurrent writes cannot lose updates**. Two pipelines firing at the same millisecond simply sum their deltas.

#### Periodic decay (optional, Phase 4 follow-up)

Pure accumulation lets old-but-once-popular spaces dominate forever. Option:

- **EventBridge Scheduler** (CDK `Schedule`) triggers a Lambda every 24h
- Lambda scans the `HOT#PUB_PUB` bucket and applies `ADD hot_score :decay` where `:decay = -floor(row.hot_score * 0.05)` (5% daily decay) or a flat negative offset for rows idle longer than N days (using `last_activity_at`)
- Decay itself is an `ADD` → still atomic

This is **not in the initial rollout**. Without decay, rank inflation is bounded by the signup/action rate of active spaces over stagnant ones; revisit only if rankings feel stale.

### B. `SpaceActionCount` — denormalized action counts

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpaceActionCount {
    pub pk: Partition,        // "SPACE#{space_id}"
    pub sk: EntityType,       // EntityType::SpaceActionCount

    pub poll_count: i64,
    pub discussion_count: i64,
    pub quiz_count: i64,
    pub follow_count: i64,
    pub total_actions: i64,

    pub updated_at: i64,
}
```

No GSI — always accessed by (space_pk, EntityType::SpaceActionCount) via `get` or `batch_get`.

#### Increment contract

Atomic increment via the `increase_*` / `decrease_*` setters that `by-macros`
already generates for `i64` fields (see `Post::increase_likes(1)` in
`features/posts/models/post.rs:289` for the existing precedent):

```rust
pub async fn bump_action_count(
    cli: &aws_sdk_dynamodb::Client,
    space_pk: &Partition,
    action_type: SpaceActionType,
    delta: i64, // +1 on create, -1 on delete
    now: i64,
) -> Result<()> {
    let (pk, sk) = SpaceActionCount::keys(space_pk);
    let mut u = SpaceActionCount::updater(&pk, &sk).with_updated_at(now);

    let abs = delta.unsigned_abs() as i64;
    let inc = delta > 0;

    u = match action_type {
        SpaceActionType::Poll            => if inc { u.increase_poll_count(abs) }       else { u.decrease_poll_count(abs) },
        SpaceActionType::TopicDiscussion => if inc { u.increase_discussion_count(abs) } else { u.decrease_discussion_count(abs) },
        SpaceActionType::Quiz            => if inc { u.increase_quiz_count(abs) }       else { u.decrease_quiz_count(abs) },
        SpaceActionType::Follow          => if inc { u.increase_follow_count(abs) }     else { u.decrease_follow_count(abs) },
    };
    u = if inc { u.increase_total_actions(abs) } else { u.decrease_total_actions(abs) };

    u.execute(cli).await
}
```

No read-modify-write. Concurrent writes are summed by DynamoDB.

---

## AWS-Level Changes

### 1. GSI7 on main DynamoDB table

Sort-key attribute type must be **Number** for `hot_score` to be stored and sorted natively. Update `scripts/create-indexes.sh` to accept a per-index SK type, then:

```bash
INDEXES=(
  # ... existing gsi1~gsi6 entries stay "S S" ...
  "gsi7_pk gsi7_sk gsi7-index  S  N"   # PK=String, SK=Number
)
```

Current script hardcodes `AttributeType="S"` for both keys ([scripts/create-indexes.sh:45](scripts/create-indexes.sh#L45)); parameterize so each entry can pass its own `PK_TYPE` and `SK_TYPE`.

Run in each replica region sequentially:

```
ap-northeast-2 → us-east-1 → eu-central-1
```

Index build cost is proportional to the number of rows carrying `gsi7_pk`/`gsi7_sk` attributes. Only `SpaceHotScore` rows (created during backfill, estimated ≤ 10k rows short-term) will have them, so the per-region ACTIVE wait is expected to be minutes rather than hours. No other entity is affected.

### 2. EventBridge pipelines

Three distinct write paths need to update `SpaceHotScore` + `SpaceActionCount`:

| Trigger | Pipe | DetailType | Handler action |
|---|---|---|---|
| `SpaceParticipant` INSERT (join) | **New**: `SpaceParticipantPipe` | `SpaceParticipantJoin` | `bump_hot_score(space, +SCORE_DELTA_PARTICIPANT)` |
| `SpaceAction` INSERT (action created) | **New**: `SpaceActionCountPipe` | `SpaceActionCountUpdate` | `bump_action_count(..., +1)` + `bump_hot_score(space, +SCORE_DELTA_ACTION)` |
| `SpaceAction` REMOVE (action deleted) | Same pipe, REMOVE filter on OldImage | `SpaceActionCountUpdate` | `bump_action_count(..., -1)` + `bump_hot_score(space, -SCORE_DELTA_ACTION)` |
| `SpaceCommon` MODIFY on `participants` | Existing `PopularSpacePipe` + `fan_out_popular_space` | (existing) | (optional) ensure `SpaceHotScore` bucket is current — participant path already increments via `SpaceParticipantPipe`, so no double-counting |
| `SpaceCommon` MODIFY on `visibility`/`publish_state` | (initial: skip — next participant/action event rebuckets; follow-up: dedicated pipe) | — | Update `bucket` via `with_bucket(...)` without touching `hot_score` |

CDK file: [cdk/lib/dynamo-stream-event.ts](../../cdk/lib/dynamo-stream-event.ts). Reuse the existing pattern (Pipe + Rule → `props.lambdaFunction`). `DetailType` enum in Rust must add matching variants.

### 3. Local-dev stream handler parity

[app/ratel/src/common/stream_handler.rs](../../app/ratel/src/common/stream_handler.rs) must mirror each EventBridge path so that `make run` works without EventBridge infrastructure. Follow the existing per-`sk`-prefix branch structure.

---

## Read Path

```rust
#[get("/api/home/hot-spaces?bookmark")]
pub async fn list_hot_spaces_handler(
    bookmark: Option<String>,
) -> Result<ListResponse<HotSpaceResponse>> {
    let cli = ServerConfig::default().dynamodb();

    // 1) Ranking query — GSI7 scan_index_forward=false (desc by hot_score)
    let opts = SpaceHotScore::opt_with_bookmark(bookmark).limit(10);
    let (scores, next_bookmark) =
        SpaceHotScore::find_hot_by_bucket(cli, "PUB_PUB".into(), opts).await?;

    if scores.is_empty() {
        return Ok((vec![], next_bookmark).into());
    }

    // 2) Parallel enrichment — 2 batch_get
    let space_keys: Vec<_> = scores.iter()
        .map(|s| (s.space_pk.clone(), EntityType::SpaceCommon)).collect();
    let count_keys: Vec<_> = scores.iter()
        .map(|s| (s.space_pk.clone(), EntityType::SpaceActionCount)).collect();
    let post_keys: Vec<_> = scores.iter()
        .filter_map(|s| s.space_pk.clone().to_post_key().ok())
        .map(|pk| (pk, EntityType::Post)).collect();

    let (spaces_r, counts_r, posts_r) = tokio::join!(
        SpaceCommon::batch_get(cli, space_keys),
        SpaceActionCount::batch_get(cli, count_keys),
        Post::batch_get(cli, post_keys),
    );
    let spaces = spaces_r.unwrap_or_default();
    let counts = counts_r.unwrap_or_default();
    let posts  = posts_r.unwrap_or_default();

    // 3) Assemble preserving ranking order; drop stale HIDDEN rows
    let space_map: HashMap<_, _> = spaces.into_iter().map(|s| (s.pk.to_string(), s)).collect();
    let count_map: HashMap<_, _> = counts.into_iter().map(|c| (c.pk.to_string(), c)).collect();
    let post_map:  HashMap<_, _> = posts.into_iter().map(|p| (p.pk.to_string(), p)).collect();

    let items: Vec<HotSpaceResponse> = scores.iter().enumerate()
        .filter_map(|(idx, score)| {
            let space = space_map.get(&score.space_pk.to_string())?;
            if !space.is_public() || !space.is_published() { return None; }
            let default_count = SpaceActionCount::default();
            let count = count_map.get(&space.pk.to_string()).unwrap_or(&default_count);
            let post = space.pk.clone().to_post_key().ok()
                .and_then(|pk| post_map.get(&pk.to_string()));
            Some(HotSpaceResponse {
                space_id: space.pk.clone().into(),
                rank: idx as i64 + 1,
                participants: space.participants,
                poll_count: count.poll_count,
                discussion_count: count.discussion_count,
                quiz_count: count.quiz_count,
                follow_count: count.follow_count,
                total_actions: count.total_actions,
                heat: derive_heat(space.participants),
                // ... title/description/logo/author from SpaceCommon + Post ...
                ..Default::default()
            })
        })
        .collect();

    Ok((items, next_bookmark).into())
}
```

**DynamoDB call count per home load**: 3 (1 Query + 3 BatchGetItem executed in parallel → latency = max of 3). Independent of total public-space count. `count_actions` is gone.

---

## Backfill

One-shot admin endpoint, protected by `Role::Admin`. Pattern mirrors [backfill_space_score_rank.rs](../../app/ratel/src/features/admin/controllers/migrations/backfill_space_score_rank.rs).

```rust
#[post("/api/admin/migrations/backfill-hot-scores", role: Role::Admin)]
pub async fn backfill_hot_scores_handler() -> Result<String> {
    // Walk all SpaceCommon rows via GSI6 paginated query.
    // For each space:
    //   - count actions via SpaceAction::find_by_space (one-off N+1 OK here — runs once)
    //   - seed SpaceActionCount row with absolute counts (first-time create, no ADD)
    //   - seed SpaceHotScore row with bucket + hot_score derived from
    //     participants*SCORE_DELTA_PARTICIPANT + total_actions*SCORE_DELTA_ACTION
    //   - set last_activity_at = space.updated_at (best-effort approximation)
    // Return counts processed / failed.
}
```

Run order: **GSI7 ACTIVE in all 3 regions → app deploy (event handlers live) → backfill endpoint called by admin**. Backfill uses `with_hot_score(...)` / `with_*_count(...)` (absolute setters) rather than `increase_*`, so re-running the backfill is idempotent: it overwrites seed values without double-counting. Live events arriving during backfill will re-ADD on top of seed values, so target a low-traffic window or accept a bounded overcounting error that fades as events accumulate.

---

## Concurrency Guarantees

DynamoDB `UpdateItem` with the `ADD` action is atomic on Number attributes. The `by-macros`-generated `increase_*(delta)` / `decrease_*(delta)` setters compile to `ADD`. This gives us:

- **No lost updates.** Two pipelines firing at the same millisecond sum their deltas. No read-modify-write anywhere in the hot path.
- **GSI sort key stays consistent.** Because `hot_score` *is* the sort key attribute, a single `ADD` shifts both the main item and the GSI projection atomically. There is no "score updated but index not yet refreshed" window that a separate `hot_score_sk` field would introduce.
- **Event ordering tolerance.** Out-of-order events are correct as long as deltas are sign-correct (REMOVE = `-20`). Late-arriving events simply mutate the current value.
- **Bounded drift on sign errors only.** If a handler applies the wrong sign, the error is bounded to `|delta|` per event. There is no cascading corruption.

Edge cases we do *not* handle with atomic ADD:

- **Idempotency on retry.** EventBridge occasionally delivers the same event twice. A duplicate `SpaceAction INSERT` would double-count. Mitigation options, decided at implementation time: (a) use the event's `id` field as a dedupe key in a small `ProcessedEvent` DynamoDB entity with TTL, (b) rely on EventBridge's at-least-once as acceptable noise for ranking purposes. Pick (b) for MVP unless observed ranks drift visibly.
- **Backfill vs live events interleave.** See Backfill section — backfill uses absolute setters, live events use `ADD`. During the backfill window a live event can add on top of a seeded value, causing bounded overcounting that never corrects. Run backfill during a low-traffic window or accept the drift.

---

## Phases

### Phase 0 — `by-macros` Number sort-key support

Prerequisite for everything else. Implement and merge as a separate PR so the macro change is reviewed independently.

- [ ] Add `as_number` flag to the `#[dynamo(index = "...", sk, as_number)]` attribute parser in [packages/by-macros/src/dynamo_entity/mod.rs](../../packages/by-macros/src/dynamo_entity/mod.rs)
- [ ] When `as_number` is set, emit `AttributeValue::N(sk.to_string())` instead of `AttributeValue::S(...)` for writes, and expect `AttributeValue::N` on reads at each of the ~15 SK touch points (grep `AttributeValue::S(sk` under `packages/by-macros/src/dynamo_entity/`)
- [ ] `scan_index_forward=false` behavior verified on Number SK (already the default — no code change, just confirm)
- [ ] Parameterize `scripts/create-indexes.sh` to accept per-index SK type (`S` or `N`); default remains `S` so existing GSI entries don't change
- [ ] Add one unit/integration test in `packages/by-macros/tests/` covering a toy entity with a Number SK: round-trip put → query desc → assert ordering
- [ ] `cargo check -p by-macros` passes; `cargo check --features server` on `app-shell` passes (no regressions on existing entities)

### Phase 1 — Entities and pure functions (no AWS changes)

- [ ] Add `EntityType::SpaceHotScore` and `EntityType::SpaceActionCount` enum variants (wherever `EntityType` is declared)
- [ ] Create `app/ratel/src/features/activity/models/space_hot_score.rs` with the struct, `keys()`, `bucket_for()`
- [ ] Create `app/ratel/src/features/activity/models/space_action_count.rs` with the struct and `keys()`
- [ ] Create `app/ratel/src/features/activity/services/hot_score.rs` with `bump_hot_score()` and the `SCORE_DELTA_*` constants
- [ ] Create `app/ratel/src/features/activity/services/action_count.rs` with `bump_action_count()`
- [ ] Unit tests for `bump_*` wrappers (sign handling, correct field routed per `SpaceActionType`) — use a mocked DynamoDB client or integration against `ratel-local`
- [ ] `cargo check --features server` passes with zero warnings

### Phase 2 — AWS infrastructure

- [ ] Update `scripts/create-indexes.sh` to include `"gsi7_pk gsi7_sk gsi7-index S N"` (PK String, SK Number)
- [ ] Run `create-indexes.sh` against `ratel-dev` first; confirm `gsi7-index` status `ACTIVE`
- [ ] Run in prod: `ap-northeast-2` → `us-east-1` → `eu-central-1` (script waits for each before the next)
- [ ] Add `SpaceParticipantPipe` + `SpaceActionCountPipe` + matching `events.Rule`s to `cdk/lib/dynamo-stream-event.ts`
- [ ] `cd cdk && npx tsc --noEmit` passes
- [ ] CDK deploy to dev stack; verify pipes show `RUNNING`

### Phase 3 — Event handlers

- [ ] Add `DetailType::SpaceParticipantJoin` and `DetailType::SpaceActionCountUpdate` variants to `event_bridge_envelope.rs` + `proc()` match arms
- [ ] Participant-join handler: call `bump_hot_score(space, +SCORE_DELTA_PARTICIPANT, now)`
- [ ] Action-count handler: `bump_action_count(..., delta)` then `bump_hot_score(space, delta * SCORE_DELTA_ACTION, now)` (single delta sign governs both)
- [ ] Mirror both branches in `stream_handler.rs` for local-dev parity
- [ ] Decide: keep `fan_out_popular_space`/`aggregate_score` unmodified (participants already accounted for by `SpaceParticipantPipe`) — avoid double-counting
- [ ] `cargo check --features "server,lambda"` passes

### Phase 4 — Backfill

- [ ] Implement `backfill_hot_scores_handler` at `features/admin/controllers/migrations/backfill_hot_scores.rs` using absolute `with_*` setters
- [ ] Register route in admin router
- [ ] Run against `ratel-dev` with seeded dataset; verify ranking query returns expected order across ≥ 20 spaces
- [ ] Run in prod during a low-traffic window

### Phase 5 — Read-path cutover

- [ ] Rewrite `list_hot_spaces_handler` to the GSI7 query + 3× parallel `batch_get` implementation
- [ ] Delete `activity_score()` from `list_hot_spaces.rs`
- [ ] Delete `count_actions()` from `list_hot_spaces.rs` and `list_my_home_spaces.rs`
- [ ] Update `list_my_home_spaces_handler` to use `SpaceActionCount::batch_get` instead of `count_actions`
- [ ] Integration tests in `app/ratel/src/tests/hot_score_tests.rs`: cold start → empty; participant join increments rank; action INSERT/REMOVE moves rank; HIDDEN bucket rows excluded
- [ ] `cargo check --features web` + `dx check --web` pass
- [ ] `cd playwright && CI=true npx playwright test tests/web/home.spec.js` passes

### Phase 6 — Cleanup

- [ ] Remove the TODO comments added to `list_hot_spaces.rs` and `list_my_home_spaces.rs` in the precursor PR
- [ ] Document `SpaceHotScore` + `SpaceActionCount` patterns in `.claude/rules/conventions/` (and reference `Post::increase_likes` as the original precedent for atomic counters)
- [ ] Confirm Hot carousel behavior in all three regions on `dev`

---

## Open Questions (resolve during implementation)

1. **Score weights tuning.** `+10` (participant) and `+20` (action) are initial guesses. Observe real distribution after launch; adjust constants in `services/hot_score.rs` — no DB migration needed since adjustment applies to future events.
2. **Reuse `PopularSpacePipe` vs new `SpaceParticipantPipe`?** Existing pipe fires on `SpaceCommon` MODIFY when `participants` changes. If the participant-join path already triggers a `SpaceCommon` update and we latch onto it, we eliminate one new pipe at the cost of coupling hot-score logic to the timeline fan-out. Inspect `create_participant` and decide early in Phase 2.
3. **Reuse `gsi6` vs new `gsi7`?** `SpaceHotScore` could share `gsi6` with `SpaceCommon`/`Post`/etc. via a distinct PK prefix (`HOT#PUB_PUB`) — zero DB changes. Trade-off: partition hot-spot on an already-busy GSI, and `gsi6` SK is String-typed so we would lose the Number-SK benefit. Default to new `gsi7` unless the Phase 0 macro PR is blocked.
4. **Visibility-change rebucket.** Phase 1–5 defers this: when a space toggles Public ↔ Private/Draft, its `SpaceHotScore.bucket` stays stale until the next participant/action event. For launch this is acceptable (most toggles are one-way Public). If it becomes a visible bug, add a dedicated `SpaceCommonVisibilityChangePipe` follow-up.
5. **Periodic decay.** Without decay, once-popular-now-dead spaces persist. Defer to a follow-up: add an EventBridge Scheduler → decay Lambda if rankings feel stale after 1–2 months of real usage.
6. **Duplicate event handling.** EventBridge is at-least-once. For MVP we accept bounded overcounting on retries; revisit only if observed drift matters.

---

## Rollout Safety

- All writes go to **new** entities. Existing reads/writes on `SpaceCommon`, `SpaceAction`, `SpaceParticipant` are unchanged.
- No existing EventBridge filter matches the new entities — old pipelines are unaffected.
- Phase 0 (macro change) is isolated and has no runtime effect on current entities; only entities that opt into `as_number` use the new code path.
- Backfill overwrites seed values on re-run (absolute setters) → idempotent within a quiet window.
- Rollback path: revert Phase 5 commit → home falls back to the precursor PR's in-request sort. Orphan `SpaceHotScore` / `SpaceActionCount` rows are harmless until the forward-roll.
