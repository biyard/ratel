---
sidebar_position: 3
title: Space Actions
---

import useBaseUrl from '@docusaurus/useBaseUrl';

# Space Actions

If [Apps](/spaces/apps) are the host's toolkit, **Actions** are the participant's quest board. Each Action is a single thing the host wants you to do — vote on something, weigh in on a discussion, answer a quiz, follow a few accounts, RSVP to a meet — and finishing it earns you a share of the Space's rewards while feeding your Essence with structured signal about what you think.

## The action carousel

URL:

```
/spaces/:space_id/actions
```

The Actions page is a **carousel** of quest cards. Each card is a single action; you scroll horizontally and the card centered on the screen is the active one — bigger, brighter, in focus. Cards on either side dim and shrink, so you always know what's next without the carousel becoming overwhelming.

Each card shows:

- The action **type** (poll, discussion, quiz, follow, meet)
- A short headline summarizing what's being asked
- The reward attached to the action (if the Space has an [Incentive Pool](./apps.md#-incentive-pool-beta) configured)
- Whether you've already participated, and if so, what you did

Tap a card to open the action's full page. Once you complete it, you'll come back to the carousel and see the next one in sequence.

:::tip Mobile-friendly
The carousel is designed to feel like flipping through cards on your phone — scroll-snap, no fiddly precision required. It's the fastest way to see, at a glance, exactly what you can do in a Space right now.
:::

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><path d="M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"/></svg> Discussion

URL: `/spaces/:space_id/actions/discussions/:discussion_id/edit`

A **Discussion** action invites you to contribute your opinion to a topic the host has posed. You'll see the prompt, the rules of engagement, and a rich text editor where you can write your reply.

Discussions are intentionally open-ended — they're the Space's way of asking *"what do you think?"* in a structured way that the host can later analyze, quote, and (if the Space has reports enabled) include in the published narrative. Your reply is attributed to your account, which means:

- It contributes to your contribution record on any report the host generates from this Space.
- It feeds your Essence with a clear signal about your stance on the topic.
- It earns you the discussion's reward share if the Space pays for participation.

You can read other participants' contributions while you write yours, but the host decides whether replies are visible immediately or only after you've posted your own — so a Space designed to gather independent opinions doesn't get anchored by whoever speaks first.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/discussion.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/discussion.mov')}>Download the walkthrough</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><polyline points="20 6 9 17 4 12"/></svg> Poll

URL: `/spaces/:space_id/actions/polls/:poll_id`

A **Poll** action is a vote. You'll see the question, the available options, and (depending on how the host configured it) the live tally of how everyone else has voted.

Polls are the lightest-touch way to participate — one tap and you're in. Despite the simplicity, they're the workhorse of most Spaces: they're fast for participants, easy to design, and aggregate cleanly into reports.

After you vote, you'll typically see the live results immediately. Some Spaces hide the tally until a deadline passes (so the result isn't biased by whoever votes first); the host configures that per poll. Either way, your vote is recorded against your account and counts toward your reward share.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/poll.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/poll.mov')}>Download the walkthrough</a>.
</video>

## <img src={useBaseUrl('/img/icons/file.svg')} width="22" height="22" alt="File" style={{verticalAlign: 'middle'}} /> Quiz

URL: `/spaces/:space_id/actions/quizzes/:quiz_id`

A **Quiz** action asks you a series of questions with a correct answer (or a passing threshold). Use cases include onboarding gates, certification flows, knowledge-check assessments, and skill verification before a participant unlocks deeper participation in a Space.

Quizzes are scored, and your score determines whether you've **passed**. The host sets the threshold — sometimes it's a hard pass/fail, sometimes a percentage, sometimes a minimum number of correct answers. Pass and the reward unlocks; fail and you may be allowed to retry, depending on the host's configuration.

Quiz results are useful inputs to your Essence too — they signal what you know, not just what you think.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/quiz.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/quiz.mov')}>Download the walkthrough</a>.
</video>

## <img src={useBaseUrl('/img/icons/users.svg')} width="22" height="22" alt="Users" style={{verticalAlign: 'middle'}} /> Follow

URL: `/spaces/:space_id/actions/follows/:follow_id`

A **Follow** action asks you to follow a specific account or set of accounts. Hosts use these to grow a related creator's audience, build a connected sub-community, or seed your feed with voices the Space considers important.

Each follow card shows you the target accounts, who they are, and why the host is suggesting them. You decide which to follow — you're not obligated to follow all of them — and the action is marked complete once you've followed enough to satisfy the host's threshold.

Follow actions feed Essence in a different way from polls or discussions: they tell your Essence *who* you align with, which helps recommendations and routing across the platform.

<video controls preload="metadata" width="100%" src={useBaseUrl('/media/follow.mov')} style={{borderRadius: '8px', border: '1px solid var(--ratel-line-soft)', maxWidth: '720px', display: 'block', margin: '1rem 0'}}>
  Your browser doesn't support embedded video. <a href={useBaseUrl('/media/follow.mov')}>Download the walkthrough</a>.
</video>

## <svg width="22" height="22" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" style={{verticalAlign: 'middle'}}><rect x="3" y="4" width="18" height="18" rx="2" ry="2"/><line x1="16" y1="2" x2="16" y2="6"/><line x1="8" y1="2" x2="8" y2="6"/><line x1="3" y1="10" x2="21" y2="10"/></svg> Meet

URL: `/spaces/:space_id/actions/meets/:meet_id`

A **Meet** action is a scheduled event — a livestream, a video call, a workshop, an in-person gathering, or a recurring office hours block. The card shows you the date, time, location (or link), and any agenda the host has attached.

You can RSVP from the action page, see who else is attending, and (if the host enables it) get reminders before the meet starts. Attendance — when verifiable — counts toward the action's completion and unlocks the associated reward.

Meets are the one action type that connects digital participation to time-based engagement, so they're particularly useful for Spaces that want to anchor their community around real-time gatherings rather than purely asynchronous activity.

## How action rewards work

Two things happen when you complete an action:

### 1. Incentive Pool distribution

If the Space has an Incentive Pool funded, your share is drawn from it according to the host's distribution rules:

- Some pools split rewards evenly across all completers.
- Others weight by depth (a long, well-formed discussion reply earns more than a one-line vote).
- Others stage rewards across phases of the Space.

Either way, you can see your accumulated reward from this Space on your profile, and host-side, the pool's distribution history is fully auditable.

### 2. Essence accumulation

Every action you complete becomes an **EssenceSource** — a structured record of what you participated in and what you contributed. Your votes, comments, follow choices, quiz answers, and meet RSVPs all flow into your personal Essence, ready to be embedded once Phase 0's pipeline is live.

:::note Coming soon
The full Phase 0 Essence pipeline — embedding every action into your personal knowledge base for query, agent participation, and Essence House subscriptions — is on the way. Until it ships, your activity is recorded as it happens; the embedding layer is added retroactively, so nothing you do today is lost.
:::

The result: doing things in Spaces isn't just rewarded once at the moment of completion. Every action also makes your Essence richer, more queryable, and (eventually) more valuable to anyone who subscribes to your Essence House.
