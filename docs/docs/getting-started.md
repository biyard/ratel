---
sidebar_position: 2
title: Getting Started
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Getting Started

Welcome to Ratel — the Human Essence Platform. This chapter walks you through your first day: signing in, setting up your profile, connecting external accounts, and finding your way around.

You can browse most of Ratel without an account, but you'll need one to post, vote, join spaces, build your Essence, or earn rewards.

## Sign up & sign in

Find the <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4"/><polyline points="10 17 15 12 10 7"/><line x1="15" y1="12" x2="3" y2="12"/></svg> **Sign In** button in the left-side menu (or in the bottom bar on mobile) — it's a small door-with-arrow icon. Clicking it opens the **Join the Movement** popup, which is where both new and returning users start.

In the popup you have three ways to continue:

- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="2" y="4" width="20" height="16" rx="2"/><path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/></svg> **Email & password** — Enter your email, click **Send**, and paste the 6-digit code we mail you. Click **Verify**, then **Create an account** at the top of the popup to set a password and finish signup. Returning users just type the email and password and click **Continue**.
- <img src={useBaseUrl('/img/icons/google.svg')} width="18" alt="Google" style={{verticalAlign: 'middle'}} /> **Continue With Google** — One-click sign-in with your Google account. If it's your first time, Ratel asks you to pick a username and accept the terms; after that Google logins are instant.
- <img src={useBaseUrl('/img/icons/walletconnect.svg')} width="18" alt="WalletConnect" style={{verticalAlign: 'middle'}} /> **Continue With Wallet** — Connects via **WalletConnect**, which lets you pair any wallet that supports it (MetaMask, Rainbow, Trust Wallet, Coinbase Wallet, and many others) by scanning a QR code or tapping a deep link. You'll be asked to sign a short message to prove you own the wallet — no transaction, no gas fee.

Forgot your password? Click **Forgot password?** under the password field and we'll email you a reset link.

> 💡 **Tip:** A wallet is optional for everyday browsing, posting, and commenting, but it's required for some on-chain features (e.g. credentials, future on-chain settlement). You can connect a wallet later from your settings.

## Connect your networks (onboarding)

Right after signup, Ratel offers you a one-step **Connections** onboarding page at:

```
/onboarding/connections
```

The page is the friendly first-run version of [Posts → Cross-posting](./posts.md#cross-posting). It explains what cross-posting is — *"Your first post reaches three networks instantly"* — and gives you a card per destination so you can connect once and have every Ratel post fan out to your other social timelines with a backlink.

Each card on the onboarding page:

| Platform | Status today | What the card says |
|---|---|---|
| <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** | Live | AT Protocol · 300 chars. Connects via app password (you generate one at `bsky.app/settings/app-passwords` and Ratel stores it encrypted). |
| <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** *(Coming soon)* | OAuth integration arrives in Phase 1B | Your professional network · 3,000 chars. *Notify me* button instead of *Connect*. |
| <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** *(Coming soon)* | Meta OAuth integration arrives in Phase 1C | Meta · ~275M users · 500 chars. Requires an Instagram Professional account once it ships. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M15 8l-7 7-3-3"/><circle cx="12" cy="12" r="10"/></svg> **Farcaster** *(Coming soon — Phase 2)* | Frames integration + agent posting in Phase 2 | 320 chars · Web3-native. *Notify me* button. |

You can **Skip for now →** at the top of the page and the onboarding closes — Ratel won't nag you about it again. To come back later, open [Posts → Cross-posting](./posts.md#cross-posting) at `/<your-handle>/settings/connections` whenever you're ready. Whatever you connect on the onboarding page is already saved when you skip the rest.

> **Pro tip from the page itself:** *"Creators who connect 2+ networks get 3.4× more subscribers in their first 30 days."*

## Set up your profile

The first thing to pick is your **handle**. This becomes your home on Ratel — your profile, your posts, your spaces, your drafts, and your rewards all live under `/<your-handle>`. For example, if your handle is `alex`:

- `/alex` — your profile home
- `/alex/posts` — posts you've written
- `/alex/spaces` — spaces you've created or joined
- `/alex/rewards` — rewards you've earned
- `/alex/drafts` — drafts in progress
- `/alex/settings` — account & profile settings

Go to **/your-handle/settings** to fill in:

- **Display name** — the name shown on your posts and comments
- **Avatar** — your profile picture (any image you upload)
- **Banner** — a wider header image shown on your profile page
- **Bio** — a short description of who you are and what you write about

Click **Save** when you're done. Changes are visible immediately.

## Connect external accounts

Ratel can talk to the platforms you already use. Open **/your-handle/settings/connections** to manage them. Each connection uses **OAuth** — you authenticate on the provider's site, Ratel never sees your password, and you can revoke access at any time from the same page.

| Icon | Platform | What it does |
|---|---|---|
| <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> | **Bluesky** | Cross-post your Ratel posts to Bluesky |
| <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> | **LinkedIn** *(planned / 예정)* | Cross-post to your LinkedIn feed |
| <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> | **Threads** *(planned / 예정)* | Cross-post to Threads |
| <img src={useBaseUrl('/img/icons/notion.svg')} width="16" alt="Notion" style={{verticalAlign: 'middle'}} /> | **Notion** *(planned / 예정)* | Pull your Notion docs into your Essence as a source |

To connect a platform, click **Connect** next to its name and follow the OAuth prompt. Once connected, you can:

- Toggle **auto-post** on a per-platform basis (whether new Ratel posts publish automatically)
- **Disconnect** at any time — connections are per-user and revocable

> 💡 **Tip:** First time signing up? You'll see a guided **Connections** onboarding step at `/onboarding/connections` — feel free to skip it and come back later.

## Orient yourself

Ratel's main surfaces are the same on desktop and mobile, just rearranged:

- **Top navbar** — search, notifications, and your profile menu (top-right). Your profile menu links to your profile, drafts, settings, and sign-out.
- <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> **Sidebar / left rail** — quick links to **Home**, **Membership**, **Credentials**, **Rewards**, and **Settings**, plus the language and theme toggles in the footer. The collapse button (<svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="3" width="18" height="18" rx="2"/><path d="M9 3v18"/><path d="m16 15-3-3 3-3"/></svg>) at the very bottom toggles the rail between full-width and icon-only.
- <img src={useBaseUrl('/img/icons/bell.svg')} width="18" height="18" alt="Bell" style={{verticalAlign: 'middle'}} /> **Notifications** — the bell icon shows unread activity: replies, mentions, follow requests, space invitations, and reward updates.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"/></svg> **Theme toggle** — switch between **dark** (default), **light**, and **system** modes. Ratel respects your choice across pages and devices.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><path d="M2 12h20"/><path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"/></svg> **Language toggle** — Ratel speaks **English** and **한국어**. The flag in the sidebar footer flips locales.

On mobile, the sidebar collapses behind a menu button and the navbar stays pinned to the top. Everything else works the same — write, vote, join spaces, and check notifications on the go.

## Stay on top of notifications

Click the <img src={useBaseUrl('/img/icons/bell.svg')} width="16" alt="Bell" style={{verticalAlign: 'middle'}} /> **bell icon** in the top navbar to open your notifications panel. The icon shows an unread-count badge whenever there's something new (capped at 100+ for sanity).

Inside the panel:

- **Newest first** — your inbox is paginated; scroll for older items.
- **Click an item** — opens whatever it points at (a post, a comment thread, a Space action, a reward event) and marks the item as read.
- **Mark all as read** — a single button at the top of the panel clears the badge in one shot.
- **Unread-only filter** — toggle to hide already-read items and focus on what's new.

Notifications fire for replies to your posts, mentions of your handle, new follow relationships, Space invitations and new actions in Spaces you've joined, sub-team application updates, and reward events (cycle locks, claims). Email or push delivery preferences are *(Coming soon)* — for now the bell is the canonical surface.

## First-day checklist

Spend ten minutes ticking these off and Ratel will start feeling like home:

- [ ] **Pick a handle and fill in your profile** — display name, avatar, banner, bio
- [ ] **Write your first post** — share an idea, a question, or a link from `/posts`
- [ ] **Like a post and follow someone** — likes and follows feed your Essence and the platform's recommendations
- [ ] **Browse or join a space** — spaces are where collective work happens (polls, discussions, quizzes, follow quests)
- [ ] **Open an action** — vote in a poll, answer a quiz, or weigh in on a discussion
- [ ] **Connect at least one external account** — start with Bluesky from `/your-handle/settings/connections`
- [ ] **Peek at your Essence** — Visit `/essence` to see every source feeding your Essence today.

That's it — you're set up. Next up: writing posts and joining spaces.

> 💡 **Tip:** Stuck on something? Check the **Troubleshooting** section in the sidebar, or ping us from the **Help** menu in your profile dropdown.
