# UI Design & Implementation

Workflow for designing new UI pages/components or redesigning existing ones. Follows an HTML-first approach: design visually in HTML/CSS/JS, then convert to Dioxus RSX.

## When to Use

- Creating a new page or major UI component from scratch
- Redesigning an existing page with significant visual changes
- Building game-like, immersive, or non-standard UI (arena, portal, HUD overlays)
- Any UI work where visual design should be validated before Dioxus implementation

## Step 1: Understand Requirements & Explore

- Read the spec/issue/requirements
- Explore existing components and pages in the target area
- Identify available data from server (response DTOs, hooks, context providers)
- **References**: conventions/project-structure.md, conventions/feature-module-structure.md

## Step 2: Design Direction

- Discuss visual direction with the user (aesthetic, layout, interactions)
- **Skills**: superpowers:brainstorming, frontend-design

## Step 3: Create Standalone HTML Mockup

- Create a single-file HTML mockup in `docs/design/<feature>.html` for user review
- Include inline CSS and JS for self-contained preview
- Use realistic placeholder data matching actual API response fields
- Add interactive states (hover, active, panel toggles) so the user can evaluate UX
- Iterate with the user until the design is approved

## Step 4: Separate HTML, CSS, JS

Once approved, split into three files in the component directory:

```
app/ratel/src/features/<module>/pages/<page>/
├── page.html      # Structural HTML only (kept as reference)
├── style.css      # All styling extracted from the mockup
├── script.js      # JS helpers registered on window.ratel.<module>
├── mod.rs
├── component.rs
└── i18n.rs
```

### CSS Rules
- Use CSS classes with BEM-like naming (`.block__element--modifier`)
- Use `data-*` / `aria-*` attributes for Dioxus-controlled state (`[data-open="true"]`, `[aria-pressed="true"]`)
- Use CSS classes (`.active`, `.open`) for JS-controlled state (scroll position, carousel)
- Avoid Tailwind utilities in CSS files — Tailwind is for RSX `class:` attributes only
- Keep animations and transitions in CSS
- Use space toggle for dark/light: `var(--dark, #0c0c1a) var(--light, #ffffff)`

### JS Rules
- Register helpers on `window.ratel.<module>` namespace
- Keep JS minimal — prefer Dioxus signals/events for state management
- JS is for DOM manipulation that can't be done via RSX attributes (e.g., scroll-based active card detection, animations)
- Use `classList.toggle` for JS state, not `setAttribute('data-*')` (Dioxus re-renders overwrite attributes)

### Critical: Class Name Consistency
- **Keep the exact same CSS class names and element IDs** from the HTML mockup when converting to RSX
- JS queries elements by these names — renaming breaks selectors silently
- If mockup uses `.carousel-track`, RSX must use `class: "carousel-track"`, not `class: "action-carousel__track"`

## Step 5: Convert HTML to RSX

```bash
dx translate -f <path>/page.html
```

- Copy the RSX output into `component.rs`
- **Keep all class names and IDs identical** to the HTML mockup — JS depends on them
- Replace hardcoded text with `translate!` macro references
- Replace static content with data from hooks/props

## Step 6: Build Dioxus Component

Structure the page as a parent component with sub-components in named subdirectories. See `conventions/html-first-components.md` for the full file structure pattern.

### Key patterns
- Each sub-component gets its own directory with `mod.rs`, `component.rs`, `style.css`
- Each sub-component loads its own `style.css` via `document::Link { href: asset!("./style.css") }`
- Always use `document::Script { defer: true, src: asset!("./script.js") }` — defer is required
- Parent `mod.rs` declares and re-exports sub-component modules
- i18n is shared at the page level — sub-components import from the parent
- Extract a sub-component when it is self-contained (panel, modal, card) and > ~50 lines of RSX
- Dioxus-controlled state → `data-*` / `aria-*` attributes
- JS-controlled state (scroll, animations) → CSS classes (`.active`) via `classList.toggle`
- **Reproduce the HTML mockup faithfully** — specialized card content per action type, unique detail sections, type-specific colors/icons must all be translated, not simplified
- **References**: conventions/html-first-components.md

## Step 7: Implement Logic & Event Handlers

- Replace static content with data from hooks (`use_space()`, `use_space_role()`, etc.)
- Add Dioxus signals for UI state (panel open/close, active tab, etc.)
- Wire event handlers (`onclick`, `onchange`) to signals and server calls
- Use existing patterns: `use_popup()` for modals, `LoginModal` for auth, `use_theme()` for theme switching
- **References**: conventions/dioxus-app.md, conventions/i18n.md, conventions/anti-patterns.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 8: Create i18n Translations

- Define `translate!` block in `i18n.rs` with EN and KO strings
- All user-facing text must use the `translate!` macro
- **References**: conventions/i18n.md

## Step 9: Register Module

- Add `pub mod <page>;` in the parent `mod.rs`
- Wire into route if this is a routable page

## Step 10: Lint & Format

```bash
rustywind --custom-regex "class: \"(.*)\"" --write <file>.rs
dx fmt -f <file>.rs
```

- **References**: conventions/lint-and-format.md

## Step 11: Verify Build

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --web --features web
```

- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion
