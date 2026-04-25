# Badge System

**Status**: Ready for development (Stage 3)
**Slug**: `badge`
**Primary use case**: Motivate recurring civic participation by recognizing public activity with non-transferable badges

## Problem

Ratel rewards **spaces** — a participant completes a space's quests, gets credit, and leaves. There's nothing that ties a user's activity **across spaces** together or signals "this person is a regular" to either the user themselves or to others viewing their profile. The closest signal today is the `points` field on `User`.

Concrete pain points the team has surfaced:

- Users who have participated in dozens of spaces look identical on their profile to users who joined yesterday.
- There's no lightweight "streak / milestone" feedback loop to pull a user back for a second or third session after the first reward is claimed.
- Space creators and followers have no at-a-glance heuristic for "is this account real / engaged / new?" when deciding whether to trust them.
- Non-monetary recognition is absent — everything boils down to credit (CR). Users who care about the civic mission more than the credit payout have nothing to show for it.

Products like GitHub (Achievements, profile trophies), Duolingo (streak + league badges), and Stack Overflow (badges tied to specific activity thresholds) demonstrate that cheap, non-transferable recognition is a surprisingly durable motivator. Ratel has all the raw activity data already; it's just not surfaced as recognition.

## Goal

Give every Ratel user a **badge ledger** on their profile that grows as their public activity crosses predefined thresholds, and design the badge catalog so it recognizes the full arc of civic participation (joining → deliberating → creating → networking → staying consistent). Badges are **recognition only** — they **do not grant CR or any other in-app economic benefit**.

## Non-goals

- **No CR / points / monetary reward.** Earning a badge is its own reward. The existing `user.points` counter is unaffected by badges and continues to reflect only space-reward credit.
- **No trading / gifting / selling.** Badges are bound to the user account that earned them; there is no secondary market, transfer, or "gift-to-friend" flow.
- **No user-defined custom badges in Phase 1.** The catalog is author-controlled (platform-defined) so we can reason about consistency and abuse. User-defined achievements are a Phase 2 consideration.
- **No retroactive revocation on downward activity.** If a user earns "10 spaces joined" and later deletes an account they had voted in, the badge does not un-award itself. Badges are monotonic: earning is terminal.
- **No per-team or per-space badge scopes in Phase 1.** Badges are global (whole-Ratel) only.
- **No leaderboards.** We rank activity for hot spaces; we do not rank users by badge count publicly.
- **No ad-hoc admin backfills.** Criteria are data-driven evaluations, not staff-issued honorifics. (One exception: time-boxed "Special" badges tied to a launch window; those are parameterized by date range, not by name.)
- **No notifications spam.** At most one notification per newly-earned badge, and only to the earner — never to the user's followers.

## User stories

### Viewer (self)

- As a logged-in user, I want to see **which badges I have earned and which I have not** on my profile, so I know what kinds of activity Ratel recognizes.
- As a logged-in user, I want to see **progress toward the next badge** (e.g., "8 / 10 spaces joined") so that I have a concrete next step.
- As a logged-in user, I want to see **when and why** a badge was awarded (date + a sentence explaining the criterion) so it's legible and shareable.
- As a logged-in user, I want to receive an **in-app notification the moment I earn a new badge**, so the reward loop feels immediate.
- As a logged-in user, I want badges to render **on my existing profile page** next to my stats, not tucked away in a settings subsection.

### Viewer (of another user)

- As a citizen considering whether to trust a comment or follow a profile, I want to see **what badges that user has** so I can quickly assess their engagement level.
- As a visitor to a public profile, I want to see only **earned** badges of the viewed user.

## Functional requirements

### Catalog & criteria

1. The system SHALL maintain a platform-defined badge catalog organized into exactly five categories: Participation, Creator, Social, Achievement, Special.
2. Each catalog entry SHALL specify: name, category, rarity (`legendary` | `rare` | `common`), criterion description (one sentence), threshold (integer or boolean condition), icon, and an optional time window (Special badges only).
3. The catalog SHALL be code-resident (a Rust enum / constant) — not a user-editable database table — so changes ship via deploy.

### Awarding

4. When a user's measurable activity crosses a badge's threshold, the system SHALL award that badge to that user **exactly once** and record the award timestamp.
5. The system SHALL NOT revoke an awarded badge if the underlying counter later decreases (badges are monotonic).
6. Special badges with a time window SHALL only award when the qualifying activity occurred within the configured window. Users who satisfy the criterion outside the window SHALL NOT be eligible.
7. When a badge is awarded, the system SHALL emit exactly one in-app notification to the earner only.

### Self-view (Trophy Vault page)

8. A logged-in user SHALL be able to navigate to a "Trophy Vault" page that shows the entire catalog (earned + unearned).
9. The page SHALL display a hero with `{earned} / {total}` count and a progress ring sized to that ratio.
10. The page SHALL provide filter chips per category — All, Participation, Creator, Social, Achievement, Special — each showing that category's total count.
11. Each badge SHALL render as a hex medallion with the rarity-coloured frame, a banner with the badge name, and a hover tooltip containing the criterion text.
12. Earned badges SHALL show the award date in relative form for awards within the last 24 hours ("2h ago"), and absolute "MMM DD" for older awards.
13. Unearned badges SHALL render desaturated with a lock overlay, a progress bar filled to `progress / threshold`, and a meta line of the form `X / Y · Z to go`. When `progress >= threshold` but the badge has not yet been awarded, the meta line SHALL read `Unlock pending`.

### Self-view (Profile integration)

14. The user's own profile page SHALL include a "Trophy Case" section containing a 6-up strip of the most recently earned badges, ordered by award timestamp descending.
15. The Trophy Case section SHALL show the `{earned} / {total}` summary alongside the section heading and a "View all" link to the Trophy Vault page.

### Other-user view

16. When viewing another user's profile, the Trophy Case SHALL display only that user's earned badges (max 6, most recent first); locked / in-progress badges SHALL NOT be visible.
17. The "View all" link on another user's profile SHALL navigate to that user's read-only Trophy Vault, which shows only earned badges and SHALL NOT show progress for unearned badges.

### Notification & activity feed

18. A badge award SHALL appear in the earner's activity feed (`Recent Signal`) as a row with the rose-coloured badge icon, headline `Earned badge <strong>{name}</strong> — {criterion}`, and subtitle `{rarity capitalized} rarity · {category}`.
19. The notification triggered by an award SHALL deep-link the earner to the Trophy Vault page.

### Catalog Phase 1

20. The Phase 1 catalog SHALL include at least the badges illustrated in the design across all five categories. The final list lives in code; the design files (`badges.html`, `profile.html`) are the visual contract for naming, rarity, and criterion text.

## Acceptance criteria

- [ ] A logged-in user can navigate from their profile to a "Trophy Vault" page via the "View all" link.
- [ ] The Trophy Vault page renders all 5 categories with the badge counts shown in the design.
- [ ] Each badge in the vault shows: rarity-themed frame, name banner, criterion tooltip on hover.
- [ ] Earned badges display the award date; unearned badges display a progress bar and `X / Y · Z to go` meta line.
- [ ] Filter chips for each category narrow the vault to badges in that category only.
- [ ] When a user crosses a badge threshold, they receive one in-app notification with the badge name, and the badge appears in their Trophy Case strip on their profile.
- [ ] The Trophy Case on the user's own profile shows up to 6 most recently earned badges plus the `X / Y earned` summary.
- [ ] Visiting another user's profile shows their Trophy Case (earned only); the read-only Trophy Vault for that user shows no progress on unearned badges.
- [ ] The activity feed shows a badge-award entry within seconds of the award being granted.
- [ ] A badge once earned is never removed when the underlying counter decreases.
- [ ] A Special "Founding Voice" badge is only awarded to users who joined within the configured launch window; users joining after the window do not receive it even if they meet other criteria.

## Constraints

- **Performance**: The Trophy Vault page SHALL load within 1.5s (p95) for a user with the full catalog.
- **Async evaluation**: Badge-award evaluation SHALL run asynchronously. The user-facing action that triggers the award (vote, comment, etc.) SHALL NOT block on badge calculation.
- **Notification latency**: A badge notification SHALL appear in the user's inbox within 60 seconds of the qualifying action.
- **Storage**: A user's badge ledger is append-only; storage SHALL scale linearly with the total catalog size, not with user activity.
- **Catalog scale**: Phase 1 catalog is bounded to ~50 badges total. Larger catalogs are out of scope.
- **Infrastructure**: Badge awarding logic runs entirely on Ratel infrastructure (DynamoDB + EventBridge). No external paid achievement service.
- **Privacy**: Only earned badges are visible on another user's profile. Progress toward unearned badges is private to the earner.

## Open questions

- **Streak counting (`Marathoner`)**: The Marathoner badge requires "participate every day for 30 days". Stage 3 must define what counts as "participate" (any action vs. specific actions like vote/comment) and how the streak resets on a missed day (hard reset vs grace period).
- **Featured-on-Hot detection (`Signal Boost`)**: The Signal Boost badge requires being featured on the home Hot list. The Hot-list feature does not yet emit a stable event we can listen for. Stage 3 must either (a) wait for Hot-list to expose an event, or (b) gate this badge behind a temporary admin trigger.
- **Feedback tracking entity (`Beta Pilot`)**: The Beta Pilot badge requires submitting 3 bug reports or feedback items. Ratel does not yet have a `Feedback` entity that records user-submitted bug reports. Stage 3 must decide whether to add one (which then unlocks both this badge and a future "Submit feedback" UI) or to defer this badge to Phase 2.
- **Backfill at launch**: Should historical activity (votes cast, spaces joined before launch) count toward badges? Default assumption: backfill once at launch, then evaluate forward.

## References

- **Design (mockups)**: [`app/ratel/assets/design/badge/`](../app/ratel/assets/design/badge/)
  - `badges.html` — full Trophy Vault catalog page
  - `profile.html` — Trophy Case strip on user profile
- **Existing surfaces this feature integrates with**:
  - `User` model (`app/ratel/src/features/users/`) — badge ledger lives alongside user data
  - Notifications inbox (`app/ratel/src/features/notifications/`) — badge awards emit one notification
  - Profile page — Trophy Case strip is a new section
- **Comparable products**: GitHub Achievements, Duolingo streak/league, Stack Overflow badges
