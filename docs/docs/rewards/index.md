---
sidebar_position: 1
title: Rewards
---

# Rewards

The Ratel rewards economy turns activity into value through five distinct primitives, each with its own purpose:

- **XP** is earned automatically as you complete Space actions and levels up your Character.
- **Reward Points** are the in-platform reputation unit, the *unit of earning* across reward-bearing Spaces.
- **Credits** are the host-funded currency that backs an Action's reward — the upstream of Points.
- **Tokens** are the on-chain payouts a past cycle's Points convert into when you claim.
- **Incentive Pool** is the funding mechanism a host stages — what populates an Action's reward in the first place.

Rewards flow from three places: a Space's Incentive Pool (host-funded), platform-distributed pools (engagement-weighted earnings on Posts and other native activity), and — when it ships — the Phase 4 Report Revenue Share for paid reports (10% platform · 60% host · 30% contributors).

Open `/<your-handle>/rewards` to see what you've earned, what's still in motion, and what's ready to claim. The page is organized around **cycles** (one calendar month each): rewards accrue inside the current cycle, lock in at month-end, and become claimable as Tokens on the 1st of the next cycle.

## In this chapter

| Page | What it covers |
|---|---|
| **[Experience Points (XP)](./xp)** | The activity-driven progression unit. Fixed XP values per action type (Poll, Quiz, Discussion, Follow), how XP rolls up into Character levels, and the dedup rules that prevent double-counting. |
| **[Reward Points](./points)** | The in-platform reputation unit (`pts`). How Points accrue, how they feed your Direct-Activity Index, and how skill-tree boosts on Character multiply every payout. |
| **[Credits](./credits)** | The host-funded currency that backs Action rewards. Where Credits come from, how subscription tier caps the Credit allotment, and how Credits convert to Points at payout. |
| **[Tokens](./tokens)** | The on-chain expression of past-cycle Points. The two-step claim flow (connect wallet → sign and submit), the treasury rate, and the 100% collateralization model. |
| **[Incentive Pool](./incentives)** | The host-side funding surface — how an Incentive Pool is configured at `/spaces/:space_id/apps/incentive-pool`, how distribution rules work (flat / depth-weighted / staged), and how to keep an audit trail of what's been paid out. |
