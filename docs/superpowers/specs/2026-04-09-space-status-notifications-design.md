# Space Status Change Notifications — Design Spec

**Date:** 2026-04-09
**Branch:** `notification/space-status-to-participant`
**Status:** Approved, ready for implementation planning

## Problem

Ratel spaces transition through `Designing → Open → Ongoing → Finished`.
Today only one transition triggers email: `Designing → Open` sends the
existing invitation email to users on the `SpaceInvitationMember` list.
The other transitions are silent, which means:

- Team members of a team-authored space learn their space went live only
  by checking the UI.
- Users who joined a space while it was `Open` ("candidates") are not told
  when the space actually starts.
- Participants are not told when the space ends.

This spec adds email notifications for all three transitions, using the
existing `Notification` + SES pipeline and a new `SpaceStatusChangeEvent`
entity to drive asynchronous, EventBridge-based fan-out.

## Goals

- Email-only notifications on each meaningful status transition.
- No latency added to the `update_space` HTTP handler beyond a single
  extra DynamoDB write.
- Reuse the existing `Notification` → DynamoDB stream → EventBridge → SES
  delivery path with no changes to its core behavior.
- Cleanly skip notifications when there is no audience (e.g. user-authored
  space publishing).

## Non-goals

- In-app notifications.
- Push notifications via the existing `user_notification` table.
- Per-user opt-out / notification preferences.
- Localized email copy (v1 ships English only).
- A UI for editing the copy.
- Retroactive notifications for spaces already past a transition.

## Decisions (resolved during brainstorming)

| Question | Answer |
|---|---|
| Delivery channel | Email only, reusing the existing SES pipeline |
| Audience on `Designing → Open` | Team members of the owning team (`UserTeamGroup::find_by_team_pk` + `TeamOwner`), **in addition to** the existing `SpaceInvitationMember` invitation email |
| Designing → Open for **user-authored** spaces | Skip entirely (no team members to notify) |
| Audience on `Open → Ongoing` | All `SpaceParticipant` records for the space |
| Audience on `Ongoing → Finished` | All `SpaceParticipant` records for the space |
| Email template strategy | One generic SES template `space_status_notification` with `{{headline}}`, `{{body}}`, `{{space_title}}`, `{{cta_url}}` variables; three parameter sets selected in Rust |
| Fan-out architecture | EventBridge-driven: controller writes a `SpaceStatusChangeEvent` row; stream → pipe → EB rule → Lambda handler resolves recipients and creates per-chunk `Notification` rows |

## Architecture

```
update_space (controller)
  │  on Publish / Start / Finish success:
  │    write one SpaceStatusChangeEvent row
  ▼
DynamoDB Stream — INSERT on SPACE_STATUS_CHANGE_EVENT#
  │
  ▼
CDK Pipe (SpaceStatusChangeEventPipe) ── filter: INSERT + sk prefix ──▶ EventBridge Bus
  │                                                                        detailType="SpaceStatusChangeEvent"
  ▼
EventBridge Rule (SpaceStatusChangeEventRule) ──▶ app-shell Lambda
  │
  ▼
EventBridgeEnvelope::proc() → DetailType::SpaceStatusChangeEvent branch
  │
  ▼
handle_space_status_change(event)    ← new service function
  │  1. Load SpaceCommon + Post for context (title, author, URL)
  │  2. Resolve recipient user_pks by transition:
  │       • Designing → Open:  team members (skip if author is a user)
  │       • Open → Ongoing:    SpaceParticipant::find_by_space
  │       • Ongoing → Finished: SpaceParticipant::find_by_space
  │       • anything else:     no-op
  │  3. User::batch_get → collect emails, dedupe
  │  4. Pick (headline, body) copy via status_change_copy(new_status)
  │  5. Chunk emails by 50; create one Notification row per chunk
  ▼
DynamoDB Stream — INSERT on NOTIFICATION#  (existing pipe, unchanged)
  │
  ▼
Notification::process() → NotificationData::send() → SES
                                                           │
                                                           ▼
                                           EmailOperation::SpaceStatusNotification
                                           template: "space_status_notification"
                                           vars: {headline, body, space_title, cta_url}
```

Two chained stream handlers:

1. **First leg (new):** explodes a single status-change event into per-chunk
   `Notification` rows. All recipient-query work happens here, off the HTTP
   request path.
2. **Second leg (existing):** delivers each `Notification` row via SES. No
   changes required.

## Data model

### New DynamoDB entity — `SpaceStatusChangeEvent`

Location: `app/ratel/src/common/models/space/space_status_change_event.rs`.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default, DynamoEntity)]
#[cfg_attr(feature = "server", derive(JsonSchema, OperationIo))]
pub struct SpaceStatusChangeEvent {
    pub pk: Partition,      // Partition::SpaceStatusChangeEvent(uuid_v7)
    pub sk: EntityType,     // EntityType::SpaceStatusChangeEvent(uuid_v7)

    pub created_at: i64,

    pub space_pk: Partition,
    pub old_status: Option<SpaceStatus>,
    pub new_status: SpaceStatus,
}
```

- `pk == sk == uuid_v7` mirrors the `Notification` entity shape: one row
  per event, no hot partition.
- `old_status` is `Option` because `Designing → Open` comes from
  `SpaceCommon.status = None` (draft spaces do not yet hold a status),
  not `Some(Designing)`.
- The entity carries only the space key and transition. The handler
  re-reads the live `SpaceCommon` and `Post` for title / author / URL,
  avoiding staleness and keeping the event row small.
- No GSI: each row is read once via the stream payload, then discarded.

### New enum variants

```rust
// app/ratel/src/common/types/partition.rs
pub enum Partition {
    ...
    SpaceStatusChangeEvent(String),  // uuid_v7
    ...
}

// app/ratel/src/common/types/entity_type.rs
pub enum EntityType {
    ...
    SpaceStatusChangeEvent(String),  // uuid_v7 (same id as pk)
    ...
}
```

Both serialize with `SPACE_STATUS_CHANGE_EVENT#{uuid}`, matching the
existing long-prefix naming conventions in the codebase.

### `NotificationData` — new variant

```rust
// app/ratel/src/common/types/notification_data.rs
pub enum NotificationData {
    None,
    SendVerificationCode { ... },
    SendSpaceInvitation { ... },
    SendSpaceStatusUpdate {
        emails: Vec<String>,
        headline: String,
        body: String,
        cta_url: String,
        space_title: String,
    },
}
```

`NotificationData::send()` gets a new match arm that constructs an
`EmailOperation::SpaceStatusNotification` and calls the existing
`EmailTemplate::send_email` path.

### `EmailOperation` — new variant

```rust
// app/ratel/src/features/auth/types/email_operation.rs
pub enum EmailOperation {
    SignupSecurityCode { ... },
    SpaceInviteVerification { ... },
    SpaceStatusNotification {
        headline: String,
        body: String,
        space_title: String,
        cta_url: String,
    },
}

impl EmailOperation {
    pub fn template_name(&self) -> &'static str {
        match self {
            EmailOperation::SignupSecurityCode { .. }      => "signup_code",
            EmailOperation::SpaceInviteVerification { .. } => "email_verification",
            EmailOperation::SpaceStatusNotification { .. } => "space_status_notification",
        }
    }
}
```

Because the enum is `#[serde(untagged)]`, the JSON payload flattens to
`{headline, body, space_title, cta_url}`, which is exactly what the SES
template will receive as substitution variables.

### `DetailType` — new variant

```rust
// app/ratel/src/common/types/event_bridge_envelope.rs
pub enum DetailType {
    ...
    SpaceStatusChangeEvent,
    #[serde(other)]
    Unknown,
}
```

Placed immediately before `Unknown`, per the pattern in
`conventions/implementing-event-bridge.md`.

### New SES template (infra, not in this repo)

A new SES template named `space_status_notification` must be provisioned
with substitution variables `{{headline}}`, `{{body}}`, `{{space_title}}`,
and `{{cta_url}}`. Template provisioning lives outside this repo and is a
**pre-merge blocker** for the Rust change.

In `#[cfg(test)]` and `feature = "bypass"` builds, `EmailTemplate::send_email`
short-circuits before touching SES, so tests do not require the template to
exist.

## Controller change — `update_space.rs`

File: `app/ratel/src/features/spaces/space_common/controllers/update_space.rs`.

Three branches produce status transitions: `Publish`, `Start`, `Finish`.
A local `status_transition: Option<(Option<SpaceStatus>, SpaceStatus)>`
captures the transition; after the existing `transact_write!` succeeds,
if `status_transition.is_some()`, we create and persist a
`SpaceStatusChangeEvent`.

```rust
let mut status_transition: Option<(Option<SpaceStatus>, SpaceStatus)> = None;

match req {
    UpdateSpaceRequest::Publish { publish, visibility } => {
        // ...existing logic unchanged...
        status_transition = Some((space.status.clone(), SpaceStatus::Open));
    }
    UpdateSpaceRequest::Start { start } => {
        // ...existing logic unchanged...
        status_transition = Some((Some(SpaceStatus::Open), SpaceStatus::Ongoing));
    }
    UpdateSpaceRequest::Finish { finished } => {
        // ...existing logic unchanged...
        status_transition = Some((Some(SpaceStatus::Ongoing), SpaceStatus::Finished));
    }
    // other branches unchanged
}

// existing transact_write! for space (+ post) stays as-is
// existing should_send_invitation block stays as-is

if let Some((old_status, new_status)) = status_transition {
    let event_id = uuid::Uuid::new_v7(uuid::Timestamp::now(uuid::NoContext)).to_string();
    let event = SpaceStatusChangeEvent {
        pk: Partition::SpaceStatusChangeEvent(event_id.clone()),
        sk: EntityType::SpaceStatusChangeEvent(event_id),
        created_at: now,
        space_pk: space_pk.clone(),
        old_status,
        new_status,
    };
    event.create(dynamo).await?;
}
```

Notes:

- The event write is **post-commit**, not inside the space transact-write.
  This matches the existing `should_send_invitation` side-effect pattern
  (also post-commit, also non-atomic). If the event write fails, the space
  update has already committed and the user sees the error; the transition
  is not lost, but no notification fires.
- The event is written for **all three** transitions, including
  `Publish → Open` for user-authored spaces where there will be no
  recipients. The **handler** decides there is nothing to do and returns
  early. This keeps the controller's branching trivial (no special case
  for "is this a team-authored space") at the cost of a single cheap
  DynamoDB write on user-authored publishes.
- The existing `should_send_invitation` block (invitation email to
  `SpaceInvitationMember` on Publish) is **untouched**. Team-member
  notification is additive, not a replacement.

## New service module — `space_status_change_notification`

File: `app/ratel/src/features/spaces/space_common/services/space_status_change_notification.rs`.

```rust
#[cfg(feature = "server")]
pub async fn handle_space_status_change(event: SpaceStatusChangeEvent) -> Result<()> {
    let dynamo = crate::common::config::get().dynamodb();

    // 1. Load space + post for content
    let space = SpaceCommon::get(dynamo, &event.space_pk, Some(&EntityType::SpaceCommon))
        .await?
        .ok_or(Error::SpaceNotFound)?;
    let post_pk = space.pk.clone().to_post_key()?;
    let post = Post::get(dynamo, &post_pk, Some(&EntityType::Post))
        .await?
        .ok_or_else(|| {
            crate::error!("handle_space_status_change: post not found for {}", post_pk);
            SpaceStatusChangeError::PostNotFound
        })?;

    // 2. Resolve audience user_pks
    let user_pks = match (&event.old_status, &event.new_status) {
        (_, SpaceStatus::Open) => {
            // Designing → Open: team members only, skip if author is a user
            match &space.user_pk {
                Partition::Team(_) => resolve_team_member_user_pks(dynamo, &space.user_pk).await?,
                _ => return Ok(()),
            }
        }
        (Some(SpaceStatus::Open), SpaceStatus::Ongoing)
        | (Some(SpaceStatus::Ongoing), SpaceStatus::Finished) => {
            resolve_space_participant_user_pks(dynamo, &event.space_pk).await?
        }
        _ => return Ok(()),
    };

    if user_pks.is_empty() {
        return Ok(());
    }

    // 3. Resolve emails via batch_get; dedupe
    let emails = resolve_emails(dynamo, user_pks).await?;
    if emails.is_empty() {
        return Ok(());
    }

    // 4. Copy selection
    let (headline, body) = status_change_copy(&event.new_status, &post.title);
    let cta_url = build_space_url(&event.space_pk);

    // 5. Fan out into Notification rows, 50 emails per row
    for chunk in emails.chunks(50) {
        let notification = Notification::new(NotificationData::SendSpaceStatusUpdate {
            emails: chunk.to_vec(),
            headline: headline.clone(),
            body: body.clone(),
            cta_url: cta_url.clone(),
            space_title: post.title.clone(),
        });
        if let Err(e) = notification.create(dynamo).await {
            crate::error!(
                "handle_space_status_change: failed to create notification row: {e}"
            );
            // continue — don't abort fan-out on a single failed chunk
        }
    }

    Ok(())
}
```

Helper functions live in the same module:

- **`resolve_team_member_user_pks(dynamo, team_pk)`** — paginates
  `UserTeamGroup::find_by_team_pk` (100 per page, hard cap `max_pages = 10`),
  collects unique `utg.pk` values, and also fetches `TeamOwner` so the team
  owner is included even when they have no `UserTeamGroup` row.
- **`resolve_space_participant_user_pks(dynamo, space_pk)`** — paginates
  `SpaceParticipant::find_by_space` with the same `max_pages = 10`,
  collects `sp.user_pk`.
- **`resolve_emails(dynamo, user_pks)`** — batches user_pks (DynamoDB
  `batch_get` caps at 100/batch), calls `User::batch_get`, drops users
  with empty email strings, and dedupes. Anonymous `SpaceParticipant`
  records resolve to their underlying `User` (anonymity is a display-name
  concern, not a separate user record), so anonymous participants still
  receive email.
- **`status_change_copy(new_status, space_title)`** — pure function
  returning `(headline, body)` tuples per transition.
- **`build_space_url(space_pk)`** — builds the public space URL from the
  app config base URL + space id.

### Error type

New enum in `app/ratel/src/features/spaces/space_common/types/error.rs`:

```rust
#[derive(Debug, Error, Serialize, Deserialize, Translate, Clone)]
pub enum SpaceStatusChangeError {
    #[error("post not found for space")]
    #[translate(en = "Space post not found", ko = "스페이스 게시글을 찾을 수 없습니다")]
    PostNotFound,
}
```

Registered on `common::Error` with `#[from]` + `#[translate(from)]` per
`conventions/error-handling.md`. In practice the service function never
surfaces an error to a user — it runs in Lambda and its errors are
logged — but the typed enum keeps the pattern consistent.

## Dispatch wiring

**`app/ratel/src/common/types/event_bridge_envelope.rs`** — add match arm
in `proc()`:

```rust
DetailType::SpaceStatusChangeEvent => {
    let event: SpaceStatusChangeEvent = DetailType::parse_detail(&self.detail)?;
    crate::features::spaces::space_common::services::handle_space_status_change(event).await
}
```

**`app/ratel/src/common/stream_handler.rs`** — local-dev parity, add to
the INSERT arm:

```rust
} else if sk.starts_with("SPACE_STATUS_CHANGE_EVENT#") {
    let event: SpaceStatusChangeEvent = deserialize(image)?;
    if let Err(e) = crate::features::spaces::space_common::services::handle_space_status_change(event).await {
        tracing::error!(error = %e, "stream: SpaceStatusChangeEvent failed");
    }
}
```

## Initial email copy (English)

| Transition | Headline | Body |
|---|---|---|
| Designing → Open  | `"{space_title} is now live"`        | `"Your team just published this space. You can invite participants and track activity from the dashboard."` |
| Open → Ongoing    | `"{space_title} is starting now"`    | `"The space you joined has started. Head in to participate."` |
| Ongoing → Finished | `"{space_title} has ended"`         | `"This space is now closed. Thank you for participating — you can still view results on the dashboard."` |

These are placeholders suitable for shipping; the copy owner can iterate.

## CDK change — pipe + rule

File: `cdk/lib/dynamo-stream-event.ts`. Added next to the existing
`NotificationPipe` / `PopularSpacePipe`.

```typescript
// ── Pipe: SpaceStatusChangeEvent ───────────────────────────────────
new pipes.CfnPipe(this, "SpaceStatusChangeEventPipe", {
  name: `ratel-${stage}-space-status-change-event-pipe`,
  roleArn: pipeRole.roleArn,
  source: mainTableStreamArn,
  sourceParameters: {
    dynamoDbStreamParameters: {
      startingPosition: "LATEST",
      batchSize: 10,
    },
    filterCriteria: {
      filters: [
        {
          pattern: JSON.stringify({
            eventName: ["INSERT"],
            dynamodb: {
              NewImage: {
                sk: { S: [{ prefix: "SPACE_STATUS_CHANGE_EVENT#" }] },
              },
            },
          }),
        },
      ],
    },
  },
  target: eventBus.eventBusArn,
  targetParameters: {
    eventBridgeEventBusParameters: {
      source: "ratel.dynamodb.stream",
      detailType: "SpaceStatusChangeEvent",
    },
    inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
  },
});

// ── Rule: route to app-shell lambda ────────────────────────────────
new events.Rule(this, "SpaceStatusChangeEventRule", {
  eventBus,
  description: "Route space status change events to app-shell for fan-out",
  eventPattern: {
    source: ["ratel.dynamodb.stream"],
    detailType: ["SpaceStatusChangeEvent"],
  },
  targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
});
```

The filter matches on `sk` prefix only (not `pk`), which mirrors the
existing `NotificationPipe` style and is appropriate because our pk varies
by uuid. `batchSize: 10` is fine because each event is independent (one
per transition, each space transitions at most a handful of times total)
and strict ordering is not required.

No other CDK changes — the existing `NotificationPipe` already handles
the second leg (the per-chunk `Notification` rows that the handler writes).

## Testing

### 1. Controller integration tests

File: new `app/ratel/src/tests/space_status_change_tests.rs`, registered
in `app/ratel/src/tests/mod.rs`. Follows `conventions/server-function-tests.md`.

- `test_publish_creates_status_change_event` — draft space →
  `Publish { publish: true, visibility: Public }`, then assert a
  `SpaceStatusChangeEvent` row exists with `old_status = None`,
  `new_status = Open`, and the correct `space_pk`.
- `test_start_creates_status_change_event` — space in `Open`,
  `Start { start: true }`, expect `old_status = Some(Open)`,
  `new_status = Ongoing`.
- `test_finish_creates_status_change_event` — space in `Ongoing`,
  `Finish { finished: true }`, expect `old_status = Some(Ongoing)`,
  `new_status = Finished`.
- `test_non_status_update_creates_no_event` — e.g.
  `UpdateSpaceRequest::Title { ... }`, then assert no event row exists.
- Existing role-permission tests stay as-is; new code paths must not
  regress them.

### 2. Service / handler unit tests

Same test file, exercising `handle_space_status_change` directly (plain
async Rust, no HTTP).

- `test_handle_publish_to_open_notifies_team_members` — create a team,
  add two members via `UserTeamGroup`, create a team-owned space, run
  `handle_space_status_change(event)` with `new_status = Open`, assert
  the created `Notification` rows cover both team-member emails.
- `test_handle_publish_to_open_skips_user_authored` — user-authored
  space, `new_status = Open`, assert zero `Notification` rows created.
- `test_handle_open_to_ongoing_notifies_participants` — two participants
  joined while Open, run event, assert notifications with both emails.
- `test_handle_ongoing_to_finished_notifies_participants` — same as
  above but for the Finished transition.
- `test_handle_no_recipients_is_noop` — space with zero participants
  returns `Ok(())` and creates zero `Notification` rows.
- `test_handle_unknown_transition_is_noop` — e.g.
  `(Some(Finished), Open)` returns `Ok(())` and creates no rows.
- `test_handle_batches_emails_into_chunks_of_50` — seed 120 participants,
  assert exactly 3 `Notification` rows created with chunk sizes
  `[50, 50, 20]`.
- `test_handle_dedupes_duplicate_emails` — two participants sharing the
  same underlying email produce a single recipient entry.

`EmailTemplate::send_email` is already gated by
`#[cfg(any(test, feature = "bypass"))]` to skip real SES calls, so tests
run cleanly with the existing `bypass` feature.

### 3. E2E Playwright test

File: `playwright/tests/web/space-status-notifications.spec.js`.

SES is mocked in `bypass` mode, so we cannot assert against a real
inbox in CI. The Playwright scenario verifies only UI-visible behavior:
the creator clicks Start / Finish, and the space transitions to the
expected state visible in the UI. The Rust integration and service
tests above are the authoritative check that notifications are created
correctly; Playwright exists to confirm the end-to-end HTTP path still
works. No new test-only DynamoDB-inspection endpoints are added.

## Rollout / ops

1. **SES template** — provision `space_status_notification` in SES before
   merging. Variables: `{{headline}}`, `{{body}}`, `{{space_title}}`,
   `{{cta_url}}`. Template provisioning is outside this repo (infra task)
   and is a **pre-merge blocker**.
2. **Deploy order** — merge the CDK change, deploy to dev, verify the
   pipe and rule appear in the AWS console, then merge the Rust change.
   Reverse order is also safe (events pile up undelivered until the pipe
   exists), but forward order is cleaner.
3. **Backfill** — none. Existing spaces do not retroactively generate
   events. Only future transitions trigger notifications.
4. **Kill switch** — disable or delete the SES template. The
   `Notification` rows still get written, but the SES call errors out and
   the row stays in `Requested`. No code change required.
5. **Observability** — the service function logs at `info!` on entry
   (transition, recipient count) and `error!` on failure paths. No new
   metrics or alarms in this pass; add a CloudWatch alarm on
   `Notification` rows stuck in `Requested` for >5 minutes if volume
   turns out to be high.

## Known limits

- **Hard cap of 1000 recipients per transition** (`max_pages = 10`,
  100 per page). Spaces or teams beyond this size log a `warn!` and the
  overflow recipients are silently dropped. Acceptable at current product
  scale; revisit when real spaces hit the cap.
- **No retry on per-chunk `Notification::create` failures.** The loop
  logs and continues so one failed chunk does not lose the rest. If the
  whole `handle_space_status_change` call returns `Err` or panics, the
  Lambda invocation retries per the EventBridge default policy.
- **No dedupe across transitions.** Rapid Publish → Start → Finish within
  seconds produces three separate email batches per user. This is correct
  behavior, matching user intent.

## Implementation checklist

- [ ] `Partition::SpaceStatusChangeEvent` variant added
- [ ] `EntityType::SpaceStatusChangeEvent` variant added
- [ ] `SpaceStatusChangeEvent` entity created and exported from
      `common::models::space`
- [ ] `NotificationData::SendSpaceStatusUpdate` variant + `send()` arm
- [ ] `EmailOperation::SpaceStatusNotification` variant +
      `template_name()` arm
- [ ] `DetailType::SpaceStatusChangeEvent` variant + `proc()` arm
- [ ] `update_space` controller writes event post-commit on all three
      transitions
- [ ] `handle_space_status_change` service function + helpers
- [ ] `SpaceStatusChangeError` error enum + registration on
      `common::Error`
- [ ] `stream_handler.rs` branch for local-dev parity
- [ ] CDK pipe + rule in `cdk/lib/dynamo-stream-event.ts`
- [ ] Controller integration tests (4+ cases)
- [ ] Service unit tests (8+ cases)
- [ ] E2E Playwright scenario (entity-chain or UI-fallback)
- [ ] `cargo check --features "server,lambda"` passes
- [ ] `cd cdk && npx tsc --noEmit` passes
- [ ] `DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web` passes
- [ ] SES `space_status_notification` template provisioned in dev +
      prod SES (pre-merge blocker)
