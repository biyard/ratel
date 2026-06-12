# Space-scoped notifications — System Design

**Author / Date**: victor · 2026-06-08
**Branch**: feature/overview-cloning

## Summary

Inside a space, the notification bell (badge + panel + "mark all read") now
reflects **only that space's notifications** instead of the user's entire global
inbox. The global navbar bell (outside any space) is unchanged.

## Problem (confirmed)

The in-space bell rendered the **same global `NotificationPanel`/`use_inbox()`**
as the rest of the app — it showed all of the user's notifications. Notifications
are stored per-user (`pk = User(id)`); the owning space lives only inside the
`InboxPayload` (and only ~5 of 15 variants carry a `SpacePartition`).

## Approach

Server-side filtering by `space_id` (chosen over client-side), with page-refill
on the bookmark so a page that yields few matches keeps paging (capped). The
frontend scopes via **context shadowing** — zero changes to the bell/panel
components.

### Server (all gain optional `space_id` query param; global callers pass `None`)

- `InboxPayload::space_id() -> Option<SpacePartition>` — maps the 5 space-bearing
  variants (`SpaceStatusChanged`, `SpaceInvitation`, `SpaceActionOngoing`,
  `DiscussionCommentPosted`, `ReplyOnComment`), `None` otherwise.
- `list_inbox_handler(unread_only, space_id, bookmark)` — when `space_id` is set,
  loops raw pages (cap 5) filtering by `payload.space_id()` until it has ~30
  matches or runs out, returning the last raw bookmark so the client's infinite
  query resumes. Unfiltered path unchanged (single page).
- `get_unread_count_handler(space_id)` — counts only matching unread (scans up to
  5 pages when scoped). MCP `get_unread_count` registration switched to the
  `Parameters<...McpRequest>` pattern (it previously took no request).
- `mark_all_read_handler(space_id)` — marks read only matching unread rows.

### Frontend (context shadowing — bell/panel components untouched)

- `use_provide_space_unread_count(space_id)` (in `use_unread_count.rs`) — installs
  a scoped `UnreadCountSignal` polling `get_unread_count_handler(Some(space))`.
- `use_provide_space_inbox(space_id)` (in `use_inbox.rs`) — installs a scoped
  `UseInbox` (infinite query + mark-all + item-click) filtered to the space; its
  `use_unread_count()` resolves to the scoped signal.
- `SpaceIndexPage` calls both at the top (unread-count first), shadowing the
  global providers (installed by `NotificationsBootstrap` at the app root) for
  the space subtree. `NotificationBell` (in `ArenaTopbar`) and `NotificationPanel`
  read the nearest provider via `use_context`, so they pick up the scoped ones
  automatically. Suspension from the scoped infinite query is caught by the root
  `SuspenseBoundary` that wraps the whole route Outlet.

## Why context shadowing

The bell and panel already read `UseInbox` / `UnreadCountSignal` from context.
Providing scoped instances closer in the tree (at `SpaceIndexPage`) overrides
them for descendants only — no prop drilling, no component branching, and the
global navbar bell (rendered outside any space) keeps the root-level global
instances.

## Files changed

- `common/types/inbox_kind.rs` — `InboxPayload::space_id()`
- `features/notifications/controllers/{list_inbox,get_unread_count,mark_all_read}.rs`
- `common/mcp/server.rs` — `get_unread_count` → `Parameters` pattern
- `features/notifications/hooks/use_inbox.rs` — `use_provide_space_inbox` + global caller `None`
- `features/notifications/hooks/use_unread_count.rs` — `use_provide_space_unread_count` + global caller `None`
- `features/spaces/pages/index/component.rs` — install scoped providers

## Verification

`cargo check --features server` ✅, `cargo check --features web` ✅,
`dx check --web` ✅ ("No issues found").

## Known limitations / risks

- The scoped unread **badge** lags up to the 60s poll interval after switching
  directly between two spaces (the poll loop reads `space_id()` fresh each tick,
  so it self-corrects; the panel list re-queries immediately).
- Filtering scans a capped number of raw pages (5). A user whose space-related
  notifications are very sparse among hundreds of global ones may see counts/lists
  capped — acceptable given the badge caps at 100 and the panel paginates.
