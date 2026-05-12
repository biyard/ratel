---
sidebar_position: 6
title: Incentive Pool
---

# Incentive Pool

The **Incentive Pool** is the host-side funding surface — what stages reward currency before it's attached to individual Actions. It's where a Space's reward economy starts: the host loads the pool with [Credits](./credits.md), configures distribution rules, and the pool then pays out to participants as Actions complete.

If [Credits](./credits.md) are the unit and [Reward Points](./points.md) are the participant-side outcome, the Incentive Pool is the *host-side container* that ties the two together.

## Where the Incentive Pool lives

Each Space has its own Incentive Pool, attached as an optional App. Find it at:

```
/spaces/:space_id/apps/incentive-pool
```

This is also accessible from the Space's Apps panel (`/spaces/:space_id/apps`) — install the **Incentive Pool** app card if it isn't already in your installed apps. (See [Space Apps → Incentive Pool](../spaces/apps#-incentive-pool-beta) for the host's app-installation tour.)

## How the pool is configured

When you open the Incentive Pool app, you set three things:

1. **Pool size** — How many Credits the Space will fund in total. Drawn from your subscription tier's monthly Credit allotment, bounded by the per-Space cap (see [Credits](./credits.md)).
2. **Per-action allocation** — How Credits are subdivided across the Actions in the Space. You can spread them evenly or weight specific Actions more heavily.
3. **Distribution rules** — How each Action's allocation pays out to the participants who complete it.

The page shows the pool's current size, current balance (what's left after payouts), and the distribution history — every payout the pool has fired, with timestamps, recipient handles, and amounts.

## Distribution rules

Three distribution patterns are supported today, configurable per Action:

- **Flat split.** Every participant who completes the Action gets an equal share. Simple, fast, predictable. Good for Polls and Follow campaigns where contribution depth doesn't vary much.
- **Weighted by participation depth.** A long, well-formed Discussion reply earns more than a one-line vote; a Quiz with a high pass rate earns more than one barely passed. The weighting is computed off the Quality Score signal that also feeds your [Direct-Activity Index](./points.md#the-direct-activity-index). Good for Discussions and rich Quizzes where contribution quality matters.
- **Staged across phases.** Larger pools split across the lifecycle of the Space — *N* Credits for the first cohort of completers, *M* for the second wave, etc. Good for Spaces designed to seed early adoption then sustain participation.

The Action's settings panel shows you the per-rule preview: *"Average Credit per participant at current participation rate"*, *"Maximum Credit any single participant can earn"*, etc., so you can tune before launching.

## Audit trail

Every payout the pool fires is logged. The Incentive Pool app shows:

- **Pool funded at** — when you initially staged the Credits.
- **Total paid out** — sum of every distribution to date.
- **Per-Action paid out** — drill into any Action to see exactly which participants received how much, when.
- **Remaining balance** — what's left to pay out for ongoing or future Actions.

Both you and your Space's participants can see the totals (participant-level breakdown is host-only). This is by design: a transparent reward economy is easier to trust than an opaque one.

## What happens when the pool runs out

If an Action would pay out more Credits than the pool's remaining balance covers, the platform returns *Credits exceed remaining balance* and the participant's reward is **deferred** rather than minted as Points.

Two paths from there:

- **Top up the pool.** Allocate more Credits from your monthly cycle balance to the Incentive Pool. Once funded, the deferred payouts mint Points retroactively at the next reconciliation.
- **Adjust distribution rules.** Lower the per-participant allocation or switch to a weighted distribution that pays out more sustainably across remaining participants.

The deferred state is rare in practice — the pool's pre-allocated nature means a properly configured Space has already accounted for its expected participation.

## Pair with Analyzes for monetized Spaces

A Space that publishes paid Reports often runs an Incentive Pool *alongside* the [Analyzes app](../spaces/apps#-analyzes). The pool drives participation, while the Phase 4 **Report Revenue Share** (10% platform · 60% host · 30% contributors) rewards the contributors whose input made the Report worth selling.

The two feed each other: a well-funded pool pulls richer participation, which makes for a richer Report, which sells better, which feeds the contributor share — a virtuous loop the platform's Phase 4 economy is designed around.

## Related

- [Credits](./credits.md) — the host-funded unit the pool is staged with.
- [Reward Points](./points.md) — what participants receive when the pool pays out.
- [Space Apps → Incentive Pool *(Beta)*](../spaces/apps#-incentive-pool-beta) — the app installation surface and host-side configuration tour.
- [Space Actions → How action rewards work](../spaces/actions#how-action-rewards-work) — the participant-side experience of the same flow.
