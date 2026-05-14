---
sidebar_position: 5
title: Tokens
---

# Tokens

**Tokens** are the on-chain expression of a past cycle's Reward Points. Where Points are off-platform-chain bookkeeping, Tokens are blockchain-native assets you claim into a wallet you control. They settle the platform's promise that *participation has value* — and they're 100% backed by a treasury reserve.

## How Tokens are minted

Tokens are minted **once per cycle**, and only against a cycle that has *locked in*:

1. Throughout the month, you accrue Reward Points in the current cycle. The Rewards page shows your *estimated tokens* for the cycle at today's treasury rate, but you can't claim yet.
2. **At month-end**, the cycle locks in. The Points you earned that month become the basis for that cycle's Token payout, computed at the cycle's per-block-updated **treasury rate**.
3. **On the 1st of next month**, the locked cycle becomes claimable. The Rewards page surfaces a per-cycle card with the amount and a **Claim** button.

There's no "soft claim" — until you connect a wallet and submit the on-chain transaction, the Tokens haven't moved.

## The two-step claim flow

Claiming a past cycle's Tokens is a deliberately two-step process. Both steps happen on the cycle's card:

### 1. Connect your wallet

The Claim button asks you to connect a wallet. Any [WalletConnect](../getting-started#sign-up--sign-in)-compatible wallet works — MetaMask, Rainbow, Trust Wallet, Coinbase Wallet, and others. The connection also confirms the chain you're claiming on.

You don't need a wallet to *earn* Points; that flow is entirely off-platform-chain. **Claiming Tokens** is the only step in the rewards lifecycle that requires one.

### 2. Sign and submit

Ratel issues a signed claim payload — a tuple of *(amount, deadline, nonce, contract address, chain id)* — for the cycle. Your wallet then submits the on-chain transaction that transfers the tokens from the treasury contract to your address.

Once the transaction confirms, the cycle's card flips to **Claimed**. The amount, the timestamp, and the transaction hash become part of the cycle's audit record.

## Treasury rate and backing

The **treasury rate** is the per-cycle exchange rate at which Points convert to Tokens at claim time. It updates **every block**, so the rate you see at the moment of claim is what you receive.

A treasury panel on the Rewards page shows:

- **Current rate** — Points per Token at the most recent update.
- **Treasury balance** — total Tokens held in reserve to back outstanding cycles.
- **Circulating supply** — Tokens already claimed and in user wallets.
- **Backing ratio** — anchored at **100%** (fully collateralized).

The 100% backing is the platform's hard guarantee: every Token you claim is collateralized 1:1 by a Token already in the treasury reserve at the moment of claim. The treasury never over-issues.

## What's *(Coming soon)*

- **Section-level Swap All.** A single-click bulk claim that batches every claimable cycle in one transaction. Today the *Claim* button is per-cycle (the in-app label currently reads *Swap All* — the i18n key is reused — but the action operates on one cycle at a time).
- **Off-chain cashout.** Today rewards settle as Tokens via on-chain claim. An off-chain cashout flow for users who'd rather not handle a wallet is on the roadmap. Tier *subscriptions* are already billed off-chain via PortOne, but reward *payouts* are on-chain only at MVP.
- **Stablecoin denomination.** Some Spaces will be able to denominate payouts in a fiat-pegged Stablecoin instead of the volatile native Token. Optional and host-configurable. *(Coming soon)*

## Privacy and the DID link

The on-chain transaction is signed against your wallet *and* against the [DID](../credentials/attributes) bound to your account. The DID is what cryptographically links the off-platform-chain Points ledger to the on-chain claim — it's the bridge that lets the treasury contract trust that the claim payload corresponds to the right participant.

You don't need to think about this normally; it's the platform plumbing that makes "claim past-cycle Tokens" safe to expose to a wallet. But if you ever audit a Token claim transaction on-chain, the DID link is what the contract verifies before releasing the tokens.

## Related

- [Reward Points](./points.md) — the off-chain unit Tokens convert from at cycle lock-in.
- [Credentials → Attribute management](../credentials/attributes) — your DID is what signs the claim transaction.
