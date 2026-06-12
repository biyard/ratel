# Discussion: Import from Overview — System Design

**Author / Date**: victor · 2026-06-08
**Branch**: feature/overview-cloning

## Summary

Add an "Import from overview" button to the top-right of the discussion action
editor. Clicking it copies the current space overview's `title` → discussion
`title` and overview `content` → discussion `html_contents`, **always
overwriting** whatever is currently typed, then lets the existing autosave
persist the change.

## Behavior

- Button lives in the editor topbar's right area (`arena-topbar__right`), left
  of the existing "Cancel" button.
- On click: discussion title input and body editor are filled from the space
  overview. Always overwrites (no confirm, no empty-only guard — per product
  decision 2026-06-08).
- Field mapping:
  | Overview (`SpaceResponse`) | Discussion (`UpdateDiscussionRequest`) |
  |---|---|
  | `title` | `title` |
  | `content` | `html_contents` |
  (Overview has no category/attachments → those are untouched.)
- After filling, the existing 3-second debounce autosave + the explicit
  `save_title()` / `save_html()` calls persist via `update_discussion`.

## Key constraint: RichEditor remount

`common/components/editor/component.rs` snapshots `props.content` once at mount
via `use_hook(|| props.content.clone())` and never re-applies it on later
renders (deliberate — re-applying `dangerous_inner_html` destroys IME caret
position during Korean composition). Its own doc comment says: "Use `key=` on
the parent to force a remount when the editor needs to be reset."

Therefore importing overview body cannot work by only `html_contents.set(...)`.
We bump an `editor_epoch` signal and pass `key: "{editor_epoch}"` to the
`RichEditor`, forcing a remount that reads the new `html_contents` as its
initial content. The title is a plain `<input value="{title()}">` and reflects
the signal reactively with no remount needed.

## Cross-component wiring

The import button is rendered by the page (`DiscussionActionEditorPage`, which
has `use_space()` access), but the `title` / `html_contents` signals live in
the `ContentCard` child. They communicate through a trigger counter on the
shared discussion `Context`:

- `Context` gains `pub import_request: Signal<u64>`.
- Page button onclick: `ctx.import_request += 1`.
- `ContentCard` adds a `use_effect` watching `import_request()`. When it changes
  to a non-zero value, it reads `use_space()` overview, sets `title` +
  `html_contents`, bumps `editor_epoch` (remount), and calls
  `save_title()` / `save_html()`.

## Shared topbar stays generic

`ActionEditTopbar` is shared by poll/quiz/follow editors. It gains an optional
`#[props(default)] right_actions: Element` slot rendered inside
`arena-topbar__right` before the Cancel button. Default is empty, so other
action editors are unaffected. Only the discussion page passes a button into it.

## Files changed

1. `actions/components/action_edit_topbar/component.rs` — add `right_actions` slot.
2. `actions/actions/discussion/context.rs` — add `import_request: Signal<u64>`.
3. `actions/actions/discussion/views/editor/mod.rs` — render import button into
   topbar `right_actions`; onclick bumps `ctx.import_request`.
4. `actions/actions/discussion/views/editor/content_card/component.rs` — add
   `editor_epoch` signal, `key` on RichEditor, and import `use_effect`.
5. `actions/actions/discussion/views/editor/i18n.rs` — add `import_from_overview`
   label (en/ko).

## Test plan

- Manual: open a space with overview content → create/edit a discussion action →
  click "Import from overview" → title + body fill from overview, autosave fires.
- Build: `dx check --features web`, `cargo check --features web`,
  `cargo check --features server` (all with `RUSTFLAGS='-D warnings'`).

## Deviation (2026-06-08, post-test fix)

First implementation reused the existing `save_title()` / `save_html()` closures
inside the import `use_effect` and bumped `editor_epoch` with
`editor_epoch.set(editor_epoch() + 1)`. This caused an **infinite render/save
loop** (observed: page unresponsive, repeating PATCH calls):

- `editor_epoch()` was *read* then *written* in the same effect → self-trigger.
- `save_title()` / `save_html()` called synchronously in the effect body read
  `title()` / `html_contents()` / `last_saved_*()`, subscribing the effect to
  them; the savers later wrote `last_saved_*`, re-triggering the effect.

Fix: the effect's synchronous body now reads **only** `ctx.import_request()`
(the intended trigger). `editor_epoch` is bumped via `*editor_epoch.peek() + 1`
(non-subscribing), and persistence is a single `update_discussion` PATCH issued
inside a `spawn(async move { … })` so all its signal reads happen outside the
reactive scope. (This mirrors why the existing debounce effects call the savers
inside `spawn`.)

## Deviation 2 (2026-06-08, post-test fix)

Symptom: after the loop fix, the **title filled immediately but the body editor
stayed empty until a manual refresh** (the PATCH saved correctly, so a reload
re-seeded the editor from the discussion).

Root cause: `key: "{editor_epoch()}"` placed **directly on the lone `RichEditor`
component child** did NOT trigger a remount in this Dioxus version — the
component re-rendered (title updated) but the editor instance was reused, so its
mount-time `content` snapshot never refreshed.

Fix: wrap the editor in a single-item keyed `for`:
`for epoch in [editor_epoch()] { RichEditor { key: "{epoch}", … } }`. A keyed
**list item** is reconciled by key (old key removed → new key created), which
forces a real remount; a bare `key` on a single non-list component child is not.

## Risks

- RichEditor remount via `key` is the documented reset path; verified against the
  component's own guidance. If `key` on a component doesn't remount as expected,
  fall back to keying a wrapping `div.editor`.
