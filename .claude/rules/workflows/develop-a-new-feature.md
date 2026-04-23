# Develop a New Feature (Stage 3)

Stage 3 of feature development. Implement a feature from an approved roadmap spec and UI mockup.

## Prerequisites

- `roadmap/{roadmap-name}.md` exists with concrete requirements (Stage 1)
- `app/ratel/assets/design/{roadmap-name}/` contains approved HTML mockups (Stage 2)

If either is missing, go back to the earlier stage before writing any code.

## Step 1: Read all inputs

- Read `roadmap/{roadmap-name}.md` — every functional requirement, every acceptance criterion
- Walk through every file in `app/ratel/assets/design/{roadmap-name}/` — every class name, element ID, interactive state
- Explore adjacent code in `app/ratel/src/features/` that might share models or primitives
- **References**: conventions/project-structure.md, conventions/feature-module-structure.md

## Step 2: Write the system architecture document

Before writing any feature code, produce a design doc at:

```
docs/superpower/{YYYY-MM-DD}-{roadmap-name}.md
```

The date is when the doc is authored (today's date). Rename/supersede if the design changes materially.

### Required sections

```markdown
# {Roadmap title} — System Design

**Roadmap**: [roadmap/{roadmap-name}.md](../../roadmap/{roadmap-name}.md)
**Design**: [/designs/{roadmap-name}/](../../app/ratel/assets/design/{roadmap-name}/)
**Author / Date**: {you} · {YYYY-MM-DD}

## Summary
2-3 sentences: what this feature does, at the system level.

## Data model
DynamoDB entities (pk/sk, fields, indexes). Show `#[derive(DynamoEntity)]` signatures.
Reference: conventions/dynamo-prefix-convention.md, rust-dynamodb-skill.

## API surface
Server function signatures: route, method, request/response DTOs, auth guards, SubPartition typing.
Reference: conventions/server-functions.md.

## Event flow (if applicable)
If the feature uses DynamoDB Streams → EventBridge, diagram the chain:
entity change → Pipe filter → Rule → Lambda handler.
Reference: conventions/implementing-event-bridge.md.

## External integrations (if applicable)
Third-party APIs: OAuth flow, rate limits, error handling, token storage (KMS), webhooks.

## Frontend architecture
- UseFeatureName controller shape (signals, loaders, queries, actions)
- Which primitives from common/components are reused
- Route definitions if new pages are added
Reference: conventions/hooks-and-actions.md, conventions/dioxus-app.md.

## Test plan
- Server function integration tests: endpoint × success/error/unauth matrix
- Playwright e2e scenarios: which existing spec file extended, or new spec file name

## Open questions / risks
Known unknowns at implementation time. Flag for review.
```

Keep the doc **short and decision-focused**. If a section doesn't apply, delete it — don't pad. Total length rarely exceeds 300 lines.

## Step 3: Get alignment on the design doc

- Share the doc link in whatever channel the team uses
- Resolve open questions before starting code
- Update the doc if the approach shifts during implementation (the doc is a living record)

## Step 4: Design Event Bridge (if applicable)

If Step 2's doc identified queue-based or deferred processing:
- Define the Pipe filter, Rule, Lambda handler
- Add to `DetailType` enum and `EventBridgeEnvelope::proc()`
- Add local-dev branch in `stream_handler.rs`
- **References**: conventions/implementing-event-bridge.md

## Step 5: Scaffold feature module

- Create module structure under `app/ratel/src/features/<module>/`
- Use the roadmap slug as the module name where possible (kebab → snake case)
- **References**: conventions/feature-module-structure.md

## Step 6: Implement server functions

- Create controllers, models, error types per the data model and API surface sections of the design doc
- **Skills**: rust-dynamodb-skill, dioxus-knowledge-patch, rust-knowledge-patch
- **References**: conventions/server-functions.md, conventions/error-handling.md, conventions/anti-patterns.md

## Step 7: Write server function tests

- Add integration tests in `app/ratel/src/tests/<feature>_tests.rs`
- Register module in `app/ratel/src/tests/mod.rs`
- Test success, error, and unauthenticated cases for every endpoint in the API surface
- **References**: conventions/server-function-tests.md

## Step 8: Build feature controller hook

- Add a `UseFeatureName` controller in `app/ratel/src/features/<feature>/hooks/use_<feature>.rs`
- Bundle every signal, loader, query, and `use_action(...)` mutation the UI needs
- Context-cache it with `try_use_context::<UseFeature>()` + `provide_root_context(...)`
- **References**: conventions/hooks-and-actions.md

## Step 9: Convert HTML mockups to Dioxus RSX

For each file in `app/ratel/assets/design/{roadmap-name}/`:
- Follow `workflows/html-to-dioxus.md`
- **Preserve class names and element IDs exactly** — they're the contract with the mockup
- Wire components to the `UseFeatureName` controller's signals and actions
- Components must consume the controller; they MUST NOT import server `_handler` functions directly
- **References**: conventions/html-first-components.md, conventions/styling.md, conventions/dioxus-app.md, conventions/hooks-and-actions.md, conventions/i18n.md, conventions/anti-patterns.md
- **Skills**: frontend-design, figma:figma-implement-design, dioxus-knowledge-patch, rust-knowledge-patch

## Step 10: Lint & format

- **References**: conventions/lint-and-format.md

## Step 11: Verify build

- **References**: conventions/build-commands.md
- **Skills**: superpowers:verification-before-completion

## Step 12: Extend Playwright coverage

- For each acceptance criterion in `roadmap/{roadmap-name}.md`, there must be at least one Playwright step that verifies it
- **Prefer extending an existing scenario** in `playwright/tests/web/` that already touches the same page flow — add new `test()` blocks inside the existing `test.describe.serial()` suite
- **Create a new spec file** only when the flow is fundamentally new (first time visiting a new area of the app)
- **References**: conventions/playwright-tests.md
- **Skills**: playwright-scenario-writer

## Step 13: Close the loop

- Re-check every acceptance criterion in `roadmap/{roadmap-name}.md` — mark `- [x]` when verified by tests
- Update `ROADMAP.md` checkbox state if the roadmap item is fully shipped
- Note any deviations from the design doc back in `docs/superpower/{date-roadmap-name}.md`

## Rules

- **No code before the design doc.** If `docs/superpower/{date-roadmap-name}.md` doesn't exist, stop and write it.
- **Every acceptance criterion gets a test.** Roadmap spec → test coverage is traceable.
- **The design doc is the source of truth for implementation decisions.** When in doubt, update the doc, not just the code.
- **One roadmap slug, end to end.** The same `{roadmap-name}` threads through roadmap file, design dir, architecture doc, feature module name, and test file.
