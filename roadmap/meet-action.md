# Meet Action

**Status**: Ready for design (Stage 2)
**Slug**: `meet-action`
**Sibling actions**: Poll, Quiz, Discussion, Follow (Meet joins this family)

## Problem

Ratel spaces have asynchronous actions (Poll, Quiz, Discussion, Follow), but no way to hold a **synchronous, real-time meeting**. Most communities that would adopt Ratel — university clubs, departments, DAOs, internal teams — regularly run live meetings, lectures, and seminars.

Today those meetings happen off-Ratel (Google Meet, Zoom, Discord). The content disappears or lives in a third-party tool. The meeting is disconnected from the space's Essence, rewards, activity tracking, and sub-team governance. Organizations piloting Ratel still need a second tool for the single most common collective activity: getting everyone in the same room.

## Goal

Ship **Meet** as a first-class space action that lets a space admin schedule or instantly start a video meeting any participant can join, with recording and transcription automatic and archived on the same Meet page — all inside Ratel.

## Non-goals

Explicitly out of scope for Phase 1:

- **No external-only participants.** No public link joins, no outside-the-space guest invites. Access follows the space's participant model.
- **No recurring / series meetings.** Single one-off meetings only.
- **No advanced in-meeting features.** No breakout rooms, whiteboards, in-meeting polls, live captions on screen.
- **No AI-generated summaries.** Planned for Phase 2.
- **No mobile-first experience.** Web desktop is the primary surface for Phase 1; mobile integration is Phase 2.
- **No host-role delegation** beyond space admin role. Host privileges are always tied to the space admin role.
- **No per-user notification preferences.** Everyone gets all four notifications via both channels in Phase 1.
- **No per-speaker transcript opt-out** or recording redaction. Both Phase 2.
- **No recording editing.** Recordings are immutable once produced.
- **No cost guardrails** (per-space minute caps, etc.). Accepted as part of promotional / early-adoption phase; guardrails arrive as paid-tier features later.

## User stories

### Space admin (meeting host)

- As a space admin, I want to **schedule a Meet action** for a future time so members know when we'll gather live.
- As a space admin, I want to **start a Meet instantly** ("meet now") when something urgent comes up.
- As a space admin, I want members to **add the meeting to their Google Calendar with one click** so they don't miss it.
- As a space admin (= host), I want to **mute or remove a disruptive participant** during the meeting.
- As a space admin, I want to **end the meeting for everyone** when the business is done.
- As a space admin, I want to **set a credit reward** for attending so members have an incentive to show up.
- As a space admin, I want to **see the recording, transcript, and chat log** after the meeting.
- As a space admin of a paid-tier space, I want to **toggle "Include meetings in Space Essence"** so subscribers of the space's Essence House can query meeting content.

### Space participant

- As a space participant, I want to be **notified** when a Meet is scheduled, 10 minutes before, when it goes live, and when the recording is available — by both in-app and email.
- As a space participant, I want to **add the meeting to my calendar** with one click.
- As a space participant, I want to **see the participant list, chat, raise my hand, and react** during the meeting.
- As a space participant who **missed the live meeting**, I want to view the recording, transcript, and chat log afterward.
- As a space participant who attended for **at least 1 minute**, I want the attendee reward to land in my wallet automatically.

### Non-participant (public, post-space)

- As someone who was not a space participant, after the space ends, I want to view the Meet recording / transcript / chat log (same visibility rule as every other space action).

## Functional requirements

### FR-1: Meet as a space action

1. Meet MUST appear as a space action type alongside Poll, Quiz, Discussion, and Follow.
2. Meet actions MUST be creatable only by **space admins** (same permission as other space actions).
3. A space MAY contain zero or more Meet actions.
4. Every Meet action MUST transition through these states: `Scheduled` → `Live` → `Ended`. Cancel path: `Scheduled` → `Cancelled`. No-show path: `Scheduled` → `Expired`.

### FR-2: Creation & scheduling

5. On create, the admin MUST supply: **title**, optional description, start mode (**Scheduled** vs **Instant**), and (for Scheduled) start time.
6. **Estimated duration** defaults to 60 minutes, editable up to the platform ceiling (24 hours).
7. On "Instant" → Meet immediately transitions to `Live`.
8. On "Scheduled" → Meet stays in `Scheduled` until the chosen time.
9. Reward fields (host amount, attendee amount) are **optional**. Fields are disabled for free-tier spaces (existing Ratel pattern for reward-bearing actions).
10. "Include meetings in Space Essence" is a **space-level** toggle (not per-meeting); see FR-10.

### FR-3: Access & participation

11. While the parent space is active, **only space participants** MAY join a Live meeting or view the Meet action page.
12. When the parent space is ended, the Meet action (recording, transcript, chat log) becomes **visible to any logged-in Ratel user**, same as other space actions.
13. Maximum concurrent attendees per meeting = **250** (platform ceiling).
14. Participants MAY leave and re-join freely during the Live phase.

### FR-4: Calendar integration

15. The Meet action page MUST offer an **"Add to calendar"** capability with two options:
    - **ICS file download** (works for any calendar app).
    - **One-click Google Calendar add** via OAuth.
16. Google Calendar OAuth MUST request the **minimum scope** for event creation (`calendar.events.insert` or equivalent). No read scope is requested.
17. Calendar sync is **one-way** (Ratel → Google Calendar). Changes in Google Calendar do NOT sync back.
18. The calendar event MUST contain: title (with "· Ratel" suffix), start/end timestamps, description with agenda text and meeting URL, and the Ratel join link.

### FR-5: Notifications

19. Every space participant MUST receive **both in-app and email** notifications at **four events**:
    - (a) When the Meet is scheduled (or, for Instant meetings, at creation).
    - (b) **10 minutes before** scheduled start.
    - (c) When the meeting actually goes **Live**.
    - (d) When the **recording is available** after the meeting ends.
20. For Instant meetings, events (a) and (c) collapse into a single "live now" notification.
21. Email notifications MUST include a direct deep link to the Meet action page.
22. **No per-user opt-out** in Phase 1. Every space participant receives every notification.

### FR-6: In-meeting features (all participants)

23. Every participant MUST have controls to:
    - Toggle their own **video** (on/off).
    - Toggle their own **microphone** (mute/unmute).
    - **Share their screen**.
    - See the **participant list** with current status (mic/video on, hand raised).
    - Post in a **text chat**.
    - **Raise / lower hand**.
    - Post **emoji reactions**.
    - **Leave** the meeting.
24. The UI MUST display a persistent **"recording" indicator** throughout the Live phase.
25. Screen share, chat, raise-hand, and reactions MUST reach all current attendees in real time.

### FR-7: Host controls (space admins)

26. Any space admin who has joined the Live meeting MUST be able to:
    - **Mute** another participant's microphone.
    - **Remove** another participant from the meeting.
    - **End for all** with a confirmation dialog.
27. If no space admin is currently in the meeting, **no moderation controls are available** (the meeting runs without a moderator).
28. Mute and remove actions MUST be recorded in a **moderation log** visible on the post-meeting page (who did what to whom, when).
29. There is no concept of "handing over host role"; host privileges derive from space admin membership only.

### FR-8: Recording

30. Every meeting MUST be **automatically recorded** from transition to Live until it ends.
31. Recordings MUST be **retained permanently by default**. (Retention settings are not user-configurable in Phase 1.)
32. Recording files MUST be reachable only through Ratel's authenticated surfaces — **no public / direct-storage URLs**.

### FR-9: Transcription

33. Every recording MUST be transcribed with **speaker diarization** (speaker labels per segment).
34. The transcript MUST be **searchable within the Meet page** and navigable (clicking a timestamp jumps the video).

### FR-10: Chat & transcript persistence

35. The in-meeting text chat MUST be **persisted** and rendered on the post-meeting page in timestamp order.
36. Participant join/leave timestamps MUST be persisted and rendered as an aggregated **participant list** on the post-meeting page ("Alice · 42 min · joined 2x").

### FR-11: Lifecycle page UX

37. The Meet action page MUST render differently based on state, but **URL stays the same**:
    - `Scheduled`: title, description, start time, Add-to-calendar controls, participant list so far, "Join" button (active from N minutes before start).
    - `Live`: embedded video call UI with all in-meeting controls.
    - `Ended`: recording player, speaker-diarized transcript, chat log, participant list with times, moderation log.
    - `Cancelled`: cancellation notice with reason (if provided).
    - `Expired`: expiration notice; no recording/transcript/chat.

### FR-12: Cancellation & expiration

38. A space admin MAY **cancel** a `Scheduled` meeting at any time before it goes Live.
39. On cancellation, every space participant MUST receive an in-app + email notification.
40. A `Scheduled` meeting that is never started by the host within **24 hours of its scheduled start time** auto-transitions to `Expired`.
41. `Expired` and `Cancelled` meetings have no recording, no transcript, and no chat log.

### FR-13: Rewards

42. If the **host reward** is configured, the host receives it when the Live phase lasted **at least 1 minute**.
43. If the **attendee reward** is configured, each attendee with at least **1 minute of total presence** (server-side timestamps) receives the full reward. No proportional payout.
44. Rewards are distributed **automatically** when the meeting transitions from Live to Ended.
45. Reward amount inputs are **disabled for free-tier spaces** (existing Ratel reward pattern).
46. The 1-minute floor is the Phase-1 abuse-detection baseline; more sophisticated detection is a separate future roadmap item.

### FR-14: Essence integration

47. Each space MUST expose a single **"Include meetings in Space Essence"** toggle to its admins.
48. The toggle defaults to **OFF**.
49. When ON, every **subsequent** meeting's transcript flows into the **space-shared Essence pool** — not the individual host's personal Essence.
50. The toggle affects only future meetings; meetings that happened before the toggle was flipped are not retroactively included or removed.
51. When the toggle is ON, the Meet page (both pre-join and in-meeting) MUST display the notice: *"This meeting is transcribed and included in [Space name] Essence."*
52. The toggle is **disabled for free-tier spaces** (paid-tier feature).

## Acceptance criteria

Two testers — one as space admin, one as a space participant — can complete this end-to-end inside Ratel without leaving the app:

- [ ] AC-1: Admin schedules a Meet action with a future start time.
- [ ] AC-2: Every space participant gets in-app + email notifications at scheduling.
- [ ] AC-3: Participant clicks "Add to Google Calendar", completes OAuth (first time), and the event appears on their Google Calendar with correct title/time/link.
- [ ] AC-4: Participant also gets the ICS download option.
- [ ] AC-5: 10 minutes before start, all participants get the reminder (in-app + email).
- [ ] AC-6: Admin starts the meeting; all participants get the "live now" notification (in-app + email).
- [ ] AC-7: Participant joins; sees video/mic/screen-share/chat/raise-hand/reactions controls; recording indicator visible.
- [ ] AC-8: Admin mutes another participant; that participant's mic is forced off.
- [ ] AC-9: Admin removes another participant; that participant returns to the Meet page.
- [ ] AC-10: Admin clicks "End for all" with confirmation; meeting transitions to Ended for everyone.
- [ ] AC-11: Participant with ≥1 min presence receives the attendee reward automatically.
- [ ] AC-12: Host receives host reward given the meeting ran ≥1 min.
- [ ] AC-13: Within a few minutes post-meeting, the page shows recording, speaker-diarized transcript, chat log, and moderation log.
- [ ] AC-14: Clicking a transcript timestamp jumps the video player to that spot.
- [ ] AC-15: A participant who missed the live session opens the Meet page and views recording/transcript/chat.
- [ ] AC-16: While the parent space is active, a non-participant cannot access the Meet action page or recording.
- [ ] AC-17: After the parent space ends, any logged-in user can access the Meet action page, recording, transcript, and chat.
- [ ] AC-18: Admin creates an "Instant" Meet; it is Live immediately and participants get the live-now notification.
- [ ] AC-19: A scheduled Meet that is never started auto-transitions to Expired 24 hours past its start time; no recording exists.
- [ ] AC-20: Admin cancels a scheduled Meet; participants receive cancellation notification.
- [ ] AC-21: Admin toggles "Include meetings in Space Essence" ON; the next meeting's transcript appears in space Essence; a notice is visible in that meeting's UI.
- [ ] AC-22: Past meetings are not affected when the Essence toggle is flipped.
- [ ] AC-23: For a free-tier space, reward fields and the Essence toggle are visibly disabled.

## Constraints

- **Hosting platform**: AWS Chime SDK. Imposes ceilings that become product constraints: 250 max concurrent attendees, 24-hour max meeting duration.
- **Cost model**: Per-attendee-minute billing plus recording/transcription fees. **No guardrails** on space-level consumption in Phase 1 (accepted trade-off for growth phase; gating moves to paid tier later).
- **Privacy**:
  - Recordings and transcripts are never exposed via public / direct-storage URLs. Every access goes through Ratel's authenticated surface.
  - Participants are informed at join time that the meeting is recorded and (if applicable) included in Essence.
  - Speaker diarization enables per-speaker redaction in Phase 2; Phase 1 has no redaction.
- **Ecosystem alignment**:
  - Meet follows the same space-lifecycle permission model as Poll / Quiz / Discussion / Follow.
  - Meet reuses existing reward distribution pipeline.
  - Meet reuses existing notification (inbox) and email delivery pipelines.
  - Meet reuses existing Essence ingestion pipeline (recording/transcript as an EssenceSource).
- **Backward compatibility**: Existing space actions (Poll, Quiz, Discussion, Follow) are unaffected. Meet is additive.

## Open questions

- OQ-1: *Default state of the "Include meetings in Space Essence" toggle at space creation?* — Proposed: always OFF; admin turns on later.
- OQ-2: *Do we expose per-participant detailed event timelines (join/leave/mute events)?* — Proposed: no; only aggregate totals per participant. Moderation log separate.
- OQ-3: *If two space admins both click "End for all" simultaneously, does the second click error or no-op?* — Proposed: idempotent no-op.
- OQ-4: *Should Expired meetings stay visible in the space action list, or auto-hide after a period?* — Proposed: keep visible with "Expired" badge; admin may delete manually.
- OQ-5: *When the parent space ends and a non-participant views the recording, do they need a Ratel account?* — Proposed: yes, require login; no space participation needed.
- OQ-6: *When a participant drops off due to network issues and re-joins, does each re-join count as a separate attendance record for reward purposes?* — Proposed: total cumulative presence ≥1 min qualifies (the 1-minute floor is cumulative, not per-session).

## References

- User PO session establishing 17 design decisions (this branch's discussion).
- `.claude/rules/workflows/roadmap-elaboration.md` — spec template followed.
- `.claude/rules/conventions/implementing-event-bridge.md` — expected Stage 3 integration point for recording/transcript event flow.
- Existing space action implementations under `app/ratel/src/features/spaces/pages/actions/` — permission/reward patterns to mirror.
- AWS Chime SDK documentation — technology basis for Constraints section.
- `roadmap/sub-team.md` — adjacent roadmap item; department-office ↔ club use case will heavily use Meet action.
