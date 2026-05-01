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
4. The system SHALL derive **Character Level** from Character XP via a fixed quadratic curve: `xp_required(L→L+1) = round(C · L²)` where `C = 220`. Levels start at 1. The constant `C = 220` is calibrated against the observed 10-day window (avg participant ≈ 360k SpaceXP/10d ≈ 36k/day, top ≈ 65k/day) and the PO target that **one skill must be maxable within 6 months for an avg participant**. Under this curve an avg participant reaches L10 in ~2.4 days, L30 in ~1.7 months, and the max-one-skill milestone of L45 in **~180 days (6 months)**. A top participant reaches L45 in ~99 days (~3.3 months). (Tunable single constant; see §"Open questions".)
5. The system SHALL grant **Skill Points** on each level-up. Total skill points granted at level `L` SHALL be `L` (1 SP per level, including the first). With this rate, a user can buy a single L1 skill at character level 5, two L1 skills at level 10, and reaches the endgame of one fully maxed skill at level 45 (cost 45 SP under the new max-skill-level cap). (Note: this is the *granting* curve. The *spending* curve is separate — see §"Skill points: spending". See also Q7.)
6. Character XP and the derived Level / Skill Point totals SHALL be **idempotent** on stream replay: re-processing the same `SpaceScore` MODIFY event SHALL NOT double-count XP. (Implementation: store last-seen `total_score` per (user, space) and compute delta from stored value.)

### Skill points: spending

7. Each skill SHALL have a maximum level of **6**.
8. The cost to advance a skill from level `n` to level `n+1` SHALL follow a triangular curve: `cost(n→n+1) = 5 + n` (so 5 SP for level 1, 6 SP for level 2, ..., 10 SP for level 6). Total cost to max one skill SHALL be **45 SP**.
9. The system SHALL prevent spending below 0 skill points and SHALL prevent advancing a skill above its max level.
10. A user SHALL NOT be able to refund / unspend a skill point in MVP. (See Open Question 4.)

### Skill: Money Tree (RatelPoint earning multiplier)

11. The system SHALL apply a **Money Tree multiplier** to RatelPoint amounts the user receives from any `SpaceReward::award` payout the user is the recipient of (`target_pk == user.pk`).
12. The multiplier SHALL be `1 + 0.05 · skill_level` (i.e., +5% per level, capped at +30% at level 6), applied multiplicatively to `space_reward.point × space_reward.credits` *before* the amount is recorded in `UserReward.total_points` and `User.points`.
13. The multiplier SHALL be visible to the user in the reward claim breakdown ("base 10,000 + 25% Money Tree = 12,500").
14. The multiplier SHALL NOT apply to the **creator's owner-bonus** payout. Money Tree affects only the participant's primary payout.

**Per-level benefit table** (with all timing under the C = 220 quadratic curve):

| Skill Level | Multiplier (RatelPoint) | SP cost (this lv) | SP total | Char level needed | Avg time | Top time |
|---|---|---|---|---|---|---|
| L1 | +5%  | 5  | 5  | L5  | ~5 h     | ~3 h     |
| L2 | +10% | 6  | 11 | L11 | ~2.4 d   | ~1.3 d   |
| L3 | +15% | 7  | 18 | L18 | ~11 d    | ~6 d     |
| L4 | +20% | 8  | 26 | L26 | ~34 d    | ~19 d    |
| L5 | +25% | 9  | 35 | L35 | ~84 d (~2.8 mo) | ~46 d (~1.5 mo) |
| **L6** | **+30%** | **10** | **45** | **L45** | **~180 d (6 mo)** | **~99 d (~3.3 mo)** |

Worked example: a participant at Money Tree L4 claiming a `space_reward.point × credits = 10,000` payout receives `10,000 × 1.20 = 12,000` RatelPoint; the breakdown UI shows `base 10,000 + 20% Money Tree = 12,000`.

### Skill: Ranker (XP earning multiplier)

15. The system SHALL apply a **Ranker multiplier** to the `total_score` of every new `SpaceActivity` row the user records.
16. The multiplier SHALL be `1 + 0.05 · skill_level` (same curve as Money Tree, cap +30% at level 6).
17. The multiplier SHALL apply to the activity's `additional_score` portion only, leaving `base_score` unchanged so that *which actions are valuable* remains a creator-side decision and Ranker only changes *how much* a participant earns from the same action set.
   *(Recommended — see §Q1 below.)*
18. Ranker SHALL NOT apply retroactively to existing `SpaceScore.total_score` values.

**Per-level benefit table** (timing identical to Money Tree — same SP costs, same char-level prerequisites):

| Skill Level | Multiplier (additional_score) | SP cost (this lv) | SP total | Char level needed | Avg time | Top time |
|---|---|---|---|---|---|---|
| L1 | +5%  | 5  | 5  | L5  | ~5 h     | ~3 h     |
| L2 | +10% | 6  | 11 | L11 | ~2.4 d   | ~1.3 d   |
| L3 | +15% | 7  | 18 | L18 | ~11 d    | ~6 d     |
| L4 | +20% | 8  | 26 | L26 | ~34 d    | ~19 d    |
| L5 | +25% | 9  | 35 | L35 | ~84 d (~2.8 mo) | ~46 d (~1.5 mo) |
| **L6** | **+30%** | **10** | **45** | **L45** | **~180 d (6 mo)** | **~99 d (~3.3 mo)** |

Worked example: an action with `base_score = 100` and `additional_score = 50` recorded by a participant at Ranker L4 yields `total_score = 100 + 50 × 1.20 = 160`. That `total_score` flows into both `SpaceScore.total_score` (per-space ranking) and Character XP delta (account-level progression), so Ranker compounds: more SpaceXP per action → faster character leveling → more SP for the next skill investment.

### Backfill

19. On first deploy, the system SHALL run a one-time backfill that, for every existing `SpaceScore` row, sums `total_score` per user and seeds the user's `CharacterXp.total_xp` and per-(user, space) `CharacterXpSource.last_seen_score`. The backfill SHALL be idempotent (re-running produces the same result).
20. Backfills SHALL be governed by a **versioned migration framework** rooted in a singleton DynamoDB row, `LastBackfillVersion { version: i64 }`. On server startup, when `MIGRATE=true` is set in the environment, each migration whose `required_version` is greater than the stored `version` SHALL run, and after completion the stored `version` SHALL be advanced to that migration's number. Migrations SHALL run in monotonically increasing version order.
21. When `MIGRATE` is unset or not equal to `"true"`, no migration SHALL execute. (This ensures only one designated deploy/instance per release runs the backfill, even in multi-replica deployments.)
22. Backfill code MUST be idempotent on re-execution within a single startup as well as across restarts: a partial run that crashes mid-way must converge on the correct state when re-run, even if the version row has not yet been advanced. (Implementation: the backfill scans `SpaceScore`, computes per-user totals, and *upserts* `CharacterXp` + `CharacterXpSource` — never blindly increments.)

### UI

23. There SHALL be a new page at `/me/character` (Open Question 2) showing: total Character XP, current Level, XP to next level (progress bar), unspent Skill Points, and the Skill Tree.
24. The Skill Tree view SHALL list every skill with: name, description, current level / max level, "next level cost", and a "Level up" button enabled only when the user has enough unspent SP.
25. The Character page SHALL be reachable from the user's existing profile (a "Character" tab next to "Posts" / "Spaces" / "Rewards" — exact placement decided at Stage 2 design).
26. After spending a skill point, the UI SHALL reflect the new skill level, new SP balance, and any earning-rate changes (e.g., updated Money Tree percentage in the reward breakdown) without a page reload.

## Acceptance criteria

- [ ] Earning XP in a space (e.g., voting in a poll) increases `CharacterXp.total_xp` by the same delta as `SpaceScore.total_score`.
- [ ] Stream replay of the same `SpaceScore` MODIFY event does not double-count Character XP.
- [ ] Crossing a level threshold grants the correct number of new skill points (`L` total at level L, i.e., +1 SP per level).
- [ ] Spending 5 SP on Money Tree raises it to level 1; the next claim breakdown shows the 5% bonus and `User.points` is credited the boosted amount.
- [ ] Spending 5 SP on Ranker raises it to level 1; the next `SpaceActivity` recorded has its `additional_score` boosted by 5% before aggregation.
- [ ] Attempting to advance a skill above level 6 is rejected.
- [ ] Attempting to spend more SP than the user has is rejected.
- [ ] The backfill produces the same `CharacterXp.total_xp` whether run once or three times.
- [ ] Starting the server without `MIGRATE=true` does NOT run the backfill, even if `LastBackfillVersion.version` is less than the latest migration's required version.
- [ ] Starting the server with `MIGRATE=true` after the backfill has already run (i.e., `LastBackfillVersion.version >= 1`) is a no-op — the backfill is not re-executed.
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

Documented here so the data model accommodates them without rework. Both v2 skills follow the same SP-cost curve and max level (6) as MVP skills.

### Influencer (creator-side, owned spaces)

Per skill level, lower the `MIN_PARTICIPANTS_FOR_HOT` threshold for spaces *owned by this user* (currently `10`, see `app/ratel/src/features/spaces/space_common/services/space_fanout.rs:42`). Lower threshold = easier for the user's spaces to surface in the Hot tab.

| Skill Level | MIN_PARTICIPANTS_FOR_HOT (own spaces) | SP cost | SP total | Char level needed |
|---|---|---|---|---|
| L0 (default) | 10 | — | — | — |
| L1 | 9  | 5  | 5  | L5 |
| L2 | 8  | 6  | 11 | L11 |
| L3 | 7  | 7  | 18 | L18 |
| L4 | 6  | 8  | 26 | L26 |
| L5 | 5  | 9  | 35 | L35 |
| L6 | 4  | 10 | 45 | L45 |

Floor of 4 means a maxed Influencer can launch a space and surface in Hot with only 4 participants, vs. the global 10. Spaces owned by a non-Influencer continue to use the global threshold.

### Sweeper / 싹쓸이 (creator-side, owned spaces)

Per skill level, increase the **owner bonus** that the space creator receives whenever a participant claims a reward in their space, by `5% × skill_level` on top of the existing 10% owner bonus.

| Skill Level | Owner-bonus rate (per participant claim in own space) | SP cost | SP total | Char level needed |
|---|---|---|---|---|
| L0 (default) | 10%      | — | — | — |
| L1 | 15% | 5  | 5  | L5 |
| L2 | 20% | 6  | 11 | L11 |
| L3 | 25% | 7  | 18 | L18 |
| L4 | 30% | 8  | 26 | L26 |
| L5 | 35% | 9  | 35 | L35 |
| L6 | 40% | 10 | 45 | L45 |

Worked example: a maxed Sweeper hosts a space; a participant claims a 10,000-point reward; the participant receives 10,000 (or boosted by their own Money Tree), and the creator receives an extra 4,000 owner bonus instead of the default 1,000.

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
   - **Recommended: cap owner-bonus at 40% total (10% base + 5% per level × 6 levels = +30%, total 40%).** Updated from earlier 60% target after the max-skill-level was reduced from 10 to 6 to hit the 6-month one-skill-max goal.
   - This is v2 territory — answer is not blocking MVP, but please confirm direction.

7. **(Q7) SP grant rate per character level.**
   - **Decided: 1 SP per level (`total_sp_granted = L`).** PO directive. With triangular skill cost (5+n), this gives:
     - L5 → first skill at L1 (5 SP, ~7h for avg participant)
     - L10 → both skills at L1 (10 SP, ~5 days)
     - L16 → MoneyTree L2 + Ranker L1 (16 SP, ~3.5 weeks)
     - L23 → both skills at L2 (22 SP, ~3 months)
     - L95 → one skill maxed at L10 (true endgame)
   - This intentionally makes higher skill tiers a long-horizon goal, so a participant who has been around for a year has a tangible mechanical edge over a one-month account.
   - **PO override would be:** larger grant if telemetry shows the early game feels too gated.

8. **(Q8) XP curve steepness and shape.**
   - **Decided: quadratic `C · L²` with `C = 220`.** PO directive — one skill must be maxable in **≤ 6 months for an avg participant**. With max-skill at L45 (Q9) and avg activity ≈ 36k XP/day, the calibration is `cumulative_xp(45) ≈ 36k × 180 → C ≈ 220`. Earlier draft used `C = 600`, which put L45 at ~17 months. See worked-numbers table in §"Leveling math" of the design doc.
   - **PO override would be:** raise to `C = 300` (max-skill in ~8 months) or drop to `C = 150` (max-skill in ~4 months) after first-week telemetry.

9. **(Q9) Max skill level.**
   - **Decided: 6 (down from 10).** PO directive — paired with the 6-month one-skill-max target. Triangular cost (5+n) for n=0..5 gives total 45 SP to max one skill. Multiplier cap stays at +5%/lv → +30% at level 6.
   - **PO override would be:** keep max level 10 (cap +50%) if the +30% endgame multiplier feels underwhelming after first players reach it. Note: raising max level requires re-tuning `C` to keep the 6-month one-skill-max goal.

## References

- `app/ratel/src/features/activity/models/space_score.rs` — current per-space user XP entity.
- `app/ratel/src/features/activity/services/aggregate_score.rs` — the existing stream handler that materializes `SpaceScore` from `SpaceActivity`. The Character XP path will hook here.
- `app/ratel/src/common/stream_handler.rs` — central stream dispatch on sk prefix.
- `app/ratel/src/features/spaces/space_common/models/space_reward.rs` — `SpaceReward::award_if_configured` is the entry point Money Tree multiplies on.
- `app/ratel/src/features/spaces/space_common/services/space_fanout.rs:42` — `MIN_PARTICIPANTS_FOR_HOT` is the threshold v2's Influencer skill loosens.
- `roadmap/badge.md` — same "cross-space, account-level recognition" theme; treat as sibling, not competitor (badges = recognition, no economic effect; skills = economic effect, no public catalog).
