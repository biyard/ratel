# Improve an Existing Feature

## Step 1: Understand Requirements
- Read the spec/issue/requirements
- Explore existing implementation
- **References**: conventions/project-structure.md

## Step 2: Design Changes
- **Skills**: superpowers:brainstorming, superpowers:writing-plans

## Step 3: Design Event Bridge (if needed)
- Determine if the change includes processing that is not time-critical or requires queue-based processing for heavy tasks
- If yes, design an EventBridge integration
- **References**: conventions/implementing-event-bridge.md

## Step 4: Design & Implement Frontend UI (if UI changes needed)
- For major visual redesigns or new pages → follow `workflows/ui-design-implementation.md` (HTML-first approach)
- For minor UI tweaks using existing components → implement directly in RSX
- **References**: conventions/html-first-components.md, conventions/styling.md, conventions/figma-design-system.md, conventions/design-system-guide.md
- **Skills**: frontend-design, figma:figma-implement-design

## Step 5: Implement Changes
- Modify existing controllers, models, components as needed
- **References**: conventions/server-functions.md, conventions/dynamodb-patterns.md, conventions/dioxus-app.md, conventions/styling.md, conventions/error-handling.md, conventions/i18n.md, conventions/design-system-guide.md, conventions/html-first-components.md, conventions/anti-patterns.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 6: Write/Update Server Function Tests
- Add or update integration tests in `app/ratel/src/tests/<feature>_tests.rs`
- Test success, error, and unauthenticated cases for new/changed endpoints
- **References**: conventions/server-function-tests.md

## Step 7: Lint & Format
- **References**: conventions/lint-and-format.md

## Step 8: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 9: Test
- Update or write e2e tests
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer
