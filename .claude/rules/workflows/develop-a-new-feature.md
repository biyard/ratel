# Develop a New Feature

## Step 1: Understand Requirements
- Read the spec/issue/requirements
- Explore existing code in the relevant area
- **References**: conventions/project-structure.md

## Step 2: Design
- **Skills**: superpowers:brainstorming, superpowers:writing-plans

## Step 3: Scaffold Feature Module
- Create module structure under `app/ratel/src/features/<module>/`
- **References**: conventions/feature-module-structure.md

## Step 4: Implement Backend
- Create controllers, models, error types
- Write backend tests
- **References**: conventions/backend-api.md, conventions/dynamodb-patterns.md, conventions/error-handling.md, conventions/backend-testing.md

## Step 5: Implement Frontend
- Create components, views, hooks, i18n
- **References**: conventions/dioxus-app.md, conventions/styling.md, conventions/i18n.md
- **Skills**: dioxus-knowledge-patch, rust-knowledge-patch

## Step 6: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 7: Test
- Write and run e2e tests
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer
