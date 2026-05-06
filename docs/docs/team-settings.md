---
sidebar_position: 15
title: Team Settings
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Team Settings

Team Settings is where a Team's admins manage the Team itself — its public identity, its members and roles, the subscription paying for its monthly Credits, and (if it comes to it) the deletion flow. This is the **administrator's** view of a Team; for the public profile and feed, see the [Create a Team](./teams) chapter.

## Where Team Settings live

Three URLs make up the surface, all under the Team's handle:

```
/<team-handle>/team-settings              # general settings + billing dashboard (owner-only card)
/<team-handle>/team-settings/members      # member management
/<team-handle>/team-settings/subscription # tier upgrade flow (5-card plan grid, same as /membership)
```

You'll usually arrive here from the user dropdown at the bottom of the team sidebar — click *Settings* — but the URLs are stable and shareable.

### Who can access

Team Settings is **admin-only**. The exact granularity depends on the action:

- **Owners** can do everything — including operating the **Billing dashboard** card on the General page, transferring ownership *(Coming soon)*, and deleting the Team.
- **Admins** can edit Team profile fields and manage members. The Billing dashboard card and the Delete Team action are **owner-only** — admins (non-owners) don't see those cards.
- **Members** who try to open `/team-settings` see *"You don't have permission to view team settings"* in place of the page; the URL stays the same, no redirect. The Members tab on `/team-settings/members` does have a viewer-friendly variant for ordinary members, but it's read-only — invite, role change, and remove are admin-gated.

The DAO Address (when DAO is activated) and on-chain governance settings are also owner-only.

## <img src={useBaseUrl('/img/icons/settings.svg')} width="20" height="20" alt="General" style={{verticalAlign: 'middle'}} /> General settings

The main `/team-settings` page is one column of admin controls. Each section autosaves as you edit; the *Last saved* chip at the top tells you when the most recent change persisted.

### Team Profile

The first card. Everything visible on the Team's public profile is edited here:

- **Team Logo** — square upload, recommended 256 × 256 (1:1), max 2 MB. The logo appears on the Team's profile, in feeds, and next to posts published under the Team handle.
- **Banner image** — wide header upload, recommended 1200 × 400 (16:9), max 5 MB.
- **Team name (display name)** — what people see in the UI. Up to 30 characters.
- **Description** — a longer paragraph describing what the Team does. Minimum 10 characters; the platform's content filter rejects test keywords and offensive language.
- **Username (handle)** — locked. The handle is set when the Team is created and is hard to change later — contact support if it absolutely must move. The field is shown read-only so admins know what URL their Team lives at.
- **DAO Address** *(when DAO is activated)* — the on-chain treasury / governance address tied to this Team. Read-only here; DAO activation lives in the [Teams → DAO](./teams#dao--collective-governance) flow.

Changes to display name, description, logo, and banner are visible everywhere immediately after autosave.

### Danger Zone — Delete Team

At the bottom of the General page, owners (only) see a **Danger Zone** card with a single action: **Delete team**. Clicking it opens a confirm dialog (*"This will permanently delete \<Team name\>, including members, posts, and any associated governance data. This action cannot be undone."*) and a **Confirm** button.

A few things to know:

- **Owner-only.** Admins (non-owners) don't see this card at all.
- **Cascades to sub-team links.** If this Team is the parent of any recognized sub-teams, the deletion attempts to cascade-deregister those links so the children become standalone. If the cascade fails for any reason, the Team is still deleted — the children's `parent_team_id` may need a manual cleanup.
- **Irreversible.** There is no undelete; restore from a screenshot is the only way back.

## <img src={useBaseUrl('/img/icons/users.svg')} width="20" height="20" alt="Members" style={{verticalAlign: 'middle'}} /> Members & roles

The Members tab lives at `/team-settings/members`. It shows everyone currently on the Team plus controls for admins.

### Roles

Three role types ship today, by capability tier:

- **Owner** — the original creator (or whoever ownership has been transferred to). Can do everything: edit settings, manage members, change roles, delete the Team. There is exactly one owner at a time.
- **Admin** — can edit Team settings, invite and remove members, promote / demote between Member and Admin (cannot remove or demote the Owner), and operate the subscription.
- **Member** — can co-author posts under the Team handle, contribute to drafts, vote in DAO proposals, and receive Team-attributed rewards.

A Team can have multiple admins; promote a member by clicking the per-row menu and choosing **Make admin**. Demote with **Make member**.

### Add members

The **Add members** button opens an invite flow. Send an invite by username or email; invitees see the invitation in their inbox and can **Accept** or **Decline**. On accept, they appear immediately on the Members list as a Member.

Invites that haven't been accepted yet show up in a separate pending section — admins can revoke them before acceptance.

### Remove a member

The per-row menu also exposes **Remove from team**. Removed members lose Team-attribution on future actions but keep their personal account intact. Admins cannot remove the Owner.

### Transfer ownership *(Coming soon)*

Today, the Owner role is set at Team creation and cannot be transferred through the UI. A self-serve transfer flow — *"Make this admin the new Owner"* — is on the roadmap. Until it ships, contact support to migrate ownership when needed.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="Billing" style={{verticalAlign: 'middle'}} /> Billing dashboard on General settings (owner-only)

The status-and-card *dashboard* for the Team's subscription doesn't live on `/team-settings/subscription` — it sits as an additional card on the **General settings page** (`/team-settings`), and it's gated to the **owner** only. Admins (non-owners) don't see this card at all.

What it shows:

- **Current Plan badge** — Free / Pro / Max / VIP / Enterprise, with a *Change Plan* / *View Membership* link that takes you to `/team-settings/subscription` (or `/membership`) to actually swap tiers.
- **Credits** — Remaining and total Credits for the current cycle (e.g. `145 / 190`). On Free this reads `0 / 0` and the card nudges you to upgrade.
- **Expires** — When the current cycle's allotment expires. Free shows *Unlimited*.
- **Card on file** — The masked card number used to bill the subscription, plus the cardholder name. **Add Card** when no card is registered, **Change Card** to update.

The card form is the same PortOne KR checkout users see (Card Number / Expiry MM / YY / Birth or Business Registration Number / Card Password first 2 digits). PortOne supports Visa, Mastercard, AMEX, JCB, and the local methods PortOne offers in Korea.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="Subscription" style={{verticalAlign: 'middle'}} /> Team subscription — `/team-settings/subscription`

The `/team-settings/subscription` route is **the upgrade flow itself** — the same five-card plan grid as `/membership` (Free / Pro / Max / VIP / Enterprise) with the same **Get Pro** / **Get Max** / **Get VIP** / **Contact Us** call-to-action buttons. Tier change happens through the same purchase modal individuals see; the difference is the charge goes against the Team's billing card rather than a personal card.

Use the URL `/<team-handle>/team-settings/subscription` when you want to land directly on the plan grid for the Team — for example, sharing a link with the Owner to nudge them to upgrade. Use the **Billing dashboard** card on `/team-settings` (owner-only) when you want to *read* the current plan, see Credit usage, and manage the card without changing tiers.

### Tiers — same five as personal

The Team subscription uses the same five tiers as the user subscription, with the same Credit allotments and per-Space caps. See [Membership](./membership) for the full feature matrix:

- **Free** — included with every Team.
- **Pro** — 40 monthly Credits, up to 2 per reward Space, ₩30,000 / month.
- **Max** — 190 monthly Credits, up to 10 per reward Space, Trusted Creator badge *(Coming soon — Phase 1)*, ₩75,000 / month.
- **VIP** — 1,360 monthly Credits, up to 100 per reward Space, raw participant data access, ₩150,000 / month.
- **Enterprise** — fully customizable, by-contact, starting at $1,000 / month.

A few Team-specific notes:

- **Credits are billed once and shared.** A Team's monthly Credits are funded by the Team's subscription, not pooled from individual members. Any admin spending Credits on a reward Space (via [Host Actions](./spaces/host-actions)) draws from the Team's allotment.
- **Owner-only purchase.** Only the Team's owner can complete a tier upgrade — the purchase flow draws from the card on file in the owner-gated Billing dashboard.
- **Off-chain billing only.** Same as personal subscriptions — there's no on-chain settlement for tier billing.

## What's not on this page

A handful of Team-level controls aren't part of `/team-settings`:

- **DAO activation** — lives on the Team home / DAO flow, not in settings.
- **Sub-teams** (apply, manage, deregister, leave) — see [Sub-teams](./sub-teams). These are governance flows, not account settings.
- **Drafts, Posts, Spaces, Memberships sold by the Team** — managed from the Team's own arena tabs, not settings.
- **Team-level notifications preferences** — *(Coming soon)*.
- **Audit log of admin actions** — *(Coming soon)*.

## Summary table

| URL | What you change |
|---|---|
| `/<team-handle>/team-settings` | Logo, banner, name, description (admins+). Billing dashboard card — current tier, Credits, expiry, card management (owner-only). Delete Team (owner-only). |
| `/<team-handle>/team-settings/members` | Invite, remove, promote / demote between Admin and Member (admins+). |
| `/<team-handle>/team-settings/subscription` | The 5-tier plan grid — *Get Pro* / *Get Max* / *Get VIP* / *Contact Us* upgrade buttons, billed against the Team's card (owner-only completes purchase). |

## What's next

- [Create a Team](./teams) — the foundational Team chapter (members, drafts, DAO basics).
- [Sub-teams](./sub-teams) — the full governance lifecycle for parent / child Team relationships.
- [Membership](./membership) — pick the tier whose monthly Credits will pay for your Team's reward Spaces.
