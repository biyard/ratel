---
sidebar_position: 2
title: Create a Team
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Create a Team

This page walks you through creating a team and getting it set up — the popup, the arena tabs you'll see once you land, the member roles, the publishing surfaces (posts / drafts / rewards / memberships), team subscription, the built-in DAO, and a brief index of sub-team flows. For the broader context — what teams are and where the chapter goes next — see the [Teams overview](./).

## Create a team

> 🎬 **Walkthrough:** create a team end-to-end (~30 seconds).

<video
  controls
  preload="metadata"
  width="100%"
  src={useBaseUrl('/media/create-a-team.webm')}
  style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}
>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/create-a-team.webm')}>Download the walkthrough</a>.
</video>

You can start a new team from the <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2"/><circle cx="8.5" cy="7" r="4"/><line x1="20" y1="8" x2="20" y2="14"/><line x1="23" y1="11" x2="17" y2="11"/></svg> **Create Team** action in your profile dropdown (open it from the avatar at the bottom of the sidebar). A short popup walks you through the basics:

- **Team handle** — this becomes your team's URL: `/your-handle`. Pick something short and stable; the handle is hard to change later.
- **Display name** — the friendly name shown across the platform.
- **Banner and bio** — give your team an identity people can recognize at a glance.
- **Initial members** — invite a few teammates by username or email; they receive an invitation they can accept or decline.

Once created, you land on the team's home at `/your-handle/home`.

## Team arena — your team's home

Every team gets its own **arena layout**, separate from individual user pages. The HUD tabs along the top of the arena are:

| Tab | URL | Icon |
|---|---|---|
| **Home** | `/your-handle/home` | <img src={useBaseUrl('/img/icons/home.svg')} width="18" height="18" alt="Home" style={{verticalAlign: 'middle'}} /> |
| **Members** | `/your-handle/members` | <img src={useBaseUrl('/img/icons/users.svg')} width="18" height="18" alt="Users" style={{verticalAlign: 'middle'}} /> |
| **Drafts** | `/your-handle/team-drafts` | <img src={useBaseUrl('/img/icons/file-edit.svg')} width="18" height="18" alt="File edit" style={{verticalAlign: 'middle'}} /> |
| **DAO** | `/your-handle/dao` | <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="3" y1="22" x2="21" y2="22"/><line x1="6" y1="18" x2="6" y2="11"/><line x1="10" y1="18" x2="10" y2="11"/><line x1="14" y1="18" x2="14" y2="11"/><line x1="18" y1="18" x2="18" y2="11"/><polygon points="12 2 20 7 4 7"/></svg> |
| **Rewards** | `/your-handle/team-rewards` | <img src={useBaseUrl('/img/icons/award.svg')} width="18" height="18" alt="Award" style={{verticalAlign: 'middle'}} /> |
| **Memberships** | `/your-handle/team-memberships` | <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="2" y="5" width="20" height="14" rx="2"/><line x1="2" y1="10" x2="22" y2="10"/></svg> |
| **Settings** | `/your-handle/team-settings` | <img src={useBaseUrl('/img/icons/settings.svg')} width="18" height="18" alt="Settings" style={{verticalAlign: 'middle'}} /> |

Inside **Settings**:

- **Members** — `/your-handle/team-settings/members`
- **Subscription** — `/your-handle/team-settings/subscription`

Bookmark `/your-handle/home` — it's the canonical entry point for everything your team does.

## Members and roles

The Members tab (`/your-handle/members`) shows everyone currently on the team, plus pending invitations.

To bring someone in, open **Settings → Members** and send an invite. Invitees see the invitation in their inbox and can accept or decline. When they accept, they appear immediately on the public Members page.

At a high level, teams have two kinds of people:

- **Team admins** can update team settings, invite or remove members, manage the subscription, and approve DAO outcomes.
- **Team members** can co-author posts, contribute to drafts, take part in DAO votes, and receive rewards.

Most day-to-day work — writing posts, voting, claiming rewards — is open to all members. Anything that changes the team's identity or billing is admin-only.

## Posts, drafts, rewards, memberships

A team is a full publishing unit, just like an individual user.

### Posts
Posts authored under the team handle appear on the team home and in feeds attributed to the team. Any member can publish on the team's behalf.

### Drafts
The Drafts tab (`/your-handle/team-drafts`) is a shared workspace. Pick up someone else's draft, leave it for a teammate to finish, or co-edit before publishing.

### Rewards
The Rewards tab (`/your-handle/team-rewards`) tracks rewards the team has earned — from spaces it has run, posts that were rewarded, or governance participation. Rewards can stay in the team treasury or be distributed to members.

### Memberships
Teams can offer **paid memberships** to supporters at `/your-handle/team-memberships`. Supporters subscribe to your team and unlock member-only content or perks. This is separate from the team's own subscription (below).

## Team subscription

Open **Settings → Subscription** at `/your-handle/team-settings/subscription` to manage your team's plan.

The subscription unlocks team-level features (monthly Credits for reward Spaces, the Trusted Creator badge, raw participant data access at higher tiers). Billing is handled off-chain via PortOne, so you can pay with the methods you already use. See [Team Settings → Team subscription](./team-settings.md#-team-subscription--team-settingssubscription) for the full breakdown.

## DAO — collective governance

Every team has a built-in DAO at `/your-handle/dao`.

The DAO is where the team makes decisions together rather than having one person decide. Any member can open a **proposal** — for example, "allocate part of the treasury to a campaign", "approve a sub-team's application", or "change the team's bio". Members then vote, and the result becomes the team's official position.

Proposal types tie back into real team actions: budget allocations affect the rewards treasury, sub-team decisions affect team structure, and governance outcomes are recorded on the team's timeline so the history is auditable.

## Sub-teams

Teams can also have **sub-teams** — smaller groups operating under a parent team's umbrella, each with their own bylaws, docs, and announcements.

Sub-team flows live under your team's arena and cover the full lifecycle:

- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="m9 11 3 3L22 4"/><path d="M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11"/></svg> **Apply to be a sub-team** of a parent team — `/your-handle/sub-teams/apply`
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg> **Application status** — `/your-handle/sub-teams/application`
- <img src={useBaseUrl('/img/icons/users.svg')} width="18" height="18" alt="Users" style={{verticalAlign: 'middle'}} /> **Manage child sub-teams** — `/your-handle/sub-teams/manage`
- <img src={useBaseUrl('/img/icons/file.svg')} width="18" height="18" alt="File" style={{verticalAlign: 'middle'}} /> **Sub-team detail** — `/your-handle/sub-teams/:sub-team-id`, including the **deregister** flow
- <img src={useBaseUrl('/img/icons/file-edit.svg')} width="18" height="18" alt="File edit" style={{verticalAlign: 'middle'}} /> **Sub-team docs** — compose and edit shared documents under `/your-handle/sub-teams/docs/...`
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="m3 11 18-5v12L3 14v-3z"/><path d="M11.6 16.8a3 3 0 1 1-5.8-1.6"/></svg> **Sub-team announcements** — broadcast updates to members under `/your-handle/sub-teams/announcements/...`
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M9 12l2 2 4-4"/><path d="M21 12c0 4.97-4.03 9-9 9s-9-4.03-9-9 4.03-9 9-9c2.39 0 4.68.94 6.36 2.64"/></svg> **Bylaws** — `/your-handle/bylaws`
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4"/><polyline points="16 17 21 12 16 7"/><line x1="21" y1="12" x2="9" y2="12"/></svg> **Leave parent** — `/your-handle/parent/leave`

See [Sub-teams](./sub-teams.md) for the full flow — apply, manage, docs, announcements, bylaws, deregister, leave parent — covered URL by URL.
