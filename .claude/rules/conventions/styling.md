---
globs: ["app/ratel/**/*.rs", "app/ratel/tailwind.css", "app/ratel/assets/tailwind-colors.css"]
---

# Styling Conventions

## Rule: All custom CSS lives in `app/ratel/assets/main.css`

There is exactly **one** stylesheet for component CSS in the app: `app/ratel/assets/main.css`. It is loaded once globally from `app.rs` and is the only place new component styles may go.

### Why
- Per-component `style.css` files loaded via `document::Stylesheet { href: asset!("./style.css") }` unload during route/component transitions, causing **flashes of unstyled content (FOUC)** on SPA navigation.
- On 2026-04-19 (and again on 2026-04-30) all per-component stylesheets were consolidated into `main.css`. Reintroducing the per-component pattern brings the FOUC bug back.
- Full-page reloads don't hit the bug because the server-rendered HTML already has every stylesheet in `<head>`. SPA route changes do — they re-render only the matched route, so per-component `<link>` tags get torn down and re-fetched.

### How to apply

1. **Append new styles to `app/ratel/assets/main.css`** under a section marker showing the source path:

   ```css
   /* === src/features/<module>/pages/<page>/<component> === */
   .my-component { ... }
   ```

2. **Use unique class names.** Everything shares one global namespace — prefix with the component name (BEM-like) to avoid collisions: `.my-component__title`, `.my-component--active`.

3. **Prefer Tailwind utility classes in RSX first.** Only fall back to `main.css` for what Tailwind cannot express:
   - `@keyframes` animations
   - Pseudo-element tricks (`::before`, `::after` content)
   - `data-*` / `aria-*` attribute selectors for state-driven styling
   - Complex selectors (`:has()`, sibling selectors, child counters)

4. **Never create per-component `style.css` files** under `src/`. Never write `document::Stylesheet { href: asset!("./style.css") }` in a component.

5. **External stylesheets and font links go in `app.rs`** — global once, never per-page. Google Fonts (Orbitron, Outfit), `dx-components-theme.css`, `tailwind.css`, and `main.css` are all loaded from `app.rs`. Per-page `document::Link { rel: "preconnect", ... }` blocks are an anti-pattern.

6. **If you find a stray `style.css`** or a `document::Stylesheet { href: asset!("./...style.css") }`, migrate the CSS into `main.css` under a section marker and remove the link.

### Allowed `document::Link` / `document::Stylesheet` use

| Where | What | Allowed? |
|---|---|---|
| `app.rs` | `MAIN_CSS`, `tailwind.css`, `dx-components-theme.css`, Google Fonts, favicon | ✅ Yes — global, loaded once |
| `seo_meta/mod.rs` | `rel: "canonical"` | ✅ Yes — SEO metadata, not CSS |
| Any component or page | `document::Stylesheet { href: asset!("./style.css") }` | ❌ No — causes FOUC |
| Any component or page | Google Fonts / external stylesheet `<link>` | ❌ No — hoist to `app.rs` |

## Rule: Semantic color tokens only — never Tailwind palette colors

```
BAD:  text-neutral-400, bg-slate-800, text-gray-500, bg-green-500
GOOD: text-foreground-muted, bg-card-bg, text-text-primary, bg-background
```

Never use `text-neutral-*`, `bg-gray-*`, `text-zinc-*`, etc. — they bypass the theme system.
Tailwind spacing/sizing utilities (`gap-4`, `p-5`, `w-full`) are fine.

## Rule: Always use primitive components — never raw HTML for interactive UI

Use `Button` not `button`, `Input` not `input`, `Select` not `select`, `Card` for card layouts, `Row`/`Col` for flex layout.

Available primitives (via `use crate::common::*;`):
`Accordion`, `AlertDialog`, `Avatar`, `Badge`, `Button`, `Calendar`, `Card`, `Checkbox`, `Col`, `Collapsible`, `ContextMenu`, `DatePicker`, `Dialog`, `DropdownMenu`, `FileUploader`, `Form`, `Input`, `Label`, `Popover`, `Popup`, `Progress`, `RadioGroup`, `Row`, `ScrollArea`, `Select`, `SeoMeta`, `Separator`, `Sheet`, `Sidebar`, `Skeleton`, `Slider`, `Switch`, `Tabs`, `Textarea`, `Toggle`, `ToggleGroup`, `Tooltip`

## Rule: TailwindCSS bracket syntax for arbitrary values

```
BAD:  z-101 (silently ignored)
GOOD: z-[101], w-[350px], gap-[13px]
```

Standard scale values don't need brackets: `z-10`, `gap-4`.

## Rule: Status colors must use semantic tokens

Define CSS variables in `tailwind.css` with the space toggle pattern, never use raw `bg-green-500` or `text-red-400`.

## Semantic Color Classes

| Purpose | Tailwind Class |
|---------|----------------|
| Page background | `bg-background` |
| Card background | `bg-card-bg` |
| Popover/modal bg | `bg-popover` |
| Primary text | `text-text-primary` / `text-foreground` |
| Muted text | `text-foreground-muted` |
| Primary action | `bg-primary` / `text-primary` (golden #fcb300) |
| Accent | `bg-accent` / `text-accent` (teal #6eedd8) |
| Destructive | `bg-destructive` / `text-destructive` (#db2780) |
| Border | `border-border` / `border-separator` |
| Input background | `bg-input-box-bg` |
| Input border | `border-input-box-border` |
| Icon color | `[&>path]:stroke-icon-primary` |
| Focus ring | `focus-visible:border-ring focus-visible:ring-ring/50` |
| Button primary | `bg-btn-primary-bg text-btn-primary-text` |

## Theme System

- Dark/light via `data-theme` attribute on `<html>` — no conditional rendering needed in Rust
- Custom TailwindCSS v4 variant: `light:` prefix for light-mode-only styles
- All semantic token classes automatically respond to theme switching
- Token definitions: `app/ratel/tailwind.css` + `app/ratel/assets/tailwind-colors.css`
- Component theme: `app/ratel/assets/dx-components-theme.css`

### Space Toggle Pattern (for sections inside `main.css`)

When you need a custom CSS rule in `main.css`, use the space toggle for any color that differs between dark and light themes:

```css
/* === src/features/<module>/<component> === */
.my-component {
  --comp-bg: var(--dark, #0c0c1a) var(--light, #ffffff);
  --comp-text: var(--dark, #f0f0f5) var(--light, #12121a);
  --comp-border: var(--dark, rgba(255,255,255,0.06)) var(--light, rgba(0,0,0,0.08));

  background: var(--comp-bg);
  color: var(--comp-text);
}
```

This leverages `--dark` / `--light` custom properties from `dx-components-theme.css`. Never hardcode a single color value when the color should differ between themes — always provide both dark and light values via the toggle.

## Breakpoints

```
max-mobile:   ≤ 500px   (use max-mobile: prefix)
md:           ≥ 768px   (standard Tailwind)
desktop:      ≥ 1177px
```

## Placeholder/Empty State Styling

Use visually distinct styling for placeholder states (e.g., `text-foreground-muted italic`) — never same style as normal content.

## Anti-patterns

See `conventions/anti-patterns.md` for the full list of styling, component, and navigation anti-patterns.
