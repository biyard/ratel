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

Guard all `wasm_bindgen` calls with `#[cfg(not(feature = "server"))]` — JS is unavailable during SSR.

Three layers: JS source → register on `window.ratel.<module>` → Rust `#[wasm_bindgen(js_namespace = [...])]`.

```rust
#[wasm_bindgen(js_namespace = ["window", "ratel", "common", "theme"])]
extern "C" {
    pub fn load_theme() -> Option<String>;
}
```

Namespace must match exactly. JS files in `app/ratel/assets/` for `asset!()` macro.

## Accessibility

- `alt` on all `img` elements
- `aria-label` on icon-only buttons
- Use `Link { to: Route::... }` for navigation, not `div { onclick: navigator.push() }`
