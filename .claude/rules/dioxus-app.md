---
globs: ["app/ratel/**/*.rs"]
---

# Dioxus App Rules

Rules for working with the Dioxus fullstack app in `app/ratel/`.

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

## Placeholder/Empty State Styling

Use visually distinct styling for placeholder states (e.g., `text-foreground-muted italic`) — never same style as normal content.

## Breakpoints

```
max-mobile: <= 500px
md:         >= 768px
desktop:    >= 1177px
```

## Feature Gating

- Server-only code: `#[cfg(feature = "server")]`
- Web-only code: `#[cfg(not(feature = "server"))]` or `#[cfg(feature = "web")]`
- Membership fields: `#[cfg(feature = "membership")]`

## Auth Context & Membership

- `use_user_membership()` hook returns `Option<UserMembershipResponse>` — lazy-loads from server
- `is_paid()` checks `!tier.0.contains("Free")`
- Tiers: Free, Pro, Max, Vip, Enterprise(String)

## Views

Every page view should include `SeoMeta { title: "..." }` and use `translate!` for all strings.
