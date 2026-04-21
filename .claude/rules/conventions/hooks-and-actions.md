# Feature Hooks & Actions

Every feature exposes its data and mutations through a single `UseFeatureName` hook. Components consume the hook — they never call server functions directly.

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

## Rule 3: Wrap every mutation in `use_action`

Every API call that the UI triggers — `POST`, `PATCH`, `DELETE`, or any state-changing side effect — must be an `Action<Input, Output>` inside the controller. The component never awaits a server function directly.

```rust
let handle_mark_all = use_action(move || async move {
    mark_all_read_handler().await?;
    unread_count.set(0);
    inbox.refresh();
    Ok::<(), crate::common::Error>(())
});
```

Rules:
- The closure's async block must return `Result<T, E>` where `E: Into<CapturedError>`. `common::Error` satisfies this via its `thiserror::Error` derive.
- **Always** add an explicit type annotation on the trailing `Ok`: `Ok::<(), crate::common::Error>(())`. Without it, rustc can't infer `E` and reports `_: Into<CapturedError>` errors.
- Keep all post-mutation side effects (signal updates, `inbox.refresh()`, `nav.push(...)`) inside the action body — consumers call one thing.
- Action `Input` is a tuple: `Action<(), ()>` for zero-arg, `Action<(T,), U>` for one-arg, `Action<(A, B), C>` for two-arg. Match the closure signature: `move || ...`, `move |x: T| ...`, `move |a: A, b: B| ...`.

### Naming

| Kind | Convention | Example |
|---|---|---|
| Query / loader | noun | `inbox`, `unread_count`, `user_profile` |
| Action | `handle_<verb>` or `<verb>_<noun>` | `handle_mark_all`, `handle_item_click`, `create_post`, `delete_comment` |

## Rule 4: Components consume, they do not call handlers

Components destructure what they need from `UseFeature` and pass actions straight into event handlers. Do not import server-function handlers (`_handler` suffix) into component modules.

```rust
// GOOD
#[component]
pub fn NotificationPanel(open: bool, on_close: EventHandler<()>) -> Element {
    let UseInbox {
        mut inbox,
        mut handle_item_click,
        mut handle_mark_all,
        ..
    } = use_inbox()?;

    rsx! {
        button {
            onclick: move |_| handle_mark_all.call(),
            "Mark all as read"
        }
        for item in inbox.items() {
            NotificationItem {
                key: "{item.id.0}",
                item: item.clone(),
                onclick: move |it| handle_item_click.call(it),
            }
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
                    inbox.refresh();
                });
            },
            "Mark all as read"
        }
    }
}
```

Reasons components must go through actions, not `spawn(async move { handler().await })`:
- **One source of truth for side effects.** Refresh, signal updates, and navigation live in the action body. If two components call the same mutation they always get the same post-mutation behaviour.
- **Lifecycle tracking.** `Action` exposes `.pending()`, `.value()`, `.error()` — useful for disabling buttons, showing spinners, surfacing errors. Raw `spawn(...)` discards all of that.
- **Cached controller.** A single `use_feature()?` inside the component tree resolves to the same `UseFeature` — two consumers of `handle_mark_all` hit the same action instance, not two different spawns.
- **No orphan futures.** `spawn` in an event handler detaches a future from the component; if the component unmounts mid-await, unrelated side effects (notifications, etc.) still fire against a missing context.

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

## Reference implementation

`app/ratel/src/features/notifications/hooks/use_inbox.rs` + `app/ratel/src/features/notifications/components/notification_panel/component.rs` implement every rule above.
