---
sidebar_position: 3
title: Sub-teams
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Sub-teams

Sub-teams are how a parent Team formally recognizes another Team as part of its umbrella — a chapter, a working group, a partner cell — without absorbing it. Each side keeps its own posts, members, treasury, and Spaces, but the relationship is an on-platform link with rules, documents, announcements, and a public registry of who belongs to whom.

This chapter walks through the full lifecycle for both sides: how a parent Team accepts applicants, how a Team applies and tracks its application, what the recognized relationship looks like day-to-day, and how either side ends it.

The participant-facing summary is in [Create a Team → Sub-teams](./create.md#sub-teams). This chapter is the deep dive on every URL.

## What sub-teams are

A **sub-team** is a Team whose `parent_team_id` points at another Team — the **parent**. Two important points the model preserves:

- **Each Team is still its own Team.** Members, posts, drafts, Spaces, rewards, treasury — none of those merge across the link. Deregistering or leaving simply removes the link; both Teams remain intact and independently operable.
- **The parent decides who they recognize.** Sub-team status is granted by the parent's admins through an application flow, not claimed unilaterally. A Team can apply to multiple parents over time, but at most one **recognized** parent at any moment.

The applicant Team is also called the **child** in this chapter, and the relationship is symmetric in surfaces — every page exists from both vantage points (parent looking out at children; child looking out at parent).

## Who can apply, who can approve

The actions that touch the link itself are restricted to the Team's **admins or owner** on each side. Specifically:

- The parent's admins / owner can set requirements, run the queue, approve / reject / return applications, deregister recognized children, and send direct messages to a specific child.
- The child's admins / owner can fill and submit the parent's application form, save it as a draft to come back to, agree to required documents, edit and resubmit if returned, cancel a pending application, and leave a recognized parent.
- Members on either side see the public surfaces (the bylaws page, the parent / sub-team listings, and — once their Team is _Recognized_ — the current-parent summary panel on their own arena) but cannot trigger lifecycle changes. The parent-summary panel only renders for members in the _Recognized_ state; the _Edit / Cancel / Leave_ action buttons inside it remain admin-only.

A Team can host its own Spaces and run normal activity regardless of whether it has, has applied for, or has rejected applying to a parent. Standalone is a perfectly valid steady state.

## Applying to be a sub-team

> 🎬 **Walkthrough:** filling in the parent's application form, submitting, and getting approved.

<video
  controls
  preload="metadata"
  width="100%"
  src={useBaseUrl('/media/subteam-register.mov')}
  style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}
>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/subteam-register.mov')}>Download the walkthrough</a>.
</video>

The application surface lives at a single child-side URL, plus a status tracker the child uses while waiting:

```
/<child-handle>/sub-teams/apply
/<child-handle>/sub-teams/application
```

### Apply page (`/sub-teams/apply`)

When a child opens this page, Ratel asks them to **select a parent Team** to apply to. Once a parent is chosen, the page renders the parent's published application form: a list of fields (short text, long text, number, date, single-select, multi-select, URL — defined by the parent in their requirements tab), each marked **required** or optional. Below the form is a list of the parent's **must-read documents**; the applicant clicks each to open it in a modal and confirm they've read and agreed.

A live validation panel shows what's still missing — required fields, document agreements — and the **Submit** button stays disabled until every requirement is satisfied. The parent's **minimum members** and **minimum team-age** thresholds are also enforced server-side at submit / resubmit / update time, so an applicant team that doesn't meet either gate cannot proceed even if the form itself is complete.

Submitting hands the application off to the parent's queue. The page transitions to the application-status tracker.

### Application status tracker (`/sub-teams/application`)

A single page the child uses to watch the application's progress. It shows:

- **Current relationship** — _Standalone team_, _Application pending_, or _Recognized sub-team_.
- **Latest application** — the parent it was sent to, when it was submitted, and the current status.
- **Application history** — every prior submission to any parent, with timestamps and outcome.
- **Decision reason** — when an application is returned for revision, rejected, or accepted with notes, the parent's text explanation surfaces here.

While an application is **Pending**, the child can **Cancel application** (withdraw it before review). When the parent **Returns it for revision**, the page exposes **Edit and resubmit** — the form re-opens pre-filled with what was sent, the child fixes the flagged fields, and resubmits.

If the parent **Rejects**, the child can re-apply later (typically after addressing the rejection reason). If the parent **Approves**, the relationship becomes **Recognized** and the child gets a new set of pages (see _Sub-team detail_, below) plus the option to **Leave parent**.

## Managing child sub-teams as a parent

> 🎬 **Walkthrough:** the parent admin builds the application form (eligibility thresholds + custom fields) that applicants will see.

<video
  controls
  preload="metadata"
  width="100%"
  src={useBaseUrl('/media/parent-setting.mov')}
  style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}
>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/parent-setting.mov')}>Download the walkthrough</a>.
</video>

Everything a parent does to operate the relationship lives at one URL, with five tabs along the top:

```
/<parent-handle>/sub-teams/manage
```

The five tabs:

| Tab                                | What it's for                                                                                                                                                                                                                                                                                                                                                                                                                                    |
| ---------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| **Eligibility & Application Form** | Two stacked panels in one tab. _Top panel:_ toggle whether your Team accepts sub-team applications and set two thresholds applicants must meet — **minimum member count** and **minimum team age (days since the applicant Team was created)**. _Bottom panel:_ define the fields the application form will show — label, type, required-or-not — reorder by drag. This is what `/sub-teams/apply` renders for applicants. Both panels autosave. |
| **Documents**                      | Upload the must-read documents — title + body — and mark each as required reading. Reorder by drag. These are the documents applicants must agree to. Per-document edit / delete also lives here.                                                                                                                                                                                                                                                |
| **Sub-teams**                      | The roster of currently recognized sub-teams. Each row links to the child's detail page and exposes a **Deregister** action (with reason text).                                                                                                                                                                                                                                                                                                  |
| **Pending Applications**           | The review queue: each pending application is a card with the applicant's filled-in form, document agreements, submission timestamp, and three action buttons — **Approve**, **Reject** (with reason), **Return for revision** (with revision comment).                                                                                                                                                                                          |
| **Broadcast**                      | The list of past announcements you've sent to your sub-teams (Draft / Published) plus a **Write announcement** button (see Announcements below).                                                                                                                                                                                                                                                                                                 |

Eligibility, the application form, and required documents are versioned implicitly — applications already in the queue stay attached to the form they were submitted against, so changing the form mid-flight doesn't re-open old submissions.

## Sub-team detail page

Every recognized sub-team has a stable detail URL, viewable by both sides:

```
/<parent-handle>/sub-teams/:sub_team_id
```

The page is a per-child overview combining identity, activity, member-level signal, and a private channel between the two Teams. It surfaces:

- **Identity** — child Team display name, handle, banner, bio, and a link out to the child's profile.
- **Activity window** — switchable between **Weekly** and **Monthly**: post count, Space count, and a roll-up of how active the child has been in that window. Member counts are deduplicated across the owner row and the member row so a 1-person Team is counted as 1, not 2.
- **Member activity** — paginated list of the child's members ordered by recent activity in the window, useful for spotting who's actually carrying the chapter day-to-day.
- **Direct message channel** — a private 1-to-1 announcement thread between this parent and this specific child. The parent's admins can compose a short message; the most recent one is delivered to the child's feed and prior direct messages from this parent to this child are demoted so only one is pinned at a time. Both sides see the history.
- **Deregister entry point** — for admins of the parent, a button that opens the deregister modal (see _Deregistering a sub-team_).

The page is readable by both sides — the parent's admins additionally get the deregister button and the direct-message composer.

## Sub-team docs

The parent maintains a shared library of **sub-team documents** — onboarding playbooks, codes of conduct, ops handbooks. The required ones are the must-read documents an applicant has to agree to; the rest are reference material recognized children can lean on after they're in. Authoring is shared with the public _Bylaws_ surface (see below) — both write to the same backing document, just with a different category preset.

Two URLs handle authoring:

```
/<parent-handle>/bylaws/compose/:category
/<parent-handle>/sub-teams/docs/:doc_id/edit
```

### Compose (`/bylaws/compose/:category`)

A clean rich-text editor — title at the top, body below, a **Required reading** toggle, and an autosave indicator. The `:category` segment determines whether the doc is treated as a generic sub-team document or as bylaws-specific (`Bylaws`, `ClubBylaws`); the editor itself is the same component. Submit and the document appears in the parent's _Documents_ tab and (if marked required) in the must-read list applicants see, and also on the public **Bylaws** reader.

The same composer is reached from the Documents tab on `/sub-teams/manage` (for sub-team docs) and from the Bylaws page (for bylaws-category docs).

### Edit (`/sub-teams/docs/:doc_id/edit`)

The same editor pre-filled with the existing document. Edits autosave. Editing a required document **does not** re-prompt already-recognized sub-teams — the agreement was against the version they applied with — but new applicants going through `/sub-teams/apply` see the latest version.

The Documents tab on `/sub-teams/manage` also exposes per-document delete and reorder.

## Sub-team announcements

> 🎬 **Walkthrough:** the parent posts an announcement, every recognized sub-team sees it on their wall, and a member of the child team comments on it.

<video
  controls
  preload="metadata"
  width="100%"
  src={useBaseUrl('/media/announcement.mov')}
  style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}
>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/announcement.mov')}>Download the walkthrough</a>.
</video>

Parents can broadcast to every recognized sub-team without needing each child to subscribe. The composer is a full-screen focused editor (it does NOT inherit the team-arena layout) so the author can write distraction-free:

```
/<parent-handle>/sub-teams/announcements/compose
/<parent-handle>/sub-teams/announcements/:announcement_id/edit
```

### Compose (`/announcements/compose`)

A focused editor with a title field, a rich-text body, and a right-hand panel covering:

- **Posting as** — the parent Team is locked as the author, shown in a non-editable identity row.
- **Space activation** — a toggle that, when on, creates a backing Space alongside the broadcast Post. The Space type is chosen later inside the Space designer; the toggle here only opts in.
- **Tags** — free-form chip input applied to the resulting Post.
- **Autosave** — every keystroke debounces into a Draft row server-side; an inline chip indicates _Idle / Dirty / Saving / Saved / Error_.
- **Publish** / **Discard draft** — the action buttons.

Publishing does two things, both behind a DynamoDB Streams → EventBridge → Lambda chain:

1. Writes an **anchor Post** on the parent's own feed pinned as an announcement (with the Space link attached if Space-activation was on).
2. **Fans out** a per-child copy of the Post onto every recognized sub-team's feed so the broadcast actually reaches them, and dispatches an inbox notification to the child's admins.

When the broadcast has Space-activation enabled, the backing Space is created in `Draft` publish-state. The fanned-out child-feed copies are **hidden from the children's wall** until the parent opens the Space designer and explicitly publishes the Space — this avoids children seeing a half-built Space card they can't open yet. The parent's own anchor copy is hidden from the parent's wall the same way until the Space is published; the broadcast Management tab still surfaces it so the parent admin can find and continue editing.

The composer can also be navigated to from the broadcast tab; pressing the publish button there awaits the round-trip to the server before navigating back, so the row never gets stuck in Drafts.

### Edit (`/announcements/:announcement_id/edit`)

Pre-fills with the existing announcement. While _Draft_, edits and re-publish are unrestricted; once _Published_, the announcement is preserved as a record — recipients still see whatever they received originally.

The Broadcast tab on `/sub-teams/manage` lists every announcement (Draft / Published) with timestamps and exposes **Publish** (for drafts) and **Delete**.

## Bylaws — the public reader

Both parents and applicants need a stable URL for "what does it take to be a sub-team here?":

```
/<team-handle>/bylaws
```

The bylaws page is the **shared reader** for a Team's required documents. It's visible to any signed-in user (truly public access — i.e. unsigned visitors — is _(Coming soon)_) and shows:

- The Team's identity at the top.
- Its required documents (the must-read list maintained on `/sub-teams/manage` → Documents).
- If this Team is itself a recognized sub-team, a second section showing **the parent's** required documents — useful for downstream applicants who want to understand the full chain.

Use the bylaws URL when sharing a Team's governance externally — it's the closest thing to a public charter Ratel surfaces today.

## Deregistering a sub-team

Either side can sever the link. The relationship is symmetric in outcome (the link goes away) but the actions are different surfaces because of who initiates and what they see.

### Parent-side: deregister (`/sub-teams/:sub_team_id/deregister`)

Opens from the **Sub-teams** tab on `/sub-teams/manage` or directly via URL. The page asks for a **reason** (required, free-text — it goes into the notification the child's admins receive). Submit, and:

- The `SubTeamLink` row between parent and child is removed.
- The child's `parent_team_id` is cleared.
- The child's admins receive an inbox notification with the parent's reason and a link back to their own apply page (in case they want to apply elsewhere).
- All of the child's posts, members, Spaces, rewards, and treasury **stay intact** — only the link is touched.

Deregistering is irreversible at the link level — to re-establish the relationship, the child has to apply again through `/sub-teams/apply`.

### Child-side: leave parent (`/parent/leave`)

The child's mirror image. Opens from the application status tracker once the child is **Recognized**, or directly via URL. The page asks for an optional reason, shows the current relationship at a glance, and on **Leave**:

- The same `SubTeamLink` row is removed.
- The child's `parent_team_id` is cleared.
- The parent's admins receive an inbox notification.
- Same content-preservation guarantee as above.

Leaving is also irreversible — the child returns to **Standalone** and can apply elsewhere.

## What's done and what's still rolling out

Every page above is **shipped today** — fields autosave, server endpoints exist for application submit / approve / reject / return / cancel, documents and announcements have full CRUD, deregister and leave-parent dispatch real notifications. The application form, application drafts, required documents, queue review, recognized roster, sub-team detail with weekly/monthly activity + member-deduplicated counts, direct-message channel, Space-enabled broadcasts with fanout, bylaws reader, and both termination flows are live.

Things that are still maturing:

- **Application form types**. Short text, long text, number, date, single-select, multi-select, URL are all wired. Custom validation rules per field are _(Coming soon)_.
- **Member-activity drilldowns** beyond weekly / monthly windows are _(Coming soon)_.
- **Public deep-link previews** for the bylaws page (Open Graph cards) are _(Coming soon)_.
- **Unsigned-visitor access to the bylaws page** is _(Coming soon)_ — the reader is currently visible to any signed-in user only.

The platform's intent is for sub-teams to be the substrate underneath DAO-like federations of Teams; expect more on the parent-side governance tools as the Phase 1 feature work continues.

## What's next

- [Create a Team](./create.md) — the foundational Team walkthrough with members, drafts, and DAO basics.
- [Team Settings](./team-settings.md) — admin tools for the Team's own settings, including the Subscription card.
