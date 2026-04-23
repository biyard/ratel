# Sub-team Governance — System Design (Phase 1 MVP)

**Roadmap**: [roadmap/sub-team.md](../../../roadmap/sub-team.md)
**Design**: [/designs/sub-team/](../../../app/ratel/assets/design/sub-team/)
**Author / Date**: hackartists · 2026-04-23
**Status**: Draft — awaiting user review

## Summary

Give any Ratel team a first-class parent ↔ child relationship so it can govern recognized sub-teams through **bylaws publication, custom application forms, approval queues, broadcast announcements, and activity observation** — without ever touching the sub-team's own content. Phase 1 ships the full end-to-end flow behind a per-team `is_parent_eligible` flag; rich text, document attachments/versioning, and broadcast-subset targeting are deferred to Phase 2.

## Scope

**In Phase 1 (MVP, satisfies AC-1 … AC-20):**

- Parent ↔ child relationship on `Team` (single parent, no nesting, cycle-proof)
- `SubTeamDocument` as a first-class entity (title + plain-text/markdown body + required flag + order)
- Per-document explicit agreement captured at application submit (`SubTeamDocAgreement`)
- Custom application form: field definition + per-application form snapshot
- Pending sub-team → apply → parent approve/reject/return → lifecycle notifications
- Broadcast announcement (default-to-all recognized sub-teams) via DynamoDB-Stream-driven fan-out Lambda
- Activity dashboard (posts / spaces / active members, weekly + monthly windows) + per-member drill-down
- Parent-side deregister, child-side leave-parent, parent-delete cascade

**Out of Phase 1 (→ Phase 2):**

- File attachments on documents
- Document versioning + diff
- Broadcast subset targeting (FR-5 #29)
- Daily time window on activity dashboard (only weekly + monthly in AC-13)
- Image / embed / table toolbar buttons in the shared rich editor — Phase 1 keeps authoring markdown-only (heading / bold / italic / list / quote / link). Body stored as markdown, rendered via the same read-only markdown renderer already used for bylaws.
- Activity dashboard pre-aggregation (Phase 1 uses query-time counts; cap 50 sub-teams per parent)

## Data model

All new entities live under the existing single-table with `Partition` / `EntityType` prefixes. GSI usage follows `conventions/dynamo-prefix-convention.md`.

### Team — extend with parent-child fields

```rust
pub struct Team {
    pub pk: Partition,                       // Partition::Team(team_id)
    pub sk: EntityType,                      // EntityType::Team
    // ...existing fields...

    #[serde(default)]
    pub is_parent_eligible: bool,            // admin toggle (false by default)
    #[serde(default)]
    pub min_sub_team_members: i32,           // required count before apply enabled (default 3)

    // Parent-child scalars — invariants:
    //   recognized sub-team ⇔ parent_team_id.is_some()
    //   pending sub-team    ⇔ pending_parent_team_id.is_some() && parent_team_id.is_none()
    //   standalone team     ⇔ both None
    #[serde(default)]
    pub parent_team_id: Option<String>,
    #[serde(default)]
    pub pending_parent_team_id: Option<String>,
}
```

No new GSI on `Team`. Listing a parent's recognized sub-teams uses the `SubTeamLink` query below.

### SubTeamLink — "recognized child" join record

Lives under the parent's pk so listing children is a bounded sk-prefix scan.

```rust
#[derive(DynamoEntity)]
pub struct SubTeamLink {
    pub pk: Partition,             // Partition::Team(parent_team_id)
    pub sk: EntityType,            // EntityType::SubTeamLink("{child_team_id}")
                                   //   — `#[dynamo(prefix = "STLINK")]`
    pub child_team_id: String,
    pub approved_at: i64,
    pub approved_by: String,       // parent admin user_pk
    pub source_application_id: String,
}
```

A `SubTeamLink` row is created on approval and deleted on deregister / leave-parent / parent-delete cascade. The invariant `Team.parent_team_id == parent_pk ⇔ SubTeamLink exists` holds because both writes happen in a single transact-write-items batch.

### SubTeamDocument

```rust
#[derive(DynamoEntity)]
pub struct SubTeamDocument {
    pub pk: Partition,             // Partition::Team(team_id)
    pub sk: EntityType,            // EntityType::SubTeamDocument(doc_id)
                                   //   — `#[dynamo(prefix = "STDOC")]`
    pub created_at: i64,
    pub updated_at: i64,
    pub title: String,
    pub body: String,              // plain markdown; ≤ 64 KB enforced
    pub required: bool,            // if true, applicants must agree at submit
    pub order: i32,                // display sort (lower = earlier)
    pub body_hash: String,         // sha256 of body at last update — used by SubTeamDocAgreement
}
```

No GSI; `find_by_pk` with sk-prefix returns the ordered list.

### SubTeamDocAgreement — audit trail

Captured at application submit; immutable.

```rust
#[derive(DynamoEntity)]
pub struct SubTeamDocAgreement {
    pub pk: Partition,             // Partition::SubTeamApplication(application_id)
    pub sk: EntityType,            // EntityType::SubTeamDocAgreement(doc_id)
                                   //   — `#[dynamo(prefix = "STDAG")]`
    pub doc_id: String,
    pub doc_title_snapshot: String,
    pub body_hash_snapshot: String,
    pub agreed_at: i64,
    pub agreed_by: String,         // submitter user_pk
}
```

### SubTeamFormField — parent's application form schema

```rust
#[derive(DynamoEntity)]
pub struct SubTeamFormField {
    pub pk: Partition,             // Partition::Team(parent_team_id)
    pub sk: EntityType,            // EntityType::SubTeamFormField(field_id)
                                   //   — `#[dynamo(prefix = "STFLD")]`
    pub created_at: i64,
    pub updated_at: i64,
    pub label: String,
    pub field_type: SubTeamFormFieldType,
    pub required: bool,
    pub order: i32,
    pub options: Vec<String>,      // for SingleSelect / MultiSelect
}

pub enum SubTeamFormFieldType {
    ShortText, LongText, Number, Date, SingleSelect, MultiSelect, Url,
}
```

### SubTeamApplication

```rust
#[derive(DynamoEntity)]
pub struct SubTeamApplication {
    pub pk: Partition,             // Partition::Team(parent_team_id)  — parent's queue
    pub sk: EntityType,            // EntityType::SubTeamApplication(application_id)
                                   //   — `#[dynamo(prefix = "STAPP")]`
    pub created_at: i64,
    pub updated_at: i64,
    pub submitted_at: Option<i64>,
    pub decided_at: Option<i64>,

    pub application_id: String,    // uuid (also the last segment of sk)
    pub sub_team_id: String,       // the applying team
    pub submitter_user_id: String,

    pub status: SubTeamApplicationStatus,
    pub decision_reason: Option<String>,  // used on Reject + Return

    // Snapshot — the form definition as of submission. Immune to later edits.
    pub form_snapshot: Vec<SubTeamFormFieldSnapshot>,
    pub form_values: std::collections::HashMap<String, serde_json::Value>,  // { field_id → value }

    // GSI1: list a sub-team's own applications chronologically
    #[dynamo(prefix = "STAPP_SUB", index = "gsi1", pk)]
    #[dynamo(index = "gsi1", sk)]
    pub sub_team_id_idx: String,   // = sub_team_id
}

pub enum SubTeamApplicationStatus {
    Draft,         // being filled in, not yet submitted
    Pending,       // submitted, awaiting parent decision
    Approved,
    Rejected,
    Returned,      // parent requested revision; child can edit + resubmit
    Cancelled,     // child cancelled
}
```

GSI1 (pk=`STAPP_SUB#<sub_team_id>`) lets the sub-team page resolve "my current application." Parent-side queue is the main-table sk-prefix scan.

### SubTeamAnnouncement — parent's broadcast record

The canonical announcement owned by the parent. Fan-out Posts in each sub-team's feed are derived records, not authoritative.

```rust
#[derive(DynamoEntity)]
pub struct SubTeamAnnouncement {
    pub pk: Partition,             // Partition::Team(parent_team_id)
    pub sk: EntityType,            // EntityType::SubTeamAnnouncement(announcement_id)
                                   //   — `#[dynamo(prefix = "STANN")]`
    pub created_at: i64,
    pub updated_at: i64,
    pub published_at: Option<i64>,

    pub announcement_id: String,
    pub title: String,
    pub body: String,
    pub author_user_id: String,
    pub status: SubTeamAnnouncementStatus,  // Draft | Published | Deleted
    pub target_type: BroadcastTarget,       // Phase 1: AllRecognizedSubTeams only
    pub fan_out_count: i32,                 // populated by fan-out handler
}
```

### Extensions to existing `Post`

A fan-out Post in a sub-team's feed gets these optional fields (all default None/false, no breaking change):

```rust
pub announcement_id: Option<String>,
pub announcement_parent_team_id: Option<String>,
pub pinned_as_announcement: bool,
```

The feed query continues to return all posts. The Dioxus feed component renders `pinned_as_announcement == true` posts first with a distinct banner.

### Notifications (reuse existing `notifications/` feature)

New variants on the `InboxNotificationPayload` enum (or equivalent):

- `SubTeamApplicationSubmitted { parent_team_id, application_id }` → to parent admins
- `SubTeamApplicationApproved { sub_team_id, parent_team_id }` → to sub-team admin
- `SubTeamApplicationRejected { sub_team_id, parent_team_id, reason }` → to sub-team admin
- `SubTeamApplicationReturned { sub_team_id, parent_team_id, comment }` → to sub-team admin
- `SubTeamAnnouncementReceived { parent_team_id, announcement_id, post_id }` → to each sub-team member
- `SubTeamAnnouncementComment { parent_team_id, post_id, commenter_user_id }` → to parent author
- `SubTeamDeregistered { former_parent_team_id, reason }` → to sub-team admin
- `SubTeamLeftParent { former_sub_team_id, reason }` → to parent admins
- `SubTeamParentDeleted { former_parent_team_id }` → to sub-team admin

All new variants piggyback on the existing inbox render / unread-count infrastructure — no new notification primitive.

## API surface

All handlers use SubPartition types for path params and DTOs (`TeamPartition`, `SubTeamApplicationEntityType`, etc.) per `conventions/server-functions.md`. Unit errors from a feature-specific `SubTeamError` enum per `conventions/error-handling.md`.

API paths follow the resource hierarchy: every parent-admin endpoint nests under `/api/teams/{team_id}/sub-teams/...` so the URL itself reflects the parent→sub-team governance relationship. Child-side endpoints (where `{team_id}` is the applying team, not the parent) nest under `/api/teams/{team_id}/parent/...` — symmetric hierarchy, but viewed from the child's perspective.

Path-routing note: `{sub_team_id}` path segments are UUIDs and thus can never collide with literal sub-resources (`settings`, `docs`, `form-fields`, `applications`, `announcements`, `apply-context`). Axum routing also prefers literal matches over path parameters, so the ordering is robust even without the UUID-shape guarantee.

### Parent-admin endpoints (`role: TeamRole::Owner | Admin` on the parent team)

All under `/api/teams/{team_id}/sub-teams/...` where `{team_id}` is the parent team.

| Method | Path | Purpose |
|---|---|---|
| GET | `/api/teams/{team_id}/sub-teams?bookmark` | list recognized sub-teams (via `SubTeamLink`) |
| PATCH | `/api/teams/{team_id}/sub-teams/settings` | toggle `is_parent_eligible`, set `min_sub_team_members` |
| GET | `/api/teams/{team_id}/sub-teams/docs` | list docs (admin view — includes order/updated_at) |
| POST | `/api/teams/{team_id}/sub-teams/docs` | create |
| PATCH | `/api/teams/{team_id}/sub-teams/docs/{doc_id}` | update — recomputes `body_hash` |
| DELETE | `/api/teams/{team_id}/sub-teams/docs/{doc_id}` | delete |
| POST | `/api/teams/{team_id}/sub-teams/docs/reorder` | body: `{ doc_ids: [..] }` |
| GET | `/api/teams/{team_id}/sub-teams/form-fields` | list form schema |
| POST | `/api/teams/{team_id}/sub-teams/form-fields` | create field |
| PATCH | `/api/teams/{team_id}/sub-teams/form-fields/{field_id}` | update |
| DELETE | `/api/teams/{team_id}/sub-teams/form-fields/{field_id}` | delete |
| POST | `/api/teams/{team_id}/sub-teams/form-fields/reorder` | body: `{ field_ids: [..] }` |
| GET | `/api/teams/{team_id}/sub-teams/applications?status=Pending&bookmark` | queue — returns `ListResponse<SubTeamApplicationResponse>` |
| GET | `/api/teams/{team_id}/sub-teams/applications/{application_id}` | detail (full `form_snapshot` + `form_values`) |
| POST | `/api/teams/{team_id}/sub-teams/applications/{application_id}/approve` | approve |
| POST | `/api/teams/{team_id}/sub-teams/applications/{application_id}/reject` | body: `{ reason }` |
| POST | `/api/teams/{team_id}/sub-teams/applications/{application_id}/return` | body: `{ comment }` |
| GET | `/api/teams/{team_id}/sub-teams/announcements?status=…&bookmark` | list drafts + published |
| POST | `/api/teams/{team_id}/sub-teams/announcements` | create draft |
| PATCH | `/api/teams/{team_id}/sub-teams/announcements/{announcement_id}` | update draft |
| POST | `/api/teams/{team_id}/sub-teams/announcements/{announcement_id}/publish` | flip Draft → Published (triggers fan-out via stream) |
| DELETE | `/api/teams/{team_id}/sub-teams/announcements/{announcement_id}` | soft-delete |
| GET | `/api/teams/{team_id}/sub-teams/{sub_team_id}` | sub-team overview (for the detail page) |
| GET | `/api/teams/{team_id}/sub-teams/{sub_team_id}/activity?window=Weekly\|Monthly` | dashboard counts |
| GET | `/api/teams/{team_id}/sub-teams/{sub_team_id}/member-activity?window=Weekly\|Monthly&bookmark` | per-member drill-down |
| POST | `/api/teams/{team_id}/sub-teams/{sub_team_id}/deregister` | body: `{ reason }` |

### Sub-team-side endpoints (`role: TeamRole::Owner | Admin` on the applying team)

All under `/api/teams/{team_id}/parent/...` where `{team_id}` is the child / applying team.

| Method | Path | Purpose |
|---|---|---|
| GET | `/api/teams/{team_id}/parent` | current parent relationship summary (`{ status, parent_team_id?, pending_parent_team_id?, latest_application_id? }`) |
| GET | `/api/teams/{team_id}/parent/applications?bookmark` | this team's application history (via GSI1) |
| POST | `/api/teams/{team_id}/parent/applications` | submit — body: `{ parent_team_id, form_values, doc_agreements: [{ doc_id, body_hash }] }` |
| GET | `/api/teams/{team_id}/parent/applications/{application_id}` | detail for the child's own record |
| PATCH | `/api/teams/{team_id}/parent/applications/{application_id}` | edit while in Returned status |
| POST | `/api/teams/{team_id}/parent/applications/{application_id}/cancel` | cancel while Pending or Returned |
| POST | `/api/teams/{team_id}/parent/leave` | body: `{ reason?: string }` — only valid when currently recognized |

### Public / unauthenticated

Expose the parent team's program in a single endpoint so the apply-page can render the entire contract in one round-trip.

| Method | Path | Purpose |
|---|---|---|
| GET | `/api/teams/{team_id}/sub-teams/apply-context` | `{ is_parent_eligible, min_sub_team_members, recognized_count, pending_count, form_fields: [..], required_docs: [{ id, title, body, body_hash, order }] }` — everything the apply UI needs before the applicant even authenticates |

### Submit-path validation

On `POST /applications`:

1. Submitting team exists, caller is Owner/Admin.
2. Parent team exists AND `is_parent_eligible == true`.
3. Submitting team has no other Pending/Returned application (one-at-a-time invariant).
4. Member count (`UserTeam.count_members_for_team`) ≥ parent's `min_sub_team_members`.
5. Every required field from parent's current form is present and non-empty in `form_values`.
6. Every required document from parent's current docs has a matching `doc_agreements` entry with current `body_hash`. Stale hash → reject with `DocAgreementStale`.
7. Transact-write: create `SubTeamApplication` (status=Pending) + create one `SubTeamDocAgreement` per agreed doc + set `pending_parent_team_id` on the submitting Team.

All six checks return typed `SubTeamError` variants; UI surfaces them verbatim via `translate!`.

## Event flow

Two DynamoDB-Stream-driven chains follow `conventions/implementing-event-bridge.md`. Stream handler (`common/stream_handler.rs`) mirrors both for local-dev parity.

### 1. Announcement publish → fan-out

```
SubTeamAnnouncement MODIFY (status: Draft → Published)
  → DynamoDB Stream (NewImage)
  → CDK Pipe filter: sk prefix "STANN#" AND status == "Published"
  → EventBridge source=ratel.dynamodb.stream, detailType=SubTeamAnnouncementPublished
  → Rule → app-shell Lambda
  → EventBridgeEnvelope::proc() matches DetailType::SubTeamAnnouncementPublished
  → services::announcement_fanout::handle_published(announcement)
      1. Query SubTeamLink rows under pk=parent_team_id (already ordered by sk)
      2. For each child_team_id, in a single transact-write batch:
         - Create a Post with kind=Announcement, pinned_as_announcement=true,
           announcement_id, announcement_parent_team_id set
         - Flip prior announcement post (if any) pinned_as_announcement=false
         - Enqueue InboxNotification for every member of that sub-team
      3. Update SubTeamAnnouncement.fan_out_count += n
```

Fan-out is sequential per-sub-team but batched per-member within a sub-team. Phase 1 caps at 50 sub-teams × 200 members = 10,000 notifications per announcement (well within Lambda limits at 15-minute timeout).

### 2. Application decision → team status + notification

```
SubTeamApplication MODIFY (status: Pending → Approved | Rejected | Returned)
  → Stream → Pipe filter: sk prefix "STAPP#" AND status transition
  → EventBridge detailType = SubTeamApplicationDecided
  → Handler:
      match new_status
        Approved:
          - transact: set Team.parent_team_id = parent_team_id
                     clear Team.pending_parent_team_id
                     create SubTeamLink
                     notify sub-team owner/admins
        Rejected:
          - transact: clear Team.pending_parent_team_id (team becomes standalone)
                     notify sub-team owner/admins with decision_reason
        Returned:
          - notify only; Team unchanged (still pending_parent_team_id)
```

### 3. Parent-delete cascade (NO new pipe — piggybacks on existing Team deletion handler)

In the existing team-delete controller, before emitting the Team REMOVE, enumerate `SubTeamLink` rows and for each child:

- clear `parent_team_id` on child
- delete `SubTeamLink`
- notify child admin (`SubTeamParentDeleted`)

If the existing team-delete path does not already host pre-delete side effects, extend it here; otherwise use a stream-driven cleanup keyed on the Team REMOVE event.

## Frontend architecture

### Routes (added to `Route` enum)

```rust
Route::TeamSubTeamManagementPage { team_id }           // parent admin (→ subteam-management-page.html)
Route::TeamSubTeamApplyPage { team_id }                // sub-team admin applies (→ subteam-apply.html)
Route::TeamSubTeamApplicationStatusPage { team_id }    // → child-application-status.html
Route::TeamSubTeamDetailPage { team_id, sub_team_id }  // → subteam-management-detail-page.html
Route::TeamSubTeamDocComposePage { team_id, doc_id: Option<String> }    // → subteam-doc-compose.html
Route::TeamSubTeamBroadcastComposePage { team_id, announcement_id: Option<String> }  // → subteam-broadcast-compose.html
Route::TeamSubTeamDeregisterPage { team_id, sub_team_id }   // → parent-deregister.html
Route::TeamLeaveParentPage { team_id }                      // → child-leave-parent.html
Route::TeamBylawsPage { team_id }                           // → bylaws-section.html (public readable)
```

### Module layout — `app/ratel/src/features/sub_team/`

```
sub_team/
├── mod.rs, route.rs, i18n.rs
├── controllers/
│   ├── settings.rs            (is_parent_eligible, min_members)
│   ├── form_fields.rs         (CRUD + reorder)
│   ├── docs.rs                (CRUD + reorder, plus public list)
│   ├── applications_parent.rs (queue + approve/reject/return)
│   ├── applications_child.rs  (submit / edit / cancel)
│   ├── announcements.rs       (draft/publish/delete/list)
│   ├── activity.rs            (dashboard + drill-down)
│   └── parent_link.rs         (deregister + leave)
├── models/
│   ├── sub_team_link.rs
│   ├── sub_team_document.rs
│   ├── sub_team_doc_agreement.rs
│   ├── sub_team_form_field.rs
│   ├── sub_team_application.rs
│   └── sub_team_announcement.rs
├── services/
│   ├── announcement_fanout.rs
│   └── application_lifecycle.rs
├── hooks/
│   ├── use_sub_team_settings.rs     (parent requirements tab)
│   ├── use_sub_team_form.rs         (parent form tab)
│   ├── use_sub_team_docs.rs         (parent docs tab)
│   ├── use_sub_team_list.rs         (parent sub-teams tab)
│   ├── use_sub_team_queue.rs        (parent pending queue)
│   ├── use_sub_team_broadcast.rs    (parent broadcast tab)
│   ├── use_sub_team_apply.rs        (child apply page)
│   ├── use_sub_team_application_status.rs
│   ├── use_sub_team_activity.rs     (dashboard)
│   └── use_sub_team_doc_compose.rs  (child+parent doc editor)
├── pages/
│   ├── management/        (tab container; delegates per-tab controllers)
│   ├── apply/
│   ├── application_status/
│   ├── detail/
│   ├── doc_compose/
│   ├── broadcast_compose/
│   ├── deregister/
│   ├── leave_parent/
│   └── bylaws_section/
├── components/
│   ├── doc_agreement_modal/  (public overlay from subteam-apply.html)
│   ├── parent_hud_panel/     (parent-home-with-button.html HUD)
│   ├── activity_gauges/      (dashboard metric card)
│   └── member_activity_row/
└── types/
    ├── error.rs              (SubTeamError enum — per-check variants)
    └── response.rs
```

Each controller hook follows `conventions/hooks-and-actions.md`: `UseSubTeamX` struct, `try_use_context` / `provide_root_context` idempotent caching, every mutation wrapped in `use_action(...)`. The management page's seven tabs instantiate their own sub-controllers lazily so no one tab pays the cost of another's data fetches.

### Primitives reused

- `Card`, `Button`, `Badge`, `Input`, `Textarea`, `Select`, `Tabs`, `Dialog`, `Popup`, `Switch`, `Row`, `Col`, `Pagination`
- `DragAndDropList` for doc/field reorder
- Existing rich editor (new-post editor) reused in `subteam-doc-compose` and `subteam-broadcast-compose` — Phase 1 restricts toolbar to heading/bold/italic/list/quote (no image/file buttons)
- Existing notification inbox — new variants render via translate-based title + payload.url()

## Test plan

### Server function integration tests — `app/ratel/src/tests/sub_team_tests.rs`

AC traceability: each acceptance criterion from the roadmap maps to at least one test.

| AC | Test name |
|---|---|
| AC-1 | `test_parent_publishes_bylaws_doc_appears_in_list` |
| AC-2 | `test_parent_adds_custom_required_form_field` |
| AC-3 | `test_child_creates_pending_team_with_parent_candidate` |
| AC-4 | `test_member_count_updates_on_apply_context` |
| AC-5 | `test_apply_rejected_below_min_members` |
| AC-6 | `test_apply_rejected_without_required_doc_agreement` |
| AC-7 | `test_parent_sees_submitted_application_in_queue` |
| AC-8 | `test_parent_return_then_child_resubmit_flow` |
| AC-9 | `test_parent_approve_flips_team_status_and_notifies` |
| AC-10 | `test_announcement_publish_fans_out_pinned_posts` (uses local-dev stream_handler) |
| AC-11 | `test_announcement_creates_notification_per_member` |
| AC-12 | `test_announcement_comment_notifies_parent_author` |
| AC-13 | `test_activity_dashboard_weekly_and_monthly_counts` |
| AC-14 | `test_activity_drill_down_returns_per_member` |
| AC-15 | `test_activity_excludes_private_posts` |
| AC-16 | `test_deregister_clears_parent_keeps_content` |
| AC-17 | `test_leave_parent_clears_parent_notifies_parent_admin` |
| AC-18 | `test_parent_delete_cascades_to_sub_teams` |
| AC-19 | `test_sub_team_join_notice_returned_from_member_context` |
| AC-20 | `test_activity_dashboard_returns_privacy_notice_text` |

Plus negative cases: unauthenticated submit (401), non-admin approve (403), duplicate concurrent application (409), stale `body_hash` on agreement (400 + `DocAgreementStale`), cycle prevention (`test_cannot_apply_to_self_or_descendant`).

### Playwright e2e — extend `playwright/tests/web/`

New spec file `sub-team.spec.js` using `test.describe.serial`:

1. user1 (parent admin): create department team, flip `is_parent_eligible`, set `min_members=3`, add bylaw doc as 필독, add custom form field "Faculty advisor"
2. user2 (child founder) in a second context: create pending team, read → agree to each required doc via modal, invite user3 to hit member count, submit application
3. user1: see app in queue → return with comment → user2: edit + resubmit
4. user1: approve → user2: see approval notification + team now shows parent badge
5. user1: publish announcement → user2 / user3: see pinned post in sub-team feed, comment thread round-trip
6. user1: open activity dashboard (weekly), drill down into the club, verify per-member row shows user2
7. Private-post test: user2 creates a Private post → user1's dashboard weekly count is unchanged
8. Deregister flow: user1 deregisters → user2 team no longer shows parent; notification received
9. Leave flow (separate pass): re-apply + approve → user2 leaves → user1 notified

Each `test()` block contains the assertions for a single AC or pair. The spec piggybacks on `utils.js` `goto/click/fill/waitPopup` helpers — no raw Playwright APIs — per `conventions/playwright-tests.md`.

## Open questions — to resolve before implementation

- **OQ-A** (from roadmap OQ-2): dashboard drill-down shows space names (Phase 1) or only counts? *Proposed: show names of Public/Team-Shared spaces only.*
- **OQ-B**: should a returned/rejected application be visible in the child's history forever, or only the most recent? *Proposed: keep forever — user-controllable archive in Phase 2.*
- **OQ-C**: when a parent team with 50+ sub-teams hits the fan-out cap, do we block the publish or silently truncate? *Proposed: block with a typed error `BroadcastTooManySubTeams` and surface "Phase 2 will lift this limit" message.*
- **OQ-D**: do we need a GSI on `Post.announcement_parent_team_id` so the parent can view "all my published announcements" across sub-teams? *Proposed: no — parent already sees the canonical `SubTeamAnnouncement` record; fan-out Posts are child-side only.*

## Risks & mitigations

- **Announcement fan-out latency.** Mitigated by the 50-sub-team cap. If a department hits the cap, degrade to "Phase 2 needed" error rather than partial fan-out.
- **Activity dashboard query cost.** Query-time aggregation across 50 sub-teams × 4 metrics × 2 windows = 400 DynamoDB queries worst-case. Cache result per `(parent_team_id, window)` in the hook for 60 s. If that is not enough, Phase 2 introduces a `SubTeamActivitySnapshot` precompute.
- **Form snapshot drift.** Every `SubTeamApplication` stores `form_snapshot`; the parent editing their form never invalidates in-flight applications. Child sees the form they submitted in their status page — not the current form.
- **Hash-collision on `body_hash`.** sha256 → effectively zero risk. A stale-hash rejection is a recoverable error: child reopens the doc modal, re-reads the current version, re-agrees.
- **Cycle on parent-child.** Creation-time check: `child.pk != parent.pk && parent.parent_team_id != child.pk`. Two-level constraint (no grandparent) in Phase 1 makes deeper cycle-checks unnecessary.

## Rollout

1. Data-model PR first: entities + migrations + Team field additions. Behind no flag — safe no-op if unused.
2. Parent-admin CRUD PR: settings, form fields, docs. Gated behind `is_parent_eligible` flag.
3. Application flow PR: child submit + parent queue + decision events.
4. Broadcast PR: announcement entity + fan-out Lambda + stream handler.
5. Dashboard PR.
6. Leave / deregister / parent-delete cascade PR.
7. Playwright suite PR — one serial spec exercising AC-1 through AC-20.

Each PR ships behind the `is_parent_eligible` flag — no observable change for teams that haven't opted in.
