# Character XP & Skill Tree

**Status**: Draft — pending PO review (Stage 1)
**Slug**: `character-xp-skills`
**Primary use case**: Give every Ratel user a persistent, cross-space progression layer (Character XP → Level → Skill Points → upgradable passive skills) so participation in any space compounds into a personal account-wide identity, not a per-space leaderboard rank.

## Problem

XP today is **per-space**. `SpaceScore.total_score` accrues from polls / quizzes / discussions / follows inside one space and is used for that space's ranking, but the moment a participant leaves the space, none of that effort follows them. The user's profile shows `points` (RatelPoint balance, claimable in monthly pools) but **no XP, no level, no progression curve**. Two consequences:

1. **No long-term hook.** A power user who has participated in 30 spaces over six months looks identical on their profile to a user who joined yesterday. There is no "I've been here a while and it shows" signal anywhere on the account.
2. **Earning rate is fixed for everyone.** Every participant earns the same RatelPoint payout per action and the same SpaceXP per activity, regardless of how invested they are. There is no way for a long-term participant to compound their previous effort into *better future* earning rates — only into a higher per-space leaderboard rank that resets with every new space they join.

Games solve this with the canonical *XP → Level → Skill Tree* loop: activity feeds a single account-wide XP pool, level-ups grant skill points, skill points unlock passive boosts that change the next session's earning rate. The boosts give returning power users a tangible mechanical advantage that new users have to grind to match. Ratel already has all the raw activity data (`SpaceActivity`, `SpaceScore`, `UserReward`); it just doesn't aggregate them into an account-level progression.

## Goal

Add an account-level **Character XP** that is the sum of every space's `SpaceScore` the user has accumulated, derive a **Character Level** from it via a fixed curve, grant **Skill Points** on level-up, and let users spend Skill Points on a small **Skill Tree** of passive boosts that affect future XP and RatelPoint earnings.

MVP ships with **two skills** (Money Tree, Ranker — both participant-side passive multipliers). Two further skills (Influencer, Sweeper — creator-side benefits) are designed but explicitly deferred to v2.

## Non-goals

- **No PvP / combat / classes.** "Character" here means *progression character*, not RPG character. There are no stats other than XP / Level / Skill Points and no abilities other than the listed passive skills.
- **No XP loss / decay / negative levels.** Character XP is monotonic. Deleting a space, leaving a space, or admin-deleting a participant's score does **not** debit Character XP that has already been counted. (See Open Question 3 — confirm.)
- **No skill respec in MVP.** Once a skill point is spent, it is committed. Respec UI / cost can be added later if balance demands it.
- **No paid skill points / no skill-point IAP.** Skill points are earned only by leveling up; there is no shop where they can be bought with RatelPoint, real currency, or membership tier.
- **No retroactive RatelPoint backfill.** Any past activity counts toward Character XP via a one-time backfill from existing `SpaceScore` rows, but RatelPoint amounts already paid out are **not** retroactively boosted by Money Tree.
- **No leaderboards by Character Level in MVP.** Level is shown only on the user's own profile and (Open Question 5) optionally on public profile views. There is no global "top level" page.
- **No Influencer / Sweeper skills in MVP.** Both creator-side skills are designed in §"Skills v2 (deferred)" but ship in a follow-up roadmap item.

## User stories

### Participant (primary)

- As a logged-in user, I want to see **my Character XP, current level, progress to the next level, and unspent skill points** on a dedicated profile page, so that participation feels like a long-running account-level achievement, not 30 disconnected per-space ranks.
- As a logged-in user, I want each XP-earning action in any space to also visibly increment my Character XP, so I can see one progression bar grow no matter which space I'm in.
- As a logged-in user with unspent skill points, I want to see the skill tree, read each skill's effect and per-level cost, and spend points on the skills I prefer.
- As a user who has invested in **Money Tree**, I want my next reward claim from any space to pay out a higher RatelPoint amount than an un-skilled user would receive for the same action, with the boost shown in the claim breakdown.
- As a user who has invested in **Ranker**, I want my next quiz / poll / discussion in any space to award more SpaceXP (and therefore more Character XP) than an un-skilled user would receive.

### Viewer (someone else's profile) — Open Question 5

- As a visitor to another user's profile, I want to see their Character Level (but not their skill build) so I can size up how active the account has been.

## Functional requirements

Numbered, testable. Each SHALL be verifiable by an automated test or a documented manual check.

### XP & Level

1. The system SHALL maintain a per-user **Character XP** counter that is the **sum of `SpaceScore.total_score` across every space the user has ever participated in**.
2. When a `SpaceScore` row is created or updated for a user (currently driven by the DynamoDB Stream → `aggregate_score` pipeline), the system SHALL increment the same user's Character XP by the **delta** (`new_total_score − old_total_score`), never by the absolute value.
3. The Character XP increment SHALL run inside the same stream-handler dispatch as `aggregate_score`, so a user's per-space rank and account-level progression update on the same event.
4. The system SHALL derive **Character Level** from Character XP via a fixed cubic curve: `xp_required(L) = round(C · L³)` where `C` is a tunable constant chosen so that level 5 ≈ a typical first-time participant's first-week activity. Levels start at 1.
5. The system SHALL grant **Skill Points** on each level-up. Total skill points granted at level `L` SHALL be `5 × L` (5 SP per level, including the first). With this rate, a user can fully max one skill at level 19 (95 SP) and two skills at level 38 (190 SP). (Note: this is the *granting* curve. The *spending* curve is separate — see §"Skill points: spending". See also Q7.)
6. Character XP and the derived Level / Skill Point totals SHALL be **idempotent** on stream replay: re-processing the same `SpaceScore` MODIFY event SHALL NOT double-count XP. (Implementation: store last-seen `total_score` per (user, space) and compute delta from stored value.)

### Skill points: spending

7. Each skill SHALL have a maximum level of 10.
8. The cost to advance a skill from level `n` to level `n+1` SHALL follow a triangular curve: `cost(n→n+1) = 5 + n` (so 5 SP for level 1, 6 SP for level 2, ..., 14 SP for level 10). Total cost to max one skill SHALL be **95 SP**.
9. The system SHALL prevent spending below 0 skill points and SHALL prevent advancing a skill above its max level.
10. A user SHALL NOT be able to refund / unspend a skill point in MVP. (See Open Question 4.)

### Skill: Money Tree (RatelPoint earning multiplier)

11. The system SHALL apply a **Money Tree multiplier** to RatelPoint amounts the user receives from any `SpaceReward::award` payout the user is the recipient of (`target_pk == user.pk`).
12. The multiplier SHALL be `1 + 0.05 · skill_level` (i.e., +5% per level, capped at +50% at level 10), applied multiplicatively to `space_reward.point × space_reward.credits` *before* the amount is recorded in `UserReward.total_points` and `User.points`.
13. The multiplier SHALL be visible to the user in the reward claim breakdown ("base 10,000 + 25% Money Tree = 12,500").
14. The multiplier SHALL NOT apply to the **creator's owner-bonus** payout. Money Tree affects only the participant's primary payout.

### Skill: Ranker (XP earning multiplier)

15. The system SHALL apply a **Ranker multiplier** to the `total_score` of every new `SpaceActivity` row the user records.
16. The multiplier SHALL be `1 + 0.05 · skill_level` (same curve as Money Tree).
17. The multiplier SHALL apply to the activity's `additional_score` portion only, leaving `base_score` unchanged so that *which actions are valuable* remains a creator-side decision and Ranker only changes *how much* a participant earns from the same action set.
   *(Recommended — see §Q1 below.)*
18. Ranker SHALL NOT apply retroactively to existing `SpaceScore.total_score` values.

### Backfill

19. On first deploy, the system SHALL run a one-time backfill that, for every existing `SpaceScore` row, sums `total_score` per user and seeds the user's `CharacterXp.total_xp`. The backfill SHALL be idempotent (re-running produces the same result) and runnable as an admin migration.

### UI

20. There SHALL be a new page at `/me/character` (Open Question 2) showing: total Character XP, current Level, XP to next level (progress bar), unspent Skill Points, and the Skill Tree.
21. The Skill Tree view SHALL list every skill with: name, description, current level / max level, "next level cost", and a "Level up" button enabled only when the user has enough unspent SP.
22. The Character page SHALL be reachable from the user's existing profile (a "Character" tab next to "Posts" / "Spaces" / "Rewards" — exact placement decided at Stage 2 design).
23. After spending a skill point, the UI SHALL reflect the new skill level, new SP balance, and any earning-rate changes (e.g., updated Money Tree percentage in the reward breakdown) without a page reload.

## Acceptance criteria

- [ ] Earning XP in a space (e.g., voting in a poll) increases `CharacterXp.total_xp` by the same delta as `SpaceScore.total_score`.
- [ ] Stream replay of the same `SpaceScore` MODIFY event does not double-count Character XP.
- [ ] Crossing a level threshold grants the correct number of new skill points (`5 × L` total at level L, i.e., +5 SP per level).
- [ ] Spending 5 SP on Money Tree raises it to level 1; the next claim breakdown shows the 5% bonus and `User.points` is credited the boosted amount.
- [ ] Spending 5 SP on Ranker raises it to level 1; the next `SpaceActivity` recorded has its `additional_score` boosted by 5% before aggregation.
- [ ] Attempting to advance a skill above level 10 is rejected.
- [ ] Attempting to spend more SP than the user has is rejected.
- [ ] The backfill admin migration produces the same `CharacterXp.total_xp` whether run once or three times.
- [ ] The `/me/character` page shows total XP, level, XP to next level, and unspent SP, all updating live as new activities post.
- [ ] A user with no past activity who is brand new sees Level 1 and 0 unspent SP after the level-up bookkeeping (i.e., they get their level-1 SP grant on first appearance).
- [ ] A user can see their Character Level on another user's public profile (assuming Open Question 5 resolves "yes").

## Constraints

- **DynamoDB single-table design.** New entities follow `Partition` + `EntityType` conventions and use `#[dynamo(prefix)]` per `conventions/dynamo-prefix-convention.md`.
- **Stream-driven, not polled.** Character XP must update in the same place `SpaceScore` does — `stream_handler.rs` SPACE_SCORE# (or SPACE_ACTIVITY#) branch — so there is no second source of truth and no polling job.
- **Idempotent on replay.** EventBridge / DynamoDB Streams can deliver the same record more than once. The XP increment path must be safe under re-delivery.
- **No new external services.** Skills, Character XP, and Skill Points all live in the existing single DynamoDB table. No Redis, no Postgres, no third-party progression service.
- **Backfill must be safe under load.** The one-time migration runs in batches with a clear stop condition; it must not stall the table or the stream pipeline.
- **MVP ships behind no feature flag.** This feature is additive (no behavior changes for users with 0 skill points) — gating it would only complicate the rollout.

## Skills v2 (deferred — not in MVP scope)

Documented here so the data model accommodates them without rework.

- **Influencer (creator-side, owned spaces).** Per skill level, lower the `MIN_PARTICIPANTS_FOR_HOT` threshold for spaces *owned by this user* (currently `10`, see `app/ratel/src/features/spaces/space_common/services/space_fanout.rs:42`) by 1 per level, floored at 5. Effect: easier for a high-Influencer user's spaces to surface in the Hot tab.
- **Sweeper / 싹쓸이 (creator-side, owned spaces).** Per skill level, increase the **owner bonus** that the space creator receives whenever a participant claims a reward in their space, by `5% × skill_level` on top of the existing 10% owner bonus. Effect: a maxed Sweeper takes 60% of every participant payout instead of 10%. (Cap and exact curve to be confirmed; see Open Question 6.)

The data model below stores skills as a generic `(skill_id, level)` map, so adding Influencer/Sweeper later is purely additive.

## Open questions / decisions

These are the items we want PO sign-off on before Stage 2 design starts. Each lists the **recommended** choice with reasoning so the spec can move forward as-is unless the PO overrides.

1. **(Q1) Ranker multiplier applies to `additional_score` only, or to the full `total_score`?**
   - **Recommended: `additional_score` only.** `base_score` is the creator-set "this action is worth N XP" baseline; `additional_score` is the bonus for engagement quality (e.g., long discussion replies). Boosting only the bonus keeps the per-action floor stable and makes Ranker reward *quality* engagement. If we boost the full total, a Ranker player can drown out non-Ranker players in raw activity volume.
   - **PO override would be:** "boost full `total_score`" if we want Ranker to be more obviously valuable to the casual user.

2. **(Q2) Page route: `/me/character` vs. tab on existing profile.**
   - **Recommended: tab on the existing profile page.** Lower navigation cost; profile is already where "who am I as a Ratel user" lives. Route would be `/<username>/character` for self-view and visitor-view (visitor sees less info — see Q5).
   - **PO override would be:** dedicated `/me/character` route if we want the page to be more game-y / standalone.

3. **(Q3) XP monotonic, or follows `SpaceScore` deletions.**
   - **Recommended: monotonic. Deleting a space or admin-removing a `SpaceScore` row does NOT debit Character XP.** Matches the badge-system precedent ("earning is terminal") and avoids a nasty replay-attack class where a user's XP could go negative on a deletion replay.
   - **PO override would be:** strict mirror of `SpaceScore` (XP can decrease) if compliance / abuse-prevention requires it.

4. **(Q4) Respec / refund spent skill points.**
   - **Recommended: not in MVP. Add later if balance feedback demands it.** Keeps the math simple, removes a UI surface, and respec systems generally need a cost (RatelPoint, time-locked) which is its own design conversation.
   - **PO override would be:** include a one-time free respec at launch, or a paid respec at 5,000 RatelPoint.

5. **(Q5) Character Level visible on other users' public profiles?**
   - **Recommended: yes, level only — not skill build, not XP number.** Matches the badge-system "social signal" goal. Hiding it entirely makes the system feel hidden and weakens the long-term-account incentive.
   - **PO override would be:** hidden from public view (only on `/me/character`).

6. **(Q6) Sweeper cap.**
   - **Recommended: cap owner-bonus at 60% total (10% base + 5% per level × 10 levels = +50%, total 60%).** Beyond that the participant payout starts to feel hollowed out.
   - This is v2 territory — answer is not blocking MVP, but please confirm direction.

7. **(Q7) SP grant rate per character level.**
   - **Recommended: 5 SP per level (`total = 5 × L`).** With triangular skill cost (5+n), this lets a user max one skill at L19 and two skills at L38, which feels achievable on a multi-month account horizon. The original draft had `4 + L` (1 SP/level after the first), which would have required L91 to max even one skill — too slow.
   - **PO override would be:** larger grant (e.g., 10 SP/level → max 1 skill at L10) for a faster early game, or smaller grant (e.g., 3 SP/level) for a slower / more long-term progression.

## References

- `app/ratel/src/features/activity/models/space_score.rs` — current per-space user XP entity.
- `app/ratel/src/features/activity/services/aggregate_score.rs` — the existing stream handler that materializes `SpaceScore` from `SpaceActivity`. The Character XP path will hook here.
- `app/ratel/src/common/stream_handler.rs` — central stream dispatch on sk prefix.
- `app/ratel/src/features/spaces/space_common/models/space_reward.rs` — `SpaceReward::award_if_configured` is the entry point Money Tree multiplies on.
- `app/ratel/src/features/spaces/space_common/services/space_fanout.rs:42` — `MIN_PARTICIPANTS_FOR_HOT` is the threshold v2's Influencer skill loosens.
- `roadmap/badge.md` — same "cross-space, account-level recognition" theme; treat as sibling, not competitor (badges = recognition, no economic effect; skills = economic effect, no public catalog).
