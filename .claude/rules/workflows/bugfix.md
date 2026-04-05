# Bugfix

## Step 1: Reproduce & Understand
- Reproduce the bug
- Identify the root cause
- **Skills**: superpowers:systematic-debugging

## Step 2: Fix
- Implement the fix in the relevant layer (backend/frontend/both)
- **References**: conventions/backend-api.md, conventions/dynamodb-patterns.md, conventions/dioxus-app.md, conventions/styling.md, conventions/error-handling.md

## Step 3: Write Regression Test
- Add backend test covering the bug
- Add e2e test if user-facing
- **References**: conventions/backend-testing.md, conventions/playwright-tests.md

## Step 4: Verify Build
- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion
