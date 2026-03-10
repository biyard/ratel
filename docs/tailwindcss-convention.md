# TailwindCSS Convention Guide

This document defines the TailwindCSS conventions for the Ratel Dioxus fullstack application. All code reviews should verify compliance with these rules.

---

## 1. Configuration Overview

- **Version:** TailwindCSS v4 with `@import "tailwindcss"`
- **Entrypoint:** `app/ratel/tailwind.css`
- **Source scanning:** `@source "./src/**/*.{rs,css}"`
- **Theme:** Attribute-based (`data-theme="light"` / dark default) — NOT media-query-based
- **Custom variant:** `@custom-variant light (&:where([data-theme=light], [data-theme=light] *))` for light theme overrides
- **Fonts:** Raleway (primary), Inter (secondary) via Google Fonts

---

## 2. Design Tokens (Semantic Color Naming)

Always use semantic token classes instead of raw color values.

### Naming Pattern: `<property>-<category>-<attribute>`

**Text:**

| Class | Usage |
|-------|-------|
| `text-text-primary` | Main body text |
| `text-text-secondary` | Secondary/muted text |
| `text-text-tertiary` | Tertiary/hint text |
| `text-card-meta` | Card metadata |
| `text-web-font-primary` | Web-specific primary text |
| `text-web-font-neutral` | Web-specific neutral text |
| `text-popover-foreground` | Popover text |

**Backgrounds:**

| Class | Usage |
|-------|-------|
| `bg-card` | Card background |
| `bg-card-bg` | Legacy card background |
| `bg-component-bg` | Component background |
| `bg-popover` | Popover background |
| `bg-primary` | Brand primary (golden yellow) |
| `bg-primary/10` | Primary with 10% opacity |

**Borders:**

| Class | Usage |
|-------|-------|
| `border-separator` | Divider lines |
| `border-border` | Default border |
| `border-subtle` | Subtle border |
| `border-card-border` | Card border |
| `border-primary` | Brand primary border |

**Buttons (full state chain):**

| Class | Usage |
|-------|-------|
| `bg-btn-primary-bg` | Button default bg |
| `text-btn-primary-text` | Button default text |
| `border-btn-primary-outline` | Button default border |
| `hover:bg-btn-primary-hover-bg` | Button hover bg |
| `disabled:bg-btn-primary-disable-bg` | Button disabled bg |

Replace `primary` with `secondary` or `outline` for other button styles.

### Rules

- **DO:** Use semantic tokens — `bg-card`, `text-text-primary`, `border-separator`
- **DON'T:** Use raw colors — `bg-neutral-800`, `text-gray-400`, `border-gray-700`
- **Exception:** When no semantic token exists and you need a one-off value, prefer creating a token in `tailwind.css` over using a raw color

---

## 3. Theme Handling (Light / Dark)

The project uses attribute-based theming with `data-theme`, NOT Tailwind's default media-query `dark:` mode.

```rust
// CORRECT: Use the custom `light:` variant for light theme overrides
div { class: "bg-card light:bg-white text-text-primary light:text-neutral-900" }

// WRONG: Do not use `dark:` — dark is the default, no prefix needed
div { class: "dark:bg-card bg-white" }  // WRONG
```

**Rules:**
- Dark theme is the default — no prefix needed
- Use `light:` variant for light theme overrides
- Do NOT use `dark:` — it uses media queries which conflict with the attribute-based system

---

## 4. Responsive Breakpoints

Custom breakpoints are defined in the theme:

| Breakpoint | Width | Prefix |
|------------|-------|--------|
| Desktop | 1177px | `desktop:` |
| Tablet | 900px | `tablet:` / `max-tablet:` |
| Mobile | 500px | `mobile:` / `max-mobile:` |
| Small Mobile | 380px | `sm-mobile:` |

**Rules:**
- Use `max-tablet:` and `max-mobile:` for responsive overrides (most common)
- Example: `div { class: "flex flex-row max-tablet:flex-col gap-5 max-mobile:gap-3" }`
- Container widths: `max-w-desktop`, `max-w-tablet`

---

## 5. Conditional Styling: `aria-selected` Over if/else Classes

Prefer `aria-selected` with Tailwind variants over Rust if/else class selection. This is both more accessible and more concise.

### Before (anti-pattern)

```rust
let card_class = if selected {
    "border-primary bg-primary/10"
} else {
    "border-border hover:border-text-tertiary"
};

button { class: "{card_class}" }
```

### After (preferred)

```rust
button {
    "aria-selected": selected,
    class: "border-border hover:border-text-tertiary aria-selected:border-primary aria-selected:bg-primary/10",
}
```

### Parent-Child: `group-aria-selected`

Use `group` on the parent and `group-aria-selected:` on children to style child elements based on parent selection state:

```rust
button {
    class: "group flex items-center gap-4",
    "aria-selected": selected,

    // Child radio indicator styled by parent's aria-selected
    div { class: "border-2 border-text-tertiary group-aria-selected:border-primary" }
}
```

### Rules

- Use `aria-selected` for selection states (tabs, option cards, radio-like selectors)
- Add `group` class on parent, use `group-aria-selected:` on children
- This also applies to other ARIA attributes: `aria-invalid:`, `aria-disabled:`, `aria-expanded:`
- Avoid creating multiple Rust variables for selected/unselected class strings

---

## 6. SVG Icon Styling

Icons are styled via arbitrary CSS selectors targeting SVG child elements:

```rust
icons::internet_script::Internet {
    class: "w-5 h-5 [&>path]:stroke-primary [&>circle]:stroke-primary"
}

icons::security::Lock1 {
    class: "w-5 h-5 [&>path]:stroke-primary [&>rect]:stroke-primary [&>circle]:stroke-primary"
}
```

**Common selectors:**
- `[&>path]:stroke-*` — stroke color on `<path>` elements
- `[&>path]:fill-*` — fill color on `<path>` elements
- `[&>circle]:stroke-*`, `[&>rect]:stroke-*`, `[&>ellipse]:stroke-*`, `[&>line]:stroke-*`

**Hover with group:**
```rust
div { class: "group",
    icons::Icon { class: "[&>path]:stroke-icon-primary group-hover:[&>path]:stroke-primary" }
}
```

---

## 7. Component Variant Pattern (strum Enums)

Reusable components use `strum::Display` enums to map variants to Tailwind class strings:

```rust
#[derive(Default, strum::Display)]
pub enum ButtonSize {
    #[default]
    #[strum(serialize = "py-3 px-5 text-[14px]/[16px] font-bold")]
    Medium,

    #[strum(serialize = "p-1 text-[14px]/[14px] font-medium")]
    Icon,
}
```

Usage in RSX:

```rust
button {
    class: "{size} {style} {shape} {class}",  // Enum Display + custom class prop
    ..attributes,
    {children}
}
```

**Rules:**
- Components should accept a `class: String` prop for consumer customization
- Use enum variants for predefined styles (size, style, shape)
- Combine with `{class}` in the template for consumer overrides

---

## 8. Font Usage

| Token | Font | Usage |
|-------|------|-------|
| `font-raleway` | Raleway | Primary — headings, labels, body text |
| `font-inter` | Inter | Secondary — UI elements |

**Weight classes:** `font-medium`, `font-semibold`, `font-bold`

**Typography pattern for headings:**

```rust
p { class: "font-bold font-raleway text-[24px]/[28px] tracking-[-0.24px] text-web-font-primary" }
```

**Typography pattern for body text:**

```rust
p { class: "font-normal leading-6 font-raleway text-[15px] tracking-[0.5px] text-card-meta" }
```

---

## 9. Common Anti-Patterns

### 9.1. Excessive `!important` overrides

```rust
// BAD: Overriding component styles with !important
class: "!bg-neutral-800 !text-white !px-5 !py-3 !rounded-[8px]"

// GOOD: Use the component's class prop or style variant
Button {
    style: ButtonStyle::Text,
    class: "bg-neutral-800 text-white",
}
```

**Rule:** If you need `!important`, the component likely needs a refactor to accept a `class` prop or additional style variants.

### 9.2. Inline styles for fixed values

```rust
// BAD: Inline style for a fixed value
div { style: "background-color: #191919;" }

// GOOD: Use a Tailwind class or design token
div { class: "bg-card" }
```

**Exception:** Inline styles are acceptable for dynamic values that cannot be known at build time:

```rust
// OK: Dynamic width percentage
div { style: format!("width: {}%", progress) }
```

### 9.3. Raw color values instead of tokens

```rust
// BAD
div { class: "bg-neutral-800 text-gray-400 border-gray-700" }

// GOOD
div { class: "bg-card text-text-secondary border-separator" }
```

### 9.4. Using `dark:` instead of `light:`

```rust
// BAD: media-query based dark mode
div { class: "bg-white dark:bg-card" }

// GOOD: attribute-based theming (dark is default)
div { class: "bg-card light:bg-white" }
```

### 9.5. Rust if/else for styling that can use ARIA variants

```rust
// BAD: Multiple class variables
let cls = if active { "bg-primary text-white" } else { "bg-card text-gray-400" };
button { class: cls }

// GOOD: Single class string with aria variant
button {
    "aria-selected": active,
    class: "bg-card text-gray-400 aria-selected:bg-primary aria-selected:text-white",
}
```

---

## Quick Reference

| Concern | Convention |
|---------|-----------|
| Colors | Semantic tokens (`bg-card`, `text-text-primary`), not raw values |
| Theme | `light:` variant for light overrides; dark is default, no prefix |
| Responsive | `max-tablet:`, `max-mobile:` for responsive overrides |
| Selection state | `aria-selected:` + `group-aria-selected:` variants |
| Icon colors | `[&>path]:stroke-*` arbitrary selectors |
| Component variants | `strum::Display` enums for class strings |
| Font | `font-raleway` (primary), `font-inter` (secondary) |
| Avoid | `!important`, inline styles, raw colors, `dark:` prefix, Rust if/else for class toggle |
