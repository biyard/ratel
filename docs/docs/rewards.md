---
sidebar_position: 12
title: Rewards
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Rewards

Every Space you participate in, every post you publish, every comment that earns engagement — these accumulate as rewards in your account. The Rewards page is where you see what you've earned, what's still in motion, and what's ready to claim.

## Where your rewards live

```
/<your-handle>/rewards
```

Open it from the user dropdown at the bottom of the sidebar (click *Rewards*) or paste the URL directly. The page renders for any visitor who knows the URL — points, cycles, and transactions are queryable by username today. Only the **Claim** button on past-cycle cards is gated by your wallet — you can't claim someone else's rewards. (Stricter privacy is on the roadmap.)

The page is organized around **cycles**. A cycle is one calendar month: rewards accrue inside the current cycle, get a chance to "lock in" at month-end, and become **claimable** as Tokens on the 1st of the next cycle.

## Points vs Tokens — the primitives

Two units flow through the rewards system, and they do different jobs. (Both are defined more formally in the [Glossary → Rewards](./intro/glossary#rewards) section.)

- **Points** (`pts`) — In-platform reputation, awarded for completing actions. Points accumulate inside a cycle and feed your Direct-Activity Index. They're the *unit of earning*, weighted by skill-tree boosts on your [Character](./my-essence#-character-mecharacter) page.
- **Tokens** — On-chain payouts you can claim once a cycle locks in. Tokens are backed 1:1 by a treasury reserve, and the per-cycle exchange rate updates every block. The Rewards page shows both your **estimated tokens** for the current cycle and the actual token amount available to claim from past cycles.

Credits (the host-funded unit attached to a specific Action's reward in a Space) are the upstream of these — when an Action you complete pays out, its Credits are converted to your Points balance for the cycle.

## What the page shows you

Top to bottom, the Rewards page is built around your current cycle and your history.

### This cycle, at a glance

The hero of the page tells you four things:

- **Earning · this cycle** — total Points accrued so far in the current cycle.
- **Share of Pool** — your slice of the platform-wide reward pool, expressed both as a percentage and as `<your pts> of <total pts>`.
- **Your position / Rank** — where you sit in the leaderboard for this cycle.
- **Estimated tokens** — what your current Points would convert to at today's treasury rate when the cycle locks in. A live timer shows when the cycle locks in (*"Cycle locks in 4d 12h"*) and a note reads *— claim opens next cycle*.

### Treasury and price

A treasury panel shows the current rate, the treasury balance, the circulating supply, and the backing ratio (anchored at 100% — fully collateralized). A 30-day price chart sits next to it so you can see how the rate has moved.

### Source breakdown — where your Points come from

A donut chart at the center of the page breaks down this cycle's earned Points by source:

- <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> **Polls** — Points awarded when participants respond to polls you host.
- <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" height="14" alt="Posts" style={{verticalAlign: 'middle'}} /> **Posts** — Engagement-weighted earnings from posts you've published.
- <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><path d="M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3"/><line x1="12" y1="17" x2="12.01" y2="17"/></svg> **Quizzes** — From quizzes you host that participants pass.
- <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> **Discussions** — From discussions where participants contribute.
- <img src={useBaseUrl('/img/icons/users.svg')} width="14" height="14" alt="Follows" style={{verticalAlign: 'middle'}} /> **Follows** — From follow campaigns you've run.

The donut center reads *POINTS · NOW* — the total for the current cycle.

### Monthly chart and activity log

Below the breakdown:

- **Monthly chart** — Last 6 cycles plotted side-by-side, with overlays for both Points earned and Tokens received. Useful for spotting trends.
- **This cycle's activity** — A scrollable log of every reward entry from this cycle: amount, source, timestamp, and whether each entry was *Received* or *Spent*. Use **Load more** to walk back into the cycle.

### Past cycles — claim tokens

A grid of cycle cards at the bottom shows every locked cycle:

- The card lists the cycle's **points**, your **share** of that cycle's pool, and the **tokens** you can claim.
- A counter at the top reads *N claimable* across all cards.
- Each past cycle has a per-card **Claim** button. (The in-app label currently reads *Swap All* — the code reuses the `swap_all` i18n key for the per-card claim action.) Once claimed, the card flips to a **Claimed** state.
- A true section-level **Swap All** that batches every claimable cycle in one click is *(Coming soon)*.

If you don't have any past cycles yet, you'll see *"Your first cycle wraps up at month-end. Tokens become claimable on the 1st of next month."*

## How rewards accrue

You don't have to do anything special to earn — the platform credits Points automatically as soon as the qualifying action completes. The main paths are:

- **Hosting a Space Action** — When participants complete a Poll, Discussion, Quiz, or Follow you host, Credits from the Space's [Incentive Pool *(Beta)*](./spaces/apps#-incentive-pool-beta) are converted to Points in your account.
- **Completing a participant Action** — When you complete a reward-bearing Action as a participant, you receive Points drawn from that Space's Incentive Pool. (See [Space Actions → How action rewards work](./spaces/actions#how-action-rewards-work).)
- **Post engagement** — Posts that gather likes, comments, and reshares earn engagement-weighted Points back to the author.
- **Character skill multipliers** — Boosts you've bought on your [Character](./my-essence#-character-mecharacter) page (Money Tree at MVP) multiply every Point payout you receive.

The **Direct-Activity Index** is computed off your participation history (writing, voting, discussing weighted higher than passive activity) so the leaderboard rewards real engagement; it's a signal the platform uses for ranking and weighting, not a number rendered on the page today.

## On-chain settlement

Claiming a past cycle's tokens is a **two-step flow** today:

1. **Connect your wallet.** The Claim button asks you to connect a wallet (MetaMask, Rainbow, Trust Wallet, Coinbase Wallet, or any other WalletConnect-compatible wallet) and confirms the chain.
2. **Sign and submit.** Ratel issues a signed claim payload (amount, deadline, nonce, contract address, chain id) for the cycle, and your wallet submits the on-chain transaction that transfers the tokens from the treasury contract to your address.

Once the transaction confirms, the cycle's card flips to **Claimed**. You don't need a wallet to *earn* Points — that flow is entirely off-platform-chain bookkeeping — but **claiming Tokens** does require one.

A few things to know:

- **Backing** — Tokens are 100% collateralized by the treasury reserve. The rate you see at *Treasury Rate* is what you get per token at the moment of claim.
- **Fiat / off-chain cashout** — *(Coming soon)*. Today rewards settle as Tokens via on-chain claim; an off-chain cashout flow for users who'd rather not handle a wallet is on the roadmap.
- **Stablecoin denomination** — *(Coming soon)*. Some Spaces will be able to denominate payouts in a fiat-pegged Stablecoin instead of the volatile native Token. Optional and host-configurable.

## What's next

Once you have a feel for the page, the most useful follow-ups are:

- [Character](./my-essence#-character-mecharacter) — Spend the SP your earnings have unlocked. Money Tree boosts every future Point payout multiplicatively.
- [Space Actions → How action rewards work](./spaces/actions#how-action-rewards-work) — Where Action-level rewards come from before they become Points in your account.
- [Glossary → Rewards](./intro/glossary#rewards) — Formal definitions of Point, Credit, Token, Stablecoin, and the off-chain / on-chain settlement split.
