# Improve an Existing Feature (Stage 3, enhancement path)

Apply when a feature already exists in the codebase and needs enhancement — not a brand-new roadmap item, but a meaningful change to scope, behavior, or UX.

## Prerequisites

- `roadmap/{roadmap-name}.md` spec exists and captures the **new/changed** requirements (Stage 1)
  - If this is the first change after initial shipping, update the original roadmap file; don't fork a new one
  - If the change is large enough to warrant its own roadmap item (e.g., "Cross-posting v2: threading"), create a new roadmap file with its own slug
- If UI changes are involved, approved mockups exist at `app/ratel/assets/design/{roadmap-name}/` (Stage 2)

If requirements aren't clear yet, go to `workflows/roadmap-elaboration.md`. If UI direction isn't settled, go to `workflows/ui-design-implementation.md`.

## Step 1: Read the updated spec + existing code

- Read `roadmap/{roadmap-name}.md` — focus on what changed
- Read the existing feature module under `app/ratel/src/features/` — understand current data model, controllers, hook
- Identify the **minimal surface area** of the change
- **References**: conventions/project-structure.md

## Step 2: Write (or update) the system design doc

Create or amend the design doc at:

```
docs/superpower/{YYYY-MM-DD}-{roadmap-name}.md
```

- If this is the first design doc for the feature, follow the full template in `workflows/develop-a-new-feature.md` Step 2
- If a prior doc already exists, write a new dated doc that references the prior one and focuses on **what's changing** — don't restate unchanged architecture

Required sections for an enhancement doc:
- Summary of the change (1-2 sentences)
- What's changing (data model / API / UI / events) and what stays the same
- Migration plan if data model changes (backfill? in-place? new entity?)
- Test plan — which existing tests break, what new tests are needed

## Step 3: Design Event Bridge changes (if applicable)

- If the enhancement involves new async processing, design the integration
- **References**: conventions/implementing-event-bridge.md

## Step 4: Update the UI (if needed)

- For major visual changes → ensure `app/ratel/assets/design/{roadmap-name}/` has the updated mockups
- For minor UI tweaks → implement directly in RSX, preserving existing class names / testids
- **References**: conventions/html-first-components.md, conventions/styling.md, conventions/design-system-guide.md
- **Skills**: frontend-design, figma:figma-implement-design

## Step 5: Implement changes

- Modify existing controllers, models, components per the design doc
- For new mutations or data fetches, **extend the feature's `UseFeatureName` controller hook** — don't call server `_handler`s directly from components
- Remove dead code paths the enhancement obsoletes
- **References**: conventions/server-functions.md, conventions/dynamodb-patterns.md, conventions/dioxus-app.md, conventions/hooks-and-actions.md, conventions/styling.md, conventions/error-handling.md, conventions/i18n.md, conventions/anti-patterns.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 6: Write / update server function tests

- Add or update integration tests in `app/ratel/src/tests/<feature>_tests.rs`
- Cover success, error, and unauthenticated cases for new/changed endpoints
- Delete tests for removed behavior
- **References**: conventions/server-function-tests.md

## Step 7: Lint & format

- **References**: conventions/lint-and-format.md

## Step 8: Verify build

- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 9: Extend Playwright coverage

- For every new/changed acceptance criterion in the roadmap spec, add a Playwright step
- **Prefer extending an existing scenario** — add new `test()` blocks into the existing serial suite that covers the feature's flow
- **Create a new spec file** only if the enhancement introduces a wholly new flow
- Update or remove obsolete assertions in existing tests
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer

## Step 10: Close the loop

- Mark newly verified acceptance criteria `- [x]` in `roadmap/{roadmap-name}.md`
- Update `ROADMAP.md` if the overall item's status changed
- Record deviations from the plan in `docs/superpower/{date-roadmap-name}.md`

## Rules

- **Same roadmap slug, same design dir, same feature module.** Don't fork paths unless the change is big enough to be a new roadmap item.
- **Dated design docs stack.** A feature can have multiple `docs/superpower/{date}-{name}.md` files over time — newer ones supersede older. Don't delete history.
- **Prefer extending tests over replacing them.** Regression coverage is valuable.
