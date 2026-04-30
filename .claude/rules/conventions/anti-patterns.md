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

// GOOD (default) — mutation lives as an `async fn` method on the context;
// component awaits it and decides UX (close popup, navigate, toast).
#[component]
fn TeamCreationPopup() -> Element {
    let mut team_ctx = use_team_context();
    let mut popup    = use_popup();
    let nav          = use_navigator();
    rsx! {
        TeamCreationForm {
            on_submit: move |payload| async move {
                if let Ok(team) = team_ctx.create_team(payload).await {
                    popup.close();
                    nav.push(Route::TeamHome { username: team.username });
                }
            },
        }
    }
}

// GOOD (when UI binds to lifecycle) — mutation is a use_action field, component
// calls action.call() and reads .pending() / .error() to drive the UI.
#[component]
fn NotificationPanel() -> Element {
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

Every feature exposes its mutations through a context (or `UseFeatureName` hook). Components either await an `async fn` method on the context (default for button handlers) or call a `use_action` field (when the UI binds to `.pending()` / `.value()` / `.error()`). Components never import server `_handler` functions. See `conventions/hooks-and-actions.md` § Rule 3 for the picking guide.

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

## JS Interop — Direct `wasm_bindgen` Object Binding

```rust
// BAD — direct object binding to a runtime JS namespace via extern "C".
// Forces wasm_bindgen + web-sys + serde-wasm-bindgen onto the dep graph,
// breaks non-web targets without cfg gates, and silently drifts when the
// JS module renames `signMessage`.
#[wasm_bindgen(js_namespace = ["window", "ratel", "auth", "wallet"])]
extern "C" {
    #[wasm_bindgen(js_name = signMessage)]
    fn wallet_sign_message_promise(message: &str) -> Promise;
}

pub async fn wallet_sign_message(message: &str) -> Result<String> {
    let v = JsFuture::from(wallet_sign_message_promise(message))
        .await
        .map_err(|_| AuthError::WalletConnectFailed)?;
    v.as_string().ok_or_else(|| AuthError::WalletConnectFailed.into())
}

// GOOD — one tiny driver script per call, JSON over dioxus.recv()/send().
// Compiles on every target, no extern "C", no JsValue downcasting.
use dioxus::document::eval as dx_eval;

pub async fn wallet_sign_message(message: &str) -> Result<String> {
    let mut runner = dx_eval (include_str!("web/wc_sign_message.js"));
    runner.send(serde_json::json!(message))
        .map_err(|_| AuthError::WalletConnectFailed)?;
    runner.recv::<Option<String>>()
        .await
        .map_err(|_| AuthError::WalletConnectFailed)?
        .ok_or_else(|| AuthError::WalletConnectFailed.into())
}
```

```js
// web/wc_sign_message.js
const message = await dioxus.recv();
try {
  const sig = await window.ratel.auth.wallet.signMessage(message);
  dioxus.send(sig);
} catch (e) {
  dioxus.send(null);
}
```

Direct `#[wasm_bindgen(js_namespace = [...])] extern "C" { ... }` blocks are an anti-pattern. Use the `dioxus::document::eval` channel pattern with a per-call JS driver in a sibling `web/` directory. See `conventions/dioxus-app.md` § JS Interop. Reference migration: `app/ratel/src/features/auth/interop/wallet_connect.rs` (good) vs `app/ratel/src/features/auth/interop/web.rs` (legacy, pending migration).

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

### Per-component `style.css` and `document::Stylesheet` in components

```rust
// BAD — per-component stylesheet causes FOUC on SPA route changes
#[component]
pub fn MyComponent() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("./style.css") }
        // ...
    }
}

// BAD — preload + stylesheet Link pair for the same CSS asset
document::Link { rel: "preload", href: asset!("./style.css"), r#as: "style" }
document::Link { rel: "stylesheet", href: asset!("./style.css") }

// BAD — Google Fonts (or any external CSS) declared per-page
#[component]
pub fn MyPage() -> Element {
    rsx! {
        document::Link { rel: "preconnect", href: "https://fonts.googleapis.com" }
        document::Stylesheet { href: "https://fonts.googleapis.com/css2?family=Outfit..." }
        // ...
    }
}

// GOOD — append the rules to app/ratel/assets/main.css and let app.rs
// load it once globally; component RSX has no document::Stylesheet at all.
#[component]
pub fn MyComponent() -> Element {
    rsx! {
        // No document::Stylesheet here — main.css is loaded globally in app.rs.
        div { class: "my-component", /* ... */ }
    }
}
```

```css
/* app/ratel/assets/main.css */
/* === src/features/<module>/pages/<page>/<component> === */
.my-component {
  --comp-bg: var(--dark, #0c0c1a) var(--light, #ffffff);
  background: var(--comp-bg);
}
```

Why this is an anti-pattern:
- Per-component stylesheets loaded via `document::Stylesheet` / `document::Link` **unload during SPA route changes**, causing flashes of unstyled content (FOUC). Full-page reloads don't hit this because the server-rendered HTML already has every stylesheet in `<head>`.
- The single global `main.css` loaded from `app.rs` stays in `<head>` for the entire session — no remount, no re-fetch, no FOUC.
- External stylesheets (Google Fonts, Tiptap themes, etc.) follow the same rule: declare them **once in `app.rs`**, not per-page.
- Allowed exceptions: `app.rs` itself (loads `main.css`, `dx-components-theme.css`, `tailwind.css`, Google Fonts, favicon), and `seo_meta/mod.rs` (`rel: "canonical"`).
- See `conventions/styling.md` § "All custom CSS lives in `app/ratel/assets/main.css`" for full rules.

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
