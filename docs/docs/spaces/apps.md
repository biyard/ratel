---
sidebar_position: 2
title: Space Apps (Host Tools)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Space Apps — Host Tools

If you're the host of a Space, **Apps** are the building blocks you plug in to shape it. Each App handles one job — publishing info, sharing files, asking participants to vote, generating an AI-assisted report, distributing rewards — and the App you don't need today, you simply leave out.

You'll find the apps panel at:

```
/spaces/:space_id/apps
```

From there, each App has its own dedicated URL — listed in the sections below — so you can deep-link into the exact configuration screen you need.

:::tip Designed to be additive
You don't have to enable everything. A simple Space might just use **General** and **Files**. A flagship Space with monetized reports will use **Analyzes** and **Incentive Pool** together.
:::

## <img src={useBaseUrl('/img/icons/settings.svg')} width="22" height="22" alt="Settings" style={{verticalAlign: 'middle'}} /> General

URL: `/spaces/:space_id/apps/general`

The General App is the home of your Space's written content. It's where you publish the welcome page participants see, lay out the rules of engagement, embed images and videos, and link out to anything that lives off-platform. Think of it as the rich-content backbone of the Space — the same surface participants will skim before deciding whether to participate.

Use General for static, evergreen material: mission statements, FAQs, agenda overviews, code-of-conduct, and partner credits. Anything time-bound (a single poll, a one-off meet) belongs in [Actions](/spaces/actions) instead, so participants can find it from the action carousel.

## <img src={useBaseUrl('/img/icons/file.svg')} width="22" height="22" alt="File" style={{verticalAlign: 'middle'}} /> Files

URL: `/spaces/:space_id/apps/files`

The Files App is the shared asset library for the Space. Upload PDFs, slide decks, CSVs, design exports, audio recordings — anything participants might need to reference while they engage. Files attached here are visible to everyone who has joined the Space, and the host controls what gets added or removed.

Files are particularly useful when you're hosting a Space around a document — a draft proposal, a research dataset, a specification — and want every participant to anchor their discussion or vote to the same source material.

## <img src={useBaseUrl('/img/icons/grid.svg')} width="22" height="22" alt="Grid" style={{verticalAlign: 'middle'}} /> Analyzes

:::note
Available on dev / staging today; production rollout pending validation.
:::

URLs:
- Browse analyses — `/spaces/:space_id/apps/analyzes`
- Create a new analysis — `/spaces/:space_id/apps/analyzes/create`
- View an analysis — `/spaces/:space_id/apps/analyzes/report/:report_id`
- Per-analysis raw records — `/spaces/:space_id/apps/analyzes/report/:report_id/records`
- Drill into a single poll's data — `/spaces/:space_id/apps/analyzes/poll/:poll_id`
- Drill into a single discussion's data — `/spaces/:space_id/apps/analyzes/discussion/:discussion_id`

The Analyzes App is Ratel's **cross-filter analysis engine** — *not* a longform AI narrative generator. (For the longform AI narrative side, see [Reports](./reports), which covers `/spaces/:space_id/report`.) Each analysis you save is a saved **cross-filter** over your Space's polls, quizzes, follows, and discussions, with a drilldown to the per-participant records that match.

### How a typical analysis flow runs

1. **Browse** at `/spaces/:space_id/apps/analyzes` — a horizontal carousel of saved analyses. The first card is always **+ Create new analysis**; cards to its right are the analyses you've already saved. Status badges read **Running** / **Analysis complete** / **Failed**.
2. **Create** at `/spaces/:space_id/apps/analyzes/create` — a two-step builder. Step 1 (*Pick cross filters*): every poll question, quiz question, follow target, and discussion thread becomes a tile; pick one or more, each becoming a chip. Comma-separated **keywords** are also supported (each keyword becomes its own filter). A live counter shows how many participants and records currently match. Step 2 (*Preview*): name the analysis, confirm the chips and a sample of the matching raw data, click **Generate report**.
3. **Read** at `/spaces/:space_id/apps/analyzes/report/:report_id` — the saved analysis renders the cross-filter results: distribution charts for poll responses, scoring for quizzes, follow counts, comment frequencies — all narrowed to the chip set. Chips are **frozen at save time**, so reopening a month later shows the exact slice you saved.
4. **Drill into raw records** at `/spaces/:space_id/apps/analyzes/report/:report_id/records` — paginated table of every individual record that matched (User · Question · Answer · Post · Comment · Followed). Click any chip to filter the records further. A **Download Excel** button at the top exports the matching records as a spreadsheet.
5. **Single-source drilldowns** — when you don't need a cross-filter, two deep links take you directly to per-source views: `/apps/analyzes/poll/:poll_id` for one poll, `/apps/analyzes/discussion/:discussion_id` for one discussion thread. These are the same charting surfaces used inside the saved analysis, just scoped to one source.

### Contribution records and the Phase 4 revenue split

The `:report_id/records` page is also the **contribution records** surface: per-participant breakdowns of how each respondent's activity matched the analysis you saved. This is what feeds the Phase 4 revenue split — when you publish a paid Report ([Reports](./reports)), sales are split **10% platform · 60% host · 30% contributors**, with the contributor share weighted by relevance to the final Report.

:::note Coming soon
The full Phase 4 revenue split engine (with on-chain settlement opt-in) is rolling out alongside the broader agent economy. Analyses themselves are usable today on dev / staging; the monetization layer appears in your host dashboard as it ships.
:::

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Users" style={{verticalAlign: 'middle'}} /> Panels

URL: `/spaces/:space_id/apps/panels`

The Panels App helps you reach the right people. Build a target audience for the Space — by interest, by reputation tier, by past participation, or by membership in another Space — and send a tailored invitation. Panels are also how you scope an action to a subset of participants when you don't want to surface it to *everyone*.

This is the App to reach for when your Space's value depends on *who* shows up: expert reviews, jury-style discussions, member-gated polls, or partner-specific outreach.

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M8 19c.53 0 1.04-.21 1.41-.59.38-.37.59-.88.59-1.41V9c0-1.06-.42-2.08-1.17-2.83C8.08 5.42 7.06 5 6 5M6 5c-1.06 0-2.08.42-2.83 1.17C2.42 6.92 2 7.94 2 9v8c0 .53.21 1.04.59 1.41.37.38.88.59 1.41.59h16c.53 0 1.04-.21 1.41-.59.38-.37.59-.88.59-1.41V9c0-1.06-.42-2.08-1.17-2.83C19.92 5.42 18.94 5 18 5H6zM2 11h20M16 11v3"/></svg> Incentive Pool (Beta)

URL: `/spaces/:space_id/apps/incentive-pool`

Available on beta builds; rolling out to production builds soon.

The Incentive Pool App is where you fund and configure the rewards participants earn from completing Actions. Set the pool size, the per-action allocation, and the rules for distribution — flat split, weighted by participation depth, or staged across phases of the Space.

When a participant completes an action that pays out, their share is drawn from this pool automatically. The pool size, current balance, and distribution history are all visible from the same screen, so you can audit your reward economy at a glance.

:::tip Pair it with Analyzes
A Space that publishes paid reports often also runs an Incentive Pool — the pool drives participation while the report split rewards the contributors whose input made the report worth selling.
:::

:::note A note on host-side rewards
Rewards distribution is tracked from your User Rewards page (`/your-handle/rewards`) — there's no host-side Rewards app today. You'll see host share from published reports, platform-distributed bonuses, and reward credits routed to your account, aggregated across every Space you host.
:::

## Putting it together

A typical reward-bearing Space uses these Apps in concert:

1. **General** publishes the framing and the rules.
2. The host adds **Actions** (polls, discussions, quizzes, follows, meets — see [Space Actions](/spaces/actions)).
3. The **Incentive Pool** funds the rewards, and **Analyzes** turns the activity into a report.

Every App is optional, but together they form the production line that turns a community of participants into a publishable, monetizable artifact — without losing the link back to the people who made it.
