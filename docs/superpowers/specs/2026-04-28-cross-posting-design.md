# Cross-posting (Bluesky / LinkedIn / Threads) — System Design

**Roadmap**: [roadmap/cross-posting.md](../../../roadmap/cross-posting.md)
**Design**: [app/ratel/assets/design/cross-posting/](../../../app/ratel/assets/design/cross-posting/)
**Author / Date**: doyooon · 2026-04-28
**Reviewers / Review date**: doyooon · 2026-04-28
**Status**: Approved (2026-04-28)

## Summary

Connect a Ratel creator's external social accounts once, and on every public Ratel post fan-out a backlinked summary to each enabled platform. Connections, syndication state, retries, and engagement snapshots are first-class entities; the publish pipeline is event-driven (`Post` MODIFY on Draft→Published transition → Stage 1 factory → SyndicationJob INSERT → platform API call) so each platform fails independently and a single outage never blocks Ratel publish. The `Post` entity (keyed by `Partition::Feed(post_id)`) is intentionally kept clean of per-platform formatting concerns — all platform-specific shaping happens at the factory boundary.

**Codebase fact note**: Ratel's existing post lifecycle is *create-as-draft → update-with-publish=true*. `create_post_handler` writes an empty `Post` with `status=Draft`; the actual publish moment is `update_post_handler` (`PUT /api/posts/:post_id`) with `UpdatePostRequest::Publish { publish: true, ... }` flipping `status` to `Published`. Cross-posting therefore hooks into the **MODIFY stream event of `Post`** (not the INSERT of an empty draft), and the `PostSyndicationDirective` is written inside the `update_post_handler` Publish branch (not in `create_post_handler`).

## Phasing

The roadmap calls Phase 1 a single shippable feature, but the rollout is staged inside Phase 1 to absorb external dependencies (Meta App Review 1–3 weeks for Threads):

| Sub-phase | Scope | External blocker | Public landing 동작 | 배포 가시성 |
|---|---|---|---|---|
| **1A** | Bluesky end-to-end (connect via app password, fan-out, post-detail panel) | None — code-merge first | Reuses post-detail route with relaxed auth (FR-8 #46–#48 기본 충족). 배너는 Tier-3 generic 만. | **Internal only** — Cargo flag `cf_cross_posting=on` 환경에서만 활성. |
| **1B** | LinkedIn OAuth + fan-out | LinkedIn dev app + OAuth review (~days) | 동일 (변경 없음) | Internal only |
| **1C** | Threads (Meta) OAuth + fan-out | Meta App Review (1–3 weeks) — start in parallel with 1A | 동일 (변경 없음) | Internal only |
| **1D** | Post-signup interstitial + 3-attempt retry sweeper + adaptive engagement scheduler + landing polish | None — depends on 1A jobs existing | 3-tier banner (UTM / Referer / generic) + KO/EN copy 분기 + Lighthouse pass. | **Production enable here** — 1D 완료 시점에 prod feature flag ON. |

Data model and dispatcher must be platform-agnostic from day one so 1B/1C add adapters, not schema migrations.

**왜 1A~1C 는 production 에 노출되지 않는가**: 스펙 FR-5 #34 가 *"failed job 은 1m / 10m / 1h backoff 로 자동 재시도 MUST"* 를 못 박았는데, 자동 재시도 sweeper (Stage 3) 는 1D 에서야 들어옴. 1A~1C 만 prod 에 켜면 Failed job 이 user-initiated retry 전까지 영원히 멈춰 있어 스펙 위반. 따라서 prod 노출은 1D 완료까지 미루고, 그 사이엔 internal staging 에서만 검증.

## Data model

All entities live under `app/ratel/src/features/cross_posting/models/` and follow `conventions/dynamo-prefix-convention.md`. Names assume the existing `Partition` / `EntityType` enums grow new variants (`SocialConnection`, `PostSyndicationDirective`, `SyndicationJob`, `EngagementSnapshot`).

### `SocialConnection` — per-user, per-platform credential

```rust
pub struct SocialConnection {
    #[dynamo(prefix = "SC", index = "gsi1", name = "find_by_platform", pk)]
    pub pk: Partition,                    // User(user_id)

    pub sk: EntityType,                   // SocialConnection(platform_str)  // unique per platform per user

    #[dynamo(index = "gsi1", sk)]
    pub platform_status: String,          // "{platform}#{status}" — sparse GSI for "all connected linkedin users"

    pub platform: SocialPlatform,         // Bluesky | LinkedIn | Threads
    pub status: ConnectionStatus,         // Connected | AuthExpired | Revoked
    pub external_handle: String,          // "@user.bsky.social" / linkedin URN / threads username
    pub external_user_id: String,         // platform-side stable id (used for dedupe / refresh)

    pub credential_ciphertext: Vec<u8>,   // AEAD-sealed blob (see "Credential storage" note)
    pub token_expires_at: Option<i64>,    // Some for OAuth, None for Bluesky app password

    pub auto_post_enabled: bool,          // FR-3 #17 per-platform toggle (default true)
    pub posts_syndicated_count: i64,      // FR-3 #17 cumulative counter
    pub last_synced_at: Option<i64>,

    pub created_at: i64,
    pub updated_at: i64,
}

pub enum SocialPlatform { Bluesky, LinkedIn, Threads }
pub enum ConnectionStatus { Connected, AuthExpired, Revoked }
```

Notes:
- **One connection per user per platform** (FR-1 #1, non-goal "no multi-account-per-platform"). `sk` uses the platform name so a second connect attempt overwrites cleanly.
- **Credential storage.** Phase 1 uses **AEAD (AES-256-GCM) with the data key supplied via environment variable** rather than AWS KMS. This is a deviation from the original FR-1 #6 wording (which mandates KMS); decision recorded 2026-04-29. Rationale, key management, and the migration path back to KMS are documented in the *"Credential encryption"* subsection below. Soft-deleted connections (Revoked) zero the ciphertext field — historical resolution of `(author_user_id, platform)` still works because the row keeps the handle / external_user_id.
- **Revoke = soft delete.** We zero out `credential_ciphertext` and set `status = Revoked` rather than delete the row, so historical "posted via …" rendering on past syndicated posts can still resolve the platform handle. Past `SyndicationJob` rows look up the connection by `(author_user_id, platform)` (no foreign-key denormalization on the job side); the soft-deleted row keeps that lookup answerable.

#### Credential encryption (Phase 1: envvar AEAD)

**Decision (2026-04-29)**: Phase 1 uses AES-256-GCM with the data key delivered via environment variable, not AWS KMS. The roadmap spec (FR-1 #6) mandates KMS — this deviation is explicit and time-boxed; the migration path is preserved.

**Cipher**: `aes-gcm` crate, AES-256-GCM. Sealed blob layout (single `Vec<u8>` stored on `SocialConnection.credential_ciphertext`):

```
byte 0          : key version (matches the version label on the envvar that minted this blob)
bytes 1..13     : 96-bit nonce (random per seal call)
bytes 13..      : ciphertext + 16-byte AES-GCM authentication tag
```

**Key delivery** (matches existing ratel secret pattern — see `BBS_BLS_*`, `KAIA_*`, `P256_*` in `.github/workflows/prod-workflow.yml`):

| Env | Source | Form |
|---|---|---|
| Local dev | `.env` (gitignored) — value pinned in 1Password | `CROSS_POSTING_DATA_KEY=v1:<base64-no-pad 32 bytes>` |
| CI (PR test) | GitHub Secrets `DEV_CROSS_POSTING_DATA_KEY` → workflow `env:` | same form |
| Staging / Prod | GitHub Secrets `DEV_/PROD_CROSS_POSTING_DATA_KEY` → workflow `env:` → ECS task definition / Lambda env var | same form |

**Two-key support for rotation**: a second envvar `CROSS_POSTING_DATA_KEY_PREVIOUS` is honored on `open()` paths only. New writes always use `CURRENT`. During a rotation transition (operator publishes new CURRENT, demotes previous CURRENT to PREVIOUS), `open()` falls back to PREVIOUS for blobs whose version byte matches. Once an offline backfill re-seals all rows under the new CURRENT version, the operator removes PREVIOUS.

**Disaster recovery**: prod CURRENT key is also held in 1Password (admin-only vault). If GitHub Secrets are accidentally deleted, the key can be re-uploaded; without this backup, all stored credentials are unrecoverable and every connected user must reconnect.

**Rotation policy (Phase 1)**: no scheduled cadence. Trigger conditions:
- Suspected leak (CI dump, dev offboarding with elevated trust, etc.) — immediate
- Compliance push (SOC2 etc.) — driven by external requirement

**Migration to KMS (later)**: a `cipher_scheme` discriminator is *not* added now — the version byte already serves the same purpose. Migrating to KMS later means a new aead module that wraps `kms:Encrypt`/`kms:Decrypt` and a dual-read `open()` until all rows are KMS-sealed. Estimated cost: ~2-3 engineer days, zero downtime, ~$0.09 in KMS calls for backfilling typical Phase 1 row counts. Spec FR-1 #6 will be re-satisfied at that point.

### `PostSyndicationDirective` — sidecar that keeps `Post` clean

```rust
pub struct PostSyndicationDirective {
    #[dynamo(prefix = "PSD", pk)]
    pub pk: Partition,                    // Feed(post_id)

    pub sk: EntityType,                   // SyndicationDirective("v1")  // single per post

    pub enabled_platforms: Vec<SocialPlatform>,
    pub platform_overrides: HashMap<SocialPlatform, String>,  // empty in Phase 1; v1.5 fills

    pub author_user_id: Partition,        // User(...) — read by Stage 1 to resolve SocialConnections
    pub created_at: i64,
}
```

Notes:
- **`Post` isolation.** The canonical `Post` entity carries no per-platform fields, no override map, and no enabled-list — all platform-specific intent lives on the directive. This keeps the post body, search index, feed renderer, and timeline projections free of cross-posting concerns.
- **Atomic write — reuse existing infrastructure.** The directive is written *inside* the existing `update_post_handler` Publish branch by appending a `directive.create_transact_write_item()` to the same `Vec<TransactWriteItem>` that already carries the `Post` updater (`with_status(Published) + with_title + with_html_contents + ...`). The whole vec is committed via `crate::transact_write_items!(cli, transacts)?` — the macro that `update_post_handler` already calls today (see `app/ratel/src/features/posts/controllers/update_post.rs`). No new transactional primitive is introduced; cross-posting just adds one more item to an existing atomic batch.
- **Ratel-only path.** When `enabled_platforms` resolves to empty (sidebar all-off, or Private/team-shared visibility, or no connected platforms) the directive item is simply not pushed into `transacts` — the existing `Post` updater commits alone, exactly as today.
- **Transaction bound.** `TransactWriteItems` is bounded at 100 items / 4MB. Adding one directive item to the existing `update_post` transact batch (typically 1~3 items) stays well within the bound.
- **Phase 1 contents.** `enabled_platforms` reflects the user's compose-time sidebar choice. `platform_overrides` is always an empty map in Phase 1 — the field exists from day one so v1.5 (per-network compose variants) is a UI change only, no schema migration.
- **No directive ⇒ no syndication.** A post without a directive (e.g., legacy posts created before this feature, posts re-published from Draft→Published with sidebar all-off, or visibility-restricted posts) silently skips Stage 1 — the absence of the directive is the kill switch.

### `SyndicationJob` — one row per (post × platform)

```rust
pub struct SyndicationJob {
    #[dynamo(prefix = "SJ", pk)]
    pub pk: Partition,                    // Feed(post_id) — colocate jobs with the post

    pub sk: EntityType,                   // SyndicationJob("{platform}")  // one per platform per post

    // `#[dynamo]` attribute stacking on a single field is supported per
    // packages/by-macros/DYNAMO_ENTITY.md (gsi1 + gsi2 example, lines 291-292).

    /// Shard key for retry sweep. Computed once at insert as
    /// `format!("SDS#{}", shard_for(post_id))` where `shard_for` is the
    /// single shared utility in `cross_posting::services::shard.rs` that
    /// applies `hash(post_id) % SHARD_COUNT` with SHARD_COUNT = 4.
    /// Sparse: when the job reaches a terminal state we DELETE this attribute
    /// (set to None) so the GSI drops the row entirely.
    #[dynamo(index = "gsi1", name = "find_due_jobs", pk)]
    pub dispatch_shard: Option<String>,

    /// Shard key for engagement refresh sweep. Same hashing strategy as
    /// `dispatch_shard` but a separate prefix so the two sweepers can scale
    /// independently. Sparse: deleted when polling stops.
    #[dynamo(index = "gsi2", name = "find_due_engagement", pk)]
    pub engagement_shard: Option<String>,

    /// GSI1 sort key. Number type so DynamoDB range comparators (`<=`) work
    /// directly. Only meaningful when `dispatch_shard.is_some()`; otherwise
    /// the row is sparse off the GSI and the value is irrelevant.
    #[dynamo(index = "gsi1", sk)]
    pub next_attempt_at: i64,             // unix epoch seconds

    /// GSI2 sort key. Same Number-type rationale as `next_attempt_at`.
    #[dynamo(index = "gsi2", sk)]
    pub engagement_next_at: i64,          // unix epoch seconds

    pub author_user_id: Partition,        // User(...) — for fan-out / privacy re-check
    // Connection lookup: (author_user_id, platform) ⇒ SocialConnection
    // pk = User(author_user_id), sk = SocialConnection(platform.to_string()).
    // No denormalized connection_id field — read directly when dispatcher
    // needs decrypted credentials.

    pub platform: SocialPlatform,
    pub state: JobState,                  // Pending | Published | Failed | Skipped

    pub attempts: u8,                     // 0 .. 3
    pub last_error_category: Option<ErrorCategory>,
    pub last_error_message: Option<String>,

    pub external_post_id: Option<String>, // platform-side id (Published only)
    pub external_post_url: Option<String>,// platform-side URL (Published only)

    /// Idempotency lock for in-flight Stage 2 dispatch.
    /// Set to a fresh UUID before the platform API call; cleared after
    /// success/failure write. A second Lambda invocation observing
    /// `dispatch_lock_id.is_some() && lock_acquired_at + LOCK_TTL > now`
    /// MUST exit without calling the platform — prevents duplicate posts
    /// on Lambda retry between API success and DB write.
    pub dispatch_lock_id: Option<String>,
    pub lock_acquired_at: Option<i64>,    // unix epoch seconds

    // body_override: Option<String>      // RESERVED for v1.5 (per-network compose variants).
    //                                     // When Some, Stage 2 dispatcher uses this verbatim
    //                                     // instead of formatting from Feed. Phase 1 leaves
    //                                     // this absent; the field will be added without
    //                                     // backfill (None on legacy rows).
    pub body_snapshot_len: i32,           // length only, never the body itself (FR-10 #53)
    pub backlink_url: String,             // baked at enqueue with utm_source

    pub created_at: i64,
    pub updated_at: i64,
}

pub enum JobState { Pending, Published, Failed, Skipped }
pub enum ErrorCategory { AuthExpired, RateLimited, ContentRejected, NetworkError, Unknown }
```

Notes:
- **Independence (FR-5 #32).** Per-platform job rows guarantee one platform's failure has no transactional effect on another.
- **Idempotent dedupe.** `(pk = Feed(post_id), sk = SyndicationJob(platform))` is unique. Re-running the same job for the same `(post_id, platform)` updates the existing row instead of creating a duplicate external post (FR-5 #34, Constraints "Idempotent retries"). Before calling the platform API the worker reads the row's `state` — if `Published` already, no-op.
- **Two sparse GSIs via shard keys.** Sweepers must answer *"across all posts, find all jobs due now"* — a query that requires a bounded set of GSI partition keys, not a per-post pk. We solve this by deriving a shard key (`SDS#0` … `SDS#3`) from `hash(post_id) % SHARD_COUNT` and storing it on `dispatch_shard` (gsi1 pk) / `engagement_shard` (gsi2 pk). Sweep Lambdas fan out 4 parallel `Query` calls — one per shard — and union the results. When the job reaches a terminal state the shard attribute is set to `None`, removing the row from the GSI entirely (sparse). The two shard families are independent so long-tail engagement refresh never blocks retry latency.
- **`SHARD_COUNT = 4` rationale.** Sharding here is *not* a write-throughput concern — at the realistic Ratel scale (today: 10²–10³ posts/day; even a 100× scenario stays orders of magnitude under DynamoDB's per-partition write ceiling). The number is chosen for **sweep-Query parallelism**: each sweep cycle fans out one `Query` per shard and unions the results. 4 shards keep the per-cycle Query cost low while still bounding any single shard's result-page size. If growth later requires more parallelism, see the *"Re-sharding migration"* note in Risks.
- **Shard utility — deterministic hash only.** All call sites (Stage 1 enqueue, Stage 2 success/failure paths, both sweepers) MUST go through `cross_posting::services::shard::shard_for(post_id)`. Inline `hash(post_id) % N` at multiple call sites is forbidden. The hash function inside the utility is implementation choice (xxhash / fxhash / CRC32 — any stable non-cryptographic hash) but **`std::collections::hash_map::DefaultHasher` is forbidden** because it uses a per-process random seed and would produce different shards on different Lambda invocations.
- **`body_override` reserved for v1.5.** The factory in Stage 1 reads `directive.platform_overrides.get(platform)` and would write the result into `body_override`. Phase 1 always passes `None`; v1.5 starts passing `Some(_)` when the user authors a network-specific variant. Stage 2 dispatcher already needs to read `body_override` from day one (defaulting to format-from-Feed when absent) so Phase 1 → 1.5 is a UI/factory change, no dispatcher rewrite.

### `EngagementSnapshot` — periodic likes/comments/reposts mirror

```rust
pub struct EngagementSnapshot {
    #[dynamo(prefix = "ES", pk)]
    pub pk: Partition,                    // Feed(post_id)
    pub sk: EntityType,                   // EngagementSnapshot("{platform}")

    pub likes: i32,
    pub comments: i32,
    pub reposts: i32,
    pub fetched_at: i64,
}
```

Notes:
- One row per `(post, platform)`, overwritten on each refresh (no history).
- Refresh cadence is **adaptive** — see Event flow Stage 4 for the full schedule. The next-refresh timestamp lives on `SyndicationJob.engagement_next_at`, not here, because the scheduler walks jobs (not snapshots).
- **Author-only read** (D-5). Only surfaced via the post-detail "Syndication" section endpoint which gates by `author_user_id == session_user`.

### `UserOnboardingFlags` — sidecar for onboarding bookkeeping

```rust
pub struct UserOnboardingFlags {
    #[dynamo(prefix = "UOF", pk)]
    pub pk: Partition,                          // User(user_id)
    pub sk: EntityType,                         // OnboardingFlags("v1")

    pub cross_posting_interstitial_seen: bool,  // FR-2 #13
    // future: other onboarding flags slot in here without touching User
}
```

We do **not** add the field directly to the `User` entity. Reasons:

- **Feature-flag isolation.** Builds with the `cross_posting` Cargo feature off must compile against the same `User` shape — a sidecar avoids `#[cfg(feature = "cross_posting")]` fields on a core entity.
- **Future onboarding flags.** As more onboarding gates appear, they accumulate here (`essence_setup_seen`, `did_verification_seen`, …) instead of bloating `User`.

Default when absent is `false` (= never seen). The `POST /api/cross-posting/onboarding/dismiss` endpoint upserts the row.

## API surface

All controllers under `app/ratel/src/features/cross_posting/controllers/`. Session-authed; each enforces `pk = User(session_user_id)`.

| Method | Path | Purpose | Auth |
|---|---|---|---|
| GET | `/api/cross-posting/connections` | List user's connections (status, handles, counts, toggles) | Session |
| POST | `/api/cross-posting/connections/bluesky/connect` | Connect via app-password (handle + app password in body). Routed under `/connect` to avoid shadowing the `{platform}` PATCH/DELETE routes — see implementation comment in `connect_bluesky.rs`. | Session |
| GET | `/api/cross-posting/oauth/{platform}/start` | Begin OAuth (returns redirect URL) | Session |
| GET | `/api/cross-posting/oauth/{platform}/callback` | OAuth callback (stores tokens, redirects to settings or interstitial) | OAuth state |
| PATCH | `/api/cross-posting/connections/{platform}` | Toggle `auto_post_enabled` | Session |
| DELETE | `/api/cross-posting/connections/{platform}` | Revoke (FR-1 #7) | Session |
| GET | `/api/cross-posting/posts/{post_id}/syndication` | Author-only syndication panel data | Session + author check |
| POST | `/api/cross-posting/posts/{post_id}/jobs/{platform}/retry` | User-initiated retry (resets `attempts`, sets `state = Pending`) | Session + author check |
| POST | `/api/cross-posting/onboarding/dismiss` | Sets `cross_posting_interstitial_seen = true` | Session |

Path params use SubPartition types per `conventions/server-functions.md` — `{post_id}` is `FeedPartition`, `{platform}` is the `SocialPlatform` enum (serde-renamed lowercase).

### Publish DTO extension (existing `UpdatePostRequest::Publish`)

The compose-time sidebar (FR-4) consumes the **same `GET /connections`** endpoint plus a transient client-side `Vec<SocialPlatform>` for per-post toggle state. No new endpoint for "what would this post reach"; the UI computes `connected ∩ auto_post_enabled ∩ user_per_post_toggles` locally and submits the result by extending the **existing** `UpdatePostRequest::Publish` variant (in `app/ratel/src/features/posts/controllers/update_post.rs`) with two new optional fields:

```rust
pub enum UpdatePostRequest {
    Publish {
        title: String,
        content: String,
        image_urls: Option<Vec<String>>,
        publish: bool,
        visibility: Option<Visibility>,
        categories: Option<Vec<String>>,

        /// Per-post platform selection. Phase 1: defaults to all connected
        /// + auto_post_enabled when omitted; an explicit empty Vec means
        /// "Ratel-only" (FR-4 #27).
        enabled_platforms: Option<Vec<SocialPlatform>>,

        /// Per-platform body overrides. Phase 1: always empty (UI does not
        /// expose). v1.5: keyed by platform; absent platforms fall through
        /// to format-from-Post.
        platform_overrides: Option<HashMap<SocialPlatform, String>>,
    },
    // ... other variants unchanged
}
```

Inside the Publish branch of `update_post_handler`:
1. Existing logic builds the `Post` updater (`with_status(Published) + with_title + with_html_contents + with_visibility + ...`) and pushes its `transact_write_item()` into `transacts`.
2. **New**: when `publish == true && visibility == Public && enabled_platforms` resolves to non-empty, build a `PostSyndicationDirective` from `(enabled_platforms, platform_overrides)` and push `directive.create_transact_write_item()` into the same `transacts` vec.
3. `crate::transact_write_items!(cli, transacts)?` commits both atomically — exactly the call already on line 170 today.

Private / team-shared posts skip step 2 — the directive item is simply not pushed, the existing `Post` update commits alone, and Stage 1's "no directive ⇒ no syndication" rule (FR-9 #50 first guard) covers the privacy gate.

`create_post_handler` (POST `/api/posts`) is **not** modified — it continues to create empty drafts. Cross-posting only fires at the publish-transition moment.

## Event flow

The fan-out is event-driven across three platform stages plus an adaptive engagement stage, per `conventions/implementing-event-bridge.md`:

```
Stage 1 — factory (bake jobs from directive):
  Post MODIFY: Draft → Published, visibility=Public
    → CDK Pipe (eventName: ["MODIFY"], filter sk prefix=POST#
                AND OldImage.status != "Published"
                AND NewImage.status == "Published"
                AND NewImage.visibility == "Public")
    → DetailType::PostPublishedForSyndication
    → Lambda: read PostSyndicationDirective by post_id (pk = Feed(post_id))
              → if directive absent: silently exit (Ratel-only post, no syndication intent)
              → read author's SocialConnections
              → for each platform in directive.enabled_platforms ∩ connected:
                  bake SyndicationJob row with:
                    - body_override     = directive.platform_overrides.get(platform)  // None in Phase 1
                    - backlink_url      = canonical_url + ?utm_source={platform}
                    - state             = Pending
                    - dispatch_shard    = None    // not yet in retry queue
                    - engagement_shard  = None    // not yet polling
                    - next_attempt_at   = 0       // irrelevant while shard=None
                    - engagement_next_at = 0      // irrelevant while shard=None

Stage 2 — dispatch to platform:
  SyndicationJob INSERT or MODIFY (state=Pending)
    → CDK Pipe (eventName: ["INSERT", "MODIFY"], filter sk prefix=SYNDICATION_JOB#
                AND NewImage.state=Pending)
    → DetailType::SyndicationJobReady
    → Lambda:
        // ── (a) Acquire idempotency lock ─────────────────────────────────
        let lock_id = uuid_v7();
        let now = now_epoch_secs();
        UpdateItem job WHERE state == Pending
          CONDITION  attribute_not_exists(dispatch_lock_id)
                  OR lock_acquired_at < now - LOCK_TTL_SEC   // 60s, > Lambda max
          SET        dispatch_lock_id = lock_id, lock_acquired_at = now;
        // ConditionalCheckFailed → another invocation is mid-flight or just
        // wrote a terminal state. Exit without touching the platform.

        // ── (b) Reconcile a stolen lock (recovery path) ─────────────────
        // If we just *stole* a lock from a dead-but-may-have-published
        // attempt, we MUST verify with the platform whether the previous
        // attempt actually published before issuing a new publish call.
        // Detection: the row we just locked had a previous lock_id set.
        if previous_lock_id.is_some() {
            let recovered = adapter.find_by_backlink(creds, &job.backlink_url)?;
            if let Some(existing) = recovered {
                // Previous attempt succeeded server-side. Adopt its result.
                UpdateItem ... state=Published, external_post_id=existing.id,
                                external_post_url=existing.url,
                                dispatch_lock_id=None, lock_acquired_at=None,
                                engagement_shard=Some("SDS#{shard}"),
                                engagement_next_at = now + 1h;
                return;
            }
            // Else: previous attempt died before publishing — proceed normally.
        }

        // ── (c) Privacy guard re-check (FR-6 #39) ───────────────────────
        let post = Post::find_by_pk(cli, &job.pk).await?;
        if post.visibility != Some(Visibility::Public) || post.status != PostStatus::Published {
            UpdateItem ... state=Skipped, dispatch_shard=None,
                            engagement_shard=None,
                            dispatch_lock_id=None, lock_acquired_at=None;
            return;
        }

        // ── (d) Resolve images + body ───────────────────────────────────
        // post.urls is Vec<String>; first element is treated as the hero image
        // (FR-5 #31). Platforms with stricter limits take only the first; Bluesky
        // can take up to 4 — adapter applies its own platform_image_cap.
        let images: Vec<ImageRef> = post.urls.iter()
            .take(platform.max_images())
            .map(|url| ImageRef::from_s3(url))
            .collect();
        let body = job.body_override
            .clone()
            .map(|s| truncate_override(s, &job.backlink_url, platform.char_limit()))
            .unwrap_or_else(|| format_for_platform(&post, platform));

        // ── (e) Single platform call ────────────────────────────────────
        let result = adapter.publish(creds, body, images).await;

        // ── (f) Commit terminal state + release lock atomically ─────────
        // Conditional on dispatch_lock_id == lock_id so a stolen lock can't
        // overwrite the holder's commit.
        match result {
            Ok(published) =>
                UpdateItem job CONDITION dispatch_lock_id == lock_id
                  SET state=Published,
                      external_post_id=published.id, external_post_url=published.url,
                      engagement_shard=Some("SDS#{shard}"),
                      engagement_next_at = now + 1h,
                      dispatch_lock_id=None, lock_acquired_at=None,
            Err(retryable) if attempts < 3 =>
                UpdateItem job CONDITION dispatch_lock_id == lock_id
                  SET state=Failed, attempts = attempts + 1,
                      dispatch_shard=Some("SDS#{shard}"),
                      next_attempt_at = now + backoff(attempts),  // 1m / 10m / 1h
                      dispatch_lock_id=None, lock_acquired_at=None,
            Err(terminal) =>
                UpdateItem job CONDITION dispatch_lock_id == lock_id
                  SET state=Failed,
                      dispatch_shard=None,                        // drops off retry GSI
                      dispatch_lock_id=None, lock_acquired_at=None,
        }

Stage 3 — retry sweeper (1D):
  CloudWatch every 1 min
    → Lambda: for shard in 0..SHARD_COUNT (4 in parallel):
        Query gsi1 WHERE dispatch_shard = "SDS#{shard}" AND next_attempt_at <= now
    → for each due row: state=Pending, dispatch_shard=None, next_attempt_at=0
    → MODIFY event re-enters Stage 2 via the same Pipe (filter matches state=Pending)

Stage 4 — adaptive engagement refresh (1D):
  CloudWatch every 15 min  (separate Lambda alias from Stage 3)
    → Lambda: for shard in 0..SHARD_COUNT (4 in parallel):
        Query gsi2 WHERE engagement_shard = "SDS#{shard}" AND engagement_next_at <= now
    → for each due row:
        fetch counts via adapter.fetch_engagement(creds, external_post_id)
        upsert EngagementSnapshot
        compute next interval from Feed.created_at age (see schedule below)
        if interval is "stop": engagement_shard=None (drop from GSI)
        else: engagement_next_at = now + interval
```

For 1A we ship Stage 1 + Stage 2 only. Stage 3 (3-attempt retry policy, FR-5 #34) and Stage 4 (adaptive engagement refresh) land in 1D — until then a `Failed` job stays Failed until user-initiated retry, and engagement counts only populate after explicit user refresh from the post-detail panel.

### Adaptive engagement schedule (Stage 4)

External engagement decays sharply with post age, so we lengthen the polling interval as the post ages to keep platform-API budgets bounded (Constraints — Cost). The scheduler is intentionally separate from the retry sweeper so the slow refresh cadence does not delay retry latency.

> **Note (FR-7 #45 대비)**: 스펙은 "default: 6h 간격"으로 보수적으로 잡혀 있으나, 본 설계는 < 24 h 윈도우에서 1 h 로 *tightening* 한다 — 게시 직후 24시간이 외부 engagement 의 80% 가 발생하는 구간이라 더 조밀한 폴링이 가치 대비 비용 측면 효과적. 30 d cap 도입으로 long-tail 비용은 통제됨.

| Post age (now − Feed.created_at) | Next refresh |
|---|---|
| < 24 h | + 1 h |
| 24 h ≤ age < 7 d | + 6 h |
| 7 d ≤ age < 30 d | + 24 h |
| ≥ 30 d | stop polling (set `engagement_shard = None`) |

Schedule details:
- **Frequency.** CloudWatch fires every 15 min so the worst-case lag between "interval elapsed" and "fetch happens" is bounded at 15 min — well under the tightest 1-h interval.
- **Author-triggered refresh.** A manual refresh from the post-detail panel calls the adapter directly and writes a fresh `EngagementSnapshot` without waiting for the sweep; it does NOT change `engagement_next_at`.
- **Stop conditions.** A row's `engagement_shard` is also set to `None` (drops from gsi2) when (a) the connection becomes `Revoked` or `AuthExpired`, or (b) the syndicated copy returns 404 (deleted on the external platform).

### Local-dev parity

Mirror branches in `app/ratel/src/common/stream_handler.rs`:
- Post MODIFY branch (sk prefix `POST#`): if `OldImage.status != "Published" && NewImage.status == "Published" && NewImage.visibility == "Public"` and a directive exists for `pk = Feed(post_id)`, run Stage 1 logic.
- SyndicationJob INSERT branch: if `state == Pending`, run Stage 2 logic.
- For Stages 3 and 4, local dev runs two `tokio::spawn` pollers behind `#[cfg(feature = "server")]` keyed off the same GSIs (`find_due_jobs`, `find_due_engagement`).

### Privacy guards

Two layers per FR-9 + FR-6:
1. **At enqueue** (Stage 1 Lambda): the Pipe pattern filters `visibility=Public`, so private posts never reach Stage 1. Belt-and-braces: Stage 1 also bails if no directive exists for the post (private/team-shared posts skip directive creation entirely).
2. **At dispatch** (Stage 2 Lambda): re-read the post via `Post::find_by_pk(cli, &job.pk)` (where `job.pk = Feed(post_id)`) and check `status == Published && visibility == Some(Public)`. If not, set `state = Skipped`, `dispatch_shard = None`, `engagement_shard = None`, and return without calling the platform.

## External integrations

### Bluesky (1A)

- **Auth**: app password flow. UI modal collects `handle` + `app_password`; server calls `com.atproto.server.createSession` to validate; on success store the returned `accessJwt` + `refreshJwt` (AEAD-sealed via `crate::common::utils::aead::seal`) and the validated handle.
- **Publish**: `com.atproto.repo.createRecord` with `app.bsky.feed.post` collection. Embed images via `com.atproto.repo.uploadBlob` first.
- **Rich-link embed**: a plain backlink URL inside the post body does not produce a card preview on Bluesky clients. The adapter attaches an `app.bsky.embed.external` embed (title, description, thumb) so the syndicated copy renders as a rich link card. Metadata is pulled from the Ratel post's OG tags via a one-shot HTTP fetch of the canonical URL; fallback when extraction fails: `title = post.title`, `description = first 200 chars of stripped html_contents`, `thumb = post.urls.first().cloned()` (when present).
- **Refresh**: refresh on each publish if access token < 30s from expiry; refresh failure → `auth_expired`.
- **Rate limits**: 5,000 points / hour; each post ~3 points; well under quota.

### LinkedIn (1B)

- **Auth**: OAuth 2.0 authorization code, scopes `r_liteprofile w_member_social`. Callback exchanges code for tokens; AEAD-sealed alongside the refresh token + 60-day expiry.
- **Publish**: `POST /v2/ugcPosts` with `lifecycleState=PUBLISHED`. Image: `/v2/assets?action=registerUpload` then upload.
- **Refresh**: ~7 days before expiry, dispatcher proactively refreshes; on 401 mark `auth_expired` and notify (FR-5 #35).

### Threads (1C, blocked on Meta App Review)

- **Auth**: Meta OAuth, scopes `threads_basic threads_content_publish`. Callback verifies a linked Instagram Professional account exists (FR-3 #21); if not, return the dedicated modal error and create no connection.
- **Publish**: two-step — `POST /me/threads` to create a media container, then `POST /me/threads_publish`.

### CSRF guard for OAuth flows (LinkedIn + Threads)

`oauth/{platform}/start` generates `state = base64url(rand(32))` and stores it in the session under a 10-minute TTL. The callback handler compares the query `state` against the session value and aborts with `OAuthError::StateMismatch` on mismatch — no connection is created and the in-flight token exchange is not attempted. The session entry is deleted on first read (single-use).

### Adapter trait

```rust
#[async_trait]
pub trait CrossPostAdapter: Send + Sync {
    fn platform(&self) -> SocialPlatform;
    fn char_limit(&self) -> usize;
    fn max_images(&self) -> usize;

    async fn publish(
        &self,
        creds: DecryptedCredentials,
        formatted_body: String,
        images: Vec<ImageRef>,
    ) -> std::result::Result<PublishedRef, PlatformError>;

    async fn fetch_engagement(
        &self,
        creds: DecryptedCredentials,
        external_post_id: &str,
    ) -> std::result::Result<EngagementCounts, PlatformError>;

    /// Reconcile path used when Stage 2 steals a lock from a dead attempt.
    /// Searches the user's recent posts on the platform for a copy whose
    /// body contains `backlink_url`. The backlink (with `?utm_source=`) is
    /// unique per Ratel post, so a hit unambiguously identifies a prior
    /// successful publish that died before our DB write. Returns None if
    /// no match found within the platform's recent-post window (typically
    /// last ~50 posts — sufficient for in-flight recovery, not for archival).
    async fn find_by_backlink(
        &self,
        creds: DecryptedCredentials,
        backlink_url: &str,
    ) -> std::result::Result<Option<PublishedRef>, PlatformError>;
}
```

Stage 2 Lambda picks the adapter from `match job.platform`. New platforms = new adapter struct, no dispatcher change.

`LOCK_TTL_SEC = 60` is chosen to exceed the configured Lambda max execution time (currently 30s for Stage 2) with margin. Stealing the lock before TTL expiry would cause double-publish; setting it too high would slow recovery from a genuinely dead Lambda.

### Truncation (FR-5.5)

Implemented once in `cross_posting::services::format::format_for_platform(post: &Post, platform: SocialPlatform) -> String`. Order: `{post.title}\n\n{first_sentence_of_post.html_contents_stripped}…\n{backlink_with_utm}` when over budget, full body when under. Backlink with `?utm_source={platform}` is non-truncatable; if `{title}\n{backlink}` alone exceeds budget the title is the only thing truncated. The function strips HTML tags from `post.html_contents` before truncation (Ratel posts are rich text; external platforms expect plain text or markdown).

In Stage 2 the dispatcher resolves the body as:

```rust
let body = job.body_override
    .clone()
    .map(|s| truncate_override(s, &job.backlink_url, platform.char_limit()))
    .unwrap_or_else(|| format_for_platform(&post, platform));
```

(`post: Post` is read inside the dispatcher via `Post::find_by_pk(cli, &job.pk)` — see Stage 2 box step (c).) So Phase 1 (no overrides) always goes through `format_for_platform`; v1.5 (overrides) feeds a user-authored variant through truncation but skips the auto-formatter.

`truncate_override` is distinct from `format_for_platform` because an override is free text — there is no title/body separation:

```rust
/// User-authored override truncation (v1.5+).
/// Strategy:
///   1. Reserve `backlink.len() + "\n…\n".len()` characters at the end.
///   2. If body fits within (limit - reserved), append "\n{backlink}" verbatim.
///   3. Else truncate body at (limit - reserved) chars, append "…\n{backlink}".
/// Backlink is never truncated (FR-8 #46 — backlink integrity is non-negotiable).
fn truncate_override(body: String, backlink: &str, limit: usize) -> String;
```

Phase 1 never executes this path — `body_override` is always `None` because the UI does not expose per-platform composition. Both functions are unit-tested per platform in `cross_posting/services/format_tests.rs`.

## Frontend architecture

New feature module `app/ratel/src/features/cross_posting/` per `conventions/feature-module-structure.md`:

```
cross_posting/
├── mod.rs, route.rs
├── controllers/                 — server functions (above)
├── models/                      — SocialConnection, PostSyndicationDirective, SyndicationJob, EngagementSnapshot
├── services/                    — adapters/, format.rs, dispatcher.rs (server-only)
├── hooks/
│   ├── use_cross_posting.rs     — UseCrossPosting controller (compose + settings + onboarding)
│   └── use_syndication_panel.rs — UseSyndicationPanel for post-detail (author-only)
├── components/
│   ├── connections_page/        — Settings → Connections (Stage 2 mockup `social-connections.html`)
│   ├── compose_sidebar/         — Right-rail sidebar (Stage 2 `compose-with-crosspost.html`)
│   ├── onboarding_interstitial/ — Single-screen post-signup (Stage 2 `onboarding-connect-socials.html`)
│   ├── syndication_panel/       — Post-detail author panel (Stage 2 `post-detail-syndicated.html`)
│   ├── bluesky_connect_modal/
│   ├── threads_no_ig_modal/
│   └── public_backlink_view/    — Public landing page (Stage 2 `backlink-landing.html`)
├── i18n.rs
└── types/error.rs               — CrossPostingError per conventions/error-handling.md
```

### `UseCrossPosting` controller

Per `conventions/hooks-and-actions.md`. Exposes:

```rust
#[derive(Clone, Copy, DioxusController)]
pub struct UseCrossPosting {
    pub connections: Loader<Vec<ConnectionResponse>>,
    pub stats: Memo<ConnectionStats>,                // derived: connected_count, posts_this_month

    // Settings page actions
    pub handle_connect_bluesky:    Action<(String, String), ()>,  // handle, app_password
    pub handle_start_oauth:        Action<(SocialPlatform,), ()>,
    pub handle_toggle_auto_post:   Action<(SocialPlatform, bool), ()>,
    pub handle_disconnect:         Action<(SocialPlatform,), ()>,

    // Compose-time per-post overrides
    pub per_post_enabled: Signal<HashMap<SocialPlatform, bool>>,
    pub reach_count: Memo<usize>,

    // Onboarding
    pub handle_dismiss_interstitial: Action<(), ()>,
}
```

Components consume via `let UseCrossPosting { mut handle_connect_bluesky, .. } = use_cross_posting()?;` and never call `_handler` functions directly.

A separate `UseSyndicationPanel(post_id)` hook drives the post-detail panel — author-gated `Loader<SyndicationPanelResponse>` plus a `handle_retry: Action<(SocialPlatform,), ()>` that re-enqueues a single job, and a `handle_refresh_engagement: Action<(SocialPlatform,), ()>` that triggers an out-of-band fetch (does not perturb Stage 4 schedule).

### Routes

New `Route` enum variants:
- `Route::OnboardingConnectSocialsPage` — `/onboarding/connect-socials`, shown post-signup if `cross_posting_interstitial_seen == false`.
- `Route::SettingsConnectionsPage` — `/settings/connections`.

The OAuth callback URL given to LinkedIn / Meta is **only** the server endpoint listed in API surface (`GET /api/cross-posting/oauth/{platform}/callback`). After validating tokens the server issues a 302 to `Route::SettingsConnectionsPage` (or to the onboarding interstitial when `cross_posting_interstitial_seen == false`) — there is no separate client-side `/auth/{platform}/callback` route.

Public landing page (FR-8) **reuses the existing post-detail route** with relaxed auth: when the visitor is not signed in and the post is public, the layout swaps from the authenticated shell to a simplified "PublicBacklinkView" layout with the subscribe CTA + UTM banner. No new route — keeps SEO + canonical URL intact.

### Public landing banner — graceful degradation

OG-card previews and in-app browsers regularly strip both `Referer` headers and query strings, so the banner cannot rely on any single attribution signal. The landing page applies a three-tier fallback so the CTA is always present, only its specificity varies:

| Tier | Trigger | Banner copy (EN) | Banner copy (KO) |
|---|---|---|---|
| **1. Platform-specific (UTM)** | `?utm_source={bluesky\|linkedin\|threads}` present | "You're reading this on Ratel — the canonical source. Continue uninterrupted." (with `{platform_display}` logo) | `"{platform_display}에서 오셨군요. Ratel에서 전체 글을 끊김 없이 읽어보세요."` |
| **2. Platform-specific (Referer)** | UTM absent, but `Referer` matches a known external host (`bsky.app`, `linkedin.com`, `threads.net`) | Same as Tier 1 (`platform_display` inferred from Referer) | Same as Tier 1 |
| **3. Generic CTA** | Both UTM and Referer attribution missing | "Discover more insights from this creator on Ratel." | "이 크리에이터의 더 많은 인사이트를 Ratel에서 확인하세요." |

`{platform_display}` mapping:

| `SocialPlatform` | `platform_display` |
|---|---|
| `Bluesky` | `"Bluesky"` |
| `LinkedIn` | `"LinkedIn"` |
| `Threads` | `"Threads"` |

Implementation: SSR computes the tier from request headers + query and passes the resolved `BannerVariant` enum into the `PublicBacklinkView` component. UTM detection takes priority over Referer because UTM is more reliable across networks that strip the Referer header (D-7). The Tier 3 banner is never silently suppressed — Subscribe-to-Essence-House remains the primary conversion goal regardless of attribution.

### HTML-first conversion

Per `conventions/html-first-components.md`, each Stage 2 mockup file maps 1:1 to a component directory and uses `dx translate -f page.html` then field substitution. Class names and IDs are preserved verbatim from the mockup.

## Testing

### Server (`app/ratel/src/tests/cross_posting_tests.rs`)

Integration tests per `conventions/server-function-tests.md`:

- `test_connect_bluesky_stores_kms_encrypted_credential`
- `test_credentials_never_appear_in_response`  *(scrub guard)*
- `test_disconnect_marks_revoked_and_zeroes_credential`
- `test_oauth_callback_creates_connection`
- `test_oauth_callback_threads_without_ig_returns_error_and_no_connection`  *(D-6)*
- `test_list_connections_includes_status_and_counts`
- `test_toggle_auto_post_persists`
- `test_update_post_publish_writes_directive_in_same_transact`  *(Post isolation, atomic batch)*
- `test_update_post_publish_without_enabled_platforms_writes_no_directive`  *(Ratel-only path, FR-4 #27)*
- `test_update_post_draft_does_not_write_directive`  *(directive only on publish-transition)*
- `test_update_post_republish_after_visibility_flip_does_not_double_syndicate`  *(MODIFY filter on Draft→Published transition only, not on every Published-state save)*
- `test_stage1_factory_bakes_one_job_per_enabled_platform`
- `test_stage1_factory_skips_disconnected_platforms_in_directive`
- `test_dispatch_skips_private_post`  *(FR-6 #39)*
- `test_dispatch_skips_team_shared_post`  *(FR-9 #50)*
- `test_dispatch_idempotent_on_already_published_job`  *(FR-5 #34)*
- `test_dispatch_lock_prevents_duplicate_external_call`  *(crash mid-flight, second invocation must not re-publish)*
- `test_lock_steal_recovers_published_state_via_find_by_backlink`  *(stale lock + adapter reports prior success → adopt instead of re-publish)*
- `test_lock_steal_proceeds_when_find_by_backlink_returns_none`  *(stale lock + no prior post → publish normally)*
- `test_dispatch_uses_body_override_when_present`  *(v1.5 readiness)*
- `test_dispatch_falls_back_to_format_when_override_absent`  *(Phase 1 path)*
- `test_format_truncates_to_bluesky_limit_with_backlink_intact`  *(AC-22)*
- `test_format_includes_utm_source_per_platform`  *(AC-23)*
- `test_retry_endpoint_resets_only_target_platform`  *(AC-15)*
- `test_engagement_scheduler_uses_1h_interval_for_fresh_post`
- `test_engagement_scheduler_uses_6h_interval_after_24h`
- `test_engagement_scheduler_stops_after_30d`
- `test_syndication_panel_hidden_for_non_author`  *(AC-29)*
- `test_logs_redact_credentials_and_body_content`  *(AC-28, FR-10 #53)*
- `test_landing_banner_tier1_with_utm`
- `test_landing_banner_tier2_with_referer_only`
- `test_landing_banner_tier3_generic_when_unattributed`

### Stream handler

Extend `common/stream_handler.rs` tests for the two new branches (Post MODIFY Draft→Published factory, SyndicationJob INSERT dispatch) with mocked adapters that assert the right body / images / backlink. Engagement and retry sweepers are tested as plain async functions in `cross_posting_tests.rs`.

### Adapter unit tests

Per platform: round-trip serialization of API request bodies, error-category mapping, refresh-token logic. No live API — adapters take a `reqwest::Client` injection point so tests use a mock.

### E2E (`playwright/tests/web/cross-posting.spec.js`)

Full scenario per `conventions/playwright-tests.md`. Backend runs with `--features bypass`; platform adapters use a `BYPASS_PLATFORM_API=mock` env that records calls and returns fixed responses (so AC-12 / AC-13 / AC-22 / AC-23 / AC-27 are deterministic without hitting Bluesky).

Maps to acceptance criteria AC-1 through AC-29 across these `test()` blocks (extending one serial suite):

1. Signup → interstitial visible (AC-1)
2. Connect Bluesky in interstitial → row flips Connected (AC-2)
3. Start LinkedIn OAuth → mock callback → row Connected (AC-3)
4. Skip → home, no toast / no error (AC-4)
5. Sign out + back in → no interstitial (AC-4b)
6. Settings/Connections shows Bluesky+LinkedIn connected, Threads coming-soon (AC-5)
7. Toggle LinkedIn auto-post off → reload persists (AC-6)
8. Disconnect Bluesky → confirm modal → row Not connected (AC-7)
9. Compose post → sidebar shows both connected, Threads connect-CTA (AC-8)
10. Disable LinkedIn → reach count drops by 1 (AC-9)
11. >300 chars → Bluesky warning visible, LinkedIn clean (AC-10)
12. Private visibility → no sidebar, no directive written (AC-11)
13. Publish → both rows Pending → Published with URLs (AC-12)
14. Mock LinkedIn auth_expired → row Failed + Reconnect CTA (AC-13)
15. In-app notification visible (AC-14)
16. Retry → only LinkedIn re-enqueues (AC-15)
17. Bluesky-side body contains backlink (AC-16, AC-22, AC-23)
18. Public landing renders signed-out (AC-17)
19. Tier-1 banner present with `?utm_source=bluesky` (AC-18)
20. Engagement counts appear in panel after refresh (AC-19)
21. Public→private switch shows "syndicated copies remain visible" notice (AC-20)
22. Truncation: > 300-char body sent as `{title}\n\n{first}…\n{backlink}` (AC-22)
23. Visibility flip mid-flight → state Skipped (AC-27)
24. Logs contain post id + platform + retry-stage but no creds / body (AC-28)
25. Non-author detail page hides syndication panel (AC-29)
26. Landing page reached without UTM and without Referer → Tier-3 generic banner (graceful degradation)

Lighthouse mobile ≥ 80 / LCP < 2.5s (AC-21) is verified out of the e2e suite via a manual Lighthouse CI run on the public landing page; the e2e test only asserts the landing page renders.

`network_error` retry stages (AC-24) and `auth_expired` no-retry (AC-25) are server-test territory — driven by a stub adapter in `cross_posting_tests.rs`.

## CDK

`cdk/lib/dynamo-stream-event.ts` adds:
- **Pipe + Rule**: Post MODIFY (`sk` prefix `POST#`) with state-transition filter (`OldImage.status != "Published" AND NewImage.status == "Published" AND NewImage.visibility == "Public"`) → `DetailType::PostPublishedForSyndication` (Stage 1 factory).
- **Pipe + Rule**: SyndicationJob INSERT/MODIFY (`sk` prefix `SYNDICATION_JOB#`) with `state=Pending` filter → `DetailType::SyndicationJobReady` (Stage 2 dispatcher).
- **Schedule (1D)**: CloudWatch every 1 min → `DetailType::SyndicationRetrySweep` (Stage 3).
- **Schedule (1D)**: CloudWatch every 15 min → `DetailType::EngagementRefreshSweep` (Stage 4) — separate rule and Lambda alias from Stage 3 so concurrency, IAM scope, and alarm thresholds can diverge.

Four new variants on `DetailType` and four match arms in `EventBridgeEnvelope::proc()` per `conventions/implementing-event-bridge.md`.

## Rollout / feature flags

Add Cargo feature `cross_posting`. Stages 1A–1D land behind it; merge with the flag off to de-risk:

1. **1A Bluesky**: model + connections endpoints + Bluesky adapter + Stage 1+2 events + compose sidebar + post-detail panel + Settings page Bluesky-only. Threads/LinkedIn rows show "Coming soon".
2. **1B LinkedIn**: OAuth start/callback + LinkedIn adapter + LinkedIn row enabled.
3. **1C Threads**: Meta OAuth + adapter + IG-account guard modal. Gated on Meta App Review approval; merging the code without enabling the connect CTA is fine.
4. **1D**: Stage 3 retry sweeper + Stage 4 adaptive engagement scheduler + onboarding interstitial route + public backlink-landing layout polish (3-tier banner) + Lighthouse pass.

### Production enablement gate

Production rollout (Cargo feature `cross_posting` enabled in `make build` for `prod`) **MUST** wait until 1D is complete. Rationale: spec FR-5 #34 mandates automatic retry with 1m/10m/1h backoff, and the retry sweeper that delivers this lands in 1D — enabling 1A~1C in prod alone would violate the spec by leaving Failed jobs unrecovered until user-initiated retry.

Concrete gate criteria — *all* must be true before flipping the prod flag:

- [ ] Stage 3 retry sweeper deployed and passing soak test (24h with simulated failures showing 1m/10m/1h backoff)
- [ ] Stage 4 engagement sweep deployed and emitting `EngagementSnapshot` rows
- [ ] All 30 acceptance criteria (AC-1 ~ AC-29 + AC-4b) verified green
- [ ] Lighthouse mobile ≥ 80 / LCP < 2.5s on public landing (AC-21)
- [ ] Bluesky / LinkedIn / Threads adapters all deployed (or Threads explicitly held back behind sub-flag if Meta review still pending)

Until then, 1A~1C live in **internal staging only** (`ENV=staging` builds with `cross_posting=on`). Internal team can validate end-to-end on staging without the spec-violating gap leaking to production users.

If business pressure pushes 1A to production before 1D, the alternative is **Option B** (in-Lambda inline retry within Stage 2 invocation, partial spec coverage — 1m/10m only, not 1h). Option B is **not** the chosen path here; revisit only if 1D slips beyond an acceptable launch window.

## Resolved decisions

Resolved during Stage 3 alignment (2026-04-28); previously open as OQ-1 / OQ-2 / OQ-3 in [roadmap/cross-posting.md](../../../roadmap/cross-posting.md):

- **OQ-1 → Resolved.** *Per-network compose variants (v1.5 readiness.)* The canonical `Post` entity stays free of per-platform formatting fields. Per-post intent — both `enabled_platforms` and `platform_overrides` — is carried on a sidecar `PostSyndicationDirective` written inside the same `update_post_handler` Publish-branch transact batch as the `Post` updater. Stage 1 Lambda is the **factory**: it reads the directive, intersects with the author's connections, and bakes one `SyndicationJob` per platform with the override (when present) attached. v1.5 adds `body_override: Option<String>` to `SyndicationJob` and a UI for authoring per-network text — no schema migration on `Post`, no dispatcher rewrite.

- **OQ-2 → Resolved.** *Engagement-refresh schedule.* A **separate** CloudWatch schedule from the retry sweeper (separate Lambda alias, IAM scope, alarms), running every 15 min. Cadence is **adaptive** based on `Feed.created_at` age: 1 h while the post is < 24 h old, 6 h up to 7 d, 24 h up to 30 d, then stop. The next-fire timestamp lives on `SyndicationJob.engagement_next_at` (Number-typed GSI sort key); the GSI is partitioned by `engagement_shard` (sparse — set to `None` when polling stops or the connection is revoked, dropping the row from the index). The sweeper fans out `Query` calls across all 4 shards in parallel.

- **OQ-3 → Resolved.** *Public-landing referrer banner.* Three-tier graceful degradation: (1) UTM-attributed → platform-specific banner, (2) Referer-attributed → platform-specific banner, (3) neither → generic "이 크리에이터의 더 많은 인사이트를 Ratel에서 확인하세요" CTA. The banner is never hidden — only its specificity varies — so the subscribe conversion path is intact even on OG-card and in-app-browser visits that strip both signals.

## Cross-feature dependencies

- **Notification inbox** (`features/notifications`): the `auth_expired` notification (FR-5 #35) is delivered through the existing inbox by adding a new `InboxKind::CrossPostingAuthExpired` variant and a matching `InboxPayload` arm at `app/ratel/src/common/types/inbox_kind.rs`. Payload shape: `{ platform: SocialPlatform, connection_sk: SocialConnectionEntityType, cta_url: String }`. The `cta_url` resolves to `/settings/connections?reconnect={platform}` so the inbox click lands the user directly on the reconnect modal. The cross-posting feature PR must include the variant addition (and EN/KO translations) to avoid merge conflicts with concurrent inbox work.
- **`Post` visibility hook**: Stage 2's privacy guard relies on `Post::find_by_pk(cli, &job.pk)` returning the latest `visibility` and `status` values. Any future change to Post's visibility model (e.g., adding `Restricted` or per-list visibility) or status state-machine must extend the guard's check — flag in the `posts` feature's design doc when introduced.

## Risks

- **Meta App Review timing.** Threads connect CTA must be feature-flag-gated until approval lands. Plan: ship 1C code dark, flip the flag once Meta approves.
- **Per-platform rate limits under bursty publishing.** A creator publishing 10 posts in 60 seconds × 3 platforms = 30 outbound calls; LinkedIn's per-member rate is the tightest. Stage 2 dispatcher must serialize per `(user, platform)` — implemented via a per-user-per-platform 1-second token bucket in the Lambda or by relying on adapter-level retry-on-429.
- **Credential leakage via logs.** Highest-blast-radius risk. Mitigate with a `Redacted` newtype around `Vec<u8>` whose `Debug`/`Display` print `<redacted>`, plus a logger boundary check that strips known fields. Verified by `test_logs_redact_credentials_and_body_content`.
- **Duplicate external posts on Lambda mid-flight death.** None of the Phase 1 platforms expose a server-side idempotency token (Bluesky's `createRecord` mints a fresh CID each call), so a Lambda that crashes between *"platform API returned success"* and *"DB write Published"* would, on AWS-Lambda automatic retry, re-call the platform and double-publish. Mitigation = the Stage 2 dispatcher's two-phase commit: (1) acquire `dispatch_lock_id` via conditional `UpdateItem` before the API call, (2) on stolen-lock recovery (TTL elapsed), call `adapter.find_by_backlink(backlink_url)` to detect a prior successful publish before issuing a second one. Verified by `test_dispatch_lock_prevents_duplicate_external_call` and `test_lock_steal_recovers_published_state_via_find_by_backlink`.
- **Engagement sweep cost growth.** Stage 4's adaptive cadence keeps long-tail polling cheap, but a viral post entering "stop" at 30 d still represents (24×1) + (6×7×4) + (1×23) ≈ 215 fetches per platform over its lifetime. Monitor `EngagementRefreshSweep` invocation count + adapter call count weekly; if budget tightens, shorten the 30-d cap to 14 d.
- **Re-sharding migration (SHARD_COUNT growth).** SHARD_COUNT is fixed at 4 and is intentionally low for current scale. If sweep-Query result pages start hitting DynamoDB's 1 MB pagination limit (monitor: `Query` page count per sweep cycle ≥ 2), bump to 32 via a **dual-read** transition: (1) PR-1 changes `SHARD_COUNT` to 32 in `shard_for` AND extends both sweepers to Query *both* the legacy 4 shards and the new 32 shards (36 parallel Queries during transition); new INSERTs land on the new shards naturally. (2) Wait for legacy rows to drain — retry queue drains within ~70 min (1m+10m+1h backoff exhaustion), engagement queue drains within 30 d (post-age cap). (3) PR-2 removes the legacy-shard Query path. Engineer time ≈ 1.5 days, wall-clock ≈ 30 d, marginal DynamoDB cost during transition ≈ $5/month, no data migration / backfill required, no downtime.
