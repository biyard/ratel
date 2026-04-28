# Feature Hooks & Actions

Every feature exposes its data and mutations through a single context (or `UseFeatureName` hook). Components consume the context — they never call server functions directly.

**Two mutation shapes**, picked by what the UI does with the result:
1. **`async fn` method on the context** — default for most button handlers. Component does `ctx.do_thing(payload).await`, decides UX (close popup, navigate, toast) on the result.
2. **`use_action(...)` field on the controller** — only when the UI binds directly to `.pending()` / `.value()` / `.error()` for spinner/last-result/disabled-state.

See Rule 3 for how to pick.

## Rule 1: Feature state lives in a `UseFeatureName` controller

Each feature that has interactive state owns one `UseFeatureName` struct that bundles every signal, loader, query, and mutation the UI needs. The struct derives `Clone`, `Copy`, and `DioxusController`.

```rust
use crate::common::*;
use crate::common::hooks::{use_infinite_query, InfiniteQuery};

#[derive(Clone, Copy, DioxusController)]
pub struct UseInbox {
    pub inbox: InfiniteQuery<String, InboxNotificationResponse, ListResponse<InboxNotificationResponse>>,
    pub unread_only: Signal<bool>,
    pub unread_count: Signal<i64>,
    pub handle_item_click: Action<(InboxNotificationResponse,), ()>,
    pub handle_mark_all: Action<(), ()>,
}
```

Put the struct next to the hook that constructs it: `features/<feature>/hooks/use_<feature>.rs`.

## Rule 2: The hook is context-cached and returns `Result<UseFeature, RenderError>`

The hook must be idempotent across the component tree. The first caller builds and provides the controller to the root; every subsequent caller reads it back from context.

```rust
#[track_caller]
pub fn use_inbox() -> std::result::Result<UseInbox, RenderError> {
    // 1. Reuse any controller already installed by a parent component.
    if let Some(ctx) = try_use_context::<UseInbox>() {
        return Ok(ctx);
    }

    // 2. Build loaders / queries. `?` propagates RenderError.
    let unread_only = use_signal(|| false);
    let mut inbox = use_infinite_query(move |bookmark| {
        let unread_only = unread_only();
        async move { list_inbox_handler(Some(unread_only), bookmark).await }
    })?;

    // 3. Build actions (see Rule 3).
    let nav = use_navigator();
    let handle_item_click = use_action(move |item: InboxNotificationResponse| async move {
        mark_read_handler(item.id.clone()).await?;
        let cta = item.payload.url().to_string();
        if !cta.is_empty() { nav.push(cta); }
        Ok::<(), crate::common::Error>(())
    });

    let mut unread_count = super::use_unread_count();
    let handle_mark_all = use_action(move || async move {
        mark_all_read_handler().await?;
        unread_count.set(0);
        inbox.refresh();
        Ok::<(), crate::common::Error>(())
    });

    // 4. Provide at the root so every consumer shares one instance.
    Ok(provide_root_context(UseInbox {
        inbox, unread_only, unread_count,
        handle_item_click, handle_mark_all,
    }))
}
```

Rules:
- Return type is always `std::result::Result<UseFeature, RenderError>` — `?` bubbles up into `#[component]` bodies naturally.
- Tag with `#[track_caller]` so hook-ordering errors point at the real call site.
- Always guard with `try_use_context::<UseFeature>()` → return existing instance if present, otherwise build and `provide_root_context(...)`.
- Do not accept args that change the shape of the controller (no `use_inbox(unread_only: bool)`). Store such toggles as `Signal<T>` fields and let the caller flip them.

## Rule 3: Wrap every mutation in either an `async fn` method on the context **or** `use_action`

Every API call that the UI triggers — `POST`, `PATCH`, `DELETE`, or any state-changing side effect — must live inside the controller. The component never awaits a server function directly.

There are **two acceptable shapes**, picked by what the UI does with the result.

### Default: `async fn` method on the context

For most button handlers (create, delete, save, navigate-after-success), expose the mutation as an `async fn` method on the controller struct. Components call it directly with `.await`.

```rust
#[derive(Clone, Copy)]
pub struct TeamContext {
    pub teams: Loader<Vec<TeamItem>>,
    pub selected_index: Signal<usize>,
}

impl TeamContext {
    pub async fn create_team(
        &mut self,
        payload: TeamCreationPayload,
    ) -> crate::Result<TeamItem> {
        let req = CreateTeamRequest { /* … */ };
        let response = create_team_handler(req).await?;
        let team_item = TeamItem { pk: response.team_pk, /* … */ };
        self.teams.push(team_item.clone());   // mutation + signal update in one place
        Ok(team_item)
    }
}
```

```rust
// Component: pure UI, no business logic
on_submit: move |payload| async move {
    submitting.set(true);
    match team_ctx.create_team(payload).await {
        Ok(_) => { popup.close(); nav.push(Route::TeamHome { … }); }
        Err(e) => { error_msg.set(Some(format!("{e}"))); }
    }
    submitting.set(false);
}
```

Why this is the default:
- **Components stay UI-only.** The handler describes what to do with success/failure (close popup, navigate, show toast); the *how* (server call, signal update, refresh) lives in the context.
- **Signals + loaders mutated through one structure.** `self.teams.push(...)` keeps the loader's contents and the API result coherent. No "did I forget to call `.refresh()` after mutating?" class of bug.
- **No boxed-future plumbing.** Just an `async fn` — no `Action<Input, Output>` type-puzzle, no `Pin<Box<dyn Future<…>>>`, no explicit `Ok::<(), …>(())` annotation.
- **Errors flow naturally.** `?` propagates from the method into the component's `match`, so the component decides UX-level error handling.

### When `use_action` is the right tool

Use `use_action` only when you need the action's **lifecycle state to drive UI directly** without writing per-call signal plumbing:
- A button shows a spinner while the action is in flight → bind to `action.pending()`.
- A panel shows the last error from the action → bind to `action.error()`.
- A view shows the last successful result without storing it in a separate signal → bind to `action.value()`.
- The same mutation is fired from many sites and you want one shared `pending`/`error` observable across all of them.

```rust
let handle_mark_all = use_action(move || async move {
    mark_all_read_handler().await?;
    unread_count.set(0);
    inbox.refresh();
    Ok::<(), crate::common::Error>(())
});

// Component:
button {
    disabled: handle_mark_all.pending(),
    onclick: move |_| handle_mark_all.call(),
    "Mark all as read"
}
```

Rules when you do reach for `use_action`:
- The closure's async block must return `Result<T, E>` where `E: Into<CapturedError>`. `common::Error` satisfies this via its `thiserror::Error` derive.
- **Always** add an explicit type annotation on the trailing `Ok`: `Ok::<(), crate::common::Error>(())`. Without it, rustc can't infer `E` and reports `_: Into<CapturedError>` errors.
- Keep all post-mutation side effects (signal updates, `inbox.refresh()`, `nav.push(...)`) inside the action body — consumers call one thing.
- Action `Input` is a tuple: `Action<(), ()>` for zero-arg, `Action<(T,), U>` for one-arg, `Action<(A, B), C>` for two-arg.

### Picking between the two

| Situation | Use this |
|---|---|
| Component awaits the result and decides what to do (close popup, navigate, show error) | **`async fn` method** |
| Result must mutate context state and that's it | **`async fn` method** |
| UI binds to in-flight `pending` / `error` / `value` without separate signals | **`use_action`** |
| Same mutation fired from many components, all needing one shared lifecycle observable | **`use_action`** |

If both apply, default to the method — adding `pending` etc. to the controller as plain `Signal<bool>` fields you flip inside the method is cheaper than fighting `Action`'s ergonomics.

### Naming

| Kind | Convention | Example |
|---|---|---|
| Query / loader | noun | `inbox`, `unread_count`, `user_profile` |
| Action | `handle_<verb>` or `<verb>_<noun>` | `handle_mark_all`, `handle_item_click`, `create_post`, `delete_comment` |

## Rule 4: Components consume, they do not call handlers

Components either await a context method or call a context action. Do not import server-function handlers (`_handler` suffix) into component modules.

```rust
// GOOD — context method (default for button handlers)
#[component]
pub fn TeamCreationPopup() -> Element {
    let mut team_ctx = use_team_context();
    let mut popup = use_popup();
    let nav = use_navigator();
    let mut error_msg = use_signal(|| Option::<String>::None);

    rsx! {
        TeamCreationForm {
            on_submit: move |payload| async move {
                match team_ctx.create_team(payload).await {
                    Ok(team) => { popup.close(); nav.push(Route::TeamHome { username: team.username }); }
                    Err(e)   => error_msg.set(Some(format!("{e}"))),
                }
            },
        }
    }
}

// GOOD — use_action (when UI binds to .pending()/.value()/.error())
#[component]
pub fn NotificationPanel() -> Element {
    let UseInbox { mut handle_mark_all, .. } = use_inbox()?;
    rsx! {
        button {
            disabled: handle_mark_all.pending(),
            onclick:  move |_| handle_mark_all.call(),
            "Mark all as read"
        }
    }
}
```

```rust
// BAD — component calls server handler directly, duplicates post-mutation wiring
#[component]
pub fn NotificationPanel(...) -> Element {
    let nav = use_navigator();
    let mut inbox = use_infinite_query(...)?;

    rsx! {
        button {
            onclick: move |_| {
                spawn(async move {
                    if let Err(e) = mark_all_read_handler().await {
                        tracing::error!("{e}");
                    }
                    inbox.refresh();   // ← now duplicated at every call site
                });
            },
            "Mark all as read"
        }
    }
}
```

Reasons components must go through context methods or actions, not `spawn(async move { handler().await })`:
- **One source of truth for side effects.** Refresh, signal updates, and post-mutation context state live next to the mutation that produced them. Two components calling the same mutation always get the same post-mutation behaviour.
- **Lifecycle tracking when needed.** When a UI must show pending/error states, `use_action` exposes `.pending()`, `.value()`, `.error()`; raw `spawn(...)` discards all of that.
- **Cached controller.** A single `use_feature()` inside the component tree resolves to the same controller — two consumers hit the same instance, not two different spawns.
- **No orphan futures.** `spawn` in an event handler detaches a future from the component; if the component unmounts mid-await, unrelated side effects still fire against a missing context. `async move |_|` event handlers are tied to the component's lifetime and cancel cleanly.

### Exception: a component that does not mutate

If a component only reads data and has no buttons or state changes, and its data is not shared with siblings, using `use_loader` / `use_server_future` directly in the component is fine — no controller needed.

## Rule 5: Destructuring requires `mut` on actions

`Action::call(&mut self)` takes a mutable borrow, so action fields must be destructured as `mut`:

```rust
let UseInbox {
    mut inbox,              // refresh() needs &mut
    mut handle_item_click,  // .call() needs &mut
    mut handle_mark_all,    // .call() needs &mut
    ..
} = use_inbox()?;
```

## Rule 6: Folder layout

```
features/<feature>/
├── hooks/
│   ├── mod.rs                      pub use use_<feature>::*;
│   ├── use_<feature>.rs            controller struct + hook
│   └── use_<feature>_<subset>.rs   supporting hooks (e.g. use_unread_count)
├── components/
│   └── <component>/component.rs    consumes via use_<feature>()?
├── controllers/
│   └── <endpoint>.rs               server fn handlers (never called from components)
└── types/
    ├── error.rs
    └── response.rs
```

The hook file is the only module that imports `_handler` server functions.

## Reference implementations

- **`async fn` method shape** (default for buttons that act-and-resolve): `app/ratel/src/common/contexts/team_context.rs` (`TeamContext::create_team`) + `app/ratel/src/components/team_creation_popup/mod.rs` (component awaits the method, owns popup/nav/error UX).
- **`use_action` shape** (when `.pending()`/`.value()`/`.error()` drive UI): `app/ratel/src/features/notifications/hooks/use_inbox.rs` + `app/ratel/src/features/notifications/components/notification_panel/component.rs`.
