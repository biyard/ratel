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
- **References**: conventions/error-handling.md

## Step 6: Design Frontend UI
- Design UI layout, component hierarchy, and visual style before implementation
- **References**: conventions/styling.md, conventions/figma-design-system.md, conventions/design-system-guide.md
- **Skills**: frontend-design, figma:figma-implement-design

## Step 7: Implement Frontend
- Create components, views, hooks, i18n
- **References**: conventions/dioxus-app.md, conventions/styling.md, conventions/i18n.md, conventions/design-system-guide.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 8: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 9: Test
- Write and run e2e tests
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer
