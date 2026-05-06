---
sidebar_position: 3
title: Connect AI
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Connect AI (MCP)

Ratel exposes a single MCP endpoint per user. Plug it into the AI client you already use, and your assistant gains access to your Essence — and any Essence Houses you subscribe to — as a first-class tool.

## What is MCP?

**MCP (Model Context Protocol)** is an open standard, originally introduced by Anthropic, for letting AI assistants talk to external tools and data sources over a uniform protocol. Instead of hand-wiring a custom integration into every model, an MCP-compatible client — Claude Desktop, Cursor, Windsurf, Zed, and a growing list of others — can connect to any MCP server and discover its tools automatically.

Ratel uses MCP because it turns your Essence into something an assistant can *act on*, not just read about. Once connected, the AI you already talk to every day can query your House, draft and publish posts on your behalf, manage spaces, run polls and quizzes, and (in Phase 2) reach across every Essence House you subscribe to from a single endpoint.

## Get your unified MCP URL

Your MCP URL lives in your account settings.

1. Sign in to Ratel and open <img src={useBaseUrl('/img/icons/settings.svg')} width="16" height="16" alt="Settings" style={{verticalAlign: 'middle'}} /> **User Settings**.
2. Scroll to the <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="2" y="2" width="20" height="8" rx="2"/><rect x="2" y="14" width="20" height="8" rx="2"/><line x1="6" y1="6" x2="6.01" y2="6"/><line x1="6" y1="18" x2="6.01" y2="18"/></svg> **MCP Server** section.
3. Click <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><circle cx="7.5" cy="15.5" r="5.5"/><path d="m21 2-9.6 9.6"/><path d="m15.5 7.5 3 3L22 7l-3-3"/></svg> **Generate Secret**. Ratel will show your URL once — copy it immediately.
4. To get a new URL later, click <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.75" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M3 12a9 9 0 0 1 9-9 9.75 9.75 0 0 1 6.74 2.74L21 8"/><path d="M21 3v5h-5"/><path d="M21 12a9 9 0 0 1-9 9 9.75 9.75 0 0 1-6.74-2.74L3 16"/><path d="M3 21v-5h5"/></svg> **Regenerate**. The old token stops working as soon as a new one is issued.

The URL has the shape:

```
https://ratel.foundation/mcp/<your-secret-token>
```

The token is unique to *you*. **One user = one MCP URL**, regardless of how many Houses you subscribe to. When marketplace subscriptions ship in Phase 2, the same URL automatically gains access to every House you've subscribed to — you never juggle multiple endpoints.

:::caution
The token is shown **only once** at the moment of generation. If you lose it, regenerate — there is no way to retrieve the old one.
:::

## Set up Claude Desktop

Claude Desktop reads MCP servers from a JSON config file:

| OS      | Path                                                                    |
| ------- | ----------------------------------------------------------------------- |
| macOS   | `~/Library/Application Support/Claude/claude_desktop_config.json`       |
| Windows | `%APPDATA%\Claude\claude_desktop_config.json`                           |
| Linux   | `~/.config/Claude/claude_desktop_config.json`                           |

Open (or create) the file and add a `ratel` server entry under `mcpServers`:

```json
{
  "mcpServers": {
    "ratel": {
      "url": "https://ratel.foundation/mcp/YOUR_SECRET_TOKEN"
    }
  }
}
```

Save the file and **fully quit and relaunch Claude Desktop** (the app must restart to pick up new MCP servers). When it reopens, the tool icon in the message composer should list the Ratel tools (`get_me`, `list_posts`, `create_post`, and so on). If it doesn't, double-check the JSON is valid and that the URL works in a browser — a 405 response from `GET` is normal and means MCP is reachable.

## Set up ChatGPT

ChatGPT supports MCP servers through **custom connectors** in the desktop and web apps. The exact UI changes frequently as OpenAI rolls features out, so the canonical reference is OpenAI's own connector docs — but the general flow is the same:

1. Open **Settings → Connectors** (or **Apps & Connectors**) in ChatGPT.
2. Choose **Add a custom connector** (or **Add MCP server**).
3. Paste your full Ratel MCP URL.
4. Authorize, then enable the connector for the conversations or projects you want.

Once authorized, asking ChatGPT something like *"Use my Ratel tools to draft a post about ..."* will let it call into your Essence directly.

## Set up Cursor and other clients

**Cursor** has built-in MCP support. Open `Settings → MCP → Add new MCP server`, paste the URL, and save. Cursor will list the discovered tools alongside its own.

**Windsurf, Zed, Claude Code, and any MCP-compatible client** follow the same shape — they all accept either a JSON config block (like Claude Desktop) or a URL field in their settings UI. Drop the same URL in and you're done.

```json
{
  "mcpServers": {
    "ratel": { "url": "https://ratel.foundation/mcp/YOUR_SECRET_TOKEN" }
  }
}
```

## Available tools

The Ratel MCP server exposes tools across four areas: **identity**, **posts and feed**, **spaces and actions**, and **insights**. The list below is what's wired today — the assistant discovers them automatically when it connects.

### <img src={useBaseUrl('/img/icons/user.svg')} width="20" height="20" alt="User" style={{verticalAlign: 'middle'}} /> Identity and notifications

- **`get_me`** — current user info and membership tier.
- **`list_teams`** — every team you belong to, with role and permissions.
- **`list_inbox`** — your notification inbox, newest first; supports `unread_only` and pagination.
- **`get_unread_count`** — number of unread notifications (capped at 100).

### <img src={useBaseUrl('/img/icons/edit-square.svg')} width="20" height="20" alt="Edit square" style={{verticalAlign: 'middle'}} /> Posts and feed

- **`create_post`** — start a new draft post (optionally under a team).
- **`get_post`**, **`list_posts`** — read posts and paginate the feed.
- **`update_post`** — edit content, change visibility, publish.
- **`delete_post`**, **`like_post`** — moderation and reactions.

### <img src={useBaseUrl('/img/icons/grid.svg')} width="20" height="20" alt="Grid" style={{verticalAlign: 'middle'}} /> Spaces

- **`create_space`**, **`get_space`**, **`update_space`**, **`delete_space`** — manage a space attached to a post.
- **`install_space_app`**, **`uninstall_space_app`** — add or remove apps inside a space (General, File, Analyzes — dev/staging only; Panels; Incentive Pool — beta builds).
- **`list_actions`** — list every poll, quiz, discussion, and follow inside a space.

### <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"/></svg> Actions inside a space

- Polls — `create_poll`, `update_poll`, `delete_poll`, `get_poll`, `respond_poll`.
- Quizzes — `create_quiz`, `update_quiz`, `get_quiz`, `respond_quiz`.
- Discussions — `create_discussion`, `update_discussion`, `delete_discussion`, `get_discussion`, `add_comment`, `list_comments`.
- Follow campaigns — `create_follow`, `get_follow`, `follow_user`.
- Meets — `create_meet`, `get_meet`, `update_meet`, `delete_meet`.
- AI moderator — `update_ai_moderator` (premium tiers only).

### <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg> Insights (Analyze app)

- `list_analyze_reports`, `get_analyze_report`, `create_analyze_report`, `preview_analyze_report` — saved cross-filter reports on a space's polls, quizzes, follows, and discussions.
- `list_analyze_records`, `get_matched_users` — drill down into the users behind a filter chip.
- `list_analyze_polls`, `list_analyze_quizzes`, `list_analyze_follows`, `list_analyze_discussions` — sources available to build a report from.
- `analyze_discussion`, `list_analyze_discussion_results`, `update_discussion_topics` — run and label LDA / TF-IDF / text-network analyses on discussion comments.

### <svg width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M9.937 15.5A2 2 0 0 0 8.5 14.063l-6.135-1.582a.5.5 0 0 1 0-.962L8.5 9.936A2 2 0 0 0 9.937 8.5l1.582-6.135a.5.5 0 0 1 .963 0L14.063 8.5A2 2 0 0 0 15.5 9.937l6.135 1.582a.5.5 0 0 1 0 .962L15.5 14.063a2 2 0 0 0-1.437 1.437l-1.582 6.135a.5.5 0 0 1-.963 0z"/><path d="M20 3v4"/><path d="M22 5h-4"/></svg> Coming soon (Phase 2)

- **`list_houses`** — list every Essence House you subscribe to.
- **`query_house`** — natural-language query against one specific House.
- **`search_essence`** — semantic search across your own Essence and all subscribed Houses, in one call.

When Phase 2 ships, these tools appear in the same endpoint automatically — you do not need to update your config.

## Tokens, permissions, key rotation

Your MCP URL contains a per-user secret token. Treat it like an API key.

- **Keep it secret.** Anyone with the URL can act as you on Ratel — read your inbox, post on your behalf, manage your spaces. Never paste it into a public repo, screenshot, or shared chat.
- **Rotate it from User Settings → MCP Server → Regenerate.** A new URL is issued immediately and cached MCP sessions for the old token stop working.
- **If you suspect a leak, regenerate immediately** and update every client that was using it.
- **Per-tool permission scoping** (limit a token to read-only, or to a specific House) is on the roadmap for Phase 2.

That's it — once your URL is in the client of your choice, your AI assistant can talk to your Essence the same way it talks to any other tool.
