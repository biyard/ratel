---
sidebar_position: 7
title: Profile
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Profile

Your profile is your public corner of Ratel — the page anyone gets when they click your name or paste your handle into a browser. It lives at:

```
/<your-handle>
```

For a handle of `alex`, that's `/alex`. The same handle threads through every page tied to your account, so once you know your URL you know how to reach anything you've published.

## What `/your-handle` shows publicly

When someone opens `/<your-handle>` they land on your **profile home** with three things stacked:

- **Banner** — a wide header image you upload (or the default arena banner if you haven't picked one).
- **Avatar, display name, bio** — your picture, the name you want people to see, and a short description of who you are.
- **Posts feed** — your most recent published posts, newest first, with infinite scroll for older work.

If they're signed in and not you, they'll see a **Follow** / **Following** button on the header. Following someone weights their posts higher in the follower's feed.

A **Level chip** to the right of your name shows your Character level — a public summary of how much activity has fed your Essence. (See [My Essence → Character](./my-essence#-character-mecharacter) for what the level represents and how to spend the SP it earns you.)

## Following and followers

Following on Ratel is a one-click toggle from any profile header — your own profile shows the chip without the action; everyone else's profile shows **Follow** (when you don't follow them) or **Following** (when you do).

A few things following does:

- **Weights their posts higher** in your feed. Follows feed the *Following* tab on the home feed and the platform's recommendation signal.
- **Notifies them** that you've followed (a small inbox event — opt-out is *(Coming soon)*).
- **Gets recorded into your Essence** as a *who you align with* signal — useful for downstream Space recommendations and (eventually) agent participation.

To unfollow, click **Following** on the same profile and the chip flips back to **Follow**. Both follow and unfollow take effect immediately.

A dedicated **`/following`** / **`/followers`** list page where you see and prune everyone you follow / who follows you in one place is *(Coming soon)*. For now, you'll spot follows naturally — they show up as you visit profiles, and Spaces use following as one input to invitation suggestions.

## Deep-link pages around your handle

Several pages share the `/<your-handle>` prefix and can be linked directly:

| <img src={useBaseUrl('/img/icons/home.svg')} width="16" height="16" alt="Home" style={{verticalAlign: 'middle'}} /> Surface | URL | What's there |
|---|---|---|
| **Profile home** | `/<your-handle>` | The banner + bio + recent posts (described above). |
| <img src={useBaseUrl('/img/icons/file-text.svg')} width="16" height="16" alt="Posts" style={{verticalAlign: 'middle'}} /> **Posts** | `/<your-handle>/posts` | Every post you've published, dedicated full-page list. |
| <img src={useBaseUrl('/img/icons/grid.svg')} width="16" height="16" alt="Spaces" style={{verticalAlign: 'middle'}} /> **Spaces** | `/<your-handle>/spaces` | Spaces you host or actively participate in. |
| <img src={useBaseUrl('/img/icons/users.svg')} width="16" height="16" alt="Memberships" style={{verticalAlign: 'middle'}} /> **Memberships** | `/<your-handle>/memberships` | Your Ratel membership tier — Free, Pro, Max, Vip, Enterprise. |
| <img src={useBaseUrl('/img/icons/award.svg')} width="16" height="16" alt="Credentials" style={{verticalAlign: 'middle'}} /> **Credentials** | `/<your-handle>/credentials` | Your DID-backed verifiable credentials — issued claims about you. |
| <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"/><polyline points="14 2 14 8 20 8"/><line x1="16" y1="13" x2="8" y2="13"/><line x1="16" y1="17" x2="8" y2="17"/></svg> **Drafts** | `/<your-handle>/drafts` | Posts-in-progress that no one but you can see. |

A **Categories** sidebar runs down the left side of the page. Tap a category chip (one per category the platform exposes — they're server-defined, not user-defined) to filter your feed; tap **All** to clear the filter. The sidebar doesn't change which page you're on — filters apply where the current page understands them (primarily Posts and the profile feed).

### Privacy at a glance

- **Profile · Posts · Spaces** are **public** under your handle. Anyone with the link can read them — signed in or not.
- **Memberships · Credentials are session-scoped.** They live under the `/<handle>/...` URL but always show **the signed-in viewer's** data, not the handle owner's. Visiting someone else's `/handle/memberships` URL shows your own memberships back to you. (These pages exist so the in-app navigation is consistent — they aren't a public-disclosure surface for the handle owner.)
- **`/your-handle/drafts` is private to you.** Visitors who try to open someone else's drafts URL get nothing back. The page is meant for your own work-in-progress and only shows when you're signed in as the owner.

If you ever want to *see what your profile looks like to a stranger*, open it in a private browser window or sign out — you'll get the public view.

## Editing your profile

The profile fields you see on `/<your-handle>` are edited from your settings. To change your **display name**, **avatar**, **banner**, or **bio**:

1. Open <img src={useBaseUrl('/img/icons/settings.svg')} width="14" height="14" alt="Settings" style={{verticalAlign: 'middle'}} /> **Settings** at `/<your-handle>/settings`. (Quick path: open the user dropdown at the bottom of the sidebar and click *Settings*.)
2. Update the fields under the **Profile** card.
3. Click **Save**. Changes are visible immediately.

Your **handle** itself is set when you first sign up and is hard to change later — pick something stable. Everything else (bio, avatar, banner) you can revise any time. See [Settings](./settings) for the full tour of every settings card.

## Linking to your profile from the outside

Your profile URL is canonical and safe to share anywhere:

```
https://ratel.foundation/<your-handle>
```

It survives URL normalization on Bluesky, X, LinkedIn, Slack, and Discord — the link still resolves cleanly and shows an <img src={useBaseUrl('/img/icons/compass.svg')} width="14" height="14" alt="Open Graph" style={{verticalAlign: 'middle'}} /> Open Graph preview with your avatar, display name, and bio. If you cross-post to Bluesky from a Ratel post (see [Posts → Cross-posting](./posts#cross-posting)), the syndicated copy automatically links back to the original on your profile.

For specific deep links — *"come read my latest post"*, *"join the Space I'm hosting this week"* — point at the post URL or the Space URL directly. The profile page is for *who you are*; the deep links are for *what you've made*.

## What's next

Once your profile is set up, the natural next steps are:

- [Settings](./settings) — fine-tune the rest of your account (password, billing, MCP server).
- [My Essence](./my-essence) — see what your activity has built up so far.
- [Rewards](./rewards) — track what you've earned across every Space.
