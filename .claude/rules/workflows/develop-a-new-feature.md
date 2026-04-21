# Develop a New Feature

## Step 1: Understand Requirements
- Read the spec/issue/requirements
- Explore existing code in the relevant area
- **References**: conventions/project-structure.md

## Step 2: Design
- **Skills**: superpowers:brainstorming, superpowers:writing-plans

## Step 3: Design Event Bridge (if needed)
- Determine if the feature includes processing that is not time-critical or requires queue-based processing for heavy tasks
- If yes, design an EventBridge integration
- **References**: conventions/implementing-event-bridge.md

## Step 4: Scaffold Feature Module
- Create module structure under `app/ratel/src/features/<module>/`
- **References**: conventions/feature-module-structure.md

## Step 5: Implement Server Functions
- Create controllers, models, error types
- **Skills**: rust-dynamodb-skill, dioxus-knowledge-patch, rust-knowledge-patch
- **References**: conventions/server-functions.md, conventions/error-handling.md, conventions/anti-patterns.md

## Step 6: Write Server Function Tests
- Add integration tests in `app/ratel/src/tests/<feature>_tests.rs`
- Register module in `app/ratel/src/tests/mod.rs`
- Test success, error, and unauthenticated cases
- **References**: conventions/server-function-tests.md

## Step 7: Build Feature Controller Hook
- Add a `UseFeatureName` controller in `app/ratel/src/features/<feature>/hooks/use_<feature>.rs` bundling every signal, loader, query, and `use_action(...)` mutation the UI needs
- Context-cache it with `try_use_context::<UseFeature>()` + `provide_root_context(...)` so a single instance is shared across the tree
- **References**: conventions/hooks-and-actions.md

## Step 8: Design & Implement Frontend UI
- For new pages or major visual redesigns → follow `workflows/ui-design-implementation.md` (HTML-first approach)
- For minor UI additions using existing components → implement directly in RSX
- Components consume the controller hook from Step 7 and call actions (`handle.call(input)`); they MUST NOT import server `_handler` functions or call them directly
- **References**: conventions/html-first-components.md, conventions/styling.md, conventions/figma-design-system.md, conventions/design-system-guide.md, conventions/dioxus-app.md, conventions/hooks-and-actions.md, conventions/i18n.md, conventions/anti-patterns.md
- **Skills**: frontend-design, figma:figma-implement-design, dioxus-knowledge-patch, rust-knowledge-patch

## Step 9: Lint & Format
- **References**: conventions/lint-and-format.md

## Step 10: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 11: Test
- Write and run e2e tests
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer
