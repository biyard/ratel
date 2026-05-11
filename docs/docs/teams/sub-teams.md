---
sidebar_position: 3
title: Sub-teams
---

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

- The parent's admins / owner can set requirements, run the queue, approve / reject / return applications, and deregister recognized children.
- The child's admins / owner can fill and submit the parent's application form, agree to required documents, edit and resubmit if returned, cancel a pending application, and leave a recognized parent.
- Members on either side see the public surfaces (the bylaws page, the parent / sub-team listings) but cannot trigger lifecycle changes.

A Team can host its own Spaces and run normal activity regardless of whether it has, has applied for, or has rejected applying to a parent. Standalone is a perfectly valid steady state.

## Applying to be a sub-team

The application surface lives at a single child-side URL, plus a status tracker the child uses while waiting:

```
/<child-handle>/sub-teams/apply
/<child-handle>/sub-teams/application
```

### Apply page (`/sub-teams/apply`)

When a child opens this page, Ratel asks them to **select a parent Team** to apply to. Once a parent is chosen, the page renders the parent's published application form: a list of fields (short text, long text, number, date, single-select, multi-select, URL — defined by the parent in their requirements tab), each marked **required** or optional. Below the form is a list of the parent's **must-read documents**; the applicant clicks each to open it in a modal and confirm they've read and agreed.

A live validation panel shows what's still missing — required fields, document agreements — and the **Submit** button stays disabled until every requirement is satisfied.

Submitting hands the application off to the parent's queue. The page transitions to the application-status tracker.

### Application status tracker (`/sub-teams/application`)

A single page the child uses to watch the application's progress. It shows:

- **Current relationship** — *Standalone team*, *Application pending*, or *Recognized sub-team*.
- **Latest application** — the parent it was sent to, when it was submitted, and the current status.
- **Application history** — every prior submission to any parent, with timestamps and outcome.
- **Decision reason** — when an application is returned for revision, rejected, or accepted with notes, the parent's text explanation surfaces here.

While an application is **Pending**, the child can **Cancel application** (withdraw it before review). When the parent **Returns it for revision**, the page exposes **Edit and resubmit** — the form re-opens pre-filled with what was sent, the child fixes the flagged fields, and resubmits.

If the parent **Rejects**, the child can re-apply later (typically after addressing the rejection reason). If the parent **Approves**, the relationship becomes **Recognized** and the child gets a new set of pages (see *Sub-team detail*, below) plus the option to **Leave parent**.

## Managing child sub-teams as a parent

Everything a parent does to operate the relationship lives at one URL, with five tabs along the top:

```
/<parent-handle>/sub-teams/manage
```

The five tabs:

| Tab | What it's for |
|---|---|
| **Eligibility & Application Form** | Two stacked panels in one tab. *Top panel:* toggle whether your Team accepts sub-team applications and set a minimum-members threshold applicants must meet. *Bottom panel:* define the fields the application form will show — label, type, required-or-not — reorder by drag. This is what `/sub-teams/apply` renders for applicants. Both panels autosave. |
| **Documents** | Upload the must-read documents — title + body — and mark each as required reading. Reorder by drag. These are the documents applicants must agree to. |
| **Sub-teams** | The roster of currently recognized sub-teams. Each row links to the child's detail page and exposes a **Deregister** action (with reason text). |
| **Pending Applications** | The review queue: each pending application is a card with the applicant's filled-in form, document agreements, submission timestamp, and three action buttons — **Approve**, **Reject** (with reason), **Return for revision** (with revision comment). |
| **Broadcast** | The list of past announcements you've sent to your sub-teams, plus a **Write announcement** button (see Announcements below). |

Eligibility, the application form, and required documents are versioned implicitly — applications already in the queue stay attached to the form they were submitted against, so changing the form mid-flight doesn't re-open old submissions.

## Sub-team detail page

Every recognized sub-team has a stable detail URL, viewable by both sides:

```
/<parent-handle>/sub-teams/:sub_team_id
```

The page is a per-child overview combining identity, activity, and member-level signal. It surfaces:

- **Identity** — child Team display name, handle, banner, bio, and a link out to the child's profile.
- **Activity window** — switchable between **Weekly** and **Monthly**: post count, Space count, and a roll-up of how active the child has been in that window.
- **Member activity** — paginated list of the child's members ordered by recent activity in the window, useful for spotting who's actually carrying the chapter day-to-day.
- **Deregister entry point** — for admins of the parent, a button that opens the deregister modal (see *Deregistering a sub-team*).

The page is read-only for everyone except the parent's admins (who get the deregister button).

## Sub-team docs

The parent maintains a shared library of **sub-team documents** — onboarding playbooks, codes of conduct, ops handbooks. The required ones are the must-read documents an applicant has to agree to; the rest are reference material recognized children can lean on after they're in.

Two URLs handle authoring:

```
/<parent-handle>/sub-teams/docs/compose
/<parent-handle>/sub-teams/docs/:doc_id/edit
```

### Compose (`/sub-teams/docs/compose`)

A clean rich-text editor — title at the top, body below, a **Required reading** toggle, and an autosave indicator. Submit and the document appears in the parent's *Documents* tab and (if marked required) in the must-read list applicants see.

### Edit (`/sub-teams/docs/:doc_id/edit`)

The same editor pre-filled with the existing document. Edits autosave. Editing a required document **does not** re-prompt already-recognized sub-teams — the agreement was against the version they applied with — but new applicants going through `/sub-teams/apply` see the latest version.

The Documents tab on `/sub-teams/manage` also exposes per-document delete and reorder.

## Sub-team announcements

Parents can broadcast to the admins of every recognized sub-team without needing each child to subscribe:

```
/<parent-handle>/sub-teams/announcements/compose
/<parent-handle>/sub-teams/announcements/:announcement_id/edit
```

### Compose (`/announcements/compose`)

A title + rich-text body editor, with **Save as draft** or **Publish**. Publishing dispatches an inbox notification to the admins of every recognized sub-team and stamps the announcement as *Published*.

### Edit (`/announcements/:announcement_id/edit`)

Pre-fills with the existing announcement. While *Draft*, edits and re-publish are unrestricted; once *Published*, the announcement is preserved as a record — recipients still see whatever they received originally.

The Broadcast tab on `/sub-teams/manage` lists every announcement (Draft / Published) with timestamps and exposes **Delete**.

## Bylaws — the public reader

Both parents and applicants need a stable URL for "what does it take to be a sub-team here?":

```
/<team-handle>/bylaws
```

The bylaws page is the **shared reader** for a Team's required documents. It's visible to any signed-in user (truly public access — i.e. unsigned visitors — is *(Coming soon)*) and shows:

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

Every page above is **shipped today** — fields autosave, server endpoints exist for application submit / approve / reject / return / cancel, documents and announcements have full CRUD, deregister and leave-parent dispatch real notifications. The application form, required documents, queue review, recognized roster, sub-team detail with weekly/monthly activity, bylaws reader, and both termination flows are live.

Things that are still maturing:

- **Application form types**. Short text, long text, number, date, single-select, multi-select, URL are all wired. Custom validation rules per field are *(Coming soon)*.
- **Member-activity drilldowns** beyond weekly / monthly windows are *(Coming soon)*.
- **Public deep-link previews** for the bylaws page (Open Graph cards) are *(Coming soon)*.

The platform's intent is for sub-teams to be the substrate underneath DAO-like federations of Teams; expect more on the parent-side governance tools as the Phase 1 feature work continues.

## What's next

- [Create a Team](./create.md) — the foundational Team walkthrough with members, drafts, and DAO basics.
- [Team Settings](./team-settings.md) — admin tools for the Team's own settings, including the Subscription card.
