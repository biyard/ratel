# Prerequisite Actions Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a prerequisite actions UX flow to the Arena Space Viewer so that after participating, users see a checklist of required actions, complete them via full-screen overlays, and then see a "You're All Set" waiting card until the space starts.

**Architecture:** The existing `ArenaViewer` portal card system is extended with two new cards (`PrerequisiteCard`, `WaitingCard`) and a generic `ActiveActionOverlay` enum that replaces the quiz-specific overlay. State detection uses existing `list_actions()` data to determine which card to show for Candidate-role users.

**Tech Stack:** Rust, Dioxus 0.7, TailwindCSS v4, CSS (space toggle dark/light)

---

## File Structure

| Action | Path | Responsibility |
|--------|------|----------------|
| Create | `app/ratel/src/features/spaces/pages/index/prerequisite_card/mod.rs` | Module declaration |
| Create | `app/ratel/src/features/spaces/pages/index/prerequisite_card/component.rs` | PrerequisiteCard component |
| Create | `app/ratel/src/features/spaces/pages/index/prerequisite_card/style.css` | PrerequisiteCard styling |
| Create | `app/ratel/src/features/spaces/pages/index/waiting_card/mod.rs` | Module declaration |
| Create | `app/ratel/src/features/spaces/pages/index/waiting_card/component.rs` | WaitingCard component |
| Create | `app/ratel/src/features/spaces/pages/index/waiting_card/style.css` | WaitingCard styling |
| Modify | `app/ratel/src/features/spaces/pages/index/mod.rs` | Register new modules |
| Modify | `app/ratel/src/features/spaces/pages/index/i18n.rs` | Add translation keys |
| Modify | `app/ratel/src/features/spaces/pages/index/component.rs` | Wire state detection + overlay |
| Modify | `app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs` | Add Candidate card branches |
| Modify | `app/ratel/src/features/spaces/pages/index/action_dashboard/quiz_card/component.rs` | Use new overlay type |
| Modify | `app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs` | Use new overlay type |

---

### Task 1: Add i18n Translation Keys

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/i18n.rs`

- [ ] **Step 1: Add prerequisite and waiting card translations**

Add these entries to the `SpaceViewerTranslate` translate! block in `i18n.rs`, before the closing `}`:

```rust
    prereq_heading: {
        en: "Complete Required Actions",
        ko: "필수 액션 완료",
    },
    prereq_desc: {
        en: "Complete the following actions to secure your spot in this space.",
        ko: "이 스페이스에 참여하려면 다음 액션을 완료해주세요.",
    },
    prereq_progress: {
        en: "Progress",
        ko: "진행도",
    },
    prereq_completed: {
        en: "Completed",
        ko: "완료",
    },
    prereq_pending: {
        en: "Start",
        ko: "시작",
    },
    waiting_heading: {
        en: "You're All Set!",
        ko: "준비 완료!",
    },
    waiting_desc: {
        en: "You've completed all required actions. Waiting for the space to start.",
        ko: "모든 필수 액션을 완료했습니다. 스페이스 시작을 기다리고 있습니다.",
    },
    waiting_status: {
        en: "Waiting for Space to Start",
        ko: "스페이스 시작 대기 중",
    },
```

- [ ] **Step 2: Verify the file compiles**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

Expected: compiles (or pre-existing warnings only)

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/i18n.rs
git commit -m "feat(spaces): add i18n keys for prerequisite and waiting cards"
```

---

### Task 2: Create ActiveActionOverlay Type

This replaces the quiz-specific `ActiveQuizOverlay` with a generic enum that supports both poll and quiz overlays.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs`

- [ ] **Step 1: Replace ActiveQuizOverlay with ActiveActionOverlay**

In `app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs`, replace the `ActiveQuizOverlay` struct:

```rust
/// Context signal to open the quiz arena overlay from the action dashboard.
/// Set to `Some((space_id, quiz_id))` to open, `None` to close.
#[derive(Clone, Copy)]
pub struct ActiveQuizOverlay(pub Signal<Option<(SpacePartition, SpaceQuizEntityType)>>);
```

with:

```rust
/// Generic overlay for prerequisite/action pages (poll + quiz).
/// Set to `Some(variant)` to open, `None` to close.
#[derive(Clone, PartialEq)]
pub enum ActiveActionOverlay {
    Poll(SpacePartition, SpacePollEntityType),
    Quiz(SpacePartition, SpaceQuizEntityType),
}

/// Context signal wrapping the overlay state.
#[derive(Clone, Copy)]
pub struct ActiveActionOverlaySignal(pub Signal<Option<ActiveActionOverlay>>);
```

- [ ] **Step 2: Update QuizArenaPage to use new type**

In the same file, find:

```rust
    let mut overlay: ActiveQuizOverlay = use_context();
```

Replace with:

```rust
    let mut overlay: ActiveActionOverlaySignal = use_context();
```

And find:

```rust
    let mut close_overlay = move || {
        overlay.0.set(None);
    };
```

This stays the same — `overlay.0.set(None)` still works since the inner type is `Signal<Option<ActiveActionOverlay>>`.

- [ ] **Step 3: Update quiz_card to use new overlay type**

In `app/ratel/src/features/spaces/pages/index/action_dashboard/quiz_card/component.rs`, replace:

```rust
use crate::features::spaces::pages::index::action_pages::quiz::ActiveQuizOverlay;
```

with:

```rust
use crate::features::spaces::pages::index::action_pages::quiz::{ActiveActionOverlay, ActiveActionOverlaySignal};
```

Replace:

```rust
    let mut overlay: ActiveQuizOverlay = use_context();
```

with:

```rust
    let mut overlay: ActiveActionOverlaySignal = use_context();
```

Replace:

```rust
                overlay.0.set(Some((space_id(), quiz_id)));
```

with:

```rust
                overlay.0.set(Some(ActiveActionOverlay::Quiz(space_id(), quiz_id)));
```

- [ ] **Step 4: Update component.rs to use new overlay type**

In `app/ratel/src/features/spaces/pages/index/component.rs`, replace:

```rust
use crate::features::spaces::pages::index::action_pages::quiz::*;
```

Keep this import as-is (it re-exports `ActiveActionOverlay`, `ActiveActionOverlaySignal`, `CompletedQuizAction`).

Replace:

```rust
    let quiz_overlay = use_context_provider(|| ActiveQuizOverlay(Signal::new(None)));
```

with:

```rust
    let action_overlay = use_context_provider(|| ActiveActionOverlaySignal(Signal::new(None)));
```

Replace the overlay rendering block at the bottom:

```rust
        if let Some((sid, qid)) = quiz_overlay.0() {
            div {
                class: "fixed inset-0 z-[100]",
                "data-testid": "quiz-arena-overlay",
                SuspenseBoundary {
                    QuizArenaPage { space_id: sid.clone(), quiz_id: qid.clone() }
                }
            }
        }
```

with:

```rust
        match action_overlay.0() {
            Some(ActiveActionOverlay::Quiz(sid, qid)) => rsx! {
                div {
                    class: "fixed inset-0 z-[100]",
                    "data-testid": "quiz-arena-overlay",
                    SuspenseBoundary {
                        QuizArenaPage { space_id: sid.clone(), quiz_id: qid.clone() }
                    }
                }
            },
            Some(ActiveActionOverlay::Poll(sid, pid)) => rsx! {
                div {
                    class: "fixed inset-0 z-[100]",
                    "data-testid": "poll-arena-overlay",
                    SuspenseBoundary {
                        ActionPollViewer {
                            space_id: sid.clone(),
                            poll_id: pid.clone(),
                            can_respond: true,
                        }
                    }
                }
            },
            None => rsx! {},
        }
```

Also add this import to `component.rs` if not already present:

```rust
use crate::features::spaces::pages::index::action_pages::poll::ActionPollViewer;
```

- [ ] **Step 5: Verify build**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

Expected: compiles successfully.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs \
       app/ratel/src/features/spaces/pages/index/action_dashboard/quiz_card/component.rs \
       app/ratel/src/features/spaces/pages/index/component.rs
git commit -m "refactor(spaces): replace ActiveQuizOverlay with generic ActiveActionOverlaySignal"
```

---

### Task 3: Create PrerequisiteCard Component

**Files:**
- Create: `app/ratel/src/features/spaces/pages/index/prerequisite_card/mod.rs`
- Create: `app/ratel/src/features/spaces/pages/index/prerequisite_card/component.rs`
- Create: `app/ratel/src/features/spaces/pages/index/prerequisite_card/style.css`

- [ ] **Step 1: Create mod.rs**

```rust
mod component;
pub use component::*;
```

- [ ] **Step 2: Create component.rs**

```rust
use crate::features::spaces::pages::actions::controllers::list_actions;
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::action_pages::quiz::{
    ActiveActionOverlay, ActiveActionOverlaySignal,
};
use crate::features::spaces::pages::index::*;
use crate::features::spaces::space_common::hooks::use_space;

#[component]
pub fn PrerequisiteCard(space_id: ReadSignal<SpacePartition>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();
    let space = use_space()();
    let nav = use_navigator();
    let mut overlay: ActiveActionOverlaySignal = use_context();

    let actions = use_loader(move || async move { list_actions(space_id()).await })?;
    let actions = actions();

    let prereqs: Vec<SpaceActionSummary> = actions
        .iter()
        .filter(|a| a.prerequisite)
        .cloned()
        .collect();

    let done_count = prereqs.iter().filter(|a| a.user_participated).count();
    let total_count = prereqs.len();
    let progress_pct = if total_count > 0 {
        (done_count as f64 / total_count as f64 * 100.0) as u32
    } else {
        0
    };

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "prereq-card",
            "data-testid": "card-prerequisite",
            span { class: "prereq-card__heading", "{tr.prereq_heading}" }
            p { class: "prereq-card__desc", "{tr.prereq_desc}" }

            // Progress
            div { class: "prereq-card__progress",
                div { class: "prereq-card__progress-bar-wrap",
                    div {
                        class: "prereq-card__progress-bar",
                        style: "width: {progress_pct}%",
                    }
                }
                span { class: "prereq-card__progress-text",
                    "{done_count} / {total_count}"
                }
            }

            // Action checklist
            div { class: "prereq-card__list",
                for action in prereqs.iter() {
                    {
                        let action = action.clone();
                        let is_done = action.user_participated;
                        rsx! {
                            div {
                                key: "{action.action_id}",
                                class: "prereq-item",
                                "data-done": is_done,
                                "data-testid": "prereq-item-{action.action_id}",
                                onclick: {
                                    let action = action.clone();
                                    move |_| {
                                        if !is_done {
                                            match action.action_type {
                                                SpaceActionType::Poll => {
                                                    let poll_id: SpacePollEntityType =
                                                        action.action_id.clone().into();
                                                    overlay.0.set(Some(
                                                        ActiveActionOverlay::Poll(space_id(), poll_id),
                                                    ));
                                                }
                                                SpaceActionType::Quiz => {
                                                    let quiz_id: SpaceQuizEntityType =
                                                        action.action_id.clone().into();
                                                    overlay.0.set(Some(
                                                        ActiveActionOverlay::Quiz(space_id(), quiz_id),
                                                    ));
                                                }
                                                SpaceActionType::TopicDiscussion => {
                                                    let route = action.get_url(&space_id());
                                                    nav.push(route);
                                                }
                                                SpaceActionType::Follow => {
                                                    let route = action.get_url(&space_id());
                                                    nav.push(route);
                                                }
                                            }
                                        }
                                    }
                                },

                                // Type icon
                                div { class: "prereq-item__icon",
                                    {action_type_icon(&action.action_type)}
                                }

                                // Info
                                div { class: "prereq-item__info",
                                    span { class: "prereq-item__title", "{action.title}" }
                                    span { class: "prereq-item__type",
                                        "{action.action_type.translate(&lang())}"
                                    }
                                }

                                // Status
                                if is_done {
                                    div { class: "prereq-item__status prereq-item__status--done",
                                        svg {
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                                            polyline { points: "22 4 12 14.01 9 11.01" }
                                        }
                                    }
                                } else {
                                    div { class: "prereq-item__status prereq-item__status--pending",
                                        "{tr.prereq_pending}"
                                        svg {
                                            fill: "none",
                                            stroke: "currentColor",
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            stroke_width: "2",
                                            view_box: "0 0 24 24",
                                            xmlns: "http://www.w3.org/2000/svg",
                                            polyline { points: "9 18 15 12 9 6" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn action_type_icon(action_type: &SpaceActionType) -> Element {
    match action_type {
        SpaceActionType::Poll => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M18 20V10" }
                path { d: "M12 20V4" }
                path { d: "M6 20v-6" }
            }
        },
        SpaceActionType::Quiz => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                circle { cx: "12", cy: "12", r: "10" }
                path { d: "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" }
                line { x1: "12", x2: "12.01", y1: "17", y2: "17" }
            }
        },
        SpaceActionType::TopicDiscussion => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" }
            }
        },
        SpaceActionType::Follow => rsx! {
            svg {
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                path { d: "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2" }
                circle { cx: "9", cy: "7", r: "4" }
                line { x1: "19", x2: "19", y1: "8", y2: "14" }
                line { x1: "22", x2: "16", y1: "11", y2: "11" }
            }
        },
    }
}
```

- [ ] **Step 3: Create style.css**

```css
/* ── Prerequisite Card ─────────────────────────── */

.prereq-card {
  --prereq-bg: var(--dark, rgba(12, 12, 26, 0.65)) var(--light, rgba(255, 255, 255, 0.72));
  --prereq-border: var(--dark, rgba(255, 255, 255, 0.06)) var(--light, rgba(0, 0, 0, 0.08));
  --prereq-text: var(--dark, #8888a8) var(--light, #6b6b80);
  --prereq-text-primary: var(--dark, #f0f0f5) var(--light, #12121a);
  --prereq-shadow: var(--dark, rgba(0,0,0,0.4)) var(--light, rgba(0,0,0,0.08));
  --prereq-item-bg: var(--dark, rgba(255,255,255,0.03)) var(--light, rgba(0,0,0,0.03));
  --prereq-done-border: rgba(34,197,94,0.3);
  --prereq-done-bg: rgba(34,197,94,0.06);

  width: 380px;
  max-height: 70vh;
  padding: 32px 28px 28px;
  background: var(--prereq-bg);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border: 1px solid var(--prereq-border);
  border-radius: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  box-shadow:
    0 8px 60px var(--prereq-shadow),
    0 0 80px rgba(252,179,0,0.04),
    inset 0 1px 0 rgba(255,255,255,0.04);
  animation: card-float 6s ease-in-out infinite;
}

.prereq-card__heading {
  font-family: 'Orbitron', sans-serif;
  font-size: 13px;
  font-weight: 600;
  letter-spacing: 0.15em;
  text-transform: uppercase;
  color: #fcb300;
}

.prereq-card__desc {
  font-size: 13px;
  line-height: 1.6;
  color: var(--prereq-text);
  text-align: center;
  max-width: 300px;
}

/* ── Progress ─────────────────────────────────── */

.prereq-card__progress {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 10px;
}

.prereq-card__progress-bar-wrap {
  flex: 1;
  height: 6px;
  border-radius: 3px;
  background: var(--dark, rgba(255,255,255,0.08)) var(--light, rgba(0,0,0,0.06));
  overflow: hidden;
}

.prereq-card__progress-bar {
  height: 100%;
  border-radius: 3px;
  background: linear-gradient(90deg, #fcb300, #e5a200);
  transition: width 0.4s ease;
}

.prereq-card__progress-text {
  font-family: 'Orbitron', sans-serif;
  font-size: 11px;
  font-weight: 600;
  color: var(--prereq-text);
  white-space: nowrap;
}

/* ── Checklist ────────────────────────────────── */

.prereq-card__list {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 8px;
  overflow-y: auto;
}

.prereq-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px 14px;
  border-radius: 12px;
  border: 1px solid var(--prereq-border);
  background: var(--prereq-item-bg);
  cursor: pointer;
  transition: all 0.2s ease;
}
.prereq-item:hover:not([data-done="true"]) {
  border-color: rgba(252,179,0,0.25);
  background: var(--dark, rgba(252,179,0,0.04)) var(--light, rgba(252,179,0,0.06));
}
.prereq-item[data-done="true"] {
  border-color: var(--prereq-done-border);
  background: var(--prereq-done-bg);
  cursor: default;
}

.prereq-item__icon {
  width: 20px;
  height: 20px;
  flex-shrink: 0;
  color: var(--prereq-text);
}
.prereq-item[data-done="true"] .prereq-item__icon {
  color: #22C55E;
}

.prereq-item__info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.prereq-item__title {
  font-size: 13px;
  font-weight: 600;
  color: var(--prereq-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.prereq-item__type {
  font-size: 11px;
  color: var(--prereq-text);
}

.prereq-item__status {
  flex-shrink: 0;
  display: flex;
  align-items: center;
  gap: 4px;
}

.prereq-item__status--done {
  color: #22C55E;
}
.prereq-item__status--done svg {
  width: 18px;
  height: 18px;
}

.prereq-item__status--pending {
  font-family: 'Orbitron', sans-serif;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: #fcb300;
}
.prereq-item__status--pending svg {
  width: 14px;
  height: 14px;
}

@media (max-width: 500px) {
  .prereq-card { width: calc(100vw - 32px); padding: 24px 18px 20px; }
}
```

- [ ] **Step 4: Register module in mod.rs**

In `app/ratel/src/features/spaces/pages/index/mod.rs`, add after the `participate_card` line:

```rust
mod prerequisite_card;
```

And add in the `use` section:

```rust
use prerequisite_card::*;
```

- [ ] **Step 5: Verify build**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/prerequisite_card/ \
       app/ratel/src/features/spaces/pages/index/mod.rs
git commit -m "feat(spaces): add PrerequisiteCard component with checklist UI"
```

---

### Task 4: Create WaitingCard Component

**Files:**
- Create: `app/ratel/src/features/spaces/pages/index/waiting_card/mod.rs`
- Create: `app/ratel/src/features/spaces/pages/index/waiting_card/component.rs`
- Create: `app/ratel/src/features/spaces/pages/index/waiting_card/style.css`

- [ ] **Step 1: Create mod.rs**

```rust
mod component;
pub use component::*;
```

- [ ] **Step 2: Create component.rs**

```rust
use crate::features::spaces::pages::actions::types::{SpaceActionSummary, SpaceActionType};
use crate::features::spaces::pages::index::*;

#[component]
pub fn WaitingCard(prereqs: Vec<SpaceActionSummary>) -> Element {
    let tr: SpaceViewerTranslate = use_translate();
    let lang = use_language();

    rsx! {
        document::Link { rel: "stylesheet", href: asset!("./style.css") }

        div {
            class: "waiting-card",
            "data-testid": "card-waiting",

            // Success icon
            div { class: "waiting-card__icon",
                svg {
                    fill: "none",
                    stroke: "currentColor",
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    view_box: "0 0 24 24",
                    xmlns: "http://www.w3.org/2000/svg",
                    path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                    polyline { points: "22 4 12 14.01 9 11.01" }
                }
            }

            span { class: "waiting-card__heading", "{tr.waiting_heading}" }
            p { class: "waiting-card__desc", "{tr.waiting_desc}" }

            // Completed checklist summary
            if !prereqs.is_empty() {
                div { class: "waiting-card__list",
                    for action in prereqs.iter() {
                        div {
                            key: "{action.action_id}",
                            class: "waiting-item",
                            div { class: "waiting-item__icon",
                                svg {
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    stroke_width: "2",
                                    view_box: "0 0 24 24",
                                    xmlns: "http://www.w3.org/2000/svg",
                                    path { d: "M22 11.08V12a10 10 0 1 1-5.93-9.14" }
                                    polyline { points: "22 4 12 14.01 9 11.01" }
                                }
                            }
                            div { class: "waiting-item__info",
                                span { class: "waiting-item__title", "{action.title}" }
                                span { class: "waiting-item__type",
                                    "{action.action_type.translate(&lang())}"
                                }
                            }
                        }
                    }
                }
            }

            // Status badge
            div { class: "waiting-card__status",
                div { class: "waiting-card__pulse" }
                "{tr.waiting_status}"
            }
        }
    }
}
```

- [ ] **Step 3: Create style.css**

```css
/* ── Waiting Card ──────────────────────────────── */

.waiting-card {
  --waiting-bg: var(--dark, rgba(12, 12, 26, 0.65)) var(--light, rgba(255, 255, 255, 0.72));
  --waiting-border: var(--dark, rgba(255, 255, 255, 0.06)) var(--light, rgba(0, 0, 0, 0.08));
  --waiting-text: var(--dark, #8888a8) var(--light, #6b6b80);
  --waiting-text-primary: var(--dark, #f0f0f5) var(--light, #12121a);
  --waiting-shadow: var(--dark, rgba(0,0,0,0.4)) var(--light, rgba(0,0,0,0.08));

  width: 380px;
  padding: 36px 28px 28px;
  background: var(--waiting-bg);
  backdrop-filter: blur(24px);
  -webkit-backdrop-filter: blur(24px);
  border: 1px solid var(--waiting-border);
  border-radius: 20px;
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 16px;
  box-shadow:
    0 8px 60px var(--waiting-shadow),
    0 0 80px rgba(252,179,0,0.04),
    inset 0 1px 0 rgba(255,255,255,0.04);
  animation: card-float 6s ease-in-out infinite;
}

.waiting-card__icon {
  width: 48px;
  height: 48px;
  color: #22C55E;
}
.waiting-card__icon svg {
  width: 100%;
  height: 100%;
}

.waiting-card__heading {
  font-family: 'Orbitron', sans-serif;
  font-size: 16px;
  font-weight: 700;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: #22C55E;
}

.waiting-card__desc {
  font-size: 13px;
  line-height: 1.6;
  color: var(--waiting-text);
  text-align: center;
  max-width: 300px;
}

/* ── Completed Checklist ──────────────────────── */

.waiting-card__list {
  width: 100%;
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.waiting-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 14px;
  border-radius: 10px;
  border: 1px solid rgba(34,197,94,0.2);
  background: rgba(34,197,94,0.04);
}

.waiting-item__icon {
  width: 16px;
  height: 16px;
  flex-shrink: 0;
  color: #22C55E;
}
.waiting-item__icon svg {
  width: 100%;
  height: 100%;
}

.waiting-item__info {
  flex: 1;
  display: flex;
  flex-direction: column;
  gap: 1px;
  min-width: 0;
}

.waiting-item__title {
  font-size: 13px;
  font-weight: 600;
  color: var(--waiting-text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.waiting-item__type {
  font-size: 11px;
  color: var(--waiting-text);
}

/* ── Status Badge ─────────────────────────────── */

.waiting-card__status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 20px;
  border-radius: 24px;
  background: var(--dark, rgba(252,179,0,0.08)) var(--light, rgba(252,179,0,0.1));
  border: 1px solid rgba(252,179,0,0.2);
  font-family: 'Orbitron', sans-serif;
  font-size: 10px;
  font-weight: 600;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  color: #fcb300;
}

.waiting-card__pulse {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #fcb300;
  animation: pulse-glow 2s ease-in-out infinite;
}

@keyframes pulse-glow {
  0%, 100% { opacity: 1; box-shadow: 0 0 4px rgba(252,179,0,0.4); }
  50% { opacity: 0.5; box-shadow: 0 0 12px rgba(252,179,0,0.6); }
}

@media (max-width: 500px) {
  .waiting-card { width: calc(100vw - 32px); padding: 28px 18px 24px; }
}
```

- [ ] **Step 4: Register module in mod.rs**

In `app/ratel/src/features/spaces/pages/index/mod.rs`, add after the `prerequisite_card` line:

```rust
mod waiting_card;
```

And add in the `use` section:

```rust
use waiting_card::*;
```

- [ ] **Step 5: Verify build**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/waiting_card/ \
       app/ratel/src/features/spaces/pages/index/mod.rs
git commit -m "feat(spaces): add WaitingCard component for post-prerequisite waiting state"
```

---

### Task 5: Wire State Detection in ArenaViewer

This is the core wiring: Candidate-role users see PrerequisiteCard or WaitingCard instead of the existing Viewer content.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs`
- Modify: `app/ratel/src/features/spaces/pages/index/component.rs`

- [ ] **Step 1: Update ArenaViewer to accept candidate_view prop**

In `app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs`, add a `candidate_view` prop to show when the user is a Candidate:

Replace the component signature:

```rust
#[component]
pub fn ArenaViewer(
    space_id: ReadSignal<SpacePartition>,
    dimmed: bool,
) -> Element {
```

with:

```rust
#[component]
pub fn ArenaViewer(
    space_id: ReadSignal<SpacePartition>,
    dimmed: bool,
    #[props(default)] candidate_view: Option<Element>,
) -> Element {
```

- [ ] **Step 2: Add candidate_view rendering branch**

In the same file, find the portal div's inner content block that starts with:

```rust
                if is_logged_in && show_participate && needs_verification {
```

Add this branch **before** that block:

```rust
                if let Some(view) = candidate_view {
                    {view}
                } else if is_logged_in && show_participate && needs_verification {
```

This means when `candidate_view` is `Some(...)`, it renders instead of the sign-in/verify/participate flow.

- [ ] **Step 3: Update SpaceIndexPage to pass candidate_view**

In `app/ratel/src/features/spaces/pages/index/component.rs`, the current code at line 57 is:

```rust
    let is_participant = matches!(role, SpaceUserRole::Participant | SpaceUserRole::Candidate);
```

Replace the rendering logic (lines 80-86):

```rust
            if is_participant {
                SuspenseBoundary {
                    ActionDashboard { space_id }
                }
            } else {
                ArenaViewer { space_id, dimmed }
            }
```

with:

```rust
            if matches!(role, SpaceUserRole::Participant) {
                SuspenseBoundary {
                    ActionDashboard { space_id }
                }
            } else if matches!(role, SpaceUserRole::Candidate) {
                ArenaViewer {
                    space_id,
                    dimmed,
                    candidate_view: rsx! {
                        SuspenseBoundary {
                            CandidateView { space_id }
                        }
                    },
                }
            } else {
                ArenaViewer { space_id, dimmed }
            }
```

Also remove the now-unused `is_participant` variable.

- [ ] **Step 4: Create CandidateView helper component**

Add this component at the bottom of `component.rs` (before the `format_number` function):

```rust
#[component]
fn CandidateView(space_id: ReadSignal<SpacePartition>) -> Element {
    let space = use_space()();
    let actions = use_loader(move || async move {
        crate::features::spaces::pages::actions::controllers::list_actions(space_id()).await
    })?;
    let actions = actions();

    let prereqs: Vec<_> = actions.iter().filter(|a| a.prerequisite).cloned().collect();
    let all_done = prereqs.is_empty() || prereqs.iter().all(|a| a.user_participated);

    if all_done {
        rsx! {
            WaitingCard { prereqs }
        }
    } else {
        rsx! {
            PrerequisiteCard { space_id }
        }
    }
}
```

Add this import at the top of `component.rs`:

```rust
use crate::features::spaces::pages::actions::types::SpaceActionSummary;
```

- [ ] **Step 5: Verify build**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

- [ ] **Step 6: Lint and format changed files**

```bash
dx fmt -f app/ratel/src/features/spaces/pages/index/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/prerequisite_card/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/waiting_card/component.rs
```

- [ ] **Step 7: Verify build after formatting**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

- [ ] **Step 8: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/component.rs \
       app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs
git commit -m "feat(spaces): wire prerequisite/waiting state detection for Candidate role"
```

---

### Task 6: Handle Poll Overlay Close → Return to Prerequisite Card

Currently `ActionPollViewer` navigates to `SpaceIndexPage` after submitting. When opened as an overlay for prerequisites, it should close the overlay instead.

**Files:**
- Modify: `app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs`

- [ ] **Step 1: Add overlay close on submit**

In `app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs`, add an import:

```rust
use crate::features::spaces::pages::index::action_pages::quiz::ActiveActionOverlaySignal;
```

Inside the `ActionPollViewer` component, after the existing context/signal setup, add:

```rust
    let overlay: Option<ActiveActionOverlaySignal> = try_consume_context();
```

`try_consume_context()` returns `None` when `ActiveActionOverlaySignal` is not provided — this happens when `ActionPollViewer` is used from the route-based poll page (`PollContent`). When rendered inside the overlay from `SpaceIndexPage`, it returns `Some(...)`.

In the submit handler callback (`do_submit`), find:

```rust
                    toast.info(tr.submit_success);
                    nav.replace(crate::Route::SpaceIndexPage { space_id: space_id() });
```

Replace with:

```rust
                    toast.info(tr.submit_success);
                    if let Some(mut ov) = overlay {
                        ov.0.set(None);
                    } else {
                        nav.replace(crate::Route::SpaceIndexPage { space_id: space_id() });
                    }
```

- [ ] **Step 2: Verify build**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

- [ ] **Step 3: Lint and format**

```bash
dx fmt -f app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs
```

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs
git commit -m "feat(spaces): close overlay instead of navigating when poll submitted from prerequisite"
```

---

### Task 7: Final Verification and Cleanup

**Files:**
- All modified files from Tasks 1-6

- [ ] **Step 1: Full build check**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -20
```

Expected: successful compilation.

- [ ] **Step 2: Run rustywind on all modified RS files**

```bash
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/spaces/pages/index/prerequisite_card/component.rs
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/spaces/pages/index/waiting_card/component.rs
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/spaces/pages/index/component.rs
rustywind --custom-regex "class: \"(.*)\"" --write app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs
```

- [ ] **Step 3: Run dx fmt on all modified RS files**

```bash
dx fmt -f app/ratel/src/features/spaces/pages/index/prerequisite_card/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/waiting_card/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/arena_viewer/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/action_pages/poll/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/action_pages/quiz/component.rs
dx fmt -f app/ratel/src/features/spaces/pages/index/action_dashboard/quiz_card/component.rs
```

- [ ] **Step 4: Final build verification**

```bash
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev dx check --features web 2>&1 | tail -5
```

- [ ] **Step 5: Commit any lint/format changes**

```bash
git add -A app/ratel/src/features/spaces/pages/index/
git commit -m "style(spaces): lint and format prerequisite actions files"
```
