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

- **Hosts** decide what the Space _is_. They plug in [Apps](/spaces/apps) — info pages, file libraries, polls, quizzes, analyses — and configure how rewards flow.
- **Participants** show up to _do things_. They scroll the action carousel, vote, contribute opinions, follow people, and RSVP to meets. Each action becomes part of their Essence and, in reward-bearing Spaces, earns a piece of the Incentive Pool.

If you've never run a Space, start as a participant — the participant flow is the fastest way to understand what hosts are configuring on the other side.

## What lives at each Space URL

The arena viewer (`/spaces/:space_id/`) is where most of the action lives — the splash, the participate flow, and after joining, the action carousel. Other surfaces are either **sliding panels** triggered from the topbar (no URL of their own) or **deep-link sub-paths** for hosts and tooling.

### Pages

| URL                         | What it's for                                                                                                                                                      |
| --------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `/spaces/:space_id/`        | **Arena viewer** — the splash. Branding, headline, and the _Participate_ / _Sign in_ cards. After joining, the action carousel renders here.                       |
| `/spaces/:space_id/report/` | **Report** — published analyses the host has shared with the community. The Phase 4 revenue split (10% platform · 60% host · 30% contributors) flows through here. |

### Topbar panels (no URL)

Triggered from icons in the arena topbar; they slide in over the viewer instead of taking you to a new page. They live next to the viewer at the same `/spaces/:space_id/` URL.

| Trigger                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     | Panel                  | What it's for                                                                                                                                                        |
| ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="16" height="16" alt="Overview" style={{verticalAlign: 'middle'}} /> Overview icon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                  | **Overview panel**     | Host's narrative pitch — what this Space is about, who it's for, why you should care. Creators see an inline editor here; non-creators see read-only.                |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="3"/><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z"/></svg> Settings (gear) | **Settings panel**     | Theme · language · logout. For hosts, also lists installed and available **Apps** (General, Files, Analyzes, Panels, Incentive Pool) — install / open settings here. |
| <img src={useBaseUrl('/img/icons/bell.svg')} width="16" height="16" alt="Bell" style={{verticalAlign: 'middle'}} /> Bell icon                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               | **Notification panel** | In-app notifications for activity in this Space.                                                                                                                     |

### Host deep links

Hosts mostly reach these through the panels above. The URLs exist for tooling, e2e tests, and shareable bookmarks.

| URL                             | What it's for                                                                                                                                        |
| ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| `/spaces/:space_id/actions/...` | **Action editors** — per-action-type sub-paths. e.g. `/actions/polls/:poll_id`, `/actions/discussions/:discussion_id/edit`.                          |
| `/spaces/:space_id/apps/<slug>` | **App settings** — e.g. `/apps/general`, `/apps/files`, `/apps/panels`. Same destination the _Settings panel → app row → Settings_ button takes you. |

### Deep links

- `/spaces/:space_id/discussions/:discussion_id` — open a single discussion as its own page (great for sharing on social).
- `/spaces/:space_id/discussions/:discussion_id/comments/:comment_id` — open a discussion and scroll directly to a specific comment, with the comment highlighted. Use this when you want to send someone _exactly_ the reply you're talking about.

:::tip Sharing tip
Discussion deep links survive URL normalization — they keep working when posted into Bluesky, X, LinkedIn, or Slack. Comment deep links do the same.
:::

## Discovering Spaces

There are a few natural entry points:

- <img src={useBaseUrl('/img/icons/user.svg')} width="18" height="18" alt="User" style={{verticalAlign: 'middle'}} /> **From a profile.** Every user lists the Spaces they host or actively participate in (`/<your-handle>/spaces`).
- <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> **From your feed.** Posts can announce a Space, and the Space link drops you straight into the arena.
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

## Walkthrough

> 🎬 **Walkthrough:** opening a Space and exploring the arena, panels, and action carousel.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/overview.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/overview.mov')}>Download the walkthrough</a>.
</video>

## Where to go next

This chapter has several sub-pages — by role and by host workflow:

- **[Space Apps](/spaces/apps)** — the host's toolkit. Info pages, files, AI-assisted reports, panels, the Incentive Pool.
- **[Space Actions](/spaces/actions)** — the participant's quest board. Discussions, polls, quizzes, follows, meets — and how doing them feeds your Essence.
- **[Space Dashboard](/spaces/dashboard)** — the host's live stats: card grid + participant ranking.
- **[Host Actions](/spaces/host-actions)** — host-side editors for creating Discussions, Polls, Quizzes, and Follows.
- **[Reports](/spaces/reports)** — the AI-assisted longform Report and the cross-filter Analyzes that feed it.
