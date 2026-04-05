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
