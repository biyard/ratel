# Fact or Fold — 4-player news judgment game

**Status**: Ready for design (Stage 2 in progress · PO sign-off 2026-05-14)
**Slug**: `fact-or-fold`
**Primary use case**: Four people meet to judge the truthfulness (REAL/FAKE) of a single news headline about *something happening now*, betting their own RatelPoints in a real-time round that takes ~3 minutes. This is Ratel's first *Essence-production game* — using social pressure, time constraint, and economic stake to make users produce short, sharp *position + rationale* writing, which then gets opt-in-registered to their personal Essence.

## Problem

Growing a user's Essence (personal knowledge base) requires them to *write*. Today in Ratel there are only two motivations to write:

1. Users with an agenda of their own create Spaces and post — very high friction
2. Users join others' Spaces and comment on polls/quizzes/discussions — comments tend to be one-liners, low signal

Free-form journaling produces low average signal and rarely justifies a House subscription. By contrast, writing produced under *constraint + adversarial context + time pressure* tends to be short, compressed, and stance-clear — exactly the embedding-friendly conditions Essence needs.

A game framing produces those three conditions naturally. *Fake-news detection* is a topic of broad interest in Korea, the answer is *objectively verifiable after the fact* (truth is revealed at round end), and it sits well with Ratel's civic-governance origin.

## Goal

Ship a live multiplayer mini-game in which a user spends roughly three minutes per session to sincerely produce *one or two compressed position writeups*. The output of a round flows to two places:

1. **Short-term**: bet settlement → RatelPoint payout
2. **Long-term**: written rationale → opt-in registration into the user's Essence → input asset for downstream House / Agent / Report flows

One round ≈ 3 minutes. We assume an active user plays 1–3 rounds per day. That is the shortest path to making their Essence *grow meaningfully on a weekly basis*.

## Non-goals

Things explicitly *not* in v1 (deferred to v2+):

- **REAL-knower / mafia mode** — v1 has only one insider role (knows the truth). No deceptive insider
- **Essence policy for lying-side writeups** — since no lying is emitted in v1, the policy is moot. To be defined when mafia mode is introduced
- **User-submitted news** — all news comes from a pool curated by the Ratel team
- **External fact-check API integration** — to be revisited in v2 after v1 cold start
- **Category / sub-team matchmaking** — v1 uses a single global pool, first-come
- **Global leaderboard / seasons** — v1 shows per-round and lifetime accuracy only
- **AI agent participation** — v1 is 4 humans only; agent participation tracks against the Ratel agents roadmap separately
- **Rounds with fewer than 4** — if 4 spots aren't filled, the round doesn't start
- **Rounds with more than 4** — capacity is fixed at 4

## User stories

### Participant (uninformed × 3)

- As an active user, I want to enter the lobby, get *first-come-served matched* into a group of 4, and start a round immediately
- Inside the round, I want to read the news in 30 seconds, pick REAL/FAKE on gut feel, stake 100–1,000 RP, and write a one-liner rationale (50–200 chars) within 30 seconds
- After reading the other three's rationale, I want to debate in free chat for 70 seconds and *change my side once* in the final 10 seconds
- On the settlement screen, I want to see *how I decided and why*, and opt-in register my round writing into my Essence

### Insider (informed × 1, per round)

- I want to be told the truth (REAL or FAKE) privately at round start, and persuade others in that direction to earn an influence bonus
- If another participant *cites my rationale* when flipping their side, I want extra RP based on that participant's stake

### Host / operator (admin)

- I want to register one headline per day from the admin page (headline, body, verdict label, insider statement, sources to reveal at settlement)
- I want to pre-fill a week of the queue and get notified when the queue runs low
- I want to see results from in-progress rounds — accuracy, insider effect, reports — in a stats page

## Functional requirements

Numbered. Each MUST be *testable*.

### Round lifecycle (1-12)

1. The system SHALL maintain a *single active lobby* at all times. Users who want to start a round join this lobby.
2. The lobby fills *first-come, capacity 4*. The moment the 4th participant joins, the round starts immediately.
3. When the lobby has 1–3 users and no new user joins for *more than 2 minutes*, the system MAY display a *waiting notice* (non-blocking).
4. A single user SHALL NOT be in more than one lobby/round at the same time.
5. A round consists of 6 stages: **News reveal → 1st bet → Rationale write → Rationale reveal → Live debate → Settlement**.
6. Each stage has a *server-determined deadline*. Defaults:

   | Stage | Duration | Notes |
   |---|---|---|
   | News reveal | 30s | All start simultaneously |
   | 1st bet | 10s | Until bets lock |
   | Rationale write | 30s | 50–200 chars enforced, single submission |
   | Rationale reveal | 20s | All four rationales shown simultaneously |
   | Live debate | 70s | Free chat, 80-char cap per message |
   | Settlement | auto | Truth reveal + RP payout |

   *Total ≈ 160s of play + automatic settlement ≈ 3 minutes.* Durations are admin-tunable (§Constraints).

7. When a stage's deadline arrives, the system SHALL automatically advance to the next stage — even if some participants have not submitted.
8. Unsubmitted participants at stage deadline are handled as follows:
   - 1st bet not submitted: *auto-forfeit* the round. Bet refunded, removed from reward pool.
   - Rationale shorter than 50 chars: *that user only* is excluded from Essence registration. The round continues.
   - Final-bet change not selected: 1st bet stands.
9. Stage progression SHALL be *server-verified by time* — clients cannot forge stage state to start/skip early.
10. When the final stage of a round completes, the system SHALL emit a *settlement trigger event*, and a separate EventBridge handler SHALL perform settlement.
11. Round data (news, bets, rationale text, chat, settlement results) SHALL be *persisted permanently* — it is the input for Essence registration and statistical aggregation.
12. If a user *loses network* during a round and reconnects *within 90 seconds*, they SHALL resume in the same stage. After 90 seconds, *auto-forfeit*.

### Betting (13-18)

13. In the 1st bet, the user picks *REAL or FAKE* and stakes between *100 and 1,000 RP*.
14. Stake SHALL be *≤ the user's current RP balance*. Insufficient balance SHALL block the user *at lobby join* (§Constraints).
15. After the 1st bet, the side and amount are *locked until the rationale-writing stage* — even the bettor cannot change them.
16. In the *final 10 seconds* of the debate stage, a *bet-change slot* opens. Only in this slot may the user *flip side (REAL ↔ FAKE) once*. The stake amount is not changeable.
17. A user who flips MUST cite *which other participant's rationale* caused the change. A flip without citation is *invalid* (original side stands).
18. At settlement, *flip integrity* is verified — the cited user must actually have been in the round and submitted a rationale.

### Rationale writing (19-22)

19. After 1st bet, the user enters the rationale-writing stage. Input is *enforced to 50–200 characters*.
20. Submission is *single-shot, no rewrites*. First answer is final.
21. At deadline, unsubmitted text is *auto-submitted as whatever is in the field*. If shorter than 50 chars, per §FR-8 it is excluded from Essence registration only.
22. At rationale-reveal, all four rationales are *displayed simultaneously*. There is no informational asymmetry based on submission order.

### Insider (23-26)

23. At round start, *1 of the 4* is randomly chosen as the *insider*. That user receives the *truth (REAL/FAKE) and an auxiliary statement privately* — this works the same on REAL rounds and FAKE rounds.
24. The insider SHALL NOT publicly state that they are the insider — violation is a report-eligible offense, subject to operator action.
25. The insider earns the influence bonus (§FR-29) by persuading others *in the direction of truth*.
26. In v1 there is *no lying insider (REAL-knower lying)*. The insider's statement is always a truth-attempt (§Non-goals).

### Settlement (27-33)

27. When the round's truth is revealed, the system SHALL identify *winning side* and *losing side*.
28. **Base settlement formula**:
   - Winners: refund of stake + bonus. Bonus = `(own stake) × (correct-side multiplier − 1)` (default 0.6×).
   - Losers: stake is *forfeit in full*. That total enters the bonus pool.
   - The bonus pool is distributed to winners *in proportion to each winner's stake*.
29. **Influence bonus**: If user A *flips side* and ends up on the winning side, and the *cited rationale's author* is B, then B receives *(A's stake) × influence rate* extra (default 30%).
30. **Insider correct-bet bonus**: If the insider bet on the truth side and was correct, on top of stake refund they receive an extra *(own stake) × 0.5*.
31. Negative balances do not occur — since a user cannot stake more than their current balance (§FR-14), in the worst case the balance falls only to 0.
32. Settlement results are *persisted per round*, and shown to users as a *breakdown (base refund / correct bonus / influence / insider bonus)*.
33. Settlement runs *asynchronously in an EventBridge handler* and the result MUST be reflected in the user's UI within 3 seconds of round end.

### Essence integration (34-38)

34. Immediately after settlement, the user is shown an *Essence registration review screen*. The screen lists their *rationale + any decisive-quote-marked debate utterances* as individual items.
35. Each item is *on by default*. The user can uncheck specific items to exclude them from registration.
36. When the user clicks "Register to Essence", the selected items are registered into *Ratel's existing Essence embedding pipeline* — we do NOT build new embedding infra; instead we ride on the existing *Discussion = SpaceActivity = embedding target* flow.
37. Items not registered are *permanently discarded* (round data itself remains, but no Essence entry).
38. *Items shorter than 50 chars* are auto-excluded from registration candidates (§FR-8) — shown greyed-out on the review screen.

### Admin (39-45)

39. An operator (admin role) SHALL register news (= headlines) from a *dedicated admin page*. One headline = one round's content.
40. The headline form accepts: round ID, publish datetime, verdict label (REAL/FAKE), category tags, difficulty (★ 1–5), source label, headline text, body excerpt (200–500 chars), *private statement to be delivered to the insider*, *truth summary and 2–3 verification source URLs to reveal at settlement*.
41. Headlines MAY have a *scheduled datetime*. Scheduled headlines *auto-activate* as the live round at that time.
42. Operators can choose three actions: *save draft*, *schedule publish*, *publish now*.
43. Operators may *edit/delete existing headlines*. Once a round is in progress for a headline, the body/verdict cannot be changed; only *adding verification sources* is allowed.
44. Operators view *past round stats* (accuracy, insider effect, mind-flip count) and *user reports* on separate admin pages.
45. When the queue (scheduled future headlines) *drops to 5 days or fewer*, an alert is shown on the admin page.

## Acceptance criteria

- [ ] When a 4th user joins the lobby, the round auto-starts immediately
- [ ] When a stage's deadline arrives, the next stage begins automatically even with non-submissions
- [ ] Rationale text shorter than 50 chars is auto-submitted but excluded from Essence candidates
- [ ] After the 1st bet and until the rationale stage, side and amount cannot be changed (server-verified)
- [ ] The bet-change slot opens only in the final 10 seconds of the debate stage
- [ ] A side flip without a citation target is rejected
- [ ] Exactly 1 of 4 is designated insider per round; the truth statement is delivered only to that user
- [ ] The other 3 do not know who the insider is until round end
- [ ] If the insider *explicitly* publicly declares themselves as the insider, the action is report-eligible
- [ ] The settlement breakdown shows 4 line items (base refund / correct bonus / influence / insider)
- [ ] User balance never goes negative
- [ ] The Essence review screen allows per-item uncheck and final registration
- [ ] An operator can register, schedule, and publish a single headline from the admin page
- [ ] A scheduled headline auto-activates as the live round at its publish time
- [ ] When the queue drops to 5 days or fewer, the admin page shows an alert
- [ ] A user reconnecting within 90 seconds resumes the round in the same stage
- [ ] After 90 seconds, the round auto-forfeits and the stake is refunded

## Constraints

- **DynamoDB single-table** — new entities follow `Partition` + `EntityType` convention + `#[dynamo(prefix)]`
- **Event-driven, no polling** — stage auto-progression and settlement are EventBridge rules. Do NOT introduce new polling or cron jobs
- **Replay-safe** — even if EventBridge delivers the same event twice, RP payout must not double (idempotency key required)
- **RP economy reuse** — use existing `User.points`, `UserReward.total_points`, and `SpaceReward::award_if_configured` patterns. No new currency
- **Balance guard** — bet attempts above current balance are *blocked at lobby join*. Even if balance drops afterwards for some other reason, the bet must already be locked (no race condition)
- **No new external services** — no Redis, no separate game server, no externally hosted WebSocket. SSE/WebSocket lives inside Ratel's Axum server
- **Realtime channel** — stage transitions, other participants' progress, and chat go via *SSE or short-poll (2–3s)*. Minimize new package dependencies
- **Insider protection** — insider-identifying data is returned *only to that user's own requests*. Other participants cannot query for it
- **MVP concurrency** — assume single-digit simultaneous active rounds. Scenarios like 100 concurrent rounds are deferred to v2

## Tunable parameters (admin settings page)

| Parameter | Default | Notes |
|---|---|---|
| Round capacity | 4 | Fixed in v1 |
| Stage durations (sec) | 30 / 10 / 30 / 20 / 70 | News / bet / rationale / reveal / debate |
| Min bet RP | 100 | |
| Max bet RP | 1,000 | |
| Correct-side multiplier | 1.6× | (own refund + 0.6× own stake) |
| Insider correct bonus | +0.5× | |
| Influence bonus rate | 30% | |
| New-user signup RP | 5,000 | One-time grant |
| Reconnect grace | 90s | |
| Queue-low alert threshold | 5 days | |

## Decisions made (Stage 1 PO sign-off)

This spec rests on the following five decisions:

1. **News pool source** — Ratel team curation only. External APIs / user submissions deferred to v2
2. **Cadence** — real-time ~3-minute rounds. 24-hour async mode NOT in v1
3. **Matching** — single global lobby, first-come, instant start when 4 fill. Category/sub-team matching deferred to v2
4. **Insider default** — 1 insider per round who knows the truth (FAKE-knower equivalent). The deceptive REAL-knower (mafia mode) is deferred to v2
5. **Essence policy for lying-side text** — moot in v1 since no lying is emitted. To be decided when mafia mode lands

## Open questions

(May need decisions before Stage 2 / Stage 3 entry)

1. **Insider exposure penalty** — what happens when an insider publicly outs themselves. Options: invalidate the round / refund RP / suspend the user's insider eligibility for 24h. *Defer PO decision and adjust based on observed report patterns*
2. **Reconnect-grace duration** — 90s could be too tight or too loose; verify in beta. Too long invites intentional idling; too short punishes mobile drops
3. **Lobby join with insufficient balance** — 100 RP minimum is obvious, but the corner case is a brand-new 0 RP user trying to join *before* receiving the 5,000 RP signup bonus
4. **Insider ratio** — currently always 1 of 4 (= 25%). Lowering this (e.g., insider exists in only 50% of rounds) makes the game harder — review after v1 data, possibly adjust in v1.5
5. **Stage duration tuning** — 30s for rationale may feel tight in beta; raising to 45s lengthens rounds but raises text quality. Ship with 30s, decide after 30 days of data

## References

### Existing Ratel code (reuse / reference)

- `app/ratel/src/features/spaces/space_common/models/space_reward.rs` — `SpaceReward::award_if_configured` pattern. This game's settlement payout sits on top of this flow
- `app/ratel/src/features/activity/services/aggregate_score.rs` — DynamoDB Stream → SpaceScore aggregation handler. This game's stage auto-progression / settlement use the same EventBridge pattern
- `app/ratel/src/common/stream_handler.rs` — central stream dispatch
- `app/ratel/src/common/types/event_bridge_envelope.rs` — `DetailType` enum + `proc()` match pattern
- `app/ratel/src/features/essence/` — Essence embedding pipeline (rationale registration target)
- `roadmap/character-xp-skills.md` — Money Tree multiplier (composable with correct-side bonus)
- `roadmap/cross-posting.md` — external channel integration patterns (for sharing round results externally later)

### Design (Stage 2 input)

- `~/dev/claude_coworker_scratches/truth-bet/` — to be migrated into `app/ratel/assets/design/fact-or-fold/` in Stage 2
  - `index.html` → `lobby.html`
  - `r-2026-05-11/index.html` → `round-stage.html`
  - `admin/*.html` → `admin-*.html` (headlines / new / schedule / insiders / stats / reports / settings)
  - `style.css`, `admin/admin.css` → `shared.css`, `admin-shared.css`

### Workflows

- `.claude/rules/workflows/feature-development.md` — this spec is the Stage 1 artifact
- `.claude/rules/workflows/ui-design-implementation.md` — applies when entering Stage 2
- `.claude/rules/workflows/develop-a-new-feature.md` — applies when entering Stage 3
- `.claude/rules/workflows/implement-mcp-tools.md` — applies if admin controllers are exposed via MCP

---

**Status note**: This spec was written on top of the five PO decisions declared above; the remaining Open questions (§above) can wait until just before Stage 2 entry. **Marked *Ready for design* on 2026-05-14 — Stage 2 (design migration) in progress at `app/ratel/assets/design/fact-or-fold/`.**
