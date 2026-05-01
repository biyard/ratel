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
5. The system SHALL grant **Skill Points** on each level-up. Total skill points granted at level `L` SHALL be `5 · L` (5 SP per level, including the first). With this rate, a user can buy their first skill at L1 (5 SP) immediately on account creation, both skills at L1 by character level 2, and reaches the endgame of one fully maxed skill at character level 46 (cost 230 SP under the steeper cost curve). (Note: this is the *granting* curve. The *spending* curve is separate — see §"Skill points: spending". See also Q7.)
6. Character XP and the derived Level / Skill Point totals SHALL be **idempotent** on stream replay: re-processing the same `SpaceScore` MODIFY event SHALL NOT double-count XP. (Implementation: store last-seen `total_score` per (user, space) and compute delta from stored value.)

### Skill points: spending

7. Each skill SHALL have a maximum level of **10**.
8. The cost to advance a skill from level `n` to level `n+1` SHALL follow a steeper-than-triangular curve: `cost(n→n+1) = 5 + 4·n` (so 5 SP for level 1, 9 SP for level 2, 13 for level 3, ..., 41 SP for level 10). The L1 entry cost is intentionally low to keep the choice between Money Tree and Ranker open early; from L2 onward the per-level jump is `+4 SP`, four times steeper than the previously-considered `+1 SP` triangular curve. Total cost to max one skill SHALL be **230 SP**.
9. The system SHALL prevent spending below 0 skill points and SHALL prevent advancing a skill above its max level.
10. A user SHALL NOT be able to refund / unspend a skill point in MVP. (See Open Question 4.)

### Skill: Money Tree (RatelPoint earning multiplier)

11. The system SHALL apply a **Money Tree multiplier** to RatelPoint amounts the user receives from any `SpaceReward::award` payout the user is the recipient of (`target_pk == user.pk`).
12. The multiplier SHALL be `1 + 0.05 · skill_level` (i.e., +5% per level, capped at +50% at level 10), applied multiplicatively to `space_reward.point × space_reward.credits` *before* the amount is recorded in `UserReward.total_points` and `User.points`.
13. The multiplier SHALL be visible to the user in the reward claim breakdown ("base 10,000 + 25% Money Tree = 12,500").
14. The multiplier SHALL NOT apply to the **creator's owner-bonus** payout. Money Tree affects only the participant's primary payout.

**Per-level benefit table** (with all timing under `C = 220` and SP grant `5 · L`):

| Skill Level | Multiplier (RatelPoint) | SP cost (this lv) | SP total | Char level needed | Avg time | Top time |
|---|---|---|---|---|---|---|
| L1  | +5%  | 5  | 5   | L1  | day 0           | day 0           |
| L2  | +10% | 9  | 14  | L3  | ~45 min         | ~25 min         |
| L3  | +15% | 13 | 27  | L6  | ~4 h            | ~2 h            |
| L4  | +20% | 17 | 44  | L9  | ~15 h           | ~8 h            |
| L5  | +25% | 21 | 65  | L13 | ~2 d            | ~1.1 d          |
| L6  | +30% | 25 | 90  | L18 | ~5.5 d          | ~3 d            |
| L7  | +35% | 29 | 119 | L24 | ~13 d           | ~7 d            |
| L8  | +40% | 33 | 152 | L31 | ~29 d (~1 mo)   | ~16 d           |
| L9  | +45% | 37 | 189 | L38 | ~54 d (~1.8 mo) | ~30 d           |
| **L10** | **+50%** | **41** | **230** | **L46** | **~192 d (~6.4 mo)** | **~106 d (~3.5 mo)** |

Worked example: a participant at Money Tree L4 claiming a `space_reward.point × credits = 10,000` payout receives `10,000 × 1.20 = 12,000` RatelPoint; the breakdown UI shows `base 10,000 + 20% Money Tree = 12,000`. At max (L10), the same payout becomes `15,000` (+50%, the "1.5×" endgame target).

### Skill: Ranker (XP earning multiplier)

15. The system SHALL apply a **Ranker multiplier** to the `total_score` of every new `SpaceActivity` row the user records.
16. The multiplier SHALL be `1 + 0.05 · skill_level` (same curve as Money Tree, cap +50% at level 10 — i.e., 1.5× XP at max).
17. The multiplier SHALL apply to the activity's `additional_score` portion only, leaving `base_score` unchanged so that *which actions are valuable* remains a creator-side decision and Ranker only changes *how much* a participant earns from the same action set.
   *(Recommended — see §Q1 below.)*
18. Ranker SHALL NOT apply retroactively to existing `SpaceScore.total_score` values.

**Per-level benefit table** (timing identical to Money Tree — same SP costs, same char-level prerequisites):

| Skill Level | Multiplier (additional_score) | SP cost (this lv) | SP total | Char level needed | Avg time | Top time |
|---|---|---|---|---|---|---|
| L1  | +5%  | 5  | 5   | L1  | day 0           | day 0           |
| L2  | +10% | 9  | 14  | L3  | ~45 min         | ~25 min         |
| L3  | +15% | 13 | 27  | L6  | ~4 h            | ~2 h            |
| L4  | +20% | 17 | 44  | L9  | ~15 h           | ~8 h            |
| L5  | +25% | 21 | 65  | L13 | ~2 d            | ~1.1 d          |
| L6  | +30% | 25 | 90  | L18 | ~5.5 d          | ~3 d            |
| L7  | +35% | 29 | 119 | L24 | ~13 d           | ~7 d            |
| L8  | +40% | 33 | 152 | L31 | ~29 d (~1 mo)   | ~16 d           |
| L9  | +45% | 37 | 189 | L38 | ~54 d (~1.8 mo) | ~30 d           |
| **L10** | **+50%** | **41** | **230** | **L46** | **~192 d (~6.4 mo)** | **~106 d (~3.5 mo)** |

Worked example: an action with `base_score = 100` and `additional_score = 50` recorded by a participant at Ranker L4 yields `total_score = 100 + 50 × 1.20 = 160`. At max Ranker (L10), the same action yields `total_score = 100 + 50 × 1.50 = 175`. That `total_score` flows into both `SpaceScore.total_score` (per-space ranking) and Character XP delta (account-level progression), so Ranker compounds: more SpaceXP per action → faster character leveling → more SP for the next skill investment.

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

- [x] Earning XP in a space (e.g., voting in a poll) increases `CharacterXp.total_xp` by the same delta as `SpaceScore.total_score`.
- [x] Stream replay of the same `SpaceScore` MODIFY event does not double-count Character XP.
- [x] Crossing a level threshold grants the correct number of new skill points (`5 · L` total at level L, i.e., +5 SP per level).
- [x] Spending 5 SP on Money Tree raises it to level 1; the next claim breakdown shows the 5% bonus and `User.points` is credited the boosted amount.
- [x] Spending 5 SP on Ranker raises it to level 1; the next `SpaceActivity` recorded has its `additional_score` boosted by 5% before aggregation.
- [x] Attempting to advance a skill above level 10 is rejected.
- [x] Skill cost ramps `5, 9, 13, 17, 21, 25, 29, 33, 37, 41` with cumulative `5, 14, 27, 44, 65, 90, 119, 152, 189, 230` — verified by direct unit test.
- [x] Attempting to spend more SP than the user has is rejected.
- [x] The backfill produces the same `CharacterXp.total_xp` whether run once or three times.
- [x] Starting the server without `MIGRATE=true` does NOT run the backfill, even if `LastBackfillVersion.version` is less than the latest migration's required version.
- [x] Starting the server with `MIGRATE=true` after the backfill has already run (i.e., `LastBackfillVersion.version >= 1`) is a no-op — the backfill is not re-executed.
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

Documented here so the data model accommodates them without rework. Both v2 skills follow the same SP-cost curve and max level (10) as MVP skills.

### Influencer (creator-side, owned spaces)

Per skill level, lower the `MIN_PARTICIPANTS_FOR_HOT` threshold for spaces *owned by this user* (currently `10`, see `app/ratel/src/features/spaces/space_common/services/space_fanout.rs:42`). Lower threshold = easier for the user's spaces to surface in the Hot tab. Floored at 1 (a maxed Influencer's brand-new solo space can surface in Hot from the moment of publish).

| Skill Level | MIN_PARTICIPANTS_FOR_HOT (own spaces) | SP cost | SP total | Char level needed |
|---|---|---|---|---|
| L0 (default) | 10 | — | — | — |
| L1  | 9 | 5  | 5   | L1  |
| L2  | 8 | 9  | 14  | L3  |
| L3  | 7 | 13 | 27  | L6  |
| L4  | 6 | 17 | 44  | L9  |
| L5  | 5 | 21 | 65  | L13 |
| L6  | 4 | 25 | 90  | L18 |
| L7  | 3 | 29 | 119 | L24 |
| L8  | 2 | 33 | 152 | L31 |
| L9  | 1 | 37 | 189 | L38 |
| L10 | 1 | 41 | 230 | L46 |

(L9 already hits the floor; L10 adds no further threshold reduction. If kept this way, L10 becomes a *prestige* tier with no marginal effect — see §"Open questions" Q10.)

### Sweeper / 싹쓸이 (creator-side, owned spaces)

Per skill level, increase the **owner bonus** that the space creator receives whenever a participant claims a reward in their space, by `5% × skill_level` on top of the existing 10% owner bonus.

| Skill Level | Owner-bonus rate (per participant claim in own space) | SP cost | SP total | Char level needed |
|---|---|---|---|---|
| L0 (default) | 10%      | — | — | — |
| L1  | 15% | 5  | 5   | L1  |
| L2  | 20% | 9  | 14  | L3  |
| L3  | 25% | 13 | 27  | L6  |
| L4  | 30% | 17 | 44  | L9  |
| L5  | 35% | 21 | 65  | L13 |
| L6  | 40% | 25 | 90  | L18 |
| L7  | 45% | 29 | 119 | L24 |
| L8  | 50% | 33 | 152 | L31 |
| L9  | 55% | 37 | 189 | L38 |
| L10 | 60% | 41 | 230 | L46 |

Worked example: a maxed Sweeper hosts a space; a participant claims a 10,000-point reward; the participant receives 10,000 (or boosted by their own Money Tree), and the creator receives an extra 6,000 owner bonus instead of the default 1,000.

The data model stores skills as a generic `(skill_id, level)` map, so adding Influencer/Sweeper later is purely additive.

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
   - **Decided: 5 SP per level (`total_sp_granted = 5 · L`).** PO directive (revised after balance review). Combined with the steeper skill cost curve (Q9b) and `C = 220` quadratic XP (Q8), this puts:
     - char L1 → first skill at L1 (5 SP, day 0)
     - char L3 → first skill at L2 OR both at L1 (~45 min for avg)
     - char L18 → first skill at L6 / both at L4 (~5.5 d)
     - char L31 → first skill at L8 (~1 mo)
     - char L46 → **one skill maxed at L10 (~6.4 mo for avg / ~3.5 mo for top)** ← MVP endgame
     - char L92 → both skills maxed (~4.3 yr for avg / ~2.4 yr for top)
   - **PO override would be:** lower grant (e.g., 3 SP/level) if the early game feels too generous after first-week telemetry.

8. **(Q8) XP curve steepness and shape.**
   - **Decided: quadratic `C · L²` with `C = 220`.** PO directive — one skill must be maxable in **≤ 6 months for an avg participant**. With max-skill at L45 (Q9) and avg activity ≈ 36k XP/day, the calibration is `cumulative_xp(45) ≈ 36k × 180 → C ≈ 220`. Earlier draft used `C = 600`, which put L45 at ~17 months. See worked-numbers table in §"Leveling math" of the design doc.
   - **PO override would be:** raise to `C = 300` (max-skill in ~8 months) or drop to `C = 150` (max-skill in ~4 months) after first-week telemetry.

9. **(Q9) Max skill level.**
   - **Decided: 10.** PO directive — restored from the earlier-considered 6 to give the endgame a clean +50% (1.5×) cap, and so that the level pips on the skill-tree UI feel substantial. The 6-month one-skill-max target is preserved by the steeper cost curve (Q9b), not by capping the level.
   - **PO override would be:** drop back to 6 if telemetry shows L10 is genuinely unreachable for non-elite participants (>1 yr to max for avg).

9b. **(Q9b) Skill cost curve.**
   - **Decided: `cost(n→n+1) = 5 + 4·n`.** PO directive. L1 entry stays at 5 (low barrier so the choice between Money Tree and Ranker is open from day one); from L2 onward each level costs +4 SP more than the previous, four times steeper than the originally-considered triangular `+1 SP` curve. Total cost to max one skill: 230 SP. Char-level prerequisites at the chosen SP grant rate (5/lv): L1→L1, L2→L3, L3→L6, L4→L9, ..., L10→L46.
   - **PO override would be:** flatter (e.g. `5 + 3·n`, total 185 SP, max-1 at ~3.3 mo) if 6 months feels too long, or steeper (`5 + 5·n`, total 275 SP, max-1 at ~11 mo) if too short.

## References

- `app/ratel/src/features/activity/models/space_score.rs` — current per-space user XP entity.
- `app/ratel/src/features/activity/services/aggregate_score.rs` — the existing stream handler that materializes `SpaceScore` from `SpaceActivity`. The Character XP path will hook here.
- `app/ratel/src/common/stream_handler.rs` — central stream dispatch on sk prefix.
- `app/ratel/src/features/spaces/space_common/models/space_reward.rs` — `SpaceReward::award_if_configured` is the entry point Money Tree multiplies on.
- `app/ratel/src/features/spaces/space_common/services/space_fanout.rs:42` — `MIN_PARTICIPANTS_FOR_HOT` is the threshold v2's Influencer skill loosens.
- `roadmap/badge.md` — same "cross-space, account-level recognition" theme; treat as sibling, not competitor (badges = recognition, no economic effect; skills = economic effect, no public catalog).
