# UI Design & Implementation (Stage 2)

Stage 2 of feature development. Turn a roadmap spec into visual UI mockups (HTML/CSS/JS) the user can review and approve before any Dioxus implementation.

## When to Use

- A roadmap spec at `roadmap/{roadmap-name}.md` is ready for design
- A significant visual redesign of an existing feature is being done
- Building game-like, immersive, or non-standard UI (arena, portal, HUD overlays)

## Prerequisites

- `roadmap/{roadmap-name}.md` exists and is marked ready (Stage 1 complete)
- If not, go to `workflows/roadmap-elaboration.md` first

## Step 1: Read the roadmap spec

- Read `roadmap/{roadmap-name}.md` end-to-end
- Extract: user stories, functional requirements, non-goals, constraints
- List every UI surface the spec implies (page, modal, panel, empty state, error state)
- **References**: conventions/project-structure.md, conventions/feature-module-structure.md

## Step 2: Decide design direction with the user

- Discuss visual direction (aesthetic, layout, interactions)
- Align with existing Ratel aesthetic (arena/glass dark theme, Orbitron + Outfit, semantic color tokens)
- Identify reusable primitives from `app/ratel/src/common/components/`
- **Skills**: superpowers:brainstorming, frontend-design

## Step 3: Scaffold the design directory

Create `app/ratel/assets/design/{roadmap-name}/` and put one file per page/component:

```
app/ratel/assets/design/{roadmap-name}/
├── {page-1}.html          # full page mockup
├── {page-2}.html
├── {component-name}.html  # standalone component if worth isolating
├── shared.css             # (optional) shared styles across files
└── shared.js              # (optional) shared JS helpers
```

Each file should be self-contained enough to render standalone (inline CSS/JS or shared file references). The `/designs/{roadmap-name}/` route serves them for review.

### File naming

- One **roadmap = one directory**, never mix roadmaps in one dir
- File name describes the UI surface: `compose-with-crosspost.html`, `social-connections.html`, `notion-connect.html`
- Sub-components of a single page go in the same directory, not nested

## Step 4: Build each HTML mockup

For each UI surface listed in Step 1:

- Use realistic placeholder data matching actual API response field names (makes Stage 3 mapping trivial)
- Include **interactive states** (hover, active, open/close, loading, error, empty) — user must be able to evaluate the full UX, not just the happy path
- Wire small demo JS so state transitions are visible (toggle panels, submit forms, open modals)
- Keep visual language consistent with existing design files — reuse color tokens, font stack, spacing
- Reference: existing designs in `app/ratel/assets/design/cross-posting/` and `app/ratel/assets/design/essence-connector/`

## Step 5: Iterate with the user

- Share the `/designs/{roadmap-name}/` URL
- Collect feedback per file
- Update until every acceptance criterion in `roadmap/{roadmap-name}.md` has a visual answer
- Do **not** start Stage 3 until the user explicitly approves the design

## Step 6: Hand off to development

Once approved:
- The design directory becomes the **visual contract** for Stage 3
- Start `workflows/develop-a-new-feature.md` (or `improve-feature.md` if extending an existing feature)
- The HTML mockups are converted to Dioxus RSX via `workflows/html-to-dioxus.md`

## Rules

- **No Dioxus code in Stage 2.** Pure HTML/CSS/JS only. Rust happens in Stage 3.
- **No backend logic in Stage 2.** Mock data inline. Backend design is Stage 3.
- **Every mockup must match a roadmap requirement.** If a file exists with no spec anchor, delete it or extend the spec.
- **Class names and element IDs used here will be preserved in the Dioxus conversion.** Pick thoughtfully — they become the contract between design and implementation.

## References

- conventions/html-first-components.md
- conventions/styling.md
- conventions/figma-design-system.md
- conventions/design-system-guide.md
