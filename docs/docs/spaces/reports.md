---
sidebar_position: 6
title: Reports (Host)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Reports

A **Report** is the host's published synthesis of what happened in their Space — what participants thought, where they agreed, where they didn't, what stood out. Ratel ships two report surfaces that work together:

| Surface | URL | What it is |
|---|---|---|
| **Space Report** (longform) | `/spaces/:space_id/report` | One per Space. The host's narrative document, with an *AI-generate-then-edit* workflow. This is what subscribers will eventually pay for in Phase 4. |
| **Analyzes** (cross-filter) | `/spaces/:space_id/apps/analyzes/...` | Many per Space. Saved cross-filter analyses over polls, quizzes, follows, and discussions, with raw-records drilldown. The data you cite from the longform Space Report. |

This chapter walks through both, in the order a host typically uses them: build a few Analyzes to see what's there, then write the Space Report on top.

:::note Production availability
The Analyzes app is hidden in production while it goes through validation — it's available on dev / staging today. The longform Space Report at `/spaces/:space_id/report` is available everywhere. **On production today, the recommended *Analyze-first* workflow below isn't available** — you can still write the longform Report from scratch using your own observations and link to the Space's individual polls or discussions for evidence.
:::

## <img src={useBaseUrl('/img/icons/grid.svg')} width="22" height="22" alt="Analyzes" style={{verticalAlign: 'middle'}} /> Building an Analyze

The Analyzes app at `/spaces/:space_id/apps/analyzes` is a **horizontal carousel** of saved analyses. The first card is always **+ Create new analysis**; cards to its right are the Reports you've already built. Use the arrows to flip through and click a card to open it.

### Step 1 — Pick cross filters

`/spaces/:space_id/apps/analyzes/create`

The create flow is two steps. The first is a panel where you pick the data you want to cross-cut: every poll question, every quiz question, every follow target, every discussion thread in the Space is available as a tile. Pick one or more — each becomes a **filter chip**. You can also paste comma-separated **keywords** that each become their own filter (e.g. `evidence, statement, victim`).

A live counter beside each tile shows how many participants and records currently match your filter set, so you know whether you've narrowed too far.

### Step 2 — Preview and generate

The second step is the **Preview** panel. You give the analysis a name (used as the result-page heading and the carousel-card title), confirm the cross filters and a sample of the raw data they match, and click **Generate report**. Ratel takes you to the result page once it's ready.

A status badge on each carousel card tells you whether an analysis is **Running**, **Analysis complete**, or **Failed**. Running analyses can't be opened until they finish.

### Step 3 — Read the result

`/spaces/:space_id/apps/analyzes/report/:report_id`

The result page shows the analysis's selected filters as chips at the top, then renders the data — distribution charts for poll responses, scoring summaries for quizzes, follow counts, discussion comment frequencies — narrowed to the cross-filter you defined. The chips are frozen at report-save time, so reopening the analysis a month later shows you the exact slice you saved.

### Step 4 — Drill into the records

Click **View raw data** to open `/spaces/:space_id/apps/analyzes/report/:report_id/records` — a paginated table of every individual record (User · Question · Answer · Post · Comment · Followed) that matched your filters. Click any chip in the chip header to filter the records further.

Two more deep links are useful when you want to drill from a specific Action rather than the whole analysis:

- `/spaces/:space_id/apps/analyzes/poll/:poll_id` — Per-poll analyze view.
- `/spaces/:space_id/apps/analyzes/discussion/:discussion_id` — Per-discussion analyze view.

A **Download Excel** button on each analysis exports the matching records as a spreadsheet.

## <img src={useBaseUrl('/img/icons/file-text.svg')} width="22" height="22" alt="Report" style={{verticalAlign: 'middle'}} /> Writing the Space Report

Every Space has one canonical Report at:

```
/spaces/:space_id/report
```

It's the public-facing narrative subscribers will eventually pay for. The page has three modes:

- **Read-only** — what visitors see (and what subscribers will see in Phase 4).
- **Editable** — what you, as host, see when you click *Toggle Edit*. The same page becomes a rich-text editor.
- **Generating…** — what you see after clicking **Generate AI Report**. Ratel synthesizes a draft from the Space's accumulated activity and drops it into the editor for you to refine.

### Authoring flow

A typical host loop:

1. **Open the page** — `/spaces/:space_id/report`. The first time, it's empty.
2. **Click *Generate AI Report*** — Ratel reads the Space's activity (polls, discussions, quizzes, follows) and produces a draft narrative. You'll see a *Generating…* placeholder while it works.
3. **Click *Toggle Edit*** — switch into Editable mode. Refine the AI draft in the rich-text editor: add framing, cite specific Analyzes you built earlier, drop in conclusions the AI couldn't reach on its own.
4. **Click *Save*** — the page flips back to Read-only and is now what visitors see.

Re-run *Generate AI Report* any time activity in the Space picks up — Ratel produces a fresh draft from the current data, and you decide which parts to merge.

### What goes into a Report

Reports are most useful when they cite the data behind their claims. The natural pattern is:

1. Build a few **Analyzes** (`/apps/analyzes/create`) over the Space's polls and discussions.
2. Generate the **Space Report** AI draft.
3. In the Editable view, paste links to the Analyzes detail pages and the per-poll / per-discussion drilldowns next to the claims they support.
4. Save.

Subscribers reading the Read-only Report can click those links and verify the claim against the raw records.

## Phase 4 — Revenue split *(Coming soon)*

When Reports become a paid product in Phase 4, every sale is split:

> **10% platform · 60% host · 30% contributors**

The contributor share is **weighted by relevance to the final Report** — participants whose poll responses, comments, follow choices, and quiz answers shaped the published narrative receive proportionally more. The contribution-record drilldown at `/spaces/:space_id/apps/analyzes/report/:report_id/records` is the data layer that drives that split.

The split engine and the buy-flow itself are *(Coming soon)* — Reports are publishable today, but they aren't priced or sold today. When the engine ships, your existing Reports become eligible without any extra work on your part.

## Tips

- **Do the Analyze before the narrative.** Generating the AI Report against an empty Space yields a blank draft. Run a few Analyzes first so the AI has something to summarize.
- **One Space, one Report — many Analyzes.** Don't try to fit every cross-cut into the longform Report; that's what Analyzes are for. The Report is the headline; Analyzes are the supporting receipts.
- **Iterate.** Reports aren't fire-and-forget. Re-generate as the Space matures, then merge the new draft with your existing prose.

## What's next

- [Space Apps → Analyzes](./apps#-analyzes) — the apps-level overview of where Analyzes live.
- [Host Actions](./host-actions) — the polls, discussions, quizzes, and follows whose results you'll be analyzing.
- [Space Dashboard](./dashboard) — live stats for the Space whose activity you're synthesizing.
