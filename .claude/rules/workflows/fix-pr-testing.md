# Fix PR Testing

The PR workflow (`.github/workflows/pr-workflow.yml`) runs four jobs on every PR to `dev`:

| Job | What it tests |
|-----|--------------|
| `contracts-test` | Smart contract tests (`contracts/`) |
| `dx-check` | Dioxus compile check (`dx check --web`) |
| `ratel-app-testing` | Server function integration tests (`cargo test`) |
| `playwright-tests` | E2e browser tests (`playwright/`) |

## Step 1: Identify the Failing Job

- Read the CI failure output to determine which job failed
- Note the exact error message and failing test name

## Step 2: Reproduce Locally

Reproduce the failure locally before changing any code:

```bash
# dx-check failure
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web

# Server function test failure
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- <test_name>

# Playwright failure
cd playwright && npx playwright test <file> --headed

# Contract test failure
cd contracts && make test
```

- **References**: conventions/build-commands.md, conventions/server-function-tests.md, conventions/playwright-tests.md

## Step 3: Fix the Root Cause

- Diagnose the root cause from the reproduction — do not guess
- Apply the fix to the relevant layer
- **References**: conventions/server-functions.md, conventions/dioxus-app.md, conventions/error-handling.md, conventions/anti-patterns.md
- **Skills**: superpowers:systematic-debugging

## Step 4: Lint & Format

- **References**: conventions/lint-and-format.md

## Step 5: Verify All Jobs Pass Locally

```bash
# 1. Compile check
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web

# 2. Server function tests
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass"

# 3. Playwright tests (requires Docker infra running)
make infra
cd playwright && npx playwright test
```

- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion
