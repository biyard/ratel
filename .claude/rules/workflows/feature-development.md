# Feature Development

Feature work flows through **three ordered stages**. Each stage has a dedicated workflow and produces an artifact the next stage consumes. Do not skip stages — the artifact chain is how work stays aligned from product intent to shipped code.

```
┌──────────────────────┐   ┌──────────────────────┐   ┌──────────────────────┐
│  1. PO               │   │  2. Design           │   │  3. Development      │
│  Roadmap elaboration │ → │  UI mockup           │ → │  Implementation      │
│                      │   │                      │   │                      │
│  ROADMAP.md          │   │  HTML/CSS/JS         │   │  Architecture doc    │
│  roadmap/{name}.md   │   │  design/{name}/      │   │  Code + tests        │
└──────────────────────┘   └──────────────────────┘   └──────────────────────┘
```

## Artifact chain

| Stage | Owner | Input | Output |
|---|---|---|---|
| **1. PO** | Product owner | Problem / opportunity | `ROADMAP.md` entry + `roadmap/{roadmap-name}.md` spec |
| **2. Design** | Designer | `roadmap/{roadmap-name}.md` | `app/ratel/assets/design/{roadmap-name}/*.{html,css,js}` |
| **3. Development** | Engineer | Roadmap spec + approved mockup | `docs/superpower/{YYYY-MM-DD}-{roadmap-name}.md` + code + tests |

## Which workflow to enter

- **Elaborating a new roadmap item** → `workflows/roadmap-elaboration.md` (Stage 1)
- **Designing UI for an elaborated roadmap item** → `workflows/ui-design-implementation.md` (Stage 2)
- **Implementing a new feature from an approved design** → `workflows/develop-a-new-feature.md` (Stage 3, new)
- **Enhancing an already-implemented feature** → `workflows/improve-feature.md` (Stage 3, existing)
- **Fixing a bug in shipped code** → `workflows/bugfix.md` (separate)

## Rules

- **No code without a roadmap spec.** If a task lacks `roadmap/{name}.md`, go back to Stage 1 first.
- **No UI build without an approved mockup.** If a task lacks `app/ratel/assets/design/{name}/`, go back to Stage 2 first.
- **No implementation without a system design doc.** Stage 3 must produce `docs/superpower/{YYYY-MM-DD}-{name}.md` before writing feature code.
- **One roadmap name flows through all three stages** — the same `{roadmap-name}` slug links the roadmap entry, design directory, and architecture doc.
