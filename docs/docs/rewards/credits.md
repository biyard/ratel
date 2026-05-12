---
sidebar_position: 4
title: Credits
---

# Credits

**Credits** are the host-funded currency that backs an Action's reward in a Space. They're the upstream of [Reward Points](./points.md): when a host configures an Action's reward, they allocate Credits from their Incentive Pool — and at payout, each Credit converts into Points for the participant who earned it.

## Where Credits come from

Credits are minted by the platform against your **subscription tier**, not against a card swipe. Each tier carries a fixed monthly Credit allotment that resets every cycle:

- **Free** — 0 Credits / month
- **Pro** — 40 Credits / month, up to 2 per reward Space
- **Max** — 190 Credits / month, up to 10 per reward Space
- **VIP** — 1,360 Credits / month, up to 100 per reward Space
- **Enterprise** — fully customizable, by contract

These caps are enforced server-side. If a host tries to allocate more Credits to a Space than their tier permits, the platform returns *Credits exceed max per space* and the Action's reward configuration won't save until you adjust.

## How Credits attach to an Action

When you configure a reward on a Poll, Discussion, Quiz, or Follow, the action settings modal asks for a **Credit allocation**. That allocation:

- Draws from your Team's (or your personal) monthly Credit balance for the current cycle.
- Defines how much of the Space's [Incentive Pool](./incentives.md) this specific Action will pay out.
- Is bounded by your tier's per-Space cap (see above).

The Action's settings page shows the current Credit balance available to spend, the per-Space cap, and (after allocation) what's left in the cycle's Credit budget.

## How Credits convert to Points

When a participant completes a reward-bearing Action:

1. The platform calculates the participant's share of the Action's Credit allocation, according to the host's distribution rules (flat / weighted by participation depth / staged across phases — see [Incentive Pool](./incentives.md)).
2. Their share of Credits is converted to **Points** in their account at the platform's current conversion rate (typically 1:1 at MVP, but the platform reserves the right to tune the rate as the economy matures).
3. The Action's Credit allocation is decremented; the host's Credit balance is decremented for the cycle.

The result: Credits leave the host's balance, Points enter the participant's balance, and the Action's reward shows as paid out in the dashboard.

## Credits don't roll over

A cycle's unused Credits **do not carry forward** to the next cycle. If you have a 190 Credit / month allotment on Max and you only use 60, the remaining 130 reset on the 1st of the next month — they don't bank into a 320-Credit balance.

This is intentional: Credits are designed to encourage *active* host engagement each cycle. If your team isn't running reward-bearing Spaces in a given month, the cycle's allotment lapses.

## What Credits are not

- **Credits aren't an asset.** They're a quota the platform issues against your subscription, not a token, balance, or holding. You can't transfer them, sell them, or claim them on-chain.
- **Credits aren't refundable.** Once allocated to an Action and partially or fully paid out to participants, those Credits are gone — even if the Action is later cancelled.
- **Credits aren't fiat.** A Pro tier's ₩30,000 / month doesn't equal 40 dollars of payout — Credits are a platform-internal quota, and the platform's conversion to Points is what determines participant value at payout time.

## Errors you might see

- **`Credits exceed remaining balance`** — You're trying to allocate more Credits to an Action than the cycle's remaining balance covers. Reduce the allocation, or wait for the next cycle.
- **`Credits exceed max per space`** — You're trying to allocate more Credits to a single Space than your tier's per-Space cap allows. Either reduce the allocation, split across more Actions/Spaces, or upgrade tier.
- **`No credits configured`** — You're on Free tier (which has 0 Credits / month). Upgrade to Pro or higher to fund reward-bearing Actions.

## Related

- [Reward Points](./points.md) — what Credits become for the participant at payout.
- [Incentive Pool](./incentives.md) — the Space-level surface where Credits land before they're attached to individual Actions.
