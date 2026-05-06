---
sidebar_position: 8
title: Settings
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Settings

This is the single place to change everything about your account — how you appear, how you sign in, what plan you're on, and how an AI agent connects to your Ratel data.

## Where settings live

```
/<your-handle>/settings
```

Open it from the user dropdown at the bottom of the sidebar (click *Settings*) or paste the URL directly. The page is private — only you, signed in, can see it; nobody else can open `/<their-handle>/settings`.

The page is one column of cards, top to bottom: **Profile · Password · Subscription & Billing · MCP Server**. There's a separate sub-page at `/<your-handle>/settings/connections` for cross-posting destinations.

## <img src={useBaseUrl('/img/icons/user.svg')} width="20" height="20" alt="Profile" style={{verticalAlign: 'middle'}} /> Profile

The first card. Everything visible on `/<your-handle>` is edited here:

- **Avatar** — click the circular thumbnail (or the *Upload* placeholder) and pick any image. The new avatar appears in your sidebar, your profile, and every post you've written.
- **Username** — locked. The handle you chose at signup is hard to change; the field is shown in read-only form so you know what your URLs are built on.
- **Email** — locked. Email lives on the account itself; if you need to change it, get in touch from the Help menu.
- **Display Name** — the name shown above your posts and on your profile. Up to 30 characters.
- **Description (Bio)** — a longer paragraph about you. Use it to tell visitors what you write about, what Spaces you host, what you'd like an Essence subscriber to know.

Click **Save** to apply. Changes are visible immediately. If you type something that fails the platform's content filter (offensive language), the **Save** button stays disabled — fix the text and it re-enables.

## <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="11" width="18" height="11" rx="2" ry="2"/><path d="M7 11V7a5 5 0 0 1 10 0v4"/></svg> Password

The second card. Set or change the password used for email-and-password sign in.

- **Current Password** — what you log in with today.
- **New Password** — at least 8 characters; mixing letters, numbers, and symbols is encouraged.
- **Confirm Password** — type the new one again to catch typos.

Click **Update Password** to apply. If you signed up with Google or a wallet and never set a password, the form lets you add one for the first time so you have a fallback method.

Forgot your current password? Use **Forgot password?** on the sign-in screen — you'll get an email reset link.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="Subscription" style={{verticalAlign: 'middle'}} /> Subscription & Billing

The third card. Read your active membership tier and manage the card on file.

- **Current Plan** — A badge showing your active tier (Free, Pro, Max, Vip, Enterprise) plus a **Change Plan** link that opens [/membership](./membership) where you actually swap tiers.
- **Credits** — Remaining and total Credits for the current cycle (e.g. `145 / 190`). On Free this reads `0 / 0`.
- **Expires** — When the current cycle's allotment expires (Unlimited on Free).
- **Card on file** — A masked card number plus the cardholder name, with a button beside it: **Add Card** (no card yet), **Change Card** (a card on file already), or **Cancel** while the card form is open. Card processing is handled by **PortOne**, so you can pay with the methods PortOne supports for your region (Visa, Mastercard, AMEX, JCB, plus local options in Korea).

The card itself **does not** include an in-place tier swap or a purchase-history list — both live elsewhere. Switching tier is a click-through to `/membership`; per-purchase receipts surface in the Receipt modal at the moment of charge.

Membership billing is **off-chain only** — there's no on-chain settlement step for tier subscriptions.

## <img src={useBaseUrl('/img/icons/grid.svg')} width="20" height="20" alt="MCP" style={{verticalAlign: 'middle'}} /> MCP Server

The fourth card. This is where you mint and rotate the secret token that lets an AI assistant connect to your Ratel account.

- **Generate Secret** — first time? Click this to mint your token. Ratel constructs a URL of the form `https://ratel.foundation/mcp/<your-token>` and shows it once — copy it on the spot.
- **Regenerate** — already have a token but want a fresh one? Click *Regenerate*. The new URL appears immediately and the old one stops working. Treat the URL like an API key.

For full setup walkthroughs (Claude Desktop, Claude Code, Cursor, generic JSON), see [Connect AI](./ai-connect). The same controls also exist in a dedicated arena layout at `/my-ai`, described in [My Essence → My AI](./my-essence#-my-ai--my-ai).

## <img src={useBaseUrl('/img/icons/bluesky.svg')} width="20" height="20" alt="Connections" style={{verticalAlign: 'middle'}} /> Connections (separate page)

The connections page isn't a card on `/settings` — it's a separate route:

```
/<your-handle>/settings/connections
```

It manages the platforms Ratel can cross-post to. The hero stat card at the top tells you how many destinations are connected and a *posts this month* counter (currently always reads 0 — accurate count is *(Coming soon)*).

| Platform | Status | What it does |
|---|---|---|
| <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** | Live | Connect once, then auto-syndicate Ratel posts to your Bluesky timeline. Toggle auto-post per account. |
| <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** | *(Coming soon)* | Cross-post long-form content to your LinkedIn feed. |
| <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** | *(Coming soon)* | Cross-post to Threads with automatic length adaptation. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M15 8l-7 7-3-3"/><circle cx="12" cy="12" r="10"/></svg> **Farcaster** | *(Coming soon)* | Cross-post to Farcaster casts. |

Each platform card has **Connect** / **Disconnect** and an **Auto-post** toggle. OAuth happens on the platform's own login page — Ratel never sees your password, and you can revoke access at any time from this same page.

If you're brand new to Ratel, you'll also see the same destinations on the guided onboarding page at `/onboarding/connections`. Skip and come back later if you'd rather connect them as you need them.

> **Note: Notion isn't a cross-post destination.** Notion belongs on the *inbound* side as an Essence source — pulling your Notion docs *into* your Essence — and it's currently *(Coming soon)*. The cross-posting page is for *outbound* publishing only.

## What about language, theme, and notifications?

These three controls don't live inside `/settings` because they're more useful where you encounter them:

- <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg> **Language toggle** — in the **sidebar footer**. Flip between English and 한국어 with one click; your preference persists across pages and devices.
- <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg> **Theme toggle** — also in the **sidebar footer**. Cycle dark / light / system.
- <img src={useBaseUrl('/img/icons/bell.svg')} width="16" height="16" alt="Notifications" style={{verticalAlign: 'middle'}} /> **Notifications** — open the bell icon in the top navbar to read your inbox. Mark items as read individually, or use *Mark all as read* in the panel header.

A dedicated notifications-preferences card and a self-serve account-deletion flow are *(Coming soon)*.

## Summary table

| Card | URL | What you change |
|---|---|---|
| Profile | `/<your-handle>/settings` (top card) | Avatar, display name, bio |
| Password | same page | Sign-in password |
| Subscription & Billing | same page | View tier + Credits + expiry; manage card. Tier change → click-through to `/membership`. |
| MCP Server | same page | Generate / regenerate your AI endpoint |
| Connections | `/<your-handle>/settings/connections` | Bluesky (live) + LinkedIn / Threads / Farcaster *(Coming soon)* |
