---
sidebar_position: 2
title: Experience Points (XP)
---

# Experience Points (XP)

**XP** is the activity-driven progression unit that powers your Character on Ratel. Every Space action you complete records a `SpaceActivity` row, and that row mints a fixed XP amount into your Character total. XP doesn't directly cash out — its job is to drive your level, your Skill Points (SP), and the boosts you spend SP on.

## How XP is earned

Five Space actions today award XP, with fixed amounts per event:

| Action | XP per event |
|---|---|
| **Poll** — respond to a poll once | 50,000 |
| **Quiz** — pass a quiz | 20,000 |
| **Quiz** — fail a quiz (still attempted) | 1,000 |
| **Discussion** — leave a reply | 5,000 *(per comment)* |
| **Follow** — complete a follow campaign | 1,000 |

Posts, comments under posts, and other native activity feed your Essence directly but currently don't mint XP — XP is scoped to **Space-level participation** today. (As the platform grows, expect more event types to award XP.)

A few rules that the platform enforces server-side, so you don't need to think about them:

- **Dedup per action.** Polls, Quizzes, and Follows mint XP **once per user per action** — replying to the same poll twice doesn't double up. Discussion replies are an exception: each *comment* is its own XP-earning event, so a thoughtful multi-comment thread genuinely earns more than a one-liner.
- **No retroactive minting.** XP is recorded the moment the action completes; closing a tab mid-flow won't grant XP after the fact. Run the action through to completion.
- **Idempotent processing.** Even if Ratel's internal stream sees a duplicate event (rare, but possible), the dedup key prevents the same activity from minting XP twice.

## How XP becomes Character progress

XP rolls up into your **Character** at `/me/character` (see [My Essence → Character](../essence/my-essence#-character--mecharacter) for the full tour):

- **Total XP** drives your **Level**. Each level requires more XP than the last, on a curve.
- **Levels grant Skill Points (SP).** Hit a new level and SP land in your account, ready to spend in the skill tree.
- **Skill Points buy upgrades.** Money Tree multiplies every future Reward Point payout; Ranker boosts the bonus portion of every Space activity (compounding back into faster XP gains).

Two skills are levelable today (Money Tree and Ranker, each capping at L10 with +50% effect at max). A second cohort (Influencer, Sweeper) is *(Coming soon — v2)*.

## What XP doesn't do

- **XP isn't a payout.** It can't be converted to Points, Credits, or Tokens. It's purely a progression signal that buys SP, and SP buys multipliers on the *other* primitives.
- **XP doesn't decay.** Total XP only goes up. Levels you've reached stay reached even if you take a break.
- **XP isn't transferable.** It belongs to your account, not your wallet — there's no on-chain XP token.

## Where XP shows up in the UI

- **Character page** (`/me/character`) — Hero card with Level, total Character XP, XP-to-next-level, and SP available to spend.
- **Space arena ranking** — many Spaces show a *Total XP* leaderboard column on `/spaces/:space_id/dashboard` so participants can see how their contribution stacks up.

## Related

- [Reward Points](./points.md) — the *unit of earning* that XP boosts via skill multipliers.
- [My Essence → Character](../essence/my-essence#-character--mecharacter) — spend SP, level up Money Tree, and see what your Character does for your earnings.
