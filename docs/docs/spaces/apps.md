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

The General App is the **operational settings** screen for the Space. Everything host-controlled that isn't an Action lives here — branding, who can join, who can administer, and (at the bottom) the destructive "delete this space" path. Every change auto-saves; the sticky footer flips to a green **Synced** pill when the save lands, so there's no manual save button anywhere on the page.

The page is a single scrollable arena with these sections, in order:

1. **Space logo** — upload a square image (anything 1:1) to replace the Ratel symbol mark in the topbar tile and on every list/card surface that links to the Space. If you haven't uploaded one, a 2-letter initials placeholder rendered from the Space title is used.
2. **Start time** — datetime picker for when the Space "officially" begins. Defaults to the Space's creation timestamp; setting an explicit start time is what flips the Space's badge from _Draft_ to _Scheduled / Live_ on discovery surfaces.
3. **Visibility** — pick **Public** (anyone with the link can land on the splash) or **Private** (only invited participants and signed-in admins). Switches are radio cards; the active card is `aria-selected`.
4. **Invite participant** — comma-separate email addresses (or hit Enter between each), preview them as chips, then send. The list below shows every outstanding invitation with its status (`Pending` / `Accepted` / etc.) and lets you revoke an invitation with one click. Pagination loads more invitations as you scroll.
5. **Anonymous participation** — toggle. When on, participants' votes and comments are recorded against an anonymous handle in this Space (their identity stays in the Essence pipeline, but isn't shown to other participants).
6. **Join anytime** — toggle. When on, anyone can join after the Space is live. When off, participation is gated to the panel of invited / pre-approved users.
7. **Administrators** — add other Ratel users by username (comma-separated, Enter-to-queue, like the invite flow). Each admin appears as a chip with a remove button. Only existing admins see the "Add" field; non-admin viewers see the read-only list.
8. **Danger zone** — a single red **Delete space** button. Opens an inline arena modal that requires confirmation; on confirm, the Space, all its actions, comments, and pool funds are permanently removed.

:::tip Where rich written content goes
General is **not** where you write a long welcome page or pin a manifesto — that's the Overview panel (the topbar file-text icon on the Space arena). General is for switches and lists; Overview is for narrative.
:::

<video controls preload="metadata" width="100%" src="https://metadata.ratel.foundation/mov/general.mov" style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
Your browser doesn't support embedded video. <a href="https://metadata.ratel.foundation/mov/general.mov">Download the walkthrough</a>.
</video>

## <img src={useBaseUrl('/img/icons/file.svg')} width="22" height="22" alt="File" style={{verticalAlign: 'middle'}} /> Files

URL: `/spaces/:space_id/apps/files`

The Files App is a **read-only aggregator** of every file already attached elsewhere in the Space (the Overview narrative, Discussion boards, Quiz questions). It is _not_ an upload screen — there's no "Add file" button on this page; files reach the library by being attached at their source surface and then surface here automatically.

The arena layout, top to bottom:

1. **Tab filter** — segmented control with **All · Overview · Boards · Quiz**. Each tab filters the list to files that came from that source. The active tab is `aria-selected`. A right-aligned counter (`N files`) reflects the displayed count for the current tab; the sticky footer separately shows the all-tab total.
2. **File list** — one card per file, showing:
   - a type-colored extension badge (`JPG`, `PNG`, `PDF`, `ZIP`, `DOC`, `PPT`, `XLS`, `MP4`, `MOV`, `MKV` — these ten extensions are the entire supported set)
   - the file name and size
   - a source-tag chip when applicable (**Overview** / **Board** / **Quiz**) — tells you where the file is attached so you can navigate back to its context
   - clicking the card opens the file in a new tab (`target="_blank"`); when no URL is available the card still renders but isn't clickable
3. **Image previews** — auto-rendered grid section, only appears when the current tab has at least one JPG/PNG. Each thumbnail carries the extension label.
4. **Video previews** — auto-rendered single-column section (16:7 aspect ratio, native HTML5 controls), only appears when the current tab has at least one MP4/MOV/MKV.
5. **Empty state** — shown when the active tab matches no files. A folder icon, "No files yet" headline, and a hint that files shared in the Space appear here.

:::tip Where to actually upload
To add a file you intend participants to reference, attach it on the Overview panel (for a manifesto/spec accompanying the Space), inside a Discussion board, or on a Quiz question. The Files app then aggregates them into one browseable view.
:::

<video controls preload="metadata" width="100%" src="https://metadata.ratel.foundation/mov/file.mov" style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
Your browser doesn't support embedded video. <a href="https://metadata.ratel.foundation/mov/file.mov">Download the walkthrough</a>.
</video>

## <img src={useBaseUrl('/img/icons/grid.svg')} width="22" height="22" alt="Grid" style={{verticalAlign: 'middle'}} /> Analyzes

:::note
Available on dev / staging today; production rollout pending validation.
:::

URLs:

- Browse analyses — `/spaces/:space_id/apps/analyzes`
- Create a new analysis — `/spaces/:space_id/apps/analyzes/create`
- View an analysis — `/spaces/:space_id/apps/analyzes/report/:report_id`
- Per-analysis raw records — `/spaces/:space_id/apps/analyzes/report/:report_id/records`

The Analyzes App is Ratel's **cross-filter analysis engine** — _not_ a longform AI narrative generator. (For the longform AI narrative side, see [Reports](./reports), which covers `/spaces/:space_id/report`.) Each analysis you save is a saved **cross-filter** over your Space's polls, quizzes, follows, and discussions, with a drilldown to the per-participant records that match.

### How a typical analysis flow runs

1. **Browse** at `/spaces/:space_id/apps/analyzes` — a horizontal carousel of saved analyses. The first card is always **+ Create new analysis**; cards to its right are the analyses you've already saved. Status badges read **Running** / **Analysis complete** / **Failed**.
2. **Create** at `/spaces/:space_id/apps/analyzes/create` — a two-step builder. Step 1 (_Pick cross filters_): every poll question, quiz question, follow target, and discussion thread becomes a tile; pick one or more, each becoming a chip. Comma-separated **keywords** are also supported (each keyword becomes its own filter). A live counter shows how many participants and records currently match. Step 2 (_Preview_): name the analysis, confirm the chips and a sample of the matching raw data, click **Generate report**.
3. **Read** at `/spaces/:space_id/apps/analyzes/report/:report_id` — the saved analysis renders the cross-filter results across four panels (poll, quiz, follow, discussion), all narrowed to the chip set. Chips are **frozen at save time**, so reopening a month later shows the exact slice you saved.
4. **Drill into raw records** at `/spaces/:space_id/apps/analyzes/report/:report_id/records` — paginated table of every individual record that matched (User · Question · Answer · Post · Comment · Followed). Click any chip in the sidebar to scope the records further.

All per-source data (poll distributions, quiz pass rates, follow targets, discussion topics) lives **inside the saved-analysis report's four panels** — there are no standalone single-source drilldown pages today.

:::note Not yet shipped

- A spreadsheet (Excel / CSV) export from the records page.
- Standalone single-source drilldown pages (per-poll, per-quiz, per-follow, per-discussion). All per-source views are inside the saved-analysis report.
  :::

### Contribution records — the data layer behind future monetization

The `:report_id/records` page is also the **contribution records** surface: per-participant breakdowns of how each respondent's activity matched the analysis you saved. It is the data layer that will feed the Phase 4 revenue split when paid Reports ship — see [Reports → Phase 4 revenue split](./reports#phase-4--revenue-split-coming-soon) for the planned **10% platform · 60% host · 30% contributors** allocation.

:::note Coming soon
Paid Reports + the on-chain revenue split engine are not yet live. Analyses themselves and the contribution-records page are usable today on dev / staging; the monetization layer appears in your host dashboard as it ships.
:::

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Users" style={{verticalAlign: 'middle'}} /> Panels

URL: `/spaces/:space_id/apps/panels`

The Panels App is the Space's **demographic quota planner** — the screen survey researchers use to declare "I want N respondents, split this way across age, gender, and university." It's not a contact-list / invitation tool; it shapes the _composition_ of the panel that paid Actions and the Incentive Pool will pay out to. Like General, every change auto-saves and a sticky footer flips between **Saving…** and **Saved** as the writes land.

The arena is **Creator-only**. Admins, viewers, and participants who land on `/apps/panels` see a "no access" placeholder with a Back button — only the Space's creator can edit the panel.

Sections, top to bottom:

1. **Total quotas** — a single integer input for the total respondent count, paired with an _allocated / unassigned_ meter on the right. The meter sums the quota numbers in the conditional table below; the unassigned bar is what's left over to spread.
2. **Attribute groups** — three toggle cards for **University**, **Age**, and **Gender**. Flipping a card on enables that attribute; flipping it off rebuilds the panel rows server-side. Active cards render as `aria-selected="true"`.
3. **Collective panel** _(only renders when ≥1 attribute is in collective mode)_ — chips of the currently-collective attributes. The `+` button on the section header opens a dropdown to **promote** an attribute (Age or Gender, when total quota > 0) into the conditional table for finer-grained allocation.
4. **Conditional table** _(only renders when ≥1 attribute is in conditional mode)_ — a per-cell quota table. Rows are axis-tagged: pure-Age (`--age` swatch), pure-Gender (`--gen`), pure-University (`--uni`), or composite (`--mix`) for Age × Gender. Each row carries its own integer quota; the sum feeds the _allocated_ count in section 1.

In other words: section 2 picks **which** demographic axes matter for this Space's panel, section 3 keeps the simple "everyone counts equally for this attribute" form, and section 4 is what you reach for when you need "30 women aged 18-24, 50 men aged 25-34, …" precision. The Incentive Pool / Reward apps then pay out against the realized panel.

:::note Not yet wired
There is no "send invitation" or "import from another Space" flow in the Panels app today — interest tags, reputation tiers, cross-Space membership filters, and per-action audience scoping are _not_ implemented. The current build only models demographic quotas (university / age / gender).
:::

<video controls preload="metadata" width="100%" src="https://metadata.ratel.foundation/mov/panel.mov" style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
Your browser doesn't support embedded video. <a href="https://metadata.ratel.foundation/mov/panel.mov">Download the walkthrough</a>.
</video>

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
