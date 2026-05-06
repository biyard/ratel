---
sidebar_position: 2
title: My Essence
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# My Essence

Your Essence isn't an abstract concept — it's a real surface in the app, with two dedicated pages where you see what you've built and what it earns you. **My AI** at `/my-ai` is where you connect Ratel to your AI assistant. **Character** at `/me/character` is where your activity becomes a level, skill points, and stat boosts you can spend.

## Why your Essence has its own pages

The [Essence overview](./) describes Essence as the personal knowledge base that grows as you participate. These pages are where that idea becomes tangible: one tab plugs Essence into the AI you already use; the other turns the activity behind your Essence into a visible level and a set of upgradeable skills.

## <img src={useBaseUrl('/img/icons/grid.svg')} width="20" height="20" alt="My AI" style={{verticalAlign: 'middle'}} /> My AI — `/my-ai`

The My AI page is your personal MCP control panel. It does three things, in order:

### Generate and copy your endpoint

The hero card at the top shows your private MCP URL — the one you paste into Claude Code, Claude Desktop, Cursor, or any other MCP-compatible client. The URL takes the form:

```
https://ratel.foundation/mcp/<your-token>
```

Click **Generate** the first time to mint a token. The full URL is shown only once, right after generation — copy it immediately. To rotate, click **Regenerate**: a fresh URL appears and the old one stops working instantly. Treat it like an API key.

A status badge in the page topbar tells you at a glance whether your endpoint is **MCP Online** (token issued) or **Not Configured** (you haven't generated yet).

### See what your agent can do

Below the endpoint, **What your agent can do** lists the capabilities your token unlocks for any connected client:

- **Posts & Drafts** — Create, edit, like, and delete posts.
- **Polls & Quizzes** — Run, respond, and analyze results.
- **Spaces & Teams** — Create spaces, manage teams, follow users.
- **Discussions** — Read, comment, and start discussions.
- **Notifications** — Read your inbox, mark items as read.

Every action is scoped to your account and auditable — your agent acts as you, never as someone else.

### Pick your client and follow the setup guide

The **Setup guide** below the capabilities has a tabbed walkthrough — one tab per client:

- **Claude Code** — `claude mcp add` one-liner; verify with `/mcp` inside an active session.
- **Claude Desktop** — Drop the JSON snippet into `claude_desktop_config.json`, fully quit and relaunch.
- **Cursor** — `Settings → Cursor Settings → MCP → Add new server (HTTP / Streamable)`.
- **Generic JSON** — A `curl` smoke-test plus the JSON-RPC 2.0 / SSE shape for any MCP SDK (Python, TypeScript, Rust).

Each tab walks you through three numbered steps, with copy-ready commands and JSON snippets — `/my-ai` is the in-app destination for the same MCP setup workflow.

## <img src={useBaseUrl('/img/icons/award.svg')} width="20" height="20" alt="Character" style={{verticalAlign: 'middle'}} /> Character — `/me/character`

The Character page turns your Essence-building activity into a game. The same posts, comments, and Space actions that feed your Essence also earn you XP. XP becomes levels. Levels grant Skill Points (SP). Skill Points buy upgrades that boost your future earnings.

### Hero — Level, XP, SP

The arena hero at the top shows three numbers:

- **Level** — Your current Character level.
- **Character XP** — Total XP earned, plus the XP needed to reach the next level.
- **Skill Points (SP)** — Points ready to spend in the skill tree below.

The hero also tells you whether you have points ready to spend, or whether you need to earn more XP first.

### Skill Tree

Each skill caps at ten ranks. Every rank gives **+5%** of the skill's effect, capping at **+50% at L10**. Costs scale: 5 SP for the first rank, then +4 SP per rank thereafter (5 → 9 → 13 → … → 41 at L10, **about 230 SP to fully max one skill**).

**Available now**

- **Money Tree** — Boosts every RatelPoint payout you receive from any Space's reward, applied multiplicatively before the amount is credited to your balance.
- **Ranker** — Boosts the bonus portion of every Space activity you record. Compounds — more XP per action means faster leveling, which means more SP for future skills.

**Coming in v2** *(Coming soon)*

- **Influencer** — *v2 design:* lowers the participants-required-for-Hot threshold for Spaces you own (e.g. L6 surfaces with 4 participants instead of the global 10).
- **Sweeper** — *v2 design:* increases the owner-bonus you receive when a participant claims a reward in a Space you own (e.g. L6 takes 40% of every payout instead of the default 10%).

Each card shows its current level, what the next rank will give you, and a **Level Up** button (or **Maxed** when you've hit L10). v2 skills appear with a **v2 · Coming soon** badge and aren't levelable yet.

## How My AI and Character feed each other

Both pages are powered by the same activity stream:

- Every action you take on Ratel — writing a post, replying to a discussion, hosting a poll, completing a Space quest — feeds your **Essence** (the data layer) *and* your **Character XP** (the gameplay layer).
- An MCP-connected agent that posts on your behalf adds to both — your Essence gets richer and your Character levels up — as long as the work is yours.
- Skill upgrades you buy on Character (Money Tree available now) directly affect what you take home from Spaces and rewards.

In other words: My AI is how Essence reaches outward; Character is how Essence rewards inward. They're two views of the same engine.

## What's next

*(Coming soon)* Phase 2 brings the **Essence House** marketplace — your Essence becomes something other users can subscribe to via a single unified MCP URL. When that ships, your existing My AI endpoint automatically gains routing across every House you've subscribed to; you won't need to update any client config.

Until then, My AI is your endpoint and Character is your scoreboard. Generate the URL, plug it into your AI, and start earning XP.
