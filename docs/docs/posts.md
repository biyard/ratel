---
sidebar_position: 5
title: Posts
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Posts

## Why posts matter

Posts are the highest-signal input to your Essence. Every line you publish, every comment you leave, and every reaction you give becomes an EssenceSource — the raw material your House learns from. The more you write on Ratel, the richer your Essence grows, and the more useful your House becomes to the people who subscribe to it.

## Browse the feed

Your feed lives at `/` (home) and `/posts`. It is the main place to discover what other humans on Ratel are thinking about — and to find conversations worth jumping into.

- <img src={useBaseUrl('/img/icons/users.svg')} width="18" height="18" alt="Users" style={{verticalAlign: 'middle'}} /> **Following** — posts from people, teams, and Houses you follow.
- <img src={useBaseUrl('/img/icons/compass.svg')} width="18" height="18" alt="Compass" style={{verticalAlign: 'middle'}} /> **Discover** — a wider mix surfaced from across the network, useful when you are looking for something new.
- **Infinite scroll** — keep scrolling and the next batch loads automatically. You never need to click "next page".
- **Sort and filter** — the top of the feed lets you switch between recency and engagement, and narrow by topic where available.

:::tip
If your feed feels too quiet, follow a few more people from the Discover tab. Posts from accounts you follow are weighted highest.
:::

## Write a post

Click the <img src={useBaseUrl('/img/icons/edit.svg')} width="18" height="18" alt="Edit" style={{verticalAlign: 'middle'}} /> **compose** button anywhere in the app (or open `/posts` and start a new one) to launch the editor. Ratel's editor is built on **Tiptap**, a modern rich-text editor — no Markdown wrangling required, but everything you would expect from a serious writing tool is there.

You can use:

- **Headings** (H1–H3) for structure
- **Bold**, **italic**, and **strikethrough** for emphasis
- **Bulleted and numbered lists** for organizing thoughts
- **Block quotes** for citing other people
- **Code blocks** with syntax highlighting for technical content
- **Inline links** to anywhere on the web
- **Images** — drag and drop, paste from clipboard, or upload from your device
- **Embeds** — paste a link to a YouTube video, X post, or other supported source and Ratel will turn it into a rich preview

Posts also support **hashtags** (`#essence`) for discovery and **@mentions** to pull other Ratel users into the conversation.

When you are done, you have two choices: **Save as draft** to keep working on it later, or **Publish** to put it on your timeline and into the feeds of people who follow you.

## Drafts

Drafts live at **`/your-handle/drafts`** — replace `your-handle` with your own username. This is your private workspace for posts that aren't ready yet, and the page is owner-only — visiting someone else's drafts URL returns nothing.

### Autosave behavior

Every post in the editor is autosaved as a draft from the moment you start typing. There's no "save" button to remember — close the tab, lose connection, switch devices, and your in-progress writing is right there when you come back. The Drafts page lists each in-progress post with a **Writing now** badge while you're actively in it from another tab.

### Stat header and filters

The top of `/your-handle/drafts` is a stat strip — **Total drafts**, **Words written**, **Last edited** — useful as a quick "how productive have I been this month?" glance.

Filter chips below let you scope the list:

- **All** · **Today** · **This Week** · **Older** — by recency.
- **Space-enabled** — drafts that have a Space attached (i.e. drafts that will publish a post + create a Space simultaneously).

A **Sort** dropdown reorders the list: *Recently edited* (default), *Oldest first*, *Title A → Z*, *Most words*. The list is then bucketed into *Today / This Week / Older* sections so you can scan visually.

### Per-draft actions

Each draft tile shows a thumbnail, title (or *Untitled draft*), an excerpt, the time-ago since last edit, and an image count if you've attached any. The **`…`** menu on each tile exposes:

- **Resume editing** — open the draft back in the post editor.
- **Duplicate** — copy the draft to a new untitled draft (handy for templating recurring posts).
- **Export as Markdown** — download a `.md` file of the draft body. (Export targets the body — image attachments stay in Ratel.)
- **Delete draft** — permanent, no undo. (Restore-from-trash is *(Coming soon)*.)

### Team drafts

You can also keep separate **team drafts** for collaborative posts at `/your-handle/team-drafts`. The Team's admins / members can co-author drafts there before publishing under the Team handle. Same autosave + per-draft actions, but the workspace is shared with your Team's admin tier.

## Post detail page

Every post has its own URL: **`/posts/:post_id`**. This is the page you share when you want to point someone at a specific piece of writing.

On the post detail page you can:

- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M7 10v12"/><path d="M15 5.88 14 10h5.83a2 2 0 0 1 1.92 2.56l-2.33 8A2 2 0 0 1 17.5 22H7V10l4.5-9.5L13 1.5l2 1.94v.01l.5 1.93z"/></svg> **Like** the post — a quick signal of agreement or appreciation. Ratel uses a thumbs-up glyph for likes.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="3" width="18" height="14" rx="2"/><path d="M3 17l4 4v-4"/></svg> **Comment** — start a thread or reply to someone else's comment.
- <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="18" cy="5" r="3"/><circle cx="6" cy="12" r="3"/><circle cx="18" cy="19" r="3"/><line x1="8.59" y1="13.51" x2="15.42" y2="17.49"/><line x1="15.41" y1="6.51" x2="8.59" y2="10.49"/></svg> **Share** — copy the link to send anywhere, or share to your favorite platform.
- <img src={useBaseUrl('/img/icons/edit-square.svg')} width="18" height="18" alt="Edit square" style={{verticalAlign: 'middle'}} /> **Edit** (authors only) at **`/posts/:post_id/edit`** — fix typos, refine arguments, or update facts as the situation evolves.

Comments support the same Tiptap formatting as posts, so you can quote, link, and embed inside replies too.

## Cross-posting

One of Ratel's flagship features: write your post once, publish it everywhere that matters. Today Ratel syndicates posts to <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** automatically. Connections for <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** and <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** are in the UI and ship next.

### Connect once

Head to **`/your-handle/settings/connections`** and connect each destination. You authorize Ratel through each platform's official login flow — your credentials never touch our servers. New users are walked through this in the **`/onboarding/connections`** flow the first time they sign in.

### Toggle per post

When you compose a post, the editor sidebar shows your connected destinations. Toggle on the ones you want to publish to — leave the rest off if a post is Ratel-only. You can mix and match per post; nothing is "all or nothing".

### Mind the limits

Each platform has its own content rules, and Ratel adapts your post to fit:

- <img src={useBaseUrl('/img/icons/bluesky.svg')} width="16" alt="Bluesky" style={{verticalAlign: 'middle'}} /> **Bluesky** — 300 characters, so longer posts get gracefully trimmed with a link back to the full post on Ratel.
- <img src={useBaseUrl('/img/icons/linkedin.svg')} width="16" alt="LinkedIn" style={{verticalAlign: 'middle'}} /> **LinkedIn** *(Coming soon)* — long-form friendly, so most posts go through as-is.
- <img src={useBaseUrl('/img/icons/threads.svg')} width="16" alt="Threads" style={{verticalAlign: 'middle'}} /> **Threads** *(Coming soon)* — 500 characters, similar trimming behavior to Bluesky.

### Pick destinations at publish time

When you compose a post, the editor's right-hand **Cross-post** sidebar shows every connected destination as a row with a toggle. Defaults follow your **Auto-post** setting per destination (managed at `/your-handle/settings/connections`), but you can flip toggles per post — turn one off if a draft is Ratel-only, turn one on for a single one-off announcement.

Above the rows the sidebar shows a *Reaching N networks* header and a **Truncated** badge whenever your post will be trimmed at any destination's character limit. So you know up-front whether the published copy will lose anything.

### Length-limit handling

Each destination has a hard ceiling — Bluesky 300 chars, Threads 500 (when shipped), Farcaster 320 (when shipped), LinkedIn 3,000 (when shipped). When your Ratel post exceeds the limit:

> A 1,500-character Ratel post becomes a Bluesky 280-character excerpt followed by `… → ratel.foundation/posts/<id>` — readers click through to read the full post on Ratel.

The trim is character-aware (won't cut mid-word) and always preserves the backlink so external readers can find the canonical version.

### What the post detail page shows after publish

The published post's detail page (`/posts/:post_id`) gains a **Syndication** panel below the post body. Each connected destination renders a row with:

- A status badge — **Published** (green check + a *View* link to the external post), **Pending** (queued — awaiting dispatch), **Failed** (red, with a **Retry now** button), or **Skipped** (you turned the toggle off at publish time).
- *N succeeded · N failed* summary at the top.
- Engagement stats from the destination once they roll in — likes, comments, reposts.
- An **Attempt** counter on Failed rows so you can tell whether a retry has already been tried.

A small **Refresh** button on the panel re-pulls live engagement counts on demand.

### Failure handling

Most cross-post failures fall into two categories:

- **Token expired** — the destination platform revoked Ratel's access. Reconnect at `/<your-handle>/settings/connections`; the row's **Retry now** button works again the moment the token is refreshed.
- **Destination platform unavailable** — the panel shows *Failed* with the attempt counter. Click **Retry now** once the platform comes back up. Ratel doesn't retry indefinitely on its own — you stay in control.

:::tip
If a cross-post fails (a platform was offline, your token expired, etc.), reconnect at `/your-handle/settings/connections` and click **Retry now** on the failed row to dispatch again. Future posts will sync normally once the connection is restored.
:::

## How posts feed your Essence

Every post you publish, every comment you write, and every reaction you give is collected as an **EssenceSource**. Over time these sources are turned into embeddings and woven into your personal knowledge base — the foundation of your House. The more you write, the better your House understands you, and the more useful it becomes to subscribers asking it questions.

:::note
The full Essence pipeline (embeddings, retrieval, House Q&A) is rolling out in phases. Posts you publish today are already being captured as EssenceSources for when later phases ship.
:::
