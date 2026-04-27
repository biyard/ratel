# Notify New Ongoing Action — System Design

**Branch**: `feature/notify-new-action`
**Author / Date**: Claude · 2026-04-27
**Related**: [2026-04-09-space-status-notifications-design.md](./2026-04-09-space-status-notifications-design.md), [2026-04-20-notification-inbox-design.md](./2026-04-20-notification-inbox-design.md)

## Summary

When a `SpaceAction` transitions `Designing → Ongoing` inside a parent `Space` whose status is `Ongoing`, every `SpaceParticipant` of that space receives an in-app inbox notification and a templated email. Reuses the existing `Notification → SES` and `UserInboxNotification → bell` pipelines — no new ledger entity is added; the `SpaceAction` row's own DynamoDB Stream MODIFY event is the trigger, gated by an EventBridge Pipe filter on the `OldImage`/`NewImage` status pair.

## Trigger & Event

**No new entity.** The `SpaceAction` row's MODIFY stream event is the trigger.

**Filter condition** (encoded in CDK Pipe + mirrored in local-dev stream handler):
- `eventName == "MODIFY"`
- `NewImage.sk == "SPACE_ACTION"`
- `NewImage.status == "ONGOING"`
- `OldImage.status == "DESIGNING"`

These are the uppercase strings that `DynamoEnum` already serializes `SpaceActionStatus::Ongoing` / `Designing` to in DynamoDB images.

### Lambda path

CDK additions (`cdk/lib/dynamo-stream-event.ts`):

```ts
new pipes.CfnPipe(this, "SpaceActionStatusChangePipe", {
  name: `ratel-${stage}-space-action-status-change-pipe`,
  roleArn: pipeRole.roleArn,
  source: mainTableStreamArn,
  sourceParameters: {
    dynamoDbStreamParameters: { startingPosition: "LATEST", batchSize: 10 },
    filterCriteria: {
      filters: [{
        pattern: JSON.stringify({
          eventName: ["MODIFY"],
          dynamodb: {
            NewImage: {
              sk: { S: ["SPACE_ACTION"] },
              status: { S: ["ONGOING"] },
            },
            OldImage: {
              status: { S: ["DESIGNING"] },
            },
          },
        }),
      }],
    },
  },
  target: eventBus.eventBusArn,
  targetParameters: {
    eventBridgeEventBusParameters: {
      source: "ratel.dynamodb.stream",
      detailType: "SpaceActionStatusChange",
    },
    inputTemplate: '{"newImage": <$.dynamodb.NewImage>}',
  },
});

new events.Rule(this, "SpaceActionStatusChangeRule", {
  eventBus,
  description: "Route action Designing→Ongoing events to app-shell for notification fan-out",
  eventPattern: {
    source: ["ratel.dynamodb.stream"],
    detailType: ["SpaceActionStatusChange"],
  },
  targets: [new eventsTargets.LambdaFunction(props.lambdaFunction)],
});
```

Add `DetailType::SpaceActionStatusChange` (before `Unknown`) and a match arm in `EventBridgeEnvelope::proc()` that deserializes a `SpaceAction` and dispatches to `notify_action_ongoing`.

### Local-dev path

`app/ratel/src/common/stream_handler.rs` MODIFY arm currently only forwards `new_image`. Extend the local-dev poller and `handle_stream_record`'s MODIFY signature to also pass `old_image`, mirroring REMOVE.

New branch in MODIFY:

```rust
} else if sk == "SPACE_ACTION" {
    let new_status = get_string_field(image, "status").unwrap_or_default();
    let old_status = old_image
        .and_then(|i| get_string_field(i, "status"))
        .unwrap_or_default();
    if old_status == "DESIGNING" && new_status == "ONGOING" {
        let action: SpaceAction = deserialize(image)?;
        if let Err(e) =
            crate::features::spaces::pages::actions::services::notify_action_ongoing(action).await
        {
            tracing::error!(error = %e, "stream: SpaceActionStatusChange failed");
        }
    }
}
```

## Inbox payload

New variants in `app/ratel/src/common/types/inbox_kind.rs`:

```rust
pub enum InboxKind {
    // ... existing ...
    SpaceActionOngoing,
}

impl InboxKind {
    pub fn as_prefix(&self) -> &'static str {
        match self {
            // ...
            InboxKind::SpaceActionOngoing => "SPACE_ACT_ON",
        }
    }
}

pub enum InboxPayload {
    // ... existing ...
    SpaceActionOngoing {
        space_id: SpacePartition,
        space_title: String,
        action_id: String,
        action_type: SpaceActionType,
        action_title: String,
        cta_url: String,
    },
}
```

Update `InboxPayload::url()`, `InboxPayload::kind()`, and `Default` if relevant — the compiler enforces exhaustive matches on every helper.

## Email payload

New variant in `app/ratel/src/common/types/notification_data.rs`:

```rust
SpaceActionOngoing {
    emails: Vec<String>,
    space_title: String,
    action_title: String,
    action_type_label: String, // English label, translated server-side at fan-out
    cta_url: String,
}
```

`NotificationData::send` arm delegates to a new `EmailOperation::SpaceActionOngoingNotification` and to a new template at `app/ratel/src/features/admin/templates/space_action_ongoing_notification.rs`. Template structure mirrors `space_status_notification.rs`: subject, headline (`New activity in {space_title}`), body (`{action_title} ({action_type_label}) is now ongoing — head in to participate.`), CTA button → `cta_url`.

## Service: `notify_action_ongoing`

**File**: `app/ratel/src/features/spaces/pages/actions/services/notify_action_ongoing.rs`

```rust
pub async fn notify_action_ongoing(action: SpaceAction) -> Result<()> {
    let cfg = crate::common::CommonConfig::default();
    let cli = cfg.dynamodb();

    let space_id: SpacePartition = action.pk.0.clone();
    let space_pk: Partition = space_id.clone().into();
    let action_id = action.pk.1.clone();

    // Guard: parent space must be Ongoing.
    let space = match SpaceCommon::get(cli, &space_pk, Some(&EntityType::SpaceCommon)).await? {
        Some(s) if s.status == SpaceStatus::Ongoing => s,
        _ => return Ok(()),
    };

    let user_pks = resolve_space_participant_user_pks(cli, &space_pk).await?;
    if user_pks.is_empty() {
        return Ok(());
    }

    let post_pk = space_pk.clone().to_post_key()?;
    let post = Post::get(cli, &post_pk, Some(&EntityType::Post))
        .await?
        .ok_or(SpaceActionError::PostNotFound)?;
    let space_title = post.title.clone();

    let cta_url = action.get_cta_url();

    // Inbox fan-out (idempotent via InboxDedupMarker).
    for user_pk in &user_pks {
        let payload = InboxPayload::SpaceActionOngoing {
            space_id: space_id.clone(),
            space_title: space_title.clone(),
            action_id: action_id.clone(),
            action_type: action.space_action_type.clone(),
            action_title: action.title.clone(),
            cta_url: cta_url.clone(),
        };
        let dedup_source = format!("{}:{}", space_pk, action_id);
        if let Err(e) =
            crate::common::utils::inbox::create_inbox_row_once(user_pk.clone(), payload, &dedup_source).await
        {
            crate::error!("action-ongoing inbox row failed: {e}");
        }
    }

    // Email fan-out (50 per Notification row).
    fan_out_emails(cli, user_pks, &action, &space_title, &cta_url).await?;
    Ok(())
}
```

### Reused infrastructure

- `resolve_space_participant_user_pks` and `resolve_emails` are currently private in `space_status_change_notification.rs`. Promote both to `pub(crate)` in a shared module (`features/spaces/space_common/services/audience.rs`) so this notifier and the existing space-status notifier share one implementation. Both notifiers benefit from any future fix.
- `create_inbox_row_once` provides 7-day per-recipient dedup keyed on `(recipient, kind, source_id)` where `source_id = "{space_pk}:{action_id}"`. Defends against retries and accidental double-fires.
- `Notification::create` writes a row that the existing stream → SES path picks up (`Notification::process` is already wired).

### CTA URL on `SpaceAction`

Add a method on `SpaceAction` so any caller with the model can build the deep link:

```rust
impl SpaceAction {
    pub fn get_cta_url(&self) -> String {
        let space_id = &self.pk.0;
        let action_id = &self.pk.1;
        let route = match self.space_action_type {
            SpaceActionType::Poll => Route::PollActionPage {
                space_id: space_id.clone(),
                poll_id: action_id.clone().into(),
            },
            SpaceActionType::TopicDiscussion => Route::SpaceIndexPage {
                space_id: space_id.clone(),
            },
            SpaceActionType::Follow => Route::FollowActionPage {
                space_id: space_id.clone(),
                follow_id: action_id.clone().into(),
            },
            SpaceActionType::Quiz => Route::QuizActionPage {
                space_id: space_id.clone(),
                quiz_id: action_id.clone().into(),
            },
            SpaceActionType::Meet => Route::MeetActionPage {
                space_id: space_id.clone(),
                meet_id: action_id.clone().into(),
            },
        };
        format!("https://ratel.foundation{}", route)
    }
}
```

`SpaceActionSummary::get_url` is refactored to delegate to the same match (returning `Route` for in-app `Link` use, vs. `get_cta_url` returning the absolute string for emails/inbox). Branch logic exists in exactly one place.

## Frontend

`features/notifications/components/notification_panel/notification_item/component.rs` adds a new arm for `InboxPayload::SpaceActionOngoing`:

- Top line: action-type icon (`crate::common::icons` — poll / quiz / follow / meet / discussion) + `t.action_ongoing_title` interpolated with `action_title`.
- Subtitle: `t.action_ongoing_subtitle` interpolated with `space_title`.
- Click → existing `handle_item_click` action navigates to `cta_url`. No new wiring.

i18n additions in `features/notifications/i18n.rs`:

```rust
action_ongoing_title:    { en: "New action ongoing: {action_title}",      ko: "새 활동 시작: {action_title}" }
action_ongoing_subtitle: { en: "in {space_title}",                         ko: "{space_title}에서" }
```

Bell badge count is unchanged — `get_unread_count` counts unread rows regardless of `kind`.

## Test plan

### Server function tests

`app/ratel/src/tests/space_action_notification_tests.rs`:

1. **Happy path**: create user + space + action in `Designing`, promote space → `Ongoing`, second user joins as participant, call `update_space_action` with `Status { status: Ongoing }`. Assert participant has a `UserInboxNotification` row with `kind == SpaceActionOngoing` and matching `action_id` in payload.
2. **Dedup**: re-trigger the transition (e.g., promote a different field after the first transition that re-emits the stream event in tests) — assert no duplicate inbox row.
3. **Space not Ongoing**: same flow but parent space is `Open`. Assert no inbox row created.
4. **No participants**: Ongoing space with zero participants. Assert handler returns Ok and no rows are written.

### Stream-handler test

Extend the existing `stream_handler.rs` test cases to cover the new `SPACE_ACTION` MODIFY branch — assert it dispatches only when `OldImage.status == "DESIGNING"` and `NewImage.status == "ONGOING"`.

### Playwright (deferred)

Extend the existing notification e2e to promote an action and assert the inbox card renders with the action title and a click-through to the action page. Out of scope for v1 if it gates the merge — log as a follow-up.

## Open questions / risks

1. **Per-recipient locale.** Email templates are English-only today. Per-recipient localization would split batches by locale; out of scope for v1 (matches the `SpaceStatusUpdate` precedent).
2. **Bulk action promotion.** Promoting N actions back-to-back gives each participant N inbox rows + N emails. Acceptable for v1; if it becomes spammy we add a per-recipient debounce keyed on `(recipient, space_pk)`.
3. **Local-dev `old_image` plumbing.** The local-dev stream poller feeds only `new_image` to MODIFY callers today. The poller and `handle_stream_record`'s MODIFY signature need extending to pass `old_image` for the `SPACE_ACTION` branch. Lambda already gets both images.
4. **Action-type icon inventory.** Confirm icons exist for all 5 action types; otherwise fall back to a generic activity icon. Not a blocker.
