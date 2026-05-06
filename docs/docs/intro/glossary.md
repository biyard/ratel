---
sidebar_position: 2
title: Glossary
---

# Glossary

Quick definitions for every Ratel term you'll meet in these docs. Features marked **(Coming soon)** are on the roadmap — the underlying primitive may already exist, but the user-facing flow is still being built.

---

## AI

### Essence
Your personal, embedding-backed knowledge base — a representation of how *you* think, built from your own activity on Ratel and the external sources you connect. One Essence per user, fully under that user's control.

### EssenceSource
A single piece of input that feeds your Essence. Sources today include native Ratel activity (Posts, Post Comments, Discussion Comments, Polls and Quizzes you host); connected external content (Notion first; Google Docs, Substack, X, Slack to follow) is on the roadmap. Each source has a delete control; per-source enable/disable toggles are on the roadmap.

### Essence House
**(Coming soon)** Every user automatically gets one Essence House — a public-facing bundle of their Essence that other people can subscribe to. The owner sets the subscription price; subscribers receive query access via a unified MCP endpoint.

### Essence Connector
**(Coming soon for sources beyond Ratel-native)** The integration layer that brings external content into your Essence. Read-only OAuth plus real-time webhooks: edit a Notion page and it re-embeds within seconds. Notion ships first; Google Docs, Substack, X, and Slack are next.

### Quality Score
A per-Essence signal indicating how rich and useful your Essence is for downstream querying. Influenced by content depth, originality, and direct-activity history.

### Direct-Activity Index
Measures how much of your Essence comes from genuine, hands-on participation (writing, voting, discussing) versus passive imports. Used to keep platform signal high and to weight rewards.

### AI-authored content opt-in
By default, content authored by AI tools is **excluded** from your Essence. You can opt in per source if you want AI-assisted writing to feed your Essence — the choice is always yours and reversible.

### MCP (Model Context Protocol)
The open standard, originated by Anthropic, for connecting AI assistants (ChatGPT, Claude Desktop, Cursor, Windsurf, Zed, …) to external tools and knowledge over a uniform protocol. Ratel exposes Essences as MCP servers so any compatible client can query them.

### Unified MCP URL
A single endpoint per subscriber that routes across every Essence House they subscribe to. Today the URL gives you access to your own Essence and Ratel's tool surface; in Phase 2 the same URL automatically gains access to every House you've subscribed to — you never juggle multiple endpoints.

### Subscriber token
**(Coming soon)** The credential carried inside your Unified MCP URL. It identifies which Houses you've paid to access and gates queries accordingly. Treat it like an API key.

### Agent
**(Coming soon)** A Ratel-specific entity built from your Essence that can participate in Spaces on your behalf — answering Polls, joining Discussions, completing Follow quests. Agents are bound to a specific Essence and run inside Ratel's hosted runtime; they are not generic chatbots. Hosts can allow or block agents per Space.

### AI Moderator
**(Premium tiers)** An AI assistant that helps a host triage, summarize, or answer in their own Spaces. Configured per Space and tunable from the host's settings.

---

## Content

### Post
A standalone piece of writing published on Ratel. Posts feed your Essence and can be cross-posted to other platforms while remaining the canonical record.

### Comment
A reply to a Post or another Comment. Comments are first-class EssenceSources — what you say in a thread shapes your Essence the same way your Posts do.

### Vote
A signal of agreement or disagreement on a Post, Comment, or Poll option. Captured into your Essence as a record of the positions you've taken over time.

### Like
A lightweight reaction on a Post. Likes are weaker signals than Votes — they help discovery and recommendations but don't carry the same weight as a deliberate Vote.

### Draft
A Post-in-progress saved to `/your-handle/drafts` (or `/your-handle/team-drafts` for a team). Private to you (or your team) until you publish.

### Cross-posting
Publish a Ratel Post to external platforms — Bluesky live; LinkedIn, Threads, and more rolling out — in one step. The Ratel Post stays the source of truth; each destination's character limit is handled automatically.

---

## Spaces

### Space
A focused workspace for collective activity — discussion threads, polls, quizzes, files, analyses — all in one place. Hosted by a user or a Team; can be public or private. Lives at `/spaces/:space_id`.

### Host
The user (or Team) running a Space. Hosts choose which Apps to plug in, fund the Incentive Pool, and decide how rewards flow. They can allow or block agents and curate participation.

### Participant
Anyone who has joined a Space and engages with its Actions. Participants accumulate Essence as they engage and earn from the Incentive Pool when they complete reward-bearing Actions.

### Space App
A modular feature inside a Space. Hosts mix and match the apps they need:
- **General** — the Space's written content (welcome, rules, FAQ).
- **Files** — shared asset library (PDFs, decks, datasets).
- **Analyzes** — AI-assisted reports built from the Space's polls and discussions.
- **Panels** — targeted audience building and member-gated outreach.
- **Incentive Pool** — funding and rules for participant rewards.
- **Rewards** — host-side dashboard for what's earned and pending.

### Space Action
A specific activity inside a Space that participants take part in. Five core types — see below. Lives at `/spaces/:space_id/actions/...`.

### Discussion
An Action that invites participants to contribute their opinion to a host-posed topic. Replies become structured input to reports and feed each contributor's Essence.

### Poll
An Action that asks participants to vote among options. Lightest-touch participation; the workhorse of most Spaces.

### Quiz
An Action with scored questions and a passing threshold. Used for onboarding gates, certification, knowledge checks. Pass and the reward unlocks; fail and you may retry.

### Follow (action)
An Action that asks participants to follow a specific account or set of accounts — to grow a related creator's audience or seed a sub-community.

### Meet
An Action that schedules an event (livestream, call, workshop, in-person gathering). Participants RSVP and attendance counts toward completion.

### Report
A structured, AI-assisted document a host publishes that synthesizes the activity of a Space. Reports can be paid; sales are split with contributors via the Report Revenue Share.

---

## Rewards

### Incentive Pool
The funding a Space host sets aside to reward participants. Visible *before* participants engage, so they know what they're contributing to. Configured in the **Incentive Pool** App at `/spaces/:space_id/apps/incentive-pool`.

### Reward
The general term for any payout earned by a participant or host — drawn from a Space's Incentive Pool, distributed by the platform, or routed through the Report Revenue Share.

### Reward Point (Point)
The base unit of in-platform reputation. Points are awarded for completing actions and accumulate to your account. They feed your Direct-Activity Index and unlock platform-level perks; they are not directly cashable.

### Credit
The host-funded unit attached to a specific Action's reward. When a host configures an Action's reward, they allocate Credits from their Incentive Pool — and each Credit redeems against the pool's settlement currency at payout. Tracked in your account balance and bounded per-Space (hosts can't over-allocate).

### Token
A blockchain-native asset routed through your DID-linked wallet when you claim a past cycle's rewards. Tokens are the on-chain expression of Points earned in that cycle; the treasury is 100% collateralized, and the per-cycle exchange rate is what you receive at the moment of claim.

### Stablecoin
**(Coming soon)** A price-stable Token used as the settlement asset in Spaces where the host wants payouts denominated in a fiat-pegged currency rather than a volatile native Token. Optional and host-configurable.

### Cycle
One calendar month. Rewards accrue inside the current cycle, get a chance to lock in at month-end, and become claimable as Tokens on the 1st of the next cycle.

### Treasury Rate
The per-cycle exchange rate at which Points convert to Tokens at claim time. Backed 1:1 by a treasury reserve, updated every block.

### Off-chain settlement
**(Coming soon)** A no-wallet cashout flow for users who'd rather not handle a blockchain wallet. Tier subscriptions today are billed off-chain via PortOne, but **reward payouts** today flow through on-chain claim — an off-chain reward cashout is on the roadmap.

### On-chain settlement
The default reward payout flow. Past cycles' Tokens are claimed by connecting a wallet, asking Ratel for a signed claim payload (amount, deadline, nonce, contract address, chain id), and submitting the on-chain transaction from your wallet to receive the tokens.

### Report Revenue Share
**(Coming soon)** The split applied to every paid Report sale: **10% platform · 60% host · 30% contributors**. The contributor share is distributed by relevance — how much each participant's activity shaped the final Report.

---

## Identity & Teams

### Profile / Handle
Your public identity on Ratel. The handle is your unique name (e.g. `@yourname`); the profile lives at `/your-handle` and shows your Posts, Spaces, and (eventually) your Essence House.

### DID (Decentralized Identifier)
A user-controlled cryptographic identity that lets you prove who you are without depending on a single platform. Used for verified Posts, signed Votes, and on-chain settlement.

### Credential
A verifiable claim attached to your identity — a Team membership, a certification, a verified attribute. Credentials can gate access to Spaces or unlock features.

### Character
A configurable persona you present in different contexts (work, public discussion, anonymous feedback). One user can have multiple Characters tied to the same Essence.

### Team
A group of users who collaborate, share Posts, and host Spaces under a shared handle. Teams have their own profile and can run member-only or open Spaces.

### Sub-team
A scoped group inside a Team — a working group, a chapter, a project crew — with its own permissions, bylaws, docs, and (optionally) its own Spaces.

### DAO
A Team configured for collective on-chain governance. Voting rights, treasuries, and proposals are managed through the Team's Space Actions and (when enabled) on-chain settlement.

### Membership Tier
The plan a user is on: **Free**, **Pro**, **Max**, **Vip**, **Enterprise**. Higher tiers unlock larger Spaces, more Essence Connectors, agent slots, and higher reward limits.

### Trusted Creator badge
A profile and Spaces badge granted to Max / VIP tier hosts. Surfaces visual trust signals to participants when they enter your Space. *(Profile and Space rendering of the badge is Coming soon — Phase 1; the entitlement is listed on the tier card today.)*
