---
sidebar_position: 6
title: Reports (Host)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Reports

A **Report** is the host's published synthesis of what happened in their Space — what participants thought, where they agreed, where they didn't, what stood out. Ratel ships two report surfaces that work together:

| Surface                     | URL                                          | What it is                                                                                                                                                              |
| --------------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Space Reports** (longform) | `/spaces/:space_id/report`                  | A carousel of authored reports per Space. Each report is a rich-text document with `/data:` chart insertion, autosave, and a publish button.                            |
| **Analyzes** (cross-filter) | `/spaces/:space_id/apps/analyzes/...`        | Many per Space. Saved cross-filter analyses over polls, quizzes, follows, and discussions, with raw-records drilldown. The data you cite from your reports.             |

This chapter walks through both, in the order a host typically uses them: build a few Analyzes to see what's there, then write reports on top.

:::note Production availability
The Analyzes app is hidden in production while it goes through validation — it's available on dev / staging today. The Reports app (`/spaces/:space_id/report`) is available everywhere once installed via the space settings sidebar.
:::

## <img src={useBaseUrl('/img/icons/grid.svg')} width="22" height="22" alt="Analyzes" style={{verticalAlign: 'middle'}} /> Building an Analyze

The Analyzes app at `/spaces/:space_id/apps/analyzes` is a **horizontal carousel** of saved analyses. The first card is always **+ Create new analysis**; cards to its right are the analyses you've already built. Use the arrows to flip through and click a card to open it.

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

A **Download Excel** button on each analysis exports the matching records as a spreadsheet.

## <img src={useBaseUrl('/img/icons/file-text.svg')} width="22" height="22" alt="Reports" style={{verticalAlign: 'middle'}} /> Writing a Space Report

A short walkthrough of the full flow — installing the Reports app, drafting, publishing, and the member view:

<video controls width="100%" preload="metadata" style={{borderRadius: '8px', marginBottom: '16px'}}>
  <source src="https://metadata.ratel.foundation/mov/report.mov" type="video/mp4" />
  Your browser does not support embedded video. <a href="https://metadata.ratel.foundation/mov/report.mov">Download the demo</a> instead.
</video>

### Installing the Reports app

Reports are a separately-installable Space app. From the Space settings sidebar (right-side drawer), find **Reports** under **Available Apps** and click **+ Install** — the row moves into **Installed Apps**. Click **Settings** on the installed Reports row to land on the reports list page.

### The reports list page (`/spaces/:space_id/report`)

The list page is a **horizontal carousel** of report cards. The first card is always **+ New report**; cards to its right are the reports you've already authored. Above the carousel are three pieces of admin chrome:

- A **stats row** with `Total · Drafts · Published` counts.
- A **filter chip row** — `All` / `Drafts` / `Published` — that re-queries the server with a `?status=` parameter and re-renders the carousel.
- A top-right **`+ New Report`** button — creates a blank draft and routes you straight into the detail editor.

A kebab menu on each card lets the host delete a draft. The Drafts counter, the chips, the New Report button, and the kebab menu are all **admin-only**; non-admin members never see them.

### The report detail page (`/spaces/:space_id/report/:report_id`)

Each report has three pieces:

1. **Title** — an editable `<input>` seeded with `Untitled report`.
2. **Subtitle** — an optional editable `<input>` directly below the title.
3. **Body** — a rich-text editor with formatting toolbar (Paragraph / H1-H3 / bold / italic / underline / strike / code / lists / alignment / link / image / YouTube / table) and an **Insert Data** button that opens the chart picker.

The top bar shows:

- An **autosave status pill** ("Auto-saved · just now" / "Unsaved changes" / "Saving…").
- A **PDF Download** button that triggers the browser's print dialog (pick "Save as PDF").
- A **Publish** button (only visible while the report is still a Draft).

### Autosave

You don't click Save. Every keystroke flips the status pill to **Unsaved changes** and bumps an internal version counter; three seconds after the last edit, the autosave effect persists the latest `title / subtitle / body` to the server and the pill flips back to **Auto-saved · just now**. Subsequent edits to a Published report still autosave — they update the published copy in place (no separate "republish" step).

### Inserting charts inline

Two equivalent ways to drop an Analyze-driven chart into the body:

- **Insert Data button** (left of the format toolbar) — opens a picker drawer where you choose an Analyze, then a source (Poll / Quiz / Discussion / Follow), then the specific item.
- **`/data:` slash command** — type `/` in the body to open a 4-level autocomplete popup (`/data:` → analyze → source → item). Arrow keys + Enter pick, Esc closes.

Each inserted chart is a `<figure>` block with its own caption (`Figure N. <auto label>`) — editable inline. While editing as admin, hovering a chart figure surfaces gear + trash buttons; the gear opens a chart-type swap panel in the right rail (Bar / Pie / Table for poll-style data; LDA / TF-IDF / Network for discussions; an Open-ended Answers table for free-text questions).

### Publishing

When the draft is ready, click **Publish** in the top bar. A confirmation modal explains:

> Publishing makes the report's PDF visible to all space members. They will be able to download it from the space settings sidebar.

Click **Publish** in the modal to confirm. The PATCH that fires flips the report's `status` from Draft to Published in one shot — using whatever the latest autosaved body is at that moment, so there's no "publish before autosave settled" race.

After publish:

- The Publish button **disappears** from the top bar (publishing is a one-shot — there's no separate Republish action).
- The PDF Download button stays visible.
- Autosave keeps streaming subsequent edits into the published copy. Members reopening the report from their sidebar see the latest body the next time they navigate to it.

### The viewer (member) experience

Members never see the reports list page; the Drafts carousel is admin-only. Instead, every space member's settings sidebar surfaces a dedicated **Reports** section listing every Published report with a `View Report` button. Clicking it opens the detail page in **viewer mode**:

- No editing chrome (no toolbar, no Insert Data, no slash popup, no chart gear / trash).
- Title and subtitle render as static text.
- Body is read-only.
- PDF Download is the only action — it triggers the same `window.print()` flow the admin uses; the member picks "Save as PDF" in the system print dialog.

Unpublished drafts return a `Not found` error to members — they can't even discover that a draft exists.

## Phase 4 — Revenue split _(Coming soon)_

When Reports become a paid product in Phase 4, every sale is split:

> **10% platform · 60% host · 30% contributors**

The contributor share is **weighted by relevance to the final Report** — participants whose poll responses, comments, follow choices, and quiz answers shaped the published narrative receive proportionally more. The contribution-record drilldown at `/spaces/:space_id/apps/analyzes/report/:report_id/records` is the data layer that drives that split.

The split engine and the buy-flow itself are _(Coming soon)_ — Reports are publishable today, but they aren't priced or sold today. When the engine ships, your existing Reports become eligible without any extra work on your part.

## Tips

- **Do the Analyze before the narrative.** A blank Space gives the chart picker nothing to insert. Run a few Analyzes first so `/data:` has something to surface.
- **Publish is one-way per report; edits keep flowing.** Once you publish, the button disappears — but autosave keeps pushing your edits into the version members see. Treat publish as "open the door", not "freeze the snapshot".
- **One report, many sections.** Use H1/H2/H3 to structure the body. Use `/data:` charts and tables as evidence inline; don't try to make one chart carry the whole argument.
- **Iterate.** Reports aren't fire-and-forget. As the Space matures, edit the published report directly — members get the new version on their next visit.

## What's next

- [Space Apps → Analyzes](./apps.md#-analyzes) — the apps-level overview of where Analyzes live.
- [Host Actions](./host-actions.md) — the polls, discussions, quizzes, and follows whose results you'll be analyzing.
- [Space Dashboard](./dashboard.md) — live stats for the Space whose activity you're synthesizing.
