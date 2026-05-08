---
sidebar_position: 5
title: Host Actions (Editors)
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Host Actions

This chapter is the **host-side** companion to [Space Actions](./actions.md). The participant chapter explains what an Action looks like to someone who shows up to engage; this one walks you, the host, through creating and editing each Action type.

## Creating an Action

Open your Space's actions page at `/spaces/:space_id/actions` and click the **+ Create** affordance on the action carousel — the modal that opens is the entry point for every Action you'll author.

The modal asks one question first: **what kind of Action?** You pick from four tiles:

| Tile | What it makes |
|---|---|
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> **Poll** | A quick vote — single choice, multi-select, subjective text, or a linear scale. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg> **Quiz** | Scored questions with a passing threshold. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> **Discussion** | A host prompt with rich-text replies. |
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" alt="Follow" style={{verticalAlign: 'middle'}} /> **Follow** | A campaign that asks participants to follow a curated set of accounts. |

Pick a tile, see a live **Preview** of the empty card on the right, then click **Create**. Ratel takes you straight to the new Action's editor URL where you fill in the rest.

> **About Meets.** The fifth Action type — **Meet** (a scheduled event with RSVP) — exists in the platform and has its own viewer at `/spaces/:space_id/actions/meets/:meet_id`, but a host editor entry point is *(Coming soon)*. Today Meets are typically created via the [MCP API](../essence/my-essence#-my-ai--my-ai) (`create_meet` tool) and then surface in the participant carousel like any other Action.

## How an editor URL works

Every Action editor lives under a stable URL pattern:

| Type | URL |
|---|---|
| Discussion | `/spaces/:space_id/actions/discussions/:discussion_id/edit` |
| Poll | `/spaces/:space_id/actions/polls/:poll_id` (admin view) |
| Quiz | `/spaces/:space_id/actions/quizzes/:quiz_id` (admin view) |
| Follow | `/spaces/:space_id/actions/follows/:follow_id` (admin view) |
| Meet | `/spaces/:space_id/actions/meets/:meet_id` (admin view) |

**Discussions have a dedicated `/edit` URL; Poll / Quiz / Follow / Meet share their URL between admin and participant views** — Ratel detects whether you're an admin and flips the page into editor mode for those four. Participants visiting the URL see the participation card; you see the editor.

Edits **autosave** with a debounce; the **Save** button in the footer flushes pending saves and fires a *Saved* toast. There is no separate "Publish" button — saving and publishing are the same gesture.

## Common Configuration

Every editor is split into a **Content** card on top and a **Configuration** card below. The Configuration card has the same set of sections across all four Action types:

| Section | What you set |
|---|---|
| **Schedule** | *Starts at* and *Ends at* date-times. Outside this window the Action isn't active. |
| **Participation & Rewards** | The reward in **Credits (CR)** drawn from the Space's [Incentive Pool](./apps.md#-incentive-pool-beta). Credits convert to participant points on completion. |
| **Dependency Actions** | Other Actions in the Space a participant must complete first to unlock this one. |
| **Status** | Toggle the Action between draft / live / closed. |
| **Danger zone** | Delete the Action. Submissions/responses go with it; rewards already paid are not refunded. |

Discussions add a **Moderation** section (assigning who can hide replies); Polls add a **Voting rules** section (see below).

### Who can edit rewards and credits

Configuration is **Creator-only**. The role check happens server-side on every save:

| Role | What they can do |
|---|---|
| **Creator** | Full editor access — content, schedule, rewards, dependencies, status, deletion. The Creator is whoever created the Space (the post author who promoted their post into a Space). |
| **Member** *(team Spaces)* | Can author and edit *their own* Actions inside a team Space, but cannot change reward amounts or pull from the Incentive Pool — those stay locked to the Creator. |
| **Participant** | Read-only on Action pages. They never see the Configuration card. |
| **Viewer** | Read-only on the Space splash + Overview. Cannot open Action editors. |

The **Credits (CR)** field is the most common place where this matters: changing reward amounts moves money out of the Space's Incentive Pool, so only the Creator (or an explicitly-elevated team admin) can touch it. If you don't see the Configuration card or the Credits field is greyed out, you're not in a role that can edit rewards.

### Anonymous participation

The Space has a **General → Anonymous participation** toggle (under Settings → Apps → General → Settings) that flips two things at once:

- **Identity binding.** When *off* (default), every vote, comment, and submission is attached to the participant's handle. When *on*, the participant is shown an anonymous handle on the action carousel, and their submissions are recorded against that anonymous identity. The Creator can still see aggregate counts but not individual mappings.
- **Consent copy.** The ConsentModal that participants see on first join switches to the anonymous variant — it makes clear that responses are not personally attributable, but that aggregate analysis (Panels, Analyzes) is still on.

:::tip When to enable
Turn anonymous participation on for sensitive surveys (workplace satisfaction, health screenings, internal critique) where attribution would chill honest answers. Leave it off for community polls where social signal — *who voted for what* — is part of the value.
:::

Anonymous mode does **not** turn off the [Panels](./apps.md#-panels-beta) demographic capture — anonymity is per-submission, but if the host has enabled the Panels app, participants are still asked for their age band, gender, region, etc. so the Creator can run aggregate slicing. Both can coexist because Panel attributes are stored separately from submission identity.

## Publishing the Space

Editing Actions doesn't automatically publish the Space. A draft Space is only visible to its Creator and team admins until it's *published* — at which point the **Index page**, **Overview**, **Dashboard**, and **action carousel** become reachable by participants (and, if set to Public, by anonymous visitors).

### The Publish button

Open the Space's Index page (`/spaces/:space_id/`). The arena topbar surfaces a **Publish** button with the paper-plane icon. Clicking it opens the **Space visibility modal**:

| Option | Who can see the Space |
|---|---|
| **Public** *(공개)* | Anyone with the URL — including logged-out visitors. Hot-Spaces ranking, search, and shared links all surface it. Required for Spaces you want listed in the public discovery feeds. |
| **Private** *(비공개)* | Only invited members and team admins. Visitors without an invitation see a "not found" page; the URL works as an unlisted link only for people you've added through the General app's *Invite Participant* flow. |

Pick a visibility, hit **Publish**, and the modal flips the Space from *Draft* to the corresponding state. The topbar's Publish button is replaced by a **Start** button (which transitions the Space from *Open* → *Ongoing* once you're ready for active participation), and an *In progress* status chip appears next to the title.

### Changing visibility after publish

The visibility flag is editable post-publish — re-open the modal from the topbar's **Settings → Status** entry and pick a different option. Switching from Private → Public retroactively makes the Space discoverable; switching Public → Private hides it from new visitors immediately but doesn't kick out anyone who already joined.

:::warning Publishing is the trigger for invitation emails
If you've used the General app's *Invite Participant* flow to add emails before publishing, those invitations are dispatched at publish time — not when you add the email. So make sure your Content / Apps / Actions are ready before flipping the switch; published-then-edited Spaces show a stale picture in the invitation emails participants receive.
:::

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> Discussion editor

URL: `/spaces/:space_id/actions/discussions/:discussion_id/edit`

The Content card holds:

- **Title** — what people see at the top of the discussion card.
- **Description / body** — Markdown-supported rich text. This is the prompt participants reply to.
- **Attachments** — drag-and-drop PDF / PNG / JPG up to 25 MB each.

The Configuration card adds **Moderation** alongside the common sections. Use Moderation to set who can hide off-topic replies once the discussion is live.

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> Poll editor

URL: `/spaces/:space_id/actions/polls/:poll_id`

The Content card holds:

- **Title** — the headline of the poll.
- **Questions** — between **1 and 20 questions**. Each question chooses one of four types:
  - **Single** — one option per question (radio).
  - **Multi** — multiple options selectable (checkbox).
  - **Subjective** — free-text response with an optional inline hint shown above the input.
  - **Linear** — a numeric scale with min / max bounds (1–5, 0–10, etc.).
- **Allow "Other" option** per question — adds a free-text "Other" choice next to the fixed options.

The Configuration card adds **Voting rules** alongside the common sections:

- **Allow response editing** — when on, participants can update their answers while the poll is open. Off makes responses final on submit.
- **Encrypted upload** — when on, vote results are encrypted and stored on-chain; responses cannot be edited after submission. Use this for high-stakes votes where auditability matters.

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/></svg> Quiz editor

URL: `/spaces/:space_id/actions/quizzes/:quiz_id`

The Content card holds:

- **Title** and **Description** — the framing shown before participants start.
- **Questions** — between **3 and 20 questions**, each Single or Multi choice with a marked correct answer.
- **Pass Score** — minimum score (out of total questions) to clear the quiz.
- **Retry Count** — how many attempts each participant gets.
- **Survey Time** — total time allowed per attempt.
- **Attachments** — reference materials participants can consult: PDF / PNG / JPG up to 25 MB each.

Pass and the reward unlocks; fail and (if Retry Count allows) the participant can try again.

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Follow" style={{verticalAlign: 'middle'}} /> Follow editor

URL: `/spaces/:space_id/actions/follows/:follow_id`

The Targets card holds:

- **Title** — what the campaign is called.
- **Targets** — between **1 and 20 user accounts** the participant should follow. Each target is rendered as an inline row in the participant view with a *Follow* button right there.

The Configuration card has the common sections only — Schedule, Participation & Rewards, Dependencies, Status, Danger zone.

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg> Meet editor *(Coming soon)*

URL: `/spaces/:space_id/actions/meets/:meet_id` (admin view)

A scheduled event — livestream, video call, workshop, in-person gathering. The participant view at this URL is live, but a host editor entry point in the create-modal is *(Coming soon)*. Today, hosts who need a Meet can create one via the MCP API (`create_meet` tool — see [Connect via MCP](../essence/my-essence#-my-ai--my-ai)).

## Tips

- **Save your reward budget for the right Actions.** Credits come out of the Space's Incentive Pool. A Quiz for onboarding can be reward-free; the deep Discussion that anchors your final Report is where Credits earn their keep.
- **Use Dependencies to build a sequence.** A common pattern: a quick *Sample* Poll → a deep *Final* Poll. Make the Final depend on the Sample so participants warm up before the headline question.
- **Test with the participant view.** Open the same URL in a private window or a different browser session — that's exactly what your participants see. The admin / participant flip is automatic.

## What's next

- [Space Actions](./actions.md) — participant-side reference for the same five Action types.
- [Space Apps → Incentive Pool](./apps.md#-incentive-pool-beta) — fund the Credit pool your Action rewards draw from.
- [Reports](./reports.md) — turn the Action results into a published narrative.
