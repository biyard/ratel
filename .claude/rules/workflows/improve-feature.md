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

## Step 4: Design Frontend UI (if UI changes needed)
- Design UI layout, component hierarchy, and visual style before implementation
- **References**: conventions/styling.md, conventions/figma-design-system.md, conventions/design-system-guide.md
- **Skills**: frontend-design, figma:figma-implement-design

## Step 5: Implement Changes
- Modify existing controllers, models, components as needed
- **References**: conventions/dynamodb-patterns.md, conventions/dioxus-app.md, conventions/styling.md, conventions/error-handling.md, conventions/i18n.md, conventions/design-system-guide.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 6: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 7: Test
- Update or write e2e tests
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer
