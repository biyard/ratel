---
sidebar_position: 4
title: Space Dashboard (Host)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Space Dashboard

The Dashboard is the host's "how is my Space doing today" view — a stat-card grid plus a participant ranking, all driven directly off live activity. It lives at:

```
/spaces/:space_id/
```

(The trailing slash is required — Dashboard is a catch-all route.) Open it from the [Space tabs](./#the-tabs-inside-a-space) or paste the URL directly. The page splits into two views by role: **hosts (Creators)** see the full set of cards; **participants and viewers** see the same cards in read-only form and can find their own row in the ranking. Same URL, different controls — the dispatch is automatic.

## What the Dashboard shows

The page is one scrollable grid that adapts to screen size (multi-column on desktop, single-column on mobile), with the **Ranking** table sitting full-width below the grid when it has data.

### Card grid

Three cards always render; two more render conditionally based on whether the Space has any rewards / scoring data yet.

| Card                                                                                                                                                                                                                                                                                                                       | Always renders?                               | What it shows                                                                                                                             |
| -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg> **Stat summary** | yes                                           | Four counters at a glance: Total Participants · Total Likes · Total Comments · Total Actions.                                             |
| <img src={useBaseUrl('/img/icons/grid.svg')} width="16" alt="Actions" style={{verticalAlign: 'middle'}} /> **Action progress**                                                                                                                                                                                             | yes                                           | Per-action-type counts (Poll · Discussion · Quiz · Follow) shown as count rows.                                                           |
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" alt="Participants" style={{verticalAlign: 'middle'}} /> **Participants overview**                                                                                                                                                                                 | yes                                           | Headline participant count; the per-tab trend chart underneath is a placeholder today (no tabs are populated yet — see _What's missing_). |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="8" r="7"/><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/></svg> **Total points available**               | only when the Space has reward configurations | Total points pool and per-behavior breakdown of points the Space has available to distribute.                                             |

A brand-new Space shows the three always-on cards with mostly zeros; a Space with reward configurations adds the points card; once participants accumulate scoring activity, the ranking table fills in below.

### Ranking table

When at least one participant has accrued a score, a full-width **Ranking** table renders below the grid (top 50 entries, paginated 10 per page). Columns:

| Column          | What it shows                                                                                        |
| --------------- | ---------------------------------------------------------------------------------------------------- |
| **Rank**        | Position in the leaderboard, recomputed live from the score query. Click-to-sort is _(Coming soon)_. |
| **Participant** | Avatar + display name.                                                                               |
| **Score**       | Accumulated score for this Space. Click-to-sort is _(Coming soon)_.                                  |

A page-size pager at the bottom flips through additional pages — useful for Spaces where the leaderboard runs deep.

> **Tip.** Hosts use the Ranking table to spot top contributors before they generate a [Report](./reports). Participant scores roll up into the contributor share when the Phase 4 revenue split ships _(Coming soon)_.

## When to use Dashboard vs. Overview vs. Report

Spaces have a few host-facing surfaces. Dashboard is the only one with live stats — here's when to open which:

| Surface                                                                                                                                                                                                                                                                                                            | URL / trigger                                             | Open it when…                                                                                                                                        |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| <img src={useBaseUrl('/img/icons/grid.svg')} width="14" alt="Dashboard" style={{verticalAlign: 'middle'}} /> **Dashboard**                                                                                                                                                                                         | `/spaces/:space_id/`                                      | You want **live numbers** — participants, actions completed, leaderboard. Refresh a couple of times a day during an active campaign.                 |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" alt="Overview" style={{verticalAlign: 'middle'}} /> **Overview panel**                                                                                                                                                                                | topbar file-text icon (no URL — slides in over the arena) | You want to **edit the narrative** — what the Space is, who it's for, why someone should join. Creator sees an inline editor; viewers see read-only. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" alt="Report" style={{verticalAlign: 'middle'}} /> **Report**                                                                                                                                                                                          | `/spaces/:space_id/report/`                               | You want to **publish a longform AI-assisted report** synthesizing the Space's activity. See [Reports](./reports).                                   |
| <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/></svg> **Arena viewer** | `/spaces/:space_id/`                                      | You want to **see what a first-time visitor lands on** — the splash, the participation flow, and post-join the action carousel. Useful for QA.       |

Dashboard is the only one that updates automatically as activity rolls in; the others are about what you, as the host, are telling people.

## What's missing today

A few things the Dashboard _doesn't_ show today (each on the roadmap):

- **Per-action drilldown** _(Coming soon)_. The progress bars on the Participation's Action card are read-only summaries — you can't click a bar to drill into "which 12 people responded to this poll". Use the [Analyzes app](./apps#-analyzes) for that.
- **Date range picker** _(Coming soon)_. Today's numbers are cumulative; a "last 7 days" / "this cycle" filter is on the roadmap.
- **Export** _(Coming soon)_. CSV / PDF export of the dashboard as a snapshot. The Analyzes app does have an Excel export per analysis.

## What's next

- [Space Apps](./apps) — install the Incentive Pool that drives the dashboard's reward cards.
- [Host Actions](./host-actions) — create the polls, discussions, quizzes, and follows whose completion fills the progress bars.
- [Reports](./reports) — turn dashboard activity into a published narrative.
