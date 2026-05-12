---
sidebar_position: 3
title: Reward Points
---

# Reward Points

**Reward Points** (`pts`) are Ratel's in-platform reputation unit and the *unit of earning* across reward-bearing Spaces. Where XP measures your *progression*, Points measure your *take-home* — they accumulate inside a cycle, feed your Direct-Activity Index, and (at month-end) lock in as a claimable Token amount.

## How Points accrue

You don't have to do anything special to earn Points — the platform credits them automatically as soon as a qualifying action completes. The main paths:

- **Hosting a Space Action.** When participants complete a Poll, Discussion, Quiz, or Follow you host, Credits from the Space's [Incentive Pool](./incentives.md) are converted to Points in your account.
- **Completing a participant Action.** When you complete a reward-bearing Action *as a participant*, you receive Points drawn from that Space's Incentive Pool. (See [Space Actions → How action rewards work](../spaces/actions#how-action-rewards-work) for the per-action payout flow.)
- **Post engagement.** Posts that gather likes, comments, and reshares earn engagement-weighted Points back to the author.

Skill-tree boosts you've bought on Character (Money Tree at MVP) multiply every Point payout *before* the amount is credited to your balance, so investing SP in Money Tree compounds across every future Action.

## The Direct-Activity Index

The **Direct-Activity Index** is computed off your participation history, weighting writing, voting, and discussing higher than passive activity. It's a signal the platform uses for ranking and weighting — not a number rendered on the Rewards page today, but the input that decides where you land on a Space's leaderboard and how your Points flow at the platform level.

In practice:

- A long, well-formed Discussion reply earns more than a one-line poll vote.
- Hosting an Action that pulls real participation earns more than running an Action no one engages with.
- Passive scrolling earns nothing — only direct actions count.

## Cycles and lock-in

Points are organized around **cycles** — one calendar month each:

- **Inside the current cycle**, Points accrue in real time. The Rewards page (`/<your-handle>/rewards`) shows your running cycle total, your share of the platform-wide pool (`<your pts> of <total pts>`), and your leaderboard position.
- **At month-end**, the cycle locks in. The Points you earned that month become the basis for that cycle's Token payout.
- **On the 1st of next month**, the locked cycle becomes claimable as Tokens at the cycle's treasury rate.

A live timer on the page tells you when the current cycle locks in (*"Cycle locks in 4d 12h"*).

## What Points show on the Rewards page

The donut chart at the center of `/<your-handle>/rewards` breaks down this cycle's earned Points by source:

- **Polls** — Points awarded when participants respond to polls you host.
- **Posts** — Engagement-weighted earnings from posts you've published.
- **Quizzes** — From quizzes you host that participants pass.
- **Discussions** — From discussions where participants contribute.
- **Follows** — From follow campaigns you've run.

Below the donut sits a 6-cycle monthly chart (Points earned + Tokens received overlay) and a scrollable activity log of every reward entry from this cycle (amount, source, timestamp, *Received* vs *Spent*).

## What Points are not

- **Points aren't directly cashable.** They feed the cycle that becomes Tokens at lock-in; the path to fiat is through on-chain Token claim today, with off-chain cashout *(Coming soon)*.
- **Points aren't on-chain.** The Points ledger is off-platform-chain bookkeeping. Tokens are the on-chain expression that arrives at claim time.
- **Points don't roll over.** A cycle's Points lock into that cycle's Token payout. The next cycle starts fresh.

## Related

- [Tokens](./tokens.md) — the on-chain payout Points convert into when you claim a past cycle.
- [Credits](./credits.md) — the host-funded upstream of Points.
- [Experience Points (XP)](./xp.md) — the progression unit; SP from XP buys Money Tree, which multiplies every Point payout.
