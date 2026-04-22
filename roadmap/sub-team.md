# Sub-team Governance

**Status**: Ready for design (Stage 2)
**Slug**: `sub-team`
**Primary use case**: University department office ↔ student clubs

## Problem

Teams on Ratel are currently flat — every team is an island. Institutions that naturally have a **parent ↔ child** structure (university departments and their clubs, companies and their divisions, federations and their chapters) have no way to express that relationship on Ratel.

Concretely for the MVP use case: a university **department office** (학과사무실) wants to:
- Publish its club registration rules (학칙) in one place so every prospective club follows the same process.
- Require a prospective club to gather enough members on Ratel before accepting it as an officially recognized club.
- Send department-wide announcements to every recognized club in one action.
- Observe how active each recognized club is, so the department can keep its club list current.

Today each of these has to be done out-of-band (Google Docs, email, phone) and nothing about the department ↔ club relationship is visible on Ratel. As a result, departments have no reason to adopt Ratel, and students have no reason to register their club officially on Ratel.

## Goal

Give any Ratel team a first-class way to govern its sub-teams through **registration review, regulation publication, broadcast announcements, and activity observation** — while leaving day-to-day operation of each sub-team fully independent.

## Non-goals

- **No supervisory control over sub-team content.** Parent teams cannot edit, delete, or moderate sub-team posts/members/spaces.
- **No forced dissolution.** Parent teams cannot delete a sub-team. They can only terminate the sub-team relationship ("deregister"); the sub-team itself continues as an independent team.
- **No N-level nesting in this release.** Only two levels (parent ↔ child). Multi-level hierarchies (e.g., College ↔ Department ↔ Club) are Phase 2.
- **No multi-parent sub-teams.** Each sub-team belongs to at most one parent team.
- **No internal governance tooling for sub-teams** (elections, quorum, voting on bylaws). Sub-teams handle their internal governance however they want.
- **No external identity verification.** We don't verify a "department" is actually a real university department; it's a regular Ratel team that happens to accept sub-team applications.
- **No academic-calendar automation** (semester rollover, graduation-triggered member cleanup). Sub-teams manage their own member roster.
- **No financial / rewards integration in Phase 1.** Budget tracking, reward propagation from parent to sub-team, etc. are out of scope.

## User stories

### Department office (parent team admin)

- As a department office admin, I want to **publish registration rules (학칙)** as regular posts in a "Bylaws" category so prospective clubs know what's required of them.
- As a department office admin, I want to **customize the application form** so applicants provide exactly the fields our department needs (club name, purpose, minimum members, advisor, etc.).
- As a department office admin, I want to **review and approve/reject/return** each club application so only clubs that meet our criteria become recognized.
- As a department office admin, I want to **send announcements to every recognized club at once** so I don't have to repeat the same notice N times.
- As a department office admin, I want to **answer follow-up questions** on each announcement inline.
- As a department office admin, I want to **see an activity dashboard** of each recognized club (posts, spaces, active members) so I can identify inactive clubs and decide whether to keep them recognized.
- As a department office admin, I want to **deregister** a club (end the parent-child relationship) without deleting the club itself, if it no longer meets our standards.

### Club founder (prospective sub-team admin)

- As a student founding a club, I want to **create a pending club team** first so I have a place to gather potential members before applying for official recognition.
- As a student founding a club, I want to **read the department's bylaws and agree to them** before submitting the application.
- As a student founding a club, I want to **invite members to the pending team** so we can demonstrate minimum member count.
- As a student founding a club, I want to **submit the application only after reaching the minimum member threshold** so I don't waste the department's review time.
- As a student founding a club, I want to **re-submit after a rejection** without a cooldown period, fixing whatever was flagged.

### Club admin (recognized sub-team admin)

- As a recognized club admin, I want to **receive department announcements in-app** so my members and I don't miss notices.
- As a recognized club admin, I want to **ask follow-up questions** on announcements so I can clarify on behalf of my members.
- As a recognized club admin, I want to **publish our internal club bylaws (회칙)** in the same bylaws mechanism the department uses, so everything is consistent.
- As a recognized club admin, I want to **leave the parent team** if we no longer want to be under that department, and become an independent team.

### Club member

- As a club member, I want to **receive in-app notifications** for department announcements targeted at my club.
- As a club member, I want to **be informed at join time** that my public activity in this club contributes to an activity dashboard visible to the department.

## Functional requirements

### FR-1: Parent-child team relationship

1. A team MAY have at most one **parent team**.
2. A team MAY have zero or more **sub-teams**.
3. A team MUST NOT be a sub-team of itself or of any of its descendants (no cycles).
4. The parent-child relationship is **visible on both teams' public profile** (e.g., "Sub-team of Seoul Nat'l CS Department", and conversely "3 sub-teams").
5. A team that accepts sub-team applications is called a **parent-eligible team**. Any team can flip to parent-eligible via a setting.

### FR-2: Bylaws as a post category

6. Any team MAY mark specific posts as belonging to the **"Bylaws" category**.
7. When viewing a team's page, bylaws posts MUST appear in a dedicated **"Bylaws" section** separate from the regular feed.
8. A sub-team's page MUST display a **link to its parent team's bylaws** alongside its own bylaws.
9. A parent team MAY update bylaws posts at any time; updates do **not** retroactively require re-agreement from already-approved sub-teams.
10. Bylaws posts follow the normal post lifecycle (edit, delete, version history via post edit trail).

### FR-3: Application form customization

11. A parent-eligible team's admin MUST be able to **define a custom application form** — a list of fields that prospective sub-teams must fill in.
12. Each field has a type: **short text, long text, number, date, single select, multi select, URL**.
13. Each field has a flag: **required / optional**.
14. The form MUST include by default: **proposed team name, purpose (long text)**.
15. The parent team MAY add any additional fields (e.g., *"Faculty advisor name"*, *"Minimum members required"*, *"Meeting frequency"*).
16. Form updates do **not** retroactively affect in-flight applications; each application is locked to the form version at submission time.

### FR-4: Registration flow

17. A user MUST be able to create a **pending sub-team** (a team with `status: Pending` + designated parent candidate).
18. The pending sub-team MUST let its admin invite members via normal Ratel team invitation flow.
19. The pending sub-team's admin MUST explicitly **acknowledge the parent team's bylaws** before the application can be submitted.
20. The pending sub-team's admin MUST fill in **every required field** of the parent team's application form before submission.
21. The parent team MUST set a **minimum member count** for applications. The pending sub-team MUST meet this count before the "Submit application" button is enabled.
22. On submission, the parent team's **admins** (only) MUST receive a notification and see the application in a **pending applications queue**.
23. A parent team admin MUST be able to take one of three actions on an application: **Approve**, **Reject with reason**, **Return for revision with comment**.
24. On **Approve** → the pending sub-team's `status` becomes `Active`, its `parent_team_id` is set, and its admin receives a "Approved" notification.
25. On **Reject** → the sub-team remains as a regular standalone team (no parent), and its admin receives the rejection reason.
26. On **Return for revision** → the sub-team admin can edit the form and resubmit. No cooldown.
27. The sub-team admin MUST be able to re-submit to the same parent team or to a different parent team after rejection, with no cooldown.

### FR-5: Announcement broadcast

28. A parent team admin MUST be able to publish an **announcement** that targets **all its recognized sub-teams by default**.
29. *(Phase 1.5)* The admin MAY select a subset of sub-teams as the target. [**Out of scope for Phase 1 MVP; Phase 1.5 enhancement.**]
30. On publish, the system MUST create a **pinned post** in each target sub-team's team space, authored by the parent team.
31. Only **the most recent** parent-team announcement in each sub-team is auto-pinned; earlier announcements demote to regular posts on the next publish.
32. Every **member** of each target sub-team MUST receive an in-app notification for the new announcement.
33. The announcement post MUST support a **standard comment thread** for Q&A; both the parent team author and any sub-team member can post comments.
34. The parent team author MUST receive notifications for new comments on their announcement.

### FR-6: Activity observation (dashboard)

35. A parent team admin MUST be able to access an **activity dashboard** for each of its recognized sub-teams.
36. The dashboard MUST show three metrics per sub-team:
    - **Post count** (sum of posts authored in that sub-team's space)
    - **Space count** (number of spaces owned by that sub-team)
    - **Active member count** (members with at least one public action in the period)
37. The dashboard MUST support three time windows: **daily, weekly, monthly**.
38. The dashboard MUST offer a **drill-down** per sub-team showing per-member activity: `@handle | posts | spaces participated | last active date`.
39. The dashboard MUST only count **Public and Team-Shared** activity. **Private** posts/spaces MUST NOT appear or be counted.
40. The dashboard MUST display a fixed notice: *"This dashboard reflects public and team-shared activity only. Private posts and messages are never included."*
41. At join time, each sub-team member MUST be shown a notice: *"This team's public activity is aggregated in [Parent team name]'s activity dashboard."*
42. Member handle visibility in the dashboard is **always on**; there is no anonymization mode in Phase 1.

### FR-7: Parent-side termination (deregister)

43. A parent team admin MAY **deregister** any of its recognized sub-teams with a **written reason**.
44. On deregister:
    - The sub-team's `parent_team_id` is cleared (sub-team becomes an independent standalone team).
    - The sub-team's admin receives a notification including the deregistration reason.
    - Prior parent-team announcements remain as regular (unpinned) posts in the sub-team's space.
    - The sub-team's members are **not** removed.
45. The parent team MAY NOT delete, edit, or silently remove the sub-team's content at any point.

### FR-8: Child-side departure (leave parent)

46. A sub-team admin MAY unilaterally **leave the parent team** without parent approval.
47. On leave:
    - The sub-team's `parent_team_id` is cleared.
    - The parent team admins receive a notification that the sub-team has left, with an optional reason.
    - Prior parent-team announcements remain as regular (unpinned) posts.
48. After leaving, the sub-team is a fully independent standalone team. To re-attach to any parent (same or different), the normal application flow applies.

### FR-9: Parent team deletion

49. If a parent team is deleted, every recognized sub-team of that parent MUST become an independent standalone team (its `parent_team_id` is cleared).
50. Sub-team content MUST NOT be affected in any way.

### FR-10: Permissions summary

51. **Parent team admin** CAN: publish bylaws, set application form, approve/reject/return applications, publish broadcasts, view activity dashboard, deregister sub-teams.
52. **Parent team admin** CANNOT: edit sub-team content, manage sub-team members, delete sub-team, change sub-team settings, view private sub-team activity.
53. **Sub-team admin** CAN: everything any team admin normally can + acknowledge bylaws + submit applications + leave parent team.
54. Parent team membership does **not** automatically grant any role in sub-teams, and vice-versa.

## Acceptance criteria

The feature is shippable when a tester, acting as a department office admin and a separate tester acting as a club founder, can complete this scenario end-to-end without leaving Ratel:

- [ ] AC-1: Department admin publishes a bylaw post; it appears in the department's "Bylaws" section.
- [ ] AC-2: Department admin configures the application form with at least one custom required field (e.g., "Faculty advisor").
- [ ] AC-3: Club founder creates a pending club team that lists the department as its parent candidate.
- [ ] AC-4: Club founder invites and gains members; member count updates on the application page.
- [ ] AC-5: Club founder cannot click "Submit application" until the minimum member threshold is met.
- [ ] AC-6: Club founder must check "I agree to the bylaws" before the submit button enables.
- [ ] AC-7: Department admin sees the new application in the pending queue with the full form values.
- [ ] AC-8: Department admin returns the application with a comment; the club founder sees the comment and can resubmit.
- [ ] AC-9: Department admin approves the re-submitted application; the club's status flips to recognized; club founder receives an approval notification.
- [ ] AC-10: Department admin publishes an announcement targeting all recognized clubs; the announcement appears as a pinned post in the club's space.
- [ ] AC-11: Every club member receives an in-app notification for the announcement.
- [ ] AC-12: A club member comments on the announcement with a question; the department admin receives a notification and replies in the same thread.
- [ ] AC-13: Department admin opens the activity dashboard and sees the club's posts / spaces / active-member counts for the last week and the last month.
- [ ] AC-14: Department admin drills into a specific club and sees per-member activity (handle + post count + last active).
- [ ] AC-15: A private post in the club's space is not reflected in the dashboard counts.
- [ ] AC-16: Department admin deregisters the club with a written reason; the club becomes a standalone team; the admin receives a deregistration notification; content is unchanged.
- [ ] AC-17: A recognized club's admin leaves the parent; the department admin is notified; the club becomes standalone; content is unchanged.
- [ ] AC-18: Deleting a parent team converts every recognized sub-team into a standalone team; sub-team content is untouched.
- [ ] AC-19: Member-join screen of a recognized sub-team displays the notice about parent-team activity visibility.
- [ ] AC-20: The activity dashboard displays the "public activity only" notice at all times.

## Constraints

- **Privacy boundary is absolute.** Private posts, draft posts, DMs, and any non-public signal MUST NOT leak into the parent team's dashboard or any other parent-facing surface.
- **Notification volume.** Announcement broadcast to N sub-teams × M members must not degrade the notification inbox experience. Batching and per-team pinning preferred over per-member direct push.
- **No new primary entity categories where a post can serve.** Bylaws are posts. Announcements are posts. Applications are forms, and can be modeled as a dedicated entity — but should reuse commenting/notification infrastructure.
- **Backward compatibility.** Existing teams continue to work exactly as today. Parent/child behavior is opt-in via settings, not forced.

## Open questions

- OQ-1: *Who can author bylaws posts for a parent team?* Default: team admins only. Does the department want non-admin "secretaries" to be able to edit bylaws?
- OQ-2: *Does the activity dashboard expose space names or just counts?* Proposed default: **counts only on the overview, names only on drill-down**, and only for public/team-shared spaces.
- OQ-3: *How long is the "pending" state valid before it auto-expires?* Proposed default: no auto-expiry in Phase 1. Sub-team admin can cancel manually.
- OQ-4: *Should a sub-team see the department's draft announcements before publish?* Proposed default: no — drafts are private to the author.
- OQ-5: *Should the application form support file uploads (e.g., "constitution PDF")?* Proposed default: **no uploads in Phase 1** — Phase 2 addition.

## References

- User conversation establishing the decisions above (PO session on this branch).
- `.claude/rules/workflows/roadmap-elaboration.md` — spec template this document follows.
- Existing team/membership model in `app/ratel/src/features/social/pages/team_arena/` (implementation reference for Stage 3).
