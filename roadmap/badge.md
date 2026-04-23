# Badge System

**Status**: Ready for design (Stage 2)
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
- **No leaderboards.** We rank activity for hot spaces; we do not rank users by badge count publicly..
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
