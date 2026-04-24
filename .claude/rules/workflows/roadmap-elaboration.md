# Roadmap Elaboration (PO Stage)

Stage 1 of feature development. The Product Owner's primary job here is to turn a problem/opportunity into a **concrete, implementable specification** before any design or code work begins.

## When to Use

- A new roadmap item is being added
- An existing roadmap item needs deeper elaboration before handing off to design
- Requirements have shifted and the spec needs to be updated

## Artifacts this stage produces

1. An entry in **`ROADMAP.md`** — a checklist of roadmap titles, each linked to its detailed spec file
2. A detailed spec at **`roadmap/{roadmap-name}.md`** — full requirements, user stories, acceptance criteria, constraints

## Step 1: Pick a roadmap slug

- Use kebab-case, short, descriptive: `cross-posting`, `essence-connector`, `sub-team-governance`
- This slug will be reused across **all three stages**:
  - `ROADMAP.md` entry label
  - `roadmap/{slug}.md` spec file
  - `app/ratel/assets/design/{slug}/` design directory
  - `docs/superpower/{YYYY-MM-DD}-{slug}.md` architecture doc

## Step 2: Register in `ROADMAP.md`

Add/update the entry at repo root:

```markdown
# Ratel Roadmap

## In Progress
- [ ] [Cross-posting to Bluesky/LinkedIn/Threads](roadmap/cross-posting.md)
- [ ] [Essence Connector (Notion)](roadmap/essence-connector.md)

## Planned
- [ ] [Sub-team governance](roadmap/sub-team.md)

## Shipped
- [x] [User DID verification](roadmap/did-verification.md)
```

Each entry is a checkbox + link to the spec file. `ROADMAP.md` is the index; it never contains requirements itself.

## Step 3: Write the spec at `roadmap/{slug}.md`

The spec file is the **contract** between product, design, and engineering. Structure:

```markdown
# {Roadmap title}

## Problem
What user/business problem does this solve? Who is the target user?
What happens today without this? Why now?

## Goal
One-sentence outcome statement. What does success look like?

## Non-goals
What this feature is explicitly NOT doing. Prevents scope creep during design/dev.

## User stories
- As a {role}, I want to {action}, so that {outcome}.
- …

## Functional requirements
Numbered, testable bullets. Each one is something a reviewer can verify.
1. The system SHALL allow users to {X}.
2. When {Y}, the system SHALL {Z}.
…

## Acceptance criteria
- [ ] Criterion 1 (visible behavior a tester can check)
- [ ] Criterion 2
…

## Constraints
- Performance: e.g., "sync within 30s of source edit"
- Privacy/compliance: e.g., "tokens KMS-encrypted"
- Platform/API limits: e.g., "Notion API 3 req/s"
- Cost: e.g., "no per-user paid third-party services"

## Open questions
Decisions still to be made. Blockers to Stage 2 start.

## References
Links to user research, related roadmap items, external API docs, competitor analysis.
```

## Step 4: Iterate toward concreteness

A spec is **ready for Stage 2** when every item in this checklist is true:

- [ ] Every functional requirement is **testable** (not "the UI is intuitive", but "clicking X shows Y")
- [ ] Acceptance criteria are **behavioral**, not architectural
- [ ] Non-goals list is non-empty
- [ ] No open questions block Stage 2 (open questions about Stage 3 are OK to leave)
- [ ] At least one example user story per primary user role
- [ ] External constraints (API limits, compliance, cost) are identified

If any box is unchecked, iterate with stakeholders before handing off to design. **This is the PO's primary deliverable — quality of the spec determines quality of everything downstream.**

## Step 5: Hand off to Design

Once the spec is concrete:
- Mark the spec file "Ready for design" (can be a front-matter flag or a heading note)
- Start Stage 2: `workflows/ui-design-implementation.md`

## Rules

- **No implementation details in the spec.** "The API should use DynamoDB" is Stage 3 territory. Keep Stage 1 behavioral.
- **No visual design in the spec.** "Show a blue button" is Stage 2. Describe the behavior, not the pixels.
- **Update, don't replace.** When requirements change, edit the existing `roadmap/{slug}.md` and note the change date. The spec is a living contract.
- **Every checkbox in `ROADMAP.md` maps to exactly one file in `roadmap/`.** No unlinked entries, no orphaned specs.
