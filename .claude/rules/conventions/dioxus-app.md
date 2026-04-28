---
globs: ["app/ratel/**/*.rs"]
---

# Dioxus App Conventions

## Component Structure Pattern

```rust
use crate::common::*;

#[component]
pub fn MyComponent(
    #[props(default)] class: String,
    #[props(default)] variant: MyVariant,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "base-classes {variant} {class}",
            ..attributes,
            {children}
        }
    }
}

#[derive(Default, strum::Display, strum::EnumString)]
pub enum MyVariant {
    #[default]
    #[strum(serialize = "tailwind-classes-for-default")]
    Default,
}
```

- `#[props(default)]` for optional props
- `#[props(extends = GlobalAttributes)]` + `..attributes` for HTML attribute passthrough
- Enum variants serialize to Tailwind class strings via `strum::Display`

## Icons

- Custom: `crate::common::icons::<category>::<IconName> { class: "..." }`
- Lucide: `lucide_dioxus::<IconName> { class: "..." }`
- Color via: `[&>path]:stroke-icon-primary`
- Do NOT install new icon packages

## Assets

```rust
pub const MY_ASSET: Asset = asset!("/assets/filename.ext");
// In RSX: img { src: MY_ASSET }
```

## Views

Every page view should include `SeoMeta { title: "..." }` and use `translate!` for all strings.

## Auth Context & Membership

- `use_user_membership()` hook returns `Option<UserMembershipResponse>` — lazy-loads from server
- `is_paid()` checks `!tier.0.contains("Free")`
- Tiers: Free, Pro, Max, Vip, Enterprise(String)

## Feature Hooks & Actions

Every feature with interactive state exposes a `UseFeatureName` controller hook that bundles its signals, loaders, queries, and actions. Components consume the hook — never the server-function `_handler` directly.

- Mutations must be wrapped in `use_action(...)` and placed inside the controller. Components call them via `handle.call(input)`.
- Controller hook signature: `pub fn use_feature() -> std::result::Result<UseFeature, RenderError>` — context-cached via `try_use_context()` + `provide_root_context(...)`.
- `Action::call(&mut self)` takes a mutable borrow, so destructure action fields as `mut handle_xxx`.
- **See `conventions/hooks-and-actions.md` for full rules, examples, and folder layout.**

Reference implementation: `app/ratel/src/features/notifications/hooks/use_inbox.rs`.

## Data Loading with `use_loader`

Prefer `use_loader` over `use_server_future` for loading server data. `use_loader` returns a `Loader<T>` which requires `T: PartialEq`.

```rust
// Single async call — use_loader with closure
let resource = use_loader(move || async move {
    get_my_handler(space_id()).await
})?;
let data = resource();  // returns Result<T>

// Reactive prop — accept ReadSignal<T> so signal is Copy (no clone needed)
fn MyComponent(space_id: ReadSignal<SpacePartition>) -> Element {
    let resource = use_loader(move || async move {
        get_handler(space_id(), None).await
    })?;
}
```

- Response types must derive `PartialEq` (required by `Loader<T>`)
- Access data with `resource()` — not `.read()`
- Accept `ReadSignal<T>` props when used only in loaders — avoids `use_reactive` + `.clone()`

## Pagination with `use_infinite_query`

- Prefer over `use_server_future` for any list that may exceed one page
- Always render `{v.more_element()}` at end of list container
- Make `v` mutable: `let mut v = use_infinite_query(...)`
- Filter server-side when possible — client-side filtering after paginated fetch causes edge cases
- Hard-cap server-side DynamoDB scanning loops (`max_pages = 5`)

## Scroll Event Handlers

Never spawn unbounded async tasks from `onscroll`. Use trailing-edge throttle with `scroll_check_pending` signal guard.

## Dioxus Reactivity

- `use_effect` only re-runs when reactive signals are read **inside** the closure
- Event handlers: `onscroll: move |_| { ... }` — no outer brace wrapping needed

## Navigation with `use_navigator`

Use `use_navigator()` for programmatic navigation (after async operations, conditional redirects, etc.):

```rust
let nav = use_navigator();

// Push — adds to history stack (user can go back)
nav.push(Route::SpaceDashboardPage { space_id });

// Replace — replaces current entry (no back navigation)
nav.replace(Route::PostDetailPage { post_id });
```

- **Always use `Route` enum variants**, not format strings — ensures compile-time route validation
- Use `nav.push()` for normal navigation (post-creation redirects, menu clicks)
- Use `nav.replace()` when the current page should not remain in history (e.g., after edit → view)
- For static links in RSX, prefer `Link { to: Route::... }` over `div { onclick: nav.push() }` (accessibility)
- Place navigation **after** all `.await` points — Dioxus drops the future if the component unmounts mid-await

## Async Event Handlers

Never call `popup.close()` or navigate away before `.await` points — Dioxus drops the future when the component unmounts. Move unmounting actions after all awaits.

## JS Interop

Two patterns exist for calling JavaScript from Dioxus. **Prefer the `dioxus::document::eval` channel pattern.** The `wasm_bindgen` extern pattern is treated as an anti-pattern for new code and should be migrated when touched.

Both patterns share the same JS source contract: helpers are registered on the `window.ratel.<module>` namespace from a JS file loaded via `asset!()` or bundled into the page.

### Preferred: `dioxus::document::eval` channel

Each JS call gets its own tiny driver script in a sibling `web/` directory. The script reads args via `dioxus.recv()` and returns results via `dioxus.send(...)`. The Rust side calls `dx_eval(include_str!("web/<name>.js"))` and exchanges JSON over the resulting handle.

Reference: `app/ratel/src/features/auth/interop/wallet_connect.rs` + `app/ratel/src/features/auth/interop/web/`.

```rust
use dioxus::document::eval as dx_eval;

pub async fn wallet_sign_message(message: &str) -> crate::common::Result<String> {
    let mut runner = dx_eval(include_str!("web/wc_sign_message.js"));
    runner
        .send(serde_json::json!(message))
        .map_err(|_| AuthError::WalletConnectFailed)?;
    runner
        .recv::<Option<String>>()
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

Why this is preferred:
- **No `wasm_bindgen` / `web-sys` / `js-sys` / `serde-wasm-bindgen` plumbing** — args/returns are plain JSON
- **Compiles on every target** — the `dioxus::document::eval` runner is a no-op outside web; no per-platform `cfg` gates at the call site
- **One JS file per call** — the Rust signature and the JS body live next to each other and can change together
- **No global `extern "C"` block** that has to stay in lockstep with a JS namespace path
- **Failure modes are explicit** — `runner.recv::<T>()` returns `Result`, no opaque `JsValue` to downcast

Notes:
- The runner is async by nature, so even synchronous-looking calls (e.g. `wallet_is_connected`) become `async fn`
- Always serialize the request body with `serde_json::json!(...)` and deserialize the response with `runner.recv::<T>()` where `T: Deserialize`
- Use `Option<T>` for fallible JS calls and have the JS catch + `dioxus.send(null)` on failure — keeps error mapping in one place

### Anti-pattern: direct `wasm_bindgen` extern bindings

Direct object binding via `#[wasm_bindgen(js_namespace = [...])] extern "C" { ... }` is an anti-pattern.

```rust
// BAD — extern block forces wasm_bindgen / web-sys / serde-wasm-bindgen
// onto the dependency graph, breaks non-web targets without cfg gates,
// and couples the Rust signature to a global JS namespace path that
// drifts silently when the JS module is renamed.
#[wasm_bindgen(js_namespace = ["window", "ratel", "auth", "wallet"])]
extern "C" {
    #[wasm_bindgen(js_name = signMessage)]
    fn wallet_sign_message_promise(message: &str) -> Promise;
}

pub async fn wallet_sign_message(message: &str) -> crate::common::Result<String> {
    let js_value = JsFuture::from(wallet_sign_message_promise(message))
        .await
        .map_err(|_e| AuthError::WalletConnectFailed)?;
    js_value
        .as_string()
        .ok_or_else(|| AuthError::WalletConnectFailed.into())
}
```

Problems:
- Direct object binding via `#[wasm_bindgen(js_namespace = [...])]` is a **compile-time** contract against runtime JS — a typo or rename only fails at JS load time, not at `cargo check`
- `extern "C"` blocks must be `#[cfg(not(feature = "server"))]`-gated everywhere, and wrappers around them must be gated again at every call site
- Forces `JsFuture::from(...)` + `serde_wasm_bindgen::from_value(...)` boilerplate at every call
- Pulls `wasm_bindgen` / `wasm-bindgen-futures` / `web-sys` / `js-sys` / `serde-wasm-bindgen` into the dependency graph just to bridge two JSON values

When you encounter an existing `#[wasm_bindgen] extern "C"` block, treat it as legacy and migrate it to the channel pattern when you touch the surrounding code. Reference migration: `wallet_connect.rs` (after) vs `web.rs` (before, still pending migration).

### Layout

```
features/<module>/interop/
├── mod.rs                 # pub use of platform/web modules
├── <topic>.rs             # Rust API: pub async fn that calls dx_eval
└── web/
    ├── <topic>_init.js
    ├── <topic>_<action_a>.js
    └── <topic>_<action_b>.js
```

One JS file per call keeps `include_str!` references stable and lets each driver have its own try/catch.

## Accessibility

- `alt` on all `img` elements
- `aria-label` on icon-only buttons
- Use `Link { to: Route::... }` for navigation, not `div { onclick: navigator.push() }`
