---
globs: ["app/ratel/**/*.rs"]
---

# Anti-patterns

Patterns to avoid across the Ratel codebase. Each item shows the bad pattern and its correct replacement.

## Navigation

```rust
// BAD — String path bypasses compile-time route validation
nav.push(format!("/spaces/{}/dashboard", space_id));
nav.replace(format!("/posts/{post_id}"));
Link { to: "/spaces/{space_id}/dashboard", "Dashboard" }

// GOOD — Route enum ensures valid routes at compile time
nav.push(Route::SpaceDashboardPage { space_id });
nav.replace(Route::PostDetailPage { post_id });
Link { to: Route::SpaceDashboardPage { space_id }, "Dashboard" }
```

## Direct Server-Handler Calls from Components

```rust
// BAD — component awaits a _handler directly; side effects and lifecycle
// tracking are lost; refresh/nav logic gets duplicated at every call site.
#[component]
fn NotificationPanel() -> Element {
    let mut inbox = use_infinite_query(...)?;
    rsx! {
        button {
            onclick: move |_| {
                spawn(async move {
                    let _ = mark_all_read_handler().await;
                    inbox.refresh();
                });
            },
            "Mark all as read"
        }
    }
}

// GOOD — mutation lives in a UseFeature controller, component just calls the action
#[component]
fn NotificationPanel() -> Element {
    let UseInbox { mut handle_mark_all, .. } = use_inbox()?;
    rsx! {
        button {
            onclick: move |_| handle_mark_all.call(),
            "Mark all as read"
        }
    }
}
```

Every feature exposes a `UseFeatureName` hook that owns loaders, queries, and `use_action(...)` mutations. Components destructure from that hook and never import server `_handler` functions. See `conventions/hooks-and-actions.md`.

## Use-Action Closure Without Explicit `Ok` Type

```rust
// BAD — rustc can't infer Err, reports `_: Into<CapturedError>`
let handle_mark_all = use_action(move || async move {
    mark_all_read_handler().await?;
    inbox.refresh();
    Ok(())
});

// GOOD — explicit Err type lets CapturedError conversion resolve
let handle_mark_all = use_action(move || async move {
    mark_all_read_handler().await?;
    inbox.refresh();
    Ok::<(), crate::common::Error>(())
});
```

## Action Field Destructured Without `mut`

```rust
// BAD — Action::call takes &mut self
let UseInbox { handle_mark_all, .. } = use_inbox()?;
// error: cannot borrow `handle_mark_all` as mutable

// GOOD
let UseInbox { mut handle_mark_all, .. } = use_inbox()?;
```

## Conditional Class Strings

```rust
// BAD — conditional class string via if/else
let cls = if active { "px-3 py-1.5 bg-primary/10" } else { "px-3 py-1.5" };
Row { class: "{cls}", ... }

// GOOD — aria attribute + Tailwind variant
Row { class: "px-3 py-1.5 aria-relevant:bg-primary/10", "aria-relevant": active, ... }
```

Use `aria-selected`, `aria-owns`, `aria-`, etc. with their Tailwind variant prefixes instead of building class strings conditionally.

## Styling

- `style="color: #fcb300"` — use semantic token class instead
- `style="background: #1a1a1a"` — use `bg-background` or `bg-card-bg`
- `text-neutral-400`, `bg-slate-800`, `text-gray-500` — use `text-foreground-muted`, `bg-card-bg`, `text-text-primary`
- `z-101` (silently ignored) — use `z-[101]` for arbitrary values

## Components

## cfg-gated Component Variants (SSR Hydration)

```rust
// BAD — different output on server vs client causes hydration mismatch
#[cfg(not(feature = "server"))]
#[component]
fn MyWidget() -> Element { rsx! { div { "content" } } }

#[cfg(feature = "server")]
#[component]
fn MyWidget() -> Element { rsx! {} }

// GOOD — same component renders on both sides; data loads client-side via use_loader
#[component]
fn MyWidget() -> Element { rsx! { div { "content" } } }
```

A component must produce the same HTML structure on server and client. If the server renders nothing but the client renders a `div`, hydration will fail. Use a single component definition — server functions and `use_loader` handle data fetching naturally on the client.

## Components

- Raw `<div class="flex ...">` for layouts — use `Row` or `Col` components
- Raw `<button>` — use `Button` component
- Raw `<input>` — use `Input` component
- Raw `<select>` — use `Select` component

## Props Cloning in Closures

```rust
// BAD — cloning props to pass into closures
fn MyComponent(tag: Vec<String>, on_remove: EventHandler<Vec<String>>) {
    onclick: {
        let tag = tag.clone();
        move |_| {
            on_remove.call(tag.clone());
        }
    },
}

// GOOD — use ReadSignal, which is Copy and avoids cloning
fn MyComponent(tag: ReadSignal<Vec<String>>, on_remove: EventHandler<Vec<String>>) {
    onclick: move |_| {
        on_remove.call(tag());
    },
}
```

Dioxus auto-converts `T` to `ReadSignal<T>` when passed as a prop. `ReadSignal` implements `Copy`, so it can be moved into closures without cloning.

## Reactive Server Futures

```rust
// BAD — use_reactive + use_server_future with manual clone
fn MyComponent(space_id: SpacePartition) -> Element {
    let loader = use_server_future(use_reactive((&space_id,), |(sid,)| async move {
        get_ranking_handler(sid.clone(), None).await
    }))?;
}

// GOOD — ReadSignal prop + use_loader (signal is Copy, no clone needed)
fn MyComponent(space_id: ReadSignal<SpacePartition>) -> Element {
    let loader = use_loader(move || async move {
        get_ranking_handler(space_id(), None).await
    })?;
}
```

When a prop is used only in async loaders, accept `ReadSignal<T>` instead of `T`. `ReadSignal` is `Copy` and reactive — calling `space_id()` inside the closure automatically re-runs when the value changes, eliminating the need for `use_reactive` and `.clone()`.

## Async Event Handlers

```rust
// BAD — unnecessary spawn wrapping
onfocusout: move |_| {
    spawn(async move {
        // async work
    });
},

// GOOD — use async move directly
onfocusout: move |_| async move {
    // async work
},
```

## Async Sleep

```rust
// BAD — gloo_timers is web-only; breaks mobile/desktop builds and
// forces every caller to cfg-gate between gloo_timers and tokio::time
#[cfg(feature = "web")]
gloo_timers::future::sleep(Duration::from_millis(300)).await;
#[cfg(feature = "server")]
tokio::time::sleep(Duration::from_millis(300)).await;

// GOOD — centralized sleep works across web, server, and mobile
use crate::common::utils::time::sleep;
sleep(Duration::from_millis(300)).await;
```

`gloo_timers` must never be used directly in feature code. Call
`crate::common::utils::time::sleep` instead — it handles the feature-gating
(`tokio::time::sleep` on server, `gloo_timers` on web, no-op elsewhere) in
one place so callers stay platform-agnostic.

## Error Handling

```rust
// BAD — leaks internal details to user
Error::BadRequest(format!("DynamoDB error: {e}"))

// GOOD — log detail server-side, return generic unit error
crate::error!("failed to create entity: {e}");
MyFeatureError::CreateFailed
```

## List Response Types

```rust
// BAD — custom struct duplicating ListResponse fields
pub struct RankingResponse {
    pub entries: Vec<RankingEntryResponse>,
    pub bookmark: Option<String>,
}

// GOOD — use ListResponse<T> from common::types
pub async fn get_ranking_handler(...) -> Result<ListResponse<RankingEntryResponse>> {
    let (items, next_bookmark) = Model::find_by_pk(cli, &pk, opts).await?;
    Ok((items, next_bookmark).into())
}
```

Never create custom response structs with `items` + `bookmark` fields. Use `ListResponse<T>` which already derives `PartialEq`, implements `Bookmarker`, `ItemIter`, and converts from `(Vec<T>, Option<String>)`.

## Bookmark Option Handling

```rust
// BAD — manual if-let to set bookmark
let mut opts = Model::opt().limit(50);
if let Some(bm) = bookmark {
    opts = opts.bookmark(bm);
}

// GOOD — use opt_with_bookmark which handles Option internally
let opts = Model::opt_with_bookmark(bookmark).limit(50);
```

`opt_with_bookmark(Option<String>)` is generated by `DynamoEntity` derive and handles the `None` case internally.

## API Types

```rust
// BAD — exposes raw Partition/EntityType with prefix in API
fn update_poll(space_pk: Partition, poll_sk: EntityType)

// GOOD — uses SubPartition with id naming, no prefix
fn update_poll(space_id: SpacePartition, poll_id: SpacePollEntityType)
```

## i18n

```rust
// BAD — hardcoded string in UI
"Submit"

// GOOD — translated string
"{t.submit}"

// BAD — Display trait for enum in UI
status.to_string()

// GOOD — Translate trait for enum in UI
status.translate(&lang())
```

## HTML-First Components

### Using `document::Link` for stylesheets

```rust
// BAD — preload + stylesheet Link pair for the same CSS asset
document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
document::Link { rel: "stylesheet", href: asset!("./style.css") }

// BAD — plain stylesheet Link
document::Link { rel: "stylesheet", href: asset!("./style.css") }

// GOOD — single Stylesheet component injects into <head> and dedupes
document::Stylesheet { href: asset!("./style.css") }
```

`document::Stylesheet` is the dedicated component for CSS — it injects into
`<head>`, dedupes by href across re-renders, and handles SSR/CSR
hydration correctly. The legacy `document::Link { rel: "preload", … }`
+ `document::Link { rel: "stylesheet", … }` pair is a workaround from
before `Stylesheet` existed; it duplicates the asset in the DOM and
fights the framework's dedupe logic.

### Missing `defer` on Script

```rust
// BAD — script runs before DOM exists, getElementById returns null
document::Script { src: asset!("./script.js") }

// GOOD — defer ensures script runs after DOM is parsed
document::Script { defer: true, src: asset!("./script.js") }
```

### Renaming class names from HTML mockup

```rust
// BAD — renamed from mockup's "carousel-track", breaks JS selectors
div { class: "action-carousel__track", id: "action-carousel-track", ... }

// GOOD — exact same class/ID as the HTML mockup
div { class: "carousel-track", id: "carousel-track", ... }
```

### Using data attributes for JS-controlled state

```rust
// BAD — Dioxus re-renders overwrite data-active, breaking JS scroll detection
div { class: "quest-card", "data-active": some_signal(), ... }

// GOOD — JS toggles .active class via classList, Dioxus doesn't interfere
div { class: "quest-card", ... }
// JS: card.classList.toggle('active', i === closest);
```

### Simplifying specialized mockup content

```rust
// BAD — generic card that ignores the mockup's specialized content per action type
div { class: "quest-card",
    div { class: "quest-card__title", "{action.title}" }
    div { class: "quest-card__desc", "{action.description}" }
}

// GOOD — specialized content matching the HTML mockup for each action type
// Poll: show vote option preview rows
// Discussion: show topic tags + comment count
// Quiz: show questions/pass-rate/time stats
// Follow: show inline user list with follow buttons
```

When an HTML mockup includes specialized content for each variant, the Dioxus implementation must reproduce all of it — not simplify to a generic card.
