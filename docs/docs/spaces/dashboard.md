---
sidebar_position: 4
title: Space Dashboard (Host)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Space Dashboard

The Dashboard is the host's "how is my Space doing today" view — a single screen of stat cards and a participant ranking, refreshed live as people engage. It lives at:

```
/spaces/:space_id/dashboard
```

Open it from the [Space tabs](./#the-tabs-inside-a-space) or paste the URL directly. The page splits into two views by role: **hosts (Creators)** see the full dashboard with creator-only setup hints (e.g. the *Setup Now* button on an unfunded Incentive Pool card); **participants and viewers** see the same cards in read-only form and can find their own row in the ranking. The dispatch is automatic — same URL, different controls.

## What the Dashboard shows

The page is one scrollable column. Cards arrange themselves into rows that adapt to your screen (4 columns on desktop, 2 on tablet, 1 on mobile) and the **Ranking** table sits full-width at the bottom.

### Card grid

Five card variants are rendered in the grid, each with a quick-glance headline number on top and a short list of supporting metrics below; a sixth — the **Ranking table** — sits full-width below the grid (covered in its own section).

| Card | Headline | What it shows |
|---|---|---|
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" alt="Participants" style={{verticalAlign: 'middle'}} /> **Space Views** | total participants | Total Participants · Total Likes · Total Comments · Total Actions |
| <img src={useBaseUrl('/img/icons/award.svg')} width="16" alt="Pool" style={{verticalAlign: 'middle'}} /> **Incentive Pool** | total pool size | Total Winners · Rank Rate · Incentive Pool balance |
| <img src={useBaseUrl('/img/icons/grid.svg')} width="16" alt="Actions" style={{verticalAlign: 'middle'}} /> **Participation's Action** | total actions across types | Per-type completion bars (Poll · Discussion · Quiz · Follow), each as `count / total` with a progress bar |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg> **Total Participants** | participants over time | Tabbed chart of participation trend by Action type |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="8" r="7"/><polyline points="8.21 13.89 7 23 12 20 17 23 15.79 13.88"/></svg> **Total Points Available** | total points pool | Per-action breakdown of points the Space has available to distribute |

If you're the host and your Space has **no Incentive Pool configured yet**, the Pool card shows a violet **Setup Now** button that deep-links you to `/spaces/:space_id/apps/` so you can install the [Incentive Pool app *(Beta)*](./apps#-incentive-pool-beta). Participants viewing the same card don't see the button.

The grid is opportunistic: the cards shown for a given Space depend on what apps you've installed and what activity has accumulated. A brand-new Space mostly shows zeros; a busy Space mid-cycle fills every card.

### Ranking table

A full-width **Ranking** table at the bottom of the page lists every participant who has accrued a score in this Space. Three columns:

| Column | Sortable | What it shows |
|---|---|---|
| **Rank** | no — visual indicator only *(Coming soon)* | Position in the leaderboard, recomputed live. The sort glyph renders next to the header but click-to-sort is on the roadmap. |
| **Participant** | — | Avatar + display name. |
| **Score** | no — visual indicator only *(Coming soon)* | Their accumulated score for this Space (with an `i` info tooltip explaining what's counted). |

A page-size pager at the bottom flips through additional pages of participants — useful for Spaces where the leaderboard runs deep.

> **Tip.** Hosts use the Ranking table to spot top contributors before they generate a [Report](./reports). Participant scores roll up into the contributor share when the Phase 4 revenue split ships *(Coming soon)*.

## When to use Dashboard vs. Overview vs. Report

The Space has four host-facing tabs. They overlap a little, so here's a quick guide for which one to open when:

| Tab | URL | Open it when… |
|---|---|---|
| <img src={useBaseUrl('/img/icons/grid.svg')} width="14" alt="Dashboard" style={{verticalAlign: 'middle'}} /> **Dashboard** | `/spaces/:space_id/dashboard` | You want **live numbers** — how many people, how many actions, how the pool is being spent. Refresh a couple of times a day during an active campaign. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" alt="Overview" style={{verticalAlign: 'middle'}} /> **Overview** | `/spaces/:space_id/overview` | You want to **edit the narrative** — what the Space is, who it's for, why someone should join. This is the host's pitch, written by you. |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="14" alt="Report" style={{verticalAlign: 'middle'}} /> **Report** | `/spaces/:space_id/report` | You want to **publish a longform AI-assisted report** synthesizing the Space's activity. See [Reports](./reports). |
| <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><polygon points="16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76"/></svg> **Index** (portal) | `/spaces/:space_id/` | You want to **see the public viewer splash** — the page a first-time visitor lands on. Useful when QA-checking how your Space looks to outsiders. |

Dashboard is the only one of these that updates automatically as activity rolls in; the other three are about what you, as the host, are telling people.

## What's missing today

A few things the Dashboard *doesn't* show today (each on the roadmap):

- **Per-action drilldown** *(Coming soon)*. The progress bars on the Participation's Action card are read-only summaries — you can't click a bar to drill into "which 12 people responded to this poll". Use the [Analyzes app](./apps#-analyzes) for that.
- **Date range picker** *(Coming soon)*. Today's numbers are cumulative; a "last 7 days" / "this cycle" filter is on the roadmap.
- **Export** *(Coming soon)*. CSV / PDF export of the dashboard as a snapshot. The Analyzes app does have an Excel export per analysis.

## What's next

- [Space Apps](./apps) — install the Incentive Pool that drives the dashboard's reward cards.
- [Host Actions](./host-actions) — create the polls, discussions, quizzes, and follows whose completion fills the progress bars.
- [Reports](./reports) — turn dashboard activity into a published narrative.
