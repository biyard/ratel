---
sidebar_position: 1
title: Essence
---

# Essence

Your **Essence** is a personal knowledge base that grows from the things you do on Ratel — every post you publish, every comment you write, every poll or quiz you host, every action you complete in a Space. The platform turns each of these into an **EssenceSource**: a structured record indexed for retrieval, attributed to your account, and (later) embedded into a vector representation that an AI assistant can query.

Two scoring signals shape how your Essence is read by the platform and, eventually, by the marketplace:

- **Quality Score** — a measure of how *good* the contribution is (depth, originality, peer engagement). Polls and follows score lower than a long, well-formed discussion reply.
- **Direct-Activity Index** — a measure of how *much* of your time the platform sees as direct, intentional contribution rather than passive presence. Writing, voting, and discussing weigh heavier than scrolling.

Both feed into where your Essence ranks for retrieval, and (in Phase 2) what subscribers to your Essence House get when they query it.

A few principles worth knowing up front:

- **Your Essence is yours.** Every source has a Delete control; removing a source from your Essence index removes it from inference too. The original artifact (the post, the comment) stays put on the platform; only the Essence indexing of it is dropped.
- **Inference access only.** When the Essence House marketplace ships *(Coming soon)*, subscribers buy *inference access* — they can query your Essence, never download its raw contents.
- **AI-authored content is opt-in.** *(Coming soon)* AI-generated text is excluded from your Essence by default; you opt in per source if you want AI-assisted writing to feed it.

## In this chapter

| Page | What it covers |
|---|---|
| **[My Essence management](./my-essence.md)** | The two surfaces where your Essence becomes tangible — **My AI** at `/my-ai` (your MCP endpoint and connected-client setup) and **Character** at `/me/character` (XP, skill points, and skill tree). |
| **[Essence sources](./sources.md)** | The `/essence` control panel — every input feeding your Essence, with search, filters by source kind (Posts / Comments / Polls / Quizzes / Notion *(Coming soon)*), per-row Delete, and bulk-remove on the roadmap. |
