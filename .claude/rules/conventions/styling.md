---
globs: ["app/ratel/**/*.rs", "app/ratel/tailwind.css", "app/ratel/assets/tailwind-colors.css"]
---

# Styling Conventions

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

## Rule: All component CSS lives in `app/ratel/assets/main.css`

All custom CSS is centralized in a single `main.css` loaded once globally from `app.rs`. Do NOT create per-component `style.css` files and do NOT add `document::Link { rel: "stylesheet", href: asset!("./style.css") }` in components.

**Why**: Per-component CSS loaded via `document::Link` unloads during route transitions, causing flashes of unstyled content. A single globally-loaded `main.css` avoids this entirely.

**How to apply**:

- Add new component styles by appending to `app/ratel/assets/main.css` with a section marker comment:
  ```css
  /* === features/<module>/pages/<page>/<component> === */
  .my-component { ... }
  ```
- Keep selectors unique (use BEM-like names or component-scoped prefixes) — everything shares one global namespace.
- Prefer Tailwind utility classes inline in RSX. Only use `main.css` for things Tailwind cannot express cleanly (complex keyframes, pseudo-element tricks, state-machine `data-*` selectors).
- Never re-introduce per-component `style.css` + `document::Link`. If you see an existing one, migrate it to `main.css`.

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

### Space Toggle Pattern (for CSS in `main.css`)

When writing CSS rules in `main.css`, use the space toggle for dark/light colors:

```css
.my-component {
  --comp-bg: var(--dark, #0c0c1a) var(--light, #ffffff);
  --comp-text: var(--dark, #f0f0f5) var(--light, #12121a);
  --comp-border: var(--dark, rgba(255,255,255,0.06)) var(--light, rgba(0,0,0,0.08));

  background: var(--comp-bg);
  color: var(--comp-text);
}
```

This leverages `--dark` / `--light` custom properties from `dx-components-theme.css`. Never hardcode colors in component CSS — always provide both dark and light values.

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
