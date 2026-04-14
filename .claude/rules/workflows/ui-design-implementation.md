# UI Design & Implementation / Mockup UI

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

- Create a single-file HTML mockup in `app/ratel/assets/design/<feature>.html` for user review
- Include inline CSS and JS for self-contained preview
- Use realistic placeholder data matching actual API response fields
- Add interactive states (hover, active, panel toggles) so the user can evaluate UX
- Iterate with the user until the design is approved
