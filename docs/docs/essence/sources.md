---
sidebar_position: 3
title: Essence Sources
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Essence Sources

Your Essence has a control panel — one page where you can see every input feeding it, search across them, and clean out anything you no longer want. It lives at:

```
/essence
```

Open it any time from the [first-day checklist](../getting-started#first-day-checklist), the in-app navigation, or directly via the URL.

## What `/essence` shows you

The page is split into four bands, top to bottom:

1. **Hero** — total source count and total word count across your library.
2. **Breakdown** — the same totals split by source kind (filterable).
3. **Controls** — search and sort.
4. **Sources table** — one row per source, with per-row and bulk actions.

Everything you've authored across Ratel — posts, comments on posts, comments in discussions, polls, quizzes — is indexed here automatically. You don't add sources manually; you participate, and they appear.

## Hero — your library at a glance

The hero reads like a single sentence:

> **`N` sources · `M` words** *(example — your real numbers)*

These are the authoritative totals across your entire Essence — not just the rows currently visible in the table. Word count rolls up the actual content of every source.

## Breakdown — distribution by source kind

Below the hero is a row of cards, one per source kind. Each card shows the count, the share of your total (as a percentage), and acts as a filter chip — click it to scope the table below to just that kind:

- <img src={useBaseUrl('/img/icons/grid.svg')} width="16" height="16" alt="All" style={{verticalAlign: 'middle'}} /> **All** — every source, no filter.
- <img src={useBaseUrl('/img/icons/notion.svg')} width="16" alt="Notion" style={{verticalAlign: 'middle'}} /> **Notion** *(Coming soon)* — pages from your connected Notion workspace.
- **Posts** — every post you've published.
- **Comments** — replies under posts and inside discussion threads (both kinds counted together here).
- **Polls** — polls you host (the host's Essence indexes the poll once, regardless of how many participants respond).
- **Quizzes** — quizzes you host.

Click a card to filter; click **All** to clear the filter.

## Controls — search and sort

Above the table you'll find:

- **Search box** — type to match by title, content, or tag. Supports `⌘K` on macOS / `Ctrl K` elsewhere as a focus shortcut.
- **Sort** — pick from **Last edited ↓**, **Word count ↓**, or **Title A–Z**.

Search is live — results update as you type. Sort applies to whatever the current filter and search produce.

## Sources table — per-row controls

Each row in the table shows:

- The source **title**.
- A **kind badge** — *Post*, *Poll*, *Quiz*, *Post comment*, *Discussion comment*, or *Notion*. Comments carry the parent-context tag so you know whether the row came from a post thread or a discussion thread.
- The **word count**.
- The **last sync time** — when this source was last refreshed into your Essence.
- A **`…` menu** with row-level actions.

The row menu currently exposes **Delete** — removing a source from your Essence index. The original artifact (the post itself, the comment, the poll) stays put on the platform; only the Essence indexing of that artifact is dropped.

Pagination at the bottom of the table tells you "Showing X – Y of Z" with **Previous** / **Next** buttons. Click any column or chip and the page resets to the first page of the new view.

## Bulk actions *(Coming soon)*

A bulk-selection workflow — checkboxes on each row plus a floating bar at the bottom that reads **`N` selected · Remove** — is on the roadmap. The bar will appear once any row is selected, and **Remove** will delete every selected source from your Essence index in one shot.

Until that lands, use the per-row **`…` → Delete** menu to remove sources one at a time.

## Source kinds today (and roadmap)

**Live today** (native Ratel sources)

| Kind | What it captures |
|---|---|
| Posts | Every post you've published. |
| Post comments | Replies you've left under any post. |
| Discussion comments | Replies you've contributed to a Space's Discussion action. |
| Polls | Polls you host (the host's Essence indexes the poll once). |
| Quizzes | Quizzes you host. |

**Phase 1** *(Coming soon)*

| Kind | What it captures |
|---|---|
| Notion | Pages from a connected Notion workspace, kept in sync via webhooks. |

**Beyond** *(roadmap, not yet started)*

Per the README's connector roadmap: Google Docs, Substack, X, and Slack. These are not yet wired in any form — expect them after Notion ships and the Essence Connector framework is generalized.

Native Ratel sources are populated automatically as you act. External sources arrive via the **Essence Connector** — read-only OAuth plus real-time webhooks — and require a one-time connection from your settings before they start syncing.

## Privacy and control

Your Essence is yours. A few principles the page enforces:

- **Deletable.** Every source has a Delete in its row menu — removed sources are dropped from the index. (Bulk Remove is *(Coming soon)*; see the section above.)
- **AI-authored content opt-in** *(Coming soon)*. The platform's intent is for content authored by AI tools to be excluded from your Essence by default, with a per-source opt-in so AI-assisted writing can feed your Essence if you want. The flag isn't wired today; the policy lands in Phase 1.
- **Inference access only.** When the Essence House marketplace ships *(Coming soon)*, subscribers buy inference access — they can query your Essence, not download its raw contents. Removing a source from `/essence` removes it from inference too.

Open `/essence` whenever you want to know what your House actually knows about you — and adjust until the answer matches what you want it to say.
