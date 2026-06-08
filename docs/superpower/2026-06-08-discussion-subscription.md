# Discussion Subscription — System Design

**Slug**: `discussion-subscription`
**Author / Date**: victor · 2026-06-08
**Branch**: `feature/discussion-subscription`

## Summary

Let space members subscribe to a single discussion. While subscribed, a member
receives an in-app notification **and** an email every time a comment or reply
is posted on that discussion (excluding their own posts). A toggle button
(구독하기 / 구독중) sits in the discussion top bar. The discussion author is
auto-subscribed on creation. Each recipient gets **exactly one** notification
per comment, deduped across mention / reply / subscription with priority
**mention > reply-target > subscriber**.

## Decisions (locked)

| Question | Decision |
|---|---|
| Subscription scope | A single discussion (this discussion only) |
| Auto-subscribe | Discussion **author** auto-subscribes on creation; everyone else via button |
| Button | Toggle: 구독하기 ↔ 구독중(해지) |
| Eligibility | Logged-in space members/participants (`SpaceUserRole` + `SpaceUser`) |
| Trigger | Every comment **and** reply → in-app noti + email to all subscribers, actor excluded |
| Fan-out mechanism | Async — one `Notification` row → DynamoDB Stream → send-time recipient resolution |
| Dedup | **One notification per recipient per comment**, priority mention > reply-target > subscriber |

## Data model

New DynamoEntity, stored under the discussion's `SpacePost` partition (same
partition the discussion's comments use), so the subscriber list is one
`find_by_pk` away at send time.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, DynamoEntity)]
pub struct SpacePostSubscription {
    pub pk: Partition,        // SpacePost(post_id) — discussion post partition
    pub sk: EntityType,       // SpacePostSubscription#{user_id}
    pub space_pk: Partition,  // for CTA/context
    pub user_pk: Partition,
    pub created_at: i64,
}
```

- Central `EntityType` enum gains a `SpacePostSubscription(String)` variant
  (with its `#[dynamo(prefix = "...")]` + SubPartition wiring), suffix = user id.
- List subscribers: `find_by_pk(post_pk)` with sk `begins_with` the subscription
  prefix — a **direct DynamoDB Query, never a table Scan**.
- Is-subscribed check: `get(post_pk, sk)` — single-item GetItem.
- Toggle: `create` / `delete`.
- **Access-pattern rule**: all reads here are point GetItem or partition Query;
  no `Scan` anywhere. If a future access pattern (e.g. "discussions a user
  subscribed to") is added, back it with a GSI (`pk = user_pk`, sk = post id) —
  do **not** fall back to scanning. Out of scope for v1, so no GSI yet.
- Auto-subscribe: discussion-create controller writes one `SpacePostSubscription`
  for the author.

## API surface

```
POST   /api/spaces/{space_id}/discussions/{discussion_sk}/subscribe   → subscribe (idempotent)
DELETE /api/spaces/{space_id}/discussions/{discussion_sk}/subscribe   → unsubscribe
```

- Both: `role: SpaceUserRole, member: SpaceUser` — members only. `space_id`/
  `discussion_sk` are SubPartition path params (no prefix). Return `Result<()>`
  (or a tiny `{ subscribed: bool }` echo).
- **Subscription state** (`subscribed: bool`) is folded into the existing
  discussion detail response (`disc_loader`) — no extra round trip on page load,
  button renders correct state immediately. The detail controller already has
  the current user via extractors; add the `get(post_pk, sk)` check there.
- Files: `features/spaces/pages/actions/actions/discussion/controllers/comments/`
  sibling dir or a new `subscription/` subdir; register routes in the discussion
  controllers `mod.rs`.

## Notification fan-out (async, single notification per comment)

### Trigger

Both `add_comment` and `reply_comment` controllers, after creating the comment,
fire **one** `Notification` row:

```rust
NotificationData::DiscussionCommentPosted {
    space_id: SpacePartition,
    discussion_id: String,          // SpacePost id (to rebuild post_pk + CTA)
    discussion_title: String,
    comment_sk: String,             // the new comment's sk — dedup key
    parent_comment_sk: Option<String>, // Some(..) when this is a reply
    commenter_pk: String,
    commenter_name: String,
    comment_content: String,        // for preview + mention parsing at send time
    cta_url: String,
}
```

`add_comment` currently fires no reply notification; `reply_comment` currently
fires `NotificationData::ReplyOnComment` separately. **For discussions, that
separate `ReplyOnComment` fire is removed** — the unified notification below
covers reply targets too. The post (non-discussion) `ReplyOnComment` path is
untouched.

### Send-time resolution — `send_discussion_comment_posted()`

New server-only helper (sibling to `reply_notification.rs`). Runs via the
existing `NOTIFICATION#` stream branch → `notification.process()` →
`NotificationData::send()`. Single pass with a shared `seen` set guarantees one
notification per recipient:

1. `seen` starts with `commenter_pk` (actor never notified).
2. **Mention (priority 1):** mentions are still created synchronously in the
   controllers (`create_mention_notifications`, unchanged). Here we only
   `extract_mentioned_pks(&comment_content)` and add them to `seen` so reply/
   subscription do not re-notify mentioned users.
3. **Reply targets (priority 2):** only when `parent_comment_sk` is `Some`.
   Reuse the existing discussion-thread resolver (parent author + prior thread
   participants, like `fetch_space_discussion_thread`). For each not in `seen`:
   add to `seen`, create a `ReplyOnComment` inbox row, collect email for the
   `ReplyOnCommentNotification` template.
4. **Subscribers (priority 3):** `SpacePostSubscription::find_by_pk(post_pk)`.
   For each not in `seen`: add to `seen`, create a `DiscussionCommentPosted`
   inbox row, collect email for the new `DiscussionCommentNotification` template.
5. Send up to two SES bulk emails (reply-bucket, subscriber-bucket); each user
   appears in at most one bucket.
6. Inbox dedup markers keyed by `comment_sk` per recipient
   (`create_inbox_row_once`), so stream retries don't double-insert.

### New enum variants

- `InboxKind::DiscussionCommentPosted` (+ `as_prefix` e.g. `"DISC_CMT"`).
- `InboxPayload::DiscussionCommentPosted { space_id, discussion_id,
  discussion_title, commenter_name, commenter_profile_url, comment_preview,
  cta_url }` (+ `url()` + `kind()` arms).
- `NotificationData::DiscussionCommentPosted { .. }` (+ `send()` arm calling the
  helper).
- `EmailOperation::DiscussionCommentNotification { commenter_name,
  discussion_title, comment_preview, cta_url }` (+ `template_name()`).

### Event flow

```
add_comment / reply_comment
   └─ create comment
   └─ create_mention_notifications (sync, unchanged)        → mention inbox + email
   └─ Notification::new(DiscussionCommentPosted).create()   → 1 row
        └─ DynamoDB Stream INSERT (NOTIFICATION#)
             └─ EventBridge NotificationSend → Lambda
                  └─ NotificationData::send()
                       └─ send_discussion_comment_posted()
                            ├─ reply-target bucket  → ReplyOnComment inbox + email
                            └─ subscriber bucket    → DiscussionCommentPosted inbox + email
```

No new CDK Pipe/Rule — reuses the existing `Notification` entity and
`NotificationSend` detail type. (`stream_handler.rs` already routes `NOTIFICATION#`.)

## Frontend

- **Button**: in `discussion/component.rs` `topbar__right`, next to the status
  badge. `구독하기` (not subscribed) / `구독중` (subscribed; hover → 구독해지).
  Disabled while the toggle action is pending.
- **Controller** `UseDiscussionArena`: add `subscribed: Signal<bool>` (seeded
  from `disc_loader`'s `subscribed` field) + `handle_toggle_subscription:
  Action<(), ()>` that calls subscribe/unsubscribe handler, flips `subscribed`,
  exposes `.pending()`. Component consumes the action — no direct `_handler` calls.
- **Visibility**: render the button only for logged-in members (the page is
  already `SpaceUserRole`-gated; backend re-enforces membership).
- **i18n** (`i18n.rs`, EN+KO): `subscribe`, `subscribed`, `unsubscribe`,
  `subscribe_tooltip` ("이 토론에 새 댓글이 올라오면 알림과 이메일로 받아봅니다.").
- All custom CSS → `app/ratel/assets/main.css` under a section marker; semantic
  tokens only.

## Test plan

Server (`app/ratel/src/tests/`):
- subscribe creates a row; subscribe twice is idempotent; unsubscribe deletes.
- subscribe/unsubscribe require auth + membership (unauth → non-200).
- discussion detail response returns `subscribed` correctly per current user.
- after subscribe, a comment by another user → subscriber gets one
  `DiscussionCommentPosted` inbox row; commenter gets none. (Email is skipped
  under `bypass`, so assert on inbox rows. Call the send helper / process path
  directly.)
- dedup priority: mentioned subscriber gets only the mention row (no
  subscription row); a parent-author who also subscribes gets only the
  reply-target row (no subscription row).

Playwright: subscribe button toggles 구독하기 ↔ 구독중 and persists across reload.

## Implementation status (2026-06-08)

Shipped on `feature/discussion-subscription`:
- `SpacePostSubscription` model + `EntityType::SpacePostSubscription`.
- `POST`/`DELETE .../subscribe` (members-only, idempotent); author auto-subscribed on create.
- `subscribed: bool` on the discussion detail response (`OptionalUser`, anonymous → false).
- `InboxKind`/`InboxPayload::DiscussionCommentPosted`, `NotificationData::DiscussionCommentPosted`,
  `EmailOperation::DiscussionCommentNotification`, inbox-panel render arm.
- Unified send-time fan-out `send_discussion_comment_posted` (mention > reply-target > subscriber,
  one notification per recipient per comment); fired from `add_comment` + `reply_comment` (the latter
  replaces the old discussion `ReplyOnComment` fire). No CDK change — reuses `Notification` stream.
- Top-bar toggle button + `UseDiscussionArena.subscribed` / `handle_toggle_subscription`.

Verified: `cargo check` server/web/mobile clean (`-D warnings`); `dx check --web` clean;
9 `discussion_subscription_tests` pass; `discussion_tests` (3) + `notifications_tests` (6) regression green.
Playwright subscribe-toggle test added to `discussion-comment-deep-link.spec.js` (runs in CI's
`playwright-tests` job against the branch build).

Still pending (ops): provision the SES template `discussion_comment_notification` before production
email send (skipped under test/bypass).

## Open risks / dependencies

- **SES template provisioning**: `DiscussionCommentNotification` needs a real SES
  template created in AWS (ops). Skipped in test/bypass, so tests are unaffected,
  but production email needs the template deployed.
- **Email volume**: every comment emails every subscriber. Accepted for v1 per
  product. Batching/digest is a possible future enhancement (out of scope now).
- **Reply-target resolver scope**: reuses the existing 100-item thread scan cap
  from `fetch_space_discussion_thread`; very large threads beyond the cap won't
  notify older repliers (matches existing `ReplyOnComment` behavior).
