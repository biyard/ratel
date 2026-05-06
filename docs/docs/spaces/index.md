---
sidebar_position: 1
slug: /spaces
title: Spaces
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Spaces

A **Space** is the arena where Ratel activity happens. Discussions, polls, quizzes, follow quests, scheduled meets — everything that turns a community into a measurable conversation lives inside a Space. From a Space, hosts shape what participants see, and participants accumulate Essence as they engage.

Every Space lives at a stable URL:

```
/spaces/:space_id
```

If you have the link, you can jump straight in. If you don't, you'll usually arrive at one through a creator's profile, your feed, or a notification.

## Two roles, one arena

Inside a Space there are really only two roles:

- **Hosts** decide what the Space *is*. They plug in [Apps](/spaces/apps) — info pages, file libraries, polls, quizzes, analyses — and configure how rewards flow.
- **Participants** show up to *do things*. They scroll the action carousel, vote, contribute opinions, follow people, and RSVP to meets. Each action becomes part of their Essence and, in reward-bearing Spaces, earns a piece of the Incentive Pool.

If you've never run a Space, start as a participant — the participant flow is the fastest way to understand what hosts are configuring on the other side.

## The tabs inside a Space

When you open a Space, the same `:space_id` is the root of several views you can switch between:

| Icon | Tab | URL | What it's for |
|---|---|---|---|
| <img src={useBaseUrl('/img/icons/compass.svg')} width="18" height="18" alt="Compass" style={{verticalAlign: 'middle'}} /> | **Index** (the arena/portal) | `/spaces/:space_id/` | The viewer splash. The first thing you see — branding, headline, and the participate / sign-in cards. |
| <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="3" width="4.5" height="14" rx="0.5"/><rect x="10" y="9" width="4.5" height="8" rx="0.5"/><rect x="10" y="3" width="4.5" height="2.5" rx="0.5"/></svg> | **Dashboard** | `/spaces/:space_id/dashboard` | Live activity at a glance. Active actions, participation counts, what's been happening recently. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="18" height="18" alt="File text" style={{verticalAlign: 'middle'}} /> | **Overview** | `/spaces/:space_id/overview` | The narrative tab. The host's pitch — what this Space is about, who it's for, and why you should care. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="18" height="18" alt="File text" style={{verticalAlign: 'middle'}} /> | **Report** | `/spaces/:space_id/report` | Published analyses and reports the host has shared with the community. The Phase 4 revenue split (10% platform · 60% host · 30% contributors) flows through here. |

There are also two deep links worth knowing:

- `/spaces/:space_id/discussions/:discussion_id` — open a single discussion as its own page (great for sharing on social).
- `/spaces/:space_id/discussions/:discussion_id/comments/:comment_id` — open a discussion and scroll directly to a specific comment, with the comment highlighted. Use this when you want to send someone *exactly* the reply you're talking about.

:::tip Sharing tip
Discussion deep links survive URL normalization — they keep working when posted into Bluesky, X, LinkedIn, or Slack. Comment deep links do the same.
:::

## Discovering Spaces

There are a few natural entry points:

- <img src={useBaseUrl('/img/icons/user.svg')} width="18" height="18" alt="User" style={{verticalAlign: 'middle'}} /> **From a profile.** Every user lists the Spaces they host or actively participate in (`/<your-handle>/spaces`).
- <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> **From your feed.** Posts can announce a Space, and the Space link drops you straight into the arena.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="11" cy="11" r="8"/><line x1="21" y1="21" x2="16.65" y2="16.65"/></svg> **From a search.** Search by topic or by Space title.
- <img src={useBaseUrl('/img/icons/bell.svg')} width="18" height="18" alt="Bell" style={{verticalAlign: 'middle'}} /> **From a notification.** When a host adds an action to a Space you've joined, you'll get a nudge with a direct link to the new quest.

## Joining a Space

Joining a Space is the moment your Essence pipeline starts collecting structured signal from your participation in it. From the Index page, you'll see a **Participate** card; sign in (or connect a wallet, if the host requires verification), and you're in.

What "joining" means in practice:

- You can open every action the host has published.
- Your votes, comments, follow choices, and quiz answers are tied to your account and become EssenceSources you own.
- If the Space has an **Incentive Pool**, you become eligible to receive a share when actions are completed.
- You'll receive notifications when the host posts a new action, publishes a report, or replies to a comment you wrote.

:::note Coming soon
Phase 0's full Essence pipeline — embedding every Space action into your personal knowledge base — is on the way. Until it ships, your activity is recorded and rewarded normally; the embedding layer is added on top later, retroactively.
:::

## Where to go next

This chapter has several sub-pages — by role and by host workflow:

- **[Space Apps](/spaces/apps)** — the host's toolkit. Info pages, files, AI-assisted reports, panels, the Incentive Pool.
- **[Space Actions](/spaces/actions)** — the participant's quest board. Discussions, polls, quizzes, follows, meets — and how doing them feeds your Essence.
- **[Space Dashboard](/spaces/dashboard)** — the host's live stats: card grid + participant ranking.
- **[Host Actions](/spaces/host-actions)** — host-side editors for creating Discussions, Polls, Quizzes, and Follows.
- **[Reports](/spaces/reports)** — the AI-assisted longform Report and the cross-filter Analyzes that feed it.
