---
globs: ["app/ratel/src/**/components/**/*.rs", "app/ratel/src/**/views/**/*.rs"]
---

# Design System Guide

Guidelines for creating polished, consistent UI across the Ratel platform.

## Visual Hierarchy

- Use font size and weight to establish clear information hierarchy
- Primary content: `text-text-primary font-semibold`
- Secondary/supporting: `text-foreground-muted text-sm`
- Labels and captions: `text-foreground-muted text-xs`

## Spacing & Layout

- Use consistent spacing scale: `gap-2` (8px), `gap-3` (12px), `gap-4` (16px), `gap-6` (24px)
- Card padding: `p-4` (compact), `p-5` (normal), `p-6` (spacious)
- Section spacing: `gap-6` or `gap-8` between major sections
- Use `Row` and `Col` components for all flex layouts — never raw `div` with `flex`

## Component Composition

- Compose from primitives: `Card`, `Button`, `Badge`, `Avatar`, `Row`, `Col`
- Wrap related content in `Card` with appropriate variant
- Use `Separator` between logical sections within a card
- Group actions with consistent alignment (`MainAxisAlign::End` for action rows)

## Interactive States

- Hover: use `hover:bg-hover` or `hover:opacity-80` for subtle feedback
- Active/selected: use `aria-selected` variants over conditional class logic
- Disabled: use component `disabled` prop, not manual opacity hacks
- Focus: ensure `focus-visible:border-ring focus-visible:ring-ring/50` on interactive elements

## Empty & Loading States

- Empty states: centered text with `text-foreground-muted italic`, optionally with an icon
- Loading: use `Skeleton` component matching the expected content shape
- Error states: use `text-destructive` with clear error message

## Responsive Design

- Mobile-first: base styles for mobile, then `md:` and `desktop:` for larger screens
- Use `max-mobile:` prefix for mobile-only overrides
- Hide/show elements with `max-mobile:hidden` or `md:hidden`
- Consider touch targets: minimum 44px height for tappable elements on mobile

## Typography

- Headings: use semantic HTML (`h1`-`h4`) with appropriate text sizes
- Body text: `text-sm` (14px) default, `text-base` (16px) for emphasis
- Monospace/code: `font-mono text-xs`
- Line height: let Tailwind defaults handle it unless specific adjustment needed

## Icons

- Size icons consistently: `w-4 h-4` (inline), `w-5 h-5` (standard), `w-6 h-6` (large)
- Color with semantic tokens: `[&>path]:stroke-icon-primary`
- Icon-only buttons must have `aria-label`
- Pair icons with text labels when meaning isn't immediately obvious

## Color Usage

- Use color sparingly and purposefully — not decoratively
- Status indicators: use `Badge` with semantic `BadgeColor` variants
- Destructive actions: `bg-destructive` / `text-destructive` — reserve for delete/remove
- Success/positive: define semantic CSS variable, never raw `bg-green-500`
- Always use semantic tokens from `conventions/styling.md`

## Animation & Transitions

- Keep transitions subtle: `transition-colors duration-150` or `transition-opacity duration-200`
- Avoid layout-shifting animations (no width/height transitions on content)
- Use `animate-pulse` only for skeleton loaders
