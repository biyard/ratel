# Prerequisite Actions UX

## Overview

Add a prerequisite actions flow to the Arena Space Viewer (index page). After participating, users must complete all prerequisite actions before the space starts. Only users who complete all prerequisites are registered as candidates for the space.

## UX Flow

```
Sign In → Verify (if needed) → Participate
                                     ↓
                    ┌─────────────────────────────────────┐
                    │  has incomplete prerequisites?       │
                    │  YES → PrerequisiteCard (portal)    │
                    │  NO  → space started?                │
                    │         YES → ActionDashboard        │
                    │         NO  → WaitingCard (portal)   │
                    └─────────────────────────────────────┘
```

- Join-anytime spaces: users can join after the space starts, but must still complete prerequisites first.

## State Detection Logic

In `SpaceIndexPage` (`component.rs`):

| Role | Condition | View |
|------|-----------|------|
| Creator | — | SpaceDashboardPage (existing) |
| Participant | — | ActionDashboard (existing) |
| Candidate | incomplete prerequisites | ArenaViewer + PrerequisiteCard |
| Candidate | all prerequisites done, space Open | ArenaViewer + WaitingCard |
| Candidate | all prerequisites done, space Ongoing | ActionDashboard (join_anytime; role refreshes to Participant) |
| Viewer (not logged in) | — | ArenaViewer + SigninCard (existing) |
| Viewer (logged in, can participate) | — | ArenaViewer + ParticipateCard (existing) |

Data sources (no new endpoints):
- `list_actions()` → filter `prerequisite: true`, check `user_participated` on each
- `SpaceResponse.status` → Open vs Ongoing
- `SpaceUserRole` → Candidate vs Participant

## New Components

### PrerequisiteCard (`spaces/pages/index/prerequisite_card/`)

```
prerequisite_card/
├── mod.rs
├── component.rs
└── style.css
```

Renders inside the portal div (same position as SigninCard/ParticipateCard).

**Content:**
- Header: "Complete Required Actions"
- Checklist of prerequisite actions, each showing:
  - Action type icon (poll/quiz/discussion/follow)
  - Title
  - Status: checkmark icon (done) or clickable row (pending)
- Progress bar: `completed / total`
- Clicking a pending action opens the action page as a full-screen overlay

**Behavior:**
- Loads prerequisite actions via `list_actions()`, filtered to `prerequisite: true`
- On action completion (overlay closes), restarts the loader to refresh status
- When all prerequisites complete, auto-transitions to WaitingCard or ActionDashboard

### WaitingCard (`spaces/pages/index/waiting_card/`)

```
waiting_card/
├── mod.rs
├── component.rs
└── style.css
```

Renders inside the portal div.

**Content:**
- Success icon + header: "You're All Set!"
- Completed checklist summary (all items checked, non-interactive)
- Space status text: "Waiting for space to start" or similar

**Behavior:**
- Static display — no interactive elements beyond the checklist summary
- When space status changes to Ongoing (on next data refresh/navigation), the role upgrades to Participant and ActionDashboard takes over

### ActiveActionOverlay (generic overlay)

Extends the existing quiz-only `ActiveQuizOverlay` pattern to a generic enum supporting polls and quizzes:

```rust
#[derive(Clone)]
pub enum ActiveActionOverlay {
    Poll(SpacePartition, SpacePollEntityType),
    Quiz(SpacePartition, SpaceQuizEntityType),
}

#[derive(Clone, Copy)]
pub struct ActiveActionOverlaySignal(pub Signal<Option<ActiveActionOverlay>>);
```

- Discussion and Follow actions don't have inline viewers in `action_pages/`, so they navigate to their dedicated pages instead of opening an overlay.
- The overlay renders in `component.rs` at `fixed inset-0 z-[100]` (same as current quiz overlay).
- On close, the prerequisite card's action loader restarts to pick up the completion.

## Modified Files

### `component.rs` (SpaceIndexPage)

- Replace `ActiveQuizOverlay` context with `ActiveActionOverlaySignal`
- Change the `is_participant` branch:
  - `Participant` → ActionDashboard (unchanged)
  - `Candidate` → wrap in `SuspenseBoundary`, load prerequisite actions, branch to PrerequisiteCard or WaitingCard
- Render `ActiveActionOverlaySignal` overlay (poll or quiz) instead of quiz-only overlay

### `action_pages/quiz/component.rs`

- Update `ActiveQuizOverlay` references to use `ActiveActionOverlaySignal`
- `CompletedQuizAction` stays unchanged (used by ActionDashboard animation)

### `arena_viewer/component.rs`

- Add a new branch: when `participated && role == Candidate`, show PrerequisiteCard or WaitingCard instead of the existing Viewer/SigninCard/ParticipateCard cards

### `i18n.rs`

- Add translation keys for PrerequisiteCard and WaitingCard text

## Styling

Both new cards use the portal card aesthetic (same as `participate-card`, `signin-card`):
- Glass/translucent background matching the arena theme
- CSS in component-local `style.css` with space toggle dark/light pattern
- Checklist items styled as rows with icon + text + status indicator

## Data Flow

```
SpaceIndexPage
  ├── ArenaViewer (role == Candidate)
  │     ├── PrerequisiteCard (prerequisites incomplete)
  │     │     └── onClick item → set ActiveActionOverlaySignal
  │     └── WaitingCard (prerequisites complete, space Open)
  ├── ActionDashboard (role == Participant)
  └── ActiveActionOverlay (poll/quiz full-screen overlay)
        └── onClose → restart prerequisite action loader
```

## Not In Scope

- New server endpoints (existing `list_actions` + `SpaceResponse` provide all needed data)
- Changes to the prerequisite setting UI (admin side)
- Countdown timer for space start time (not available in SpaceResponse currently)
- Changes to ActionDashboard carousel behavior
