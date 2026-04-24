# Action Navigation

**Status**: Ready for design (Stage 2)
**Slug**: `action-navigation`
**Korean translation**: `roadmap/action-navigation-KO.md`
**Primary use case**: Space participant/admin entering, creating, and managing actions, plus desktop users navigating card-based carousel surfaces

## Problem

Ratel space actions are currently experienced as overlays, sheets, and layovers in places where users expect a normal page transition. That breaks the basic web/app mental model.

Concretely:
- Opening an action often feels like "a modal on top of the arena" instead of "I moved to the action page."
- Browser back / forward behavior becomes harder to predict because some state changes are route-based while others are overlay-based.
- Refreshing or sharing the current view is unreliable or awkward when the important UI state lives outside the canonical URL.
- Action creation and action settings also rely on layovers, so users have to manage too much temporary UI state while doing high-focus tasks.
- On desktop carousel surfaces, off-center blurred cards visually read like neighboring items in a selection rail, so users naturally expect a click to bring that card into focus first. Instead, those blurred cards remain fully clickable and immediately open/navigate on the first click.
- This is not primarily a "click failure" bug; it is an interaction-model mismatch. The visual system says "select this neighboring card first," while the behavior says "open immediately."

The result is that actions feel transient and fragile even though they are core content objects inside a space, and card-based navigation feels jumpy instead of deliberate. Users lose orientation, confidence, and navigational control.

## Goal

Make every action experience operate around a canonical page route, and make desktop carousel cards follow a predictable focus-before-enter interaction model, so users can enter, leave, refresh, share, and manage content with stable navigation semantics.

## Non-goals

- **No full redesign of every action's visual style in this stage.** The priority is navigation model and flow structure.
- **No rewrite of unrelated space panels** such as leaderboard, notifications, or overview unless the action flow directly depends on them.
- **No requirement to eliminate every modal.** Lightweight confirmations and small transient pickers may still use overlays.
- **No backend/domain-model expansion** beyond what is required to support stable action routing and entry points.
- **No mobile-native navigation redesign** beyond making the same canonical route model work coherently on mobile web.
- **No forced two-click behavior on mobile single-card layouts.** The focus-before-enter rule is for desktop/tablet carousel surfaces where multiple blurred neighboring cards are visible at once.

## User stories

### Space participant

- As a participant, I want clicking an action card to take me to a real action page so I know where I am.
- As a participant, I want refresh and browser back to preserve sensible action navigation so I do not feel trapped inside temporary UI.
- As a participant, I want to share an action link with another member so they land on the same action screen.
- As a desktop participant, I want clicking a blurred/off-center action card to move it to the center first so I can confirm I selected the right item before entering it.

### Space admin

- As a space admin, I want creating a new action to move me into the action's real editing surface so the workflow feels substantial and safe.
- As a space admin, I want action settings and editing to be organized around page-level navigation so I can reason about where configuration lives.

### Desktop user on card-based surfaces

- As a desktop user, I want blurred/off-center cards in a carousel to behave like selectable neighbors, not immediate entry targets.
- As a desktop user, I want the same centered-card interaction rule on both action cards and Home cards so the product feels internally consistent.

## Functional requirements

### FR-1: Canonical action routes

1. Every action type MUST have a canonical route that represents its primary read/view state.
2. Entering an action from the action list/dashboard MUST navigate to that canonical route instead of opening a full-screen overlay.
3. The canonical action route MUST be refresh-safe; reloading the browser on that URL MUST restore the same action view.
4. The canonical action route MUST be shareable; another authorized user opening the URL MUST land on the same action view.

### FR-2: Navigation behavior

5. Browser Back from an action page MUST return the user to the previous navigational location, not merely close an internal overlay state.
6. Browser Forward after returning from an action page MUST re-open the same action page.
7. If an action supports deep links to subordinate content (for example a specific comment or reply target), that state MUST be represented by the URL, not by in-memory overlay state alone.

### FR-3: Action creation flow

8. Starting "Create action" MAY use a lightweight picker surface, but once a type is selected the user MUST be navigated to that action type's canonical creation/editor route.
9. The initial create flow MUST NOT depend on a persistent half-screen or full-screen layover for the main authoring experience.
10. Refreshing during action creation MUST keep the user in the creation/editing route and preserve any existing draft behavior already supported by that action type.

### FR-4: Action management flow

11. Editing an existing action MUST happen on a canonical route rather than inside a transient overlay.
12. Action settings that materially change the action configuration SHOULD live on the page route or a page-scoped panel, not a detached global layover.
13. Destructive confirmations such as delete MAY remain modal/popup based.

### FR-5: UX consistency across action types

14. Poll, Quiz, Discussion, and Follow MUST all follow the same navigation model for entry and exit, even if their page internals differ.
15. Mobile and desktop MUST use the same canonical route model, while allowing different page layouts.
16. Space-level navigation chrome MUST clearly indicate when the user is viewing an action page versus the main arena/dashboard.

### FR-6: Desktop carousel selection semantics

17. On desktop/tablet carousel surfaces that visually emphasize a centered card and de-emphasize neighboring cards via blur/scale/opacity, clicking a non-centered card MUST first move that card into the centered/active position.
18. A non-centered card on those surfaces MUST NOT trigger its primary navigation or entry action on the first click.
19. Once a card is centered/active, clicking the card body MUST open the destination in that surface's primary way.
20. A clear CTA inside a centered card MAY also open the same destination, but MUST NOT conflict with the centered-card click behavior.
21. Keyboard activation on desktop/tablet carousel surfaces MUST follow the same rule: activating a non-centered card focuses/centers it first; activating a centered card opens it.
22. The visual treatment, hover treatment, and cursor semantics of non-centered cards MUST communicate "focus/select this card" rather than "open immediately."
23. The same desktop interaction rule MUST apply to the space action carousel and the team Home post carousel.

## Acceptance criteria

- [ ] AC-1: From a space action list/dashboard, clicking a Poll action navigates to a Poll page URL instead of opening an overlay.
- [ ] AC-2: From a space action list/dashboard, clicking a Quiz action navigates to a Quiz page URL instead of opening an overlay.
- [ ] AC-3: From a space action list/dashboard, clicking a Discussion action navigates to a Discussion page URL instead of opening an overlay.
- [ ] AC-4: Refreshing any action page keeps the user on that same action page.
- [ ] AC-5: Copying an action URL and opening it in another authorized session lands on the same action page.
- [ ] AC-6: Browser Back from an action page returns the user to the previous page/state in a predictable way.
- [ ] AC-7: Browser Forward after Back returns the user to the same action page.
- [ ] AC-8: A deep link to a discussion comment opens the discussion page at the correct target.
- [ ] AC-9: Starting "Create action" may show a picker first, but selecting an action type navigates into the real creation/editor page.
- [ ] AC-10: Refreshing during action creation does not dump the user back to the space arena root.
- [ ] AC-11: Editing an existing action uses page-based navigation rather than a transient full-screen layover.
- [ ] AC-12: Delete confirmation may still appear as a popup, but canceling it leaves the user on the same action page.
- [ ] AC-13: On desktop, clicking a blurred/non-centered action card moves it to the center and does not open it on the first click.
- [ ] AC-14: On desktop, activating the centered action card opens the action page/view in the defined primary way.
- [ ] AC-15: On desktop, clicking a blurred/non-centered Home post card moves it to the center and does not navigate on the first click.
- [ ] AC-16: On desktop, activating the centered Home post card opens the post/space destination in the defined primary way.
- [ ] AC-17: On desktop, keyboard activation on a blurred/non-centered action card centers it first and does not open it on the first activation.
- [ ] AC-18: On desktop, keyboard activation on a centered action card opens the action page/view.
- [ ] AC-19: On desktop, non-centered cards use hover/cursor semantics that indicate selection/focus, while centered cards indicate entry/open behavior.

## Constraints

- **Backward compatibility**: Existing action entities and permissions should continue to work without requiring a product-level migration of all action data.
- **Route stability**: Canonical action URLs should be durable enough to support refresh, deep linking, and future notifications.
- **Access control**: A user opening an action URL must still be gated by the same authorization rules already used for that action and space.
- **Incremental rollout**: The navigation model may be migrated in stages internally, but the user-facing result must feel consistent across action types before the work is considered complete.
- **UX clarity over implementation convenience**: Temporary overlay-based shortcuts should not survive if they undermine canonical navigation.
- **Interaction consistency across surfaces**: Card surfaces that share the same centered/blurred visual language should not use conflicting click semantics.

## Open questions

- None blocking Stage 2.
- Default decision for Stage 2: the action type picker may remain a lightweight modal/sheet, but type selection must immediately navigate into a canonical creation/editor route.
- Default decision for Stage 2: action pages should keep compact space context (breadcrumb/back-to-space/top context) instead of appearing as detached overlays.
- Default decision for Stage 2: material action settings should move into page-scoped structure (tab/section/panel/sub-route), not a global layover.
- Default decision for Stage 2: the product model is unified across Poll, Quiz, Discussion, and Follow even if implementation rollout is staged.
- Default decision for Stage 2: on desktop carousel surfaces, first click on a non-centered card centers it; click on the centered card opens it; CTA may duplicate the same open action.

## References

- Current space action viewer/editor flow under `app/ratel/src/features/spaces/pages/index/` and `app/ratel/src/features/spaces/pages/actions/`.
- Current action carousel implementation applies visual de-emphasis to off-center cards while still keeping full-card click entry behavior in `app/ratel/src/features/spaces/pages/index/action_dashboard/`.
- Current team Home carousel uses the same centered-card visual model with immediate full-card navigation in `app/ratel/src/features/social/pages/home/views/`.
- Existing action-related roadmap items: `roadmap/meet-action.md`.
- Product feedback from current branch discussion identifying overlay-first action navigation as a user-facing pain point.
