---
sidebar_position: 13
title: Membership
---

# Membership

Membership is Ratel's tier system for hosts and power users. Anyone can sign up for free and use the platform; paid tiers unlock **monthly Credits** you can spend on reward Spaces, plus a creator badge and (at higher tiers) raw participant data access.

## Where Membership lives

```
/membership
```

Open it from the sidebar (the *Membership* link in the left rail) or paste the URL directly. The page shows every tier side-by-side as a card grid — you compare them, click **Get \<Tier\>** on the one you want, and a purchase modal walks you through payment.

A separate **Subscription & Billing** card on your settings page (`/<your-handle>/settings`) is where you manage the tier you've already bought. See [Settings → Subscription & Billing](./settings#-subscription--billing) for that.

## What Credits are

Before the tiers, the unit they meter:

> **Credits** are monthly points you can use to create or boost reward Spaces.

Every paid tier comes with a monthly Credit allotment that resets each cycle. When you create a reward-bearing Action ([Host Actions](./spaces/host-actions)) or fund a Space's [Incentive Pool](./spaces/apps#-incentive-pool-beta), the cost is paid in Credits. Higher tiers ship with bigger monthly allotments and a higher per-Space cap.

Credits aren't refundable, can't be transferred, and don't roll over month-to-month — they're the budget you have for one cycle of host work.

## The five tiers

Five tiers ship today, top to bottom from least to most:

### Free — included with every account

- Basic membership, open to everyone.
- **Includes**: publish posts, publish spaces, network relationships, participate in reward spaces.
- **Credits**: none.
- **Price**: free.

You can do the participant side of the platform on Free indefinitely — write posts, comment, vote, join Spaces, claim rewards. What Free doesn't unlock is hosting your own reward-bearing Spaces at scale.

### Pro — for small communities

- **Includes**: everything in Free.
- **40 monthly Credits.**
- **Up to 2 Credits per reward Space** (per Space cap).
- **10% creator share** of the total rewards distributed to participants in your Spaces.
- **Price**: ₩30,000 / month.

Use Pro when you want to host one or two small reward Spaces a month — a community poll with a modest prize pool, a quiz with a few prize slots.

### Max — for larger communities

- **Includes**: everything in Free.
- **190 monthly Credits.**
- **Up to 10 Credits per reward Space.**
- **10% creator share.**
- **Trusted creator badge** *(Coming soon — Phase 1)* — listed on the tier card; profile and Space rendering of the badge is on the roadmap.
- **Price**: ₩75,000 / month.

Max is the workhorse tier for active hosts running a handful of reward Spaces or a single ongoing flagship Space.

### VIP — for influencers and marketing

- **Includes**: everything in Free.
- **1,360 monthly Credits.**
- **Up to 100 Credits per reward Space.**
- **10% creator share.**
- **Trusted creator badge** *(Coming soon — Phase 1)* — listed on the tier card; profile and Space rendering is on the roadmap.
- **Access to raw participant data** — opens the per-record drilldown beyond the aggregate Analyzes view, useful for marketing reports and audience research.
- **Price**: ₩150,000 / month.

VIP is the tier for someone running large public campaigns where the analysis itself — not just the campaign — is the deliverable.

### Enterprise — partner plan

- **Includes**: everything in Free.
- **Fully customizable.**
- **Price**: starting at $1,000 / month — *Contact Us*. (Quoted price varies by region — the Korean tier card shows ₩1,000,000 / month and up; contact sales for the exact figure that applies to you.)

Enterprise is by-contact only. Press **Contact Us** on the card to start the conversation; the team works with you on Credit allotment, custom features, dedicated support, and contract terms.

## Buying a tier

Click **Get Pro** / **Get Max** / **Get VIP** on the card you want. A purchase modal opens with two views — the plan summary on the left and a card form on the right.

The card form is **PortOne**'s (Korea-first) checkout:

- **Full Name**
- **Card Number**
- **Expiry Date** — MM / YY
- **Birth Date / Business Registration Number** — YYMMDD for personal cards, 10 digits for business
- **Card Password** — first 2 digits

Click **Proceed to Payment** and the charge is processed. PortOne supports Visa, Mastercard, AMEX, JCB, plus the local methods PortOne offers in Korea. Billing is **off-chain** — there's no on-chain settlement step for tier subscriptions.

For the **Enterprise** tier the button is **Contact Us** instead, which opens a contact flow rather than a card form.

## Receipts and management

After a successful charge, a **Receipt modal** confirms the purchase with a transaction reference, the tier you bought, the charge amount, and the next renewal date.

After that, your tier shows up in two places:

- **`/membership`** — the card grid marks your current tier as active.
- **`/<your-handle>/settings`** — the **Subscription & Billing** card shows the active tier, the card on file, and your purchase history.

### Changing or canceling a tier

Manage subscription changes from the [Settings → Subscription & Billing](./settings#-subscription--billing) card. The settings card is the canonical place for billing operations — see that chapter for the exact controls available today and what's *(Coming soon)*.

## Tier limits and roadmap

A few things to know:

- **Tiers don't auto-prorate Credits between cycles.** If you upgrade mid-cycle, the higher tier's Credit allotment kicks in at the next renewal — not retroactively for the cycle you're already in.
- **Enterprise specifics are by contract.** The card grid only shows starting price; Credit allotment, custom limits, and feature scope are negotiated per customer.
- **Region pricing.** ₩-denominated tiers are for the Korean PortOne flow today. Multi-region pricing is *(Coming soon)*.
- **On-chain billing.** *(Coming soon)*. Tier subscriptions today are PortOne-only (off-chain). On-chain billing for users who'd rather pay from a wallet is on the roadmap.

## What's next

- [Settings → Subscription & Billing](./settings#-subscription--billing) — manage the tier you just bought.
- [Host Actions](./spaces/host-actions) — spend Credits on reward Actions in your Spaces.
- [Space Apps → Incentive Pool](./spaces/apps#-incentive-pool-beta) — fund a Space's pool from your Credit allotment.
