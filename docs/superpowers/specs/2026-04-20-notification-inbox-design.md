# Notification Inbox — Design

Date: 2026-04-20
Status: Approved (brainstorm) — pending implementation plan

## Problem

Ratel currently has only an outbound notification pipeline (`Notification` model with `status: Requested|Completed`) that delivers email/push and then marks itself complete. There is no user-facing inbox: users cannot see a history of events relevant to them, and there is no unread state to surface.

We want to introduce a user notification inbox that:

1. Aggregates user-relevant events into per-user entries with unread/read state.
2. Surfaces unread notifications on the **home** page and inside every **Space arena index** page.
3. Reuses the existing slide-in panel pattern (Overview / Leaderboard / Settings) for consistency.

## Goals

- Single shared `NotificationPanel` component rendered on both home and space arena.
- Bell icon with unread-count badge in the Navbar (general pages) and ArenaTopbar (space index).
- MVP covers 8 event kinds. 4 reuse existing email pipelines; 4 are new.
- Read state managed per-item and per-user, with "mark all as read".
- Storage capped via DynamoDB TTL (90 days) to bound cost.

## Non-Goals

- Per-kind user preference toggles (future).
- Push notifications as part of this feature (existing push pipeline unchanged).
- Server-pushed real-time updates (use polling).
- Verification code / OTP messages in the inbox (kept email-only).

## Event Kinds (MVP)

| Kind | Existing email? | Source | Inbox recipient(s) |
|---|---|---|---|
| `ReplyOnComment` (A) | Yes | `common/utils/reply_notification::send_reply_on_comment` | Parent author + thread participants (same resolution as email) |
| `MentionInComment` (B) | Yes | `common/utils/mention` pipeline | Mentioned users |
| `SpaceStatusChanged` (C) | Yes | `spaces/space_common/services/space_status_change_notification` | Space participants |
| `SpaceInvitation` (D) | Yes | Space invitation controller (email send point) | Invited users |
| `NewCommentOnMine` (E) | No (inbox only) | Post/space comment creation controller | Post/space author (skip self-comment) |
| `FollowedUserPosted` (F) | No (inbox only) | EventBridge on Feed INSERT | Followers of the author |
| `NewSpaceAction` (G) | No (inbox only) | EventBridge on SpacePoll / SpaceQuiz / SpaceDiscussion INSERT | Space participants |
| `RewardGranted` (H) | No (inbox only) | Existing reward grant service | Awarded user |

For A–D, the inbox row is written **alongside** the email send (same recipient set). For E and H, the row is written inline at the triggering controller. For F and G, the fan-out is non-trivial and handled via EventBridge Pipes + Rules to the app-shell Lambda.

## Data Model

New entity `UserInboxNotification` in `app/ratel/src/common/models/notification/inbox.rs`:

```rust
pub struct UserInboxNotification {
    #[dynamo(prefix = "UIN", index = "gsi1", name = "find_inbox_by_user_unread", pk)]
    pub pk: Partition,              // User(user_id) — recipient-keyed partition

    pub sk: EntityType,             // UserInboxNotification(uuid v7) — time-ordered

    pub created_at: i64,

    #[dynamo(index = "gsi1", sk)]
    pub unread_created_at: String,  // is_read ? "R" : format!("U#{created_at}")

    pub is_read: bool,

    pub kind: InboxKind,
    pub payload: InboxPayload,      // tagged enum, per-kind fields

    pub expires_at: i64,            // DynamoDB TTL: created_at + 90 days
}

pub enum InboxKind {
    ReplyOnComment,
    MentionInComment,
    SpaceStatusChanged,
    SpaceInvitation,
    NewCommentOnMine,
    FollowedUserPosted,
    NewSpaceAction,
    RewardGranted,
}

pub enum InboxPayload {
    ReplyOnComment {
        space_id: Option<SpacePartition>,
        post_id: Option<FeedPartition>,
        comment_preview: String,
        replier_name: String,
        replier_profile_url: String,
        cta_url: String,
    },
    MentionInComment {
        comment_preview: String,
        mentioned_by_name: String,
        cta_url: String,
    },
    SpaceStatusChanged {
        space_id: SpacePartition,
        space_title: String,
        new_status: SpaceStatus,
        cta_url: String,
    },
    SpaceInvitation {
        space_id: SpacePartition,
        space_title: String,
        inviter_name: String,
        cta_url: String,
    },
    NewCommentOnMine {
        post_id: FeedPartition,
        commenter_name: String,
        preview: String,
        cta_url: String,
    },
    FollowedUserPosted {
        post_id: FeedPartition,
        author_name: String,
        author_profile_url: String,
        title: String,
        cta_url: String,
    },
    NewSpaceAction {
        space_id: SpacePartition,
        space_title: String,
        action_kind: SpaceActionKind,
        cta_url: String,
    },
    RewardGranted {
        amount: i64,
        reason: RewardReason,
        space_id: Option<SpacePartition>,
        cta_url: String,
    },
}
```

### Design notes

- **Per-user partition.** `pk = User(user_id)` makes listing a user's inbox a pk-bounded query (time-ordered by `sk`).
- **Sparse GSI for unread.** `gsi1` indexes by `(pk, unread_created_at)`. When read, `unread_created_at` is set to `"R"` which we filter out, so the index returns only unread entries. This makes both the list-unread query and unread-count very cheap.
- **Idempotency.** For A/B/E we check for an existing row for `(comment_pk, recipient_pk, kind)` before creating, to avoid double delivery on retries. For C/F/G/H the trigger fires once per source event, so no additional dedup is needed.
- **Cross-transaction isolation.** Email send and inbox row creation are independent — one failure does not block the other (log `crate::error!` only).
- **TTL.** `expires_at = created_at + 90 days`. DynamoDB TTL handles deletion server-side.
- **Structured payload over rendered text.** Rendering is done on the frontend using i18n, so the same row renders in EN or KO without backfill.

## Server API

All endpoints under new module `app/ratel/src/features/notifications/controllers/`. Session-authed; each enforces `pk = User(session_user_id)` — a user can never read or mutate another user's inbox.

| Method | Path | Purpose | Response |
|---|---|---|---|
| GET | `/api/inbox?unread_only&bookmark` | List inbox (paginated). `unread_only=true` uses sparse GSI | `ListResponse<InboxNotificationResponse>` |
| GET | `/api/inbox/unread-count` | Badge counter | `{ "count": i64 }` (capped at 100) |
| POST | `/api/inbox/{inbox_id}/read` | Mark one entry read | `()` |
| POST | `/api/inbox/read-all` | Mark all unread read | `{ "affected": i64, "has_more": bool }` |

Rules:

- `inbox_id` path param is `UserInboxNotificationEntityType` (SubPartition). No prefix leaks to clients.
- `GET /inbox` uses `Model::opt_with_bookmark(bookmark).limit(30)`.
- `read-all` walks GSI1 in pages (`max_pages=5`, `limit=30` → up to 150 per call). If more remain, `has_more=true` so the client can call again.
- `unread-count` uses `count_by_pk` on GSI1 with a 100 cap (display shows `"99+"` when `count >= 100`).

Response DTO:

```rust
pub struct InboxNotificationResponse {
    pub id: UserInboxNotificationEntityType,
    pub kind: InboxKind,
    pub payload: InboxPayload,
    pub is_read: bool,
    pub created_at: i64,
}
```

### MCP

Expose `list_inbox` and `get_unread_count` per `conventions/mcp-tools.md`. Read-state mutations are UI actions and are not exposed to MCP.

## Event → Inbox Pipeline

### Shared helper

`app/ratel/src/common/utils/inbox.rs`:

```rust
pub async fn create_inbox_row(
    recipient_pk: Partition,
    kind: InboxKind,
    payload: InboxPayload,
) -> Result<()>
```

For A/B/E, the helper exposes an idempotent variant:

```rust
pub async fn create_inbox_row_once(
    recipient_pk: Partition,
    kind: InboxKind,
    payload: InboxPayload,
    dedup_source: &str,  // e.g. comment pk
) -> Result<()>
```

that short-circuits if an existing row for `(recipient_pk, kind, dedup_source)` already exists. Dedup uses a lightweight sibling entity `InboxDedupMarker { pk: User(user_id), sk: "DEDUP#{kind}#{source_id}", expires_at }` with a 7-day TTL. A conditional put on the marker acts as the gate: if the marker exists, skip inbox creation. This keeps the inbox row's `sk` as a time-ordered uuid v7 so listing stays chronological.

### Per-kind wiring

- **A, B, C, D:** add a call next to the existing email send site in the current services / utils. On failure, log only — do not bubble up.
- **E, H:** inline in the triggering controller / service.
- **F, G:** CDK Pipe + Rule (`cdk/lib/dynamo-stream-event.ts`).
  - F: filter Feed `sk` prefix INSERT → `DetailType::FollowedUserPosted` → handler looks up followers via existing Follow GSI and batches `create_inbox_row`.
  - G: filter `SPACE_POLL#` / `SPACE_QUIZ#` / `SPACE_DISCUSSION#` INSERT → `DetailType::NewSpaceAction` → handler looks up SpaceUser participants.
  - Mirror branches in `common/stream_handler.rs` for local-dev parity.

Batch size for fan-out handlers is capped at 500 recipients per event; additional recipients are paginated across pages within the same handler invocation. If the handler exceeds the Lambda timeout, the remainder is dropped (best-effort notification — no SLA).

## Frontend

New feature module `app/ratel/src/features/notifications/`:

```
notifications/
├── controllers/             — server API (4 endpoints)
├── models/inbox.rs          — UserInboxNotification
├── components/
│   ├── notification_bell/   — bell + badge (used in Navbar + ArenaTopbar)
│   │   ├── component.rs
│   │   └── style.css
│   └── notification_panel/  — slide-in panel (home + space)
│       ├── component.rs
│       ├── notification_item/
│       │   ├── component.rs
│       │   └── style.css
│       └── style.css
├── hooks/
│   ├── use_inbox.rs         — use_infinite_query paging
│   └── use_unread_count.rs  — use_loader + 60s polling
├── i18n.rs
└── types/error.rs
```

### Insertion points

- **Navbar** (`common/components/navbar`): insert `NotificationBell` to the left of the user avatar. Covers home and all general pages.
- **ArenaTopbar** (`features/spaces/pages/index/arena_topbar/component.rs`): insert `NotificationBell` alongside Overview / Leaderboard / Settings buttons.
- **SpaceIndexPage**: add `Notifications` variant to `ActivePanel` enum so the panel shares the slide-open behavior already defined for Overview / Leaderboard / Settings.

### NotificationItem (per-kind rendering)

Each item renders: left icon (kind-specific, `lucide_dioxus`) or avatar (for user-centric kinds), title + body, relative timestamp, and a right-aligned unread dot.

- `ReplyOnComment` / `MentionInComment` / `NewCommentOnMine`: user avatar + name + 2-line preview (ellipsis).
- `SpaceStatusChanged` / `NewSpaceAction`: space logo + title + status/action message (enum values rendered via `status.translate(&lang())`).
- `SpaceInvitation`: inviter avatar + `"{inviter_name} invited you to {space_title}"`.
- `FollowedUserPosted`: author avatar + name + post title.
- `RewardGranted`: coin icon + `"+{amount} earned"` + reason.

On click: `nav.push(Route::…)` (CTA path mapped to a `Route` variant by kind) and `mark_as_read` fired in parallel via `spawn`. Navigation happens **after** awaits per the async handler rule.

### Read-state UX

- Unread: `bg-primary/10` background + left-side dot.
- Read: `opacity-70`, no dot, still visible in list.
- "Mark all as read" button at the top-right of the panel → calls `read-all`, refetches list.
- When a user clicks an item that is currently unread, the client optimistically flips it to read and calls the endpoint in the background.

### Polling

- `use_unread_count` refetches every 60 seconds using `crate::common::utils::time::sleep` (never `gloo_timers` directly).
- Opening the panel triggers an immediate refetch of the list so the user always sees current state.

### Styling + i18n

- All user-facing text uses `translate!` (EN + KO).
- Colors use semantic tokens only (`bg-card-bg`, `text-foreground`, `text-foreground-muted`, `bg-primary/10`, `bg-destructive`).
- Bell badge: `bg-destructive text-white` rounded pill, `99+` cap.
- HTML-first: mock the panel at `app/ratel/assets/design/notification-panel.html` first, approve, then `dx translate`.

## Testing

### Server tests (`app/ratel/src/tests/notifications_tests.rs`)

- `test_list_inbox_unread_only_filters_sparse_gsi`
- `test_unread_count_caps_at_100`
- `test_mark_as_read_flips_gsi_key`
- `test_read_all_processes_in_pages_and_returns_has_more`
- `test_create_inbox_row_idempotent_per_comment`
- `test_unauthenticated_access_rejected`
- `test_cross_user_inbox_access_rejected`

### Stream handler tests

- Extend `common/stream_handler.rs` tests for `FollowedUserPosted` and `NewSpaceAction` branches (fan-out correctness with mocked follower / participant sets).

### E2E (`playwright/tests/web/notifications.spec.js`)

1. user1 replies to user2's comment → user2 logs in → bell badge shows `1` → open panel → item visible → click → navigates to target → badge disappears.
2. user1 posts → user2 (follower) opens home within 30 seconds → bell badge shows `1` (covers F / EventBridge path).
3. "Mark all as read" button clears the badge.

## CDK

Add two Pipes + two Rules in `cdk/lib/dynamo-stream-event.ts`:

- **Feed INSERT → followers fan-out**: Pipe filters Feed `sk` prefix INSERT; target `DetailType::FollowedUserPosted`. Rule routes to app-shell Lambda.
- **Space action INSERT → participants fan-out**: Pipe filters `SPACE_POLL#` / `SPACE_QUIZ#` / `SPACE_DISCUSSION#` INSERT; target `DetailType::NewSpaceAction`. Rule routes to app-shell Lambda.

Add matching branches in `app/ratel/src/common/stream_handler.rs` for local-dev.

## Rollout

1. Model + server API + wiring for A–D (email pipelines stay untouched; inbox is additive).
2. Frontend bell + panel (home + space).
3. Event E (inline) — new-comment trigger.
4. Event H (inline) — reward trigger.
5. EventBridge F + G (CDK deploy, then server handlers).

### Feature flag

Add `notifications` Cargo feature, included in `full`. Stages 1–2 can merge with the flag off to de-risk.

## Open Questions

None — all questions resolved during brainstorm. Implementation plan will be produced next via `writing-plans`.
