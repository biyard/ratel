# Cross-posting (Bluesky / LinkedIn / Threads)

**Status**: Ready for development (Stage 3) — design already approved at `app/ratel/assets/design/cross-posting/`
**Slug**: `cross-posting`
**Primary use case**: Ratel creator publishes once on Ratel; the post is automatically syndicated to the creator's external social networks with a backlink that converts external readers into Essence House subscribers.

## Problem

Ratel creators today face a publishing dilemma. To grow an audience, they need to be visible on Bluesky, LinkedIn, and Threads — that's where most readers live. But maintaining three or four parallel feeds means:

- Re-typing every post into each network with that network's character limit and tone.
- No way to bring readers from external networks back to Ratel — every Bluesky reader stays on Bluesky.
- Inconsistent state: a post edited on Ratel becomes stale on the syndicated copies.
- No single place to see how a piece of content performed across networks.

The result is that creators either pick one network and starve the others, or burn time on manual cross-posting and still get no funnel back to Ratel. Either way, Ratel loses the long-term subscriber relationship that its Essence House model depends on.

## Goal

Let a Ratel creator connect their external accounts once, and have every Ratel post auto-publish to those accounts as a backlinked summary that drives external readers back to the full post on Ratel.

## Non-goals

- **No engagement parity.** Likes, replies, reposts on the external network are not synced back into Ratel as Ratel-side actions in Phase 1. We display counts only (read-only mirror).
- **No reverse syndication.** Posts authored on Bluesky / LinkedIn / Threads are NOT pulled into Ratel.
- **No editing parity.** Editing a Ratel post does NOT update the already-published syndicated copies in Phase 1; the Ratel copy is the canonical source and the external copies are immutable summaries with a backlink.
- **No deletion parity in Phase 1.** Deleting a Ratel post does not auto-delete the syndicated copies. (Phase 1.5 enhancement: optional "delete from external networks too" toggle on Ratel-side delete.)
- **No scheduled publishing in Phase 1.** Cross-post fires when the Ratel post is published, immediately. Scheduling is Phase 2.
- **No multi-account-per-platform.** A user connects at most one Bluesky account, one LinkedIn account, one Threads account. Multiple personas per platform is out of scope.
- **No DM / private message integration.** This feature only touches public posts.
- **No Farcaster in Phase 1.** Farcaster (Frames + agent posting) is Phase 2.
- **No team-level cross-posting in Phase 1.** Connections are personal to the user. Team / organization shared cross-posting is Phase 2.
- **No content adaptation for character limits in Phase 1.** If the Ratel post exceeds a network's character limit, we publish a truncated excerpt + backlink — we do not split into a thread, do not auto-summarize with AI, do not let the user write a network-specific variant per post. (Per-network-variant compose is a Phase 1.5 enhancement.)

## User stories

### New creator (just signed up)

- As a new creator, I want to **connect my external accounts during onboarding** so my first Ratel post immediately reaches my existing audience on Bluesky / LinkedIn / Threads.
- As a new creator, I want to **skip the connect step** if I'm not ready, and come back to it later from settings without losing onboarding progress.
- As a new creator, I want to **see why connecting helps** (the "3.4× more subscribers in 30 days" pitch) before deciding.

### Established creator (managing connections)

- As a creator, I want a **single Connections page** under settings where I can see every platform's status, last sync time, and posts syndicated count.
- As a creator, I want to **toggle auto-post per platform** independently — e.g., auto-post to Bluesky and LinkedIn but require explicit opt-in for Threads.
- As a creator, I want to **disconnect a platform at any time**; existing syndicated copies stay live on that network but no future Ratel post is sent there.
- As a creator, I want to **reconnect a platform** after the token expires without losing my prior syndication history.

### Composing creator (per-post control)

- As a creator drafting a post, I want to **see which networks the post will reach** in a sidebar while I type.
- As a creator drafting a post, I want to **disable specific platforms for this one post** without changing my overall connection settings (e.g., a niche post that doesn't fit LinkedIn's tone).
- As a creator drafting a post, I want to **see character-limit warnings** for each connected platform as I type, so I know what will be truncated.
- As a creator drafting a post, I want the **Publish button to clearly state how many networks it will reach** ("Publish to 3").

### Post-publish creator (monitoring syndication)

- As a creator, I want to **see per-platform status** on my post detail page: Published, Pending, or Failed — with the external URL when published and the error reason when failed.
- As a creator, I want a **retry button** for any platform that failed to publish, without re-publishing to the platforms that already succeeded.
- As a creator, I want to **see basic engagement counts** (likes / comments / reposts) for each syndicated copy without leaving Ratel.

### External reader (landing from a syndicated post)

- As an external reader who clicked a Bluesky / LinkedIn / Threads link, I want to **read the full post on Ratel** without forced sign-up.
- As an external reader, I want a **clear, one-tap path to subscribe to the creator's Essence House** so I keep getting their content.
- As an external reader on mobile, I want the **landing page to load fast** even on a slow connection.

## Functional requirements

### FR-1: Platform connection model

1. A user MAY connect **zero or more platforms** out of: **Bluesky, LinkedIn, Threads** (Phase 1).
2. Each connection MUST store: external account handle, OAuth/app-password credentials (KMS-encrypted), token expiry/refresh metadata, connection timestamp, and per-connection auto-post flag (default: `true`).
3. **Bluesky** uses an **app password** flow (user generates `xxxx-xxxx-xxxx-xxxx` at `bsky.app/settings/app-passwords` and pastes it into Ratel along with their handle).
4. **LinkedIn** uses **OAuth 2.0** with scopes `r_liteprofile` (read basic profile) and `w_member_social` (publish posts).
5. **Threads** uses **Meta OAuth** with scopes `threads_basic` and `threads_content_publish`. Requires the user to have a linked Instagram business/creator account.
6. Credentials MUST be encrypted at rest with KMS. They MUST NOT appear in logs, error messages, or notifications.
7. A user MAY revoke a connection at any time. On revoke:
    - The encrypted credential is deleted from Ratel.
    - Future cross-posts to that platform stop immediately.
    - Existing syndicated copies on the external network are NOT touched (Ratel cannot delete what it can't reach).
    - Future engagement-count refreshes for that platform stop.

### FR-2: Post-signup interstitial

8. Immediately after a user completes signup (`SignupModal` returns success), the system MUST route the user to a **single-screen "Connect your networks" interstitial** before landing on home. This interstitial is **not** part of a multi-step wizard — there is no Account/Essence stepper, because Ratel signup itself is a single modal and the Essence flow is a separate roadmap (`essence-connector`) that does not couple to this screen.
9. The interstitial MUST list each Phase 1 platform (Bluesky, LinkedIn, Threads) with a Connect button.
10. The interstitial MUST be **skippable** via a "Skip for now" link or a "Skip" button. Skip MUST land the user on home with the same UX as if signup had completed without this interstitial.
11. The interstitial MUST display the value-prop copy ("Your first post reaches three networks instantly") and the social-proof note ("creators with 2+ networks get 3.4× more subscribers in 30 days").
12. When a connection succeeds inside the interstitial, the platform's row MUST flip to a "Connected" state in-place; the user does not have to re-click anything.
13. The interstitial MUST show **at most once** per account — once the user clicks Continue or Skip (or completes any connection on this screen), the system marks `interstitial_seen=true` on the user record and never auto-shows it again. From that point on, the only entry point is **Settings → Connections** (FR-3).
14. The interstitial MUST NOT block any other onboarding side effects (welcome email, default team setup, etc.) — it sits purely between signup-success and home.

### FR-3: Connections settings page

15. A signed-in user MUST be able to access **Settings → Connections** at any time.
16. The page MUST list every Phase 1 platform, plus Phase 2 platforms displayed as "**Coming soon**" with no Connect button.
17. For each connected platform, the page MUST show: platform name + logo, status pill ("Connected"), external handle, last successful sync timestamp ("X minutes ago"), posts-syndicated count (cumulative), per-platform auto-post toggle, and a Disconnect button.
18. For each not-connected platform, the page MUST show a Connect button that opens the platform's auth flow (modal for Bluesky app-password, OAuth redirect for LinkedIn / Threads).
19. The header MUST show two stats: **"X Connected"** and **"Y posts this month"** (count of syndicated posts in the current calendar month across all platforms).
20. The Disconnect button MUST require confirmation (modal: "This stops future cross-posts. Existing copies on Bluesky stay live."). Confirming MUST execute the revoke described in FR-1.
21. When connecting Threads, if Meta OAuth callback returns no eligible Instagram business/creator account, the system MUST surface a dedicated error modal: **"Threads 연결을 위해 인스타그램 프로페셔널 계정 전환이 필요합니다."** ("To connect Threads, please switch to an Instagram Professional account.") The modal MUST link to Meta's account-conversion page and MUST NOT create a partial Threads connection.

### FR-4: Compose-time controls

22. When composing a post on Ratel, the editor MUST show a **right-hand cross-post sidebar** listing every connected platform plus any not-connected platforms (with inline Connect CTA).
23. Each connected-platform row in the sidebar MUST show: platform logo, name, handle, and a per-post **enable toggle** (default: `ON` if the platform's auto-post flag is `true`).
24. The sidebar MUST show a **"Reaching N networks"** summary that updates live as toggles flip.
25. The sidebar MUST display a **character-count indicator per platform** that turns warning-colored when the post body exceeds that platform's limit.
    - Bluesky: 300 chars
    - LinkedIn: 3,000 chars
    - Threads: 500 chars
26. The Publish button MUST display the reach count in its label: **"Publish to N"**.
27. Disabling all platforms in the sidebar MUST be allowed; the post is then Ratel-only and the Publish button reads **"Publish"**.
28. Per-post sidebar choices MUST NOT modify the user's persistent auto-post settings — they apply only to this post.

### FR-5: Publishing pipeline (immediate fan-out)

29. Cross-posting fires **immediately** when the Ratel post is published. There is no scheduling, no draft queue, no future-time enqueue in Phase 1.
30. When the user publishes a Ratel post, the system MUST enqueue **one syndication job per enabled platform**.
31. Each syndication job MUST:
    - Format the post body for that platform's character limit per the truncation rules in FR-5.5 below.
    - **Always append a backlink** to the canonical Ratel post URL with `?utm_source={platform}` query parameter forced onto the URL (e.g., `?utm_source=bluesky`, `?utm_source=linkedin`, `?utm_source=threads`). The UTM parameter is non-removable — even if the post body's URL count makes truncation tight, the backlink + UTM must survive.
    - Include the post's hero image / first attached image when the platform supports it (Bluesky up to 4 images, LinkedIn 1 image, Threads 1 image).
    - Call the platform's publish API.
    - Record the resulting state: `Pending` (queued), `Published` (success + external URL), `Failed` (error reason + category).
32. Each platform's job MUST be **independent** — Bluesky succeeding while LinkedIn fails MUST leave the LinkedIn job in `Failed` state without rolling back Bluesky.
33. A `Failed` job MUST capture an error category: `auth_expired`, `rate_limited`, `content_rejected`, `network_error`, `unknown`.
34. The system MUST automatically retry a `Failed` job using **exponential backoff up to 3 attempts** for retryable categories (`rate_limited`, `network_error`, `unknown`):
    - 1st retry: **1 minute** after initial failure
    - 2nd retry: **10 minutes** after 1st retry
    - 3rd retry: **1 hour** after 2nd retry
    - After the 3rd retry fails, the job is marked `Failed` permanently and waits for user-initiated retry from the post detail (FR-6 #38).
    - Non-retryable categories (`auth_expired`, `content_rejected`) MUST NOT be auto-retried; they wait for user action.
35. On `auth_expired`, the user MUST receive an in-app notification linking to **Settings → Connections** to reconnect.

### FR-5.5: Truncation strategy (per-platform body construction)

36. When the formatted body (post text + backlink with UTM) exceeds the platform's character limit, the system MUST truncate using this fixed order: **`{title}` + `\n\n` + `{first sentence of body}` + `…` + `\n` + `{backlink URL}`**.
    - The first sentence is the substring up to the first period / question mark / exclamation mark followed by whitespace. If no such terminator exists, the entire body is treated as one sentence and is itself truncated mid-word.
    - The ellipsis character (`…`) signals the cut to readers.
    - The backlink URL (with UTM) is **never truncated** — if the title alone would exceed the budget, the body is omitted entirely and only `{title} + \n + {backlink}` is sent.
37. When the formatted body fits within the limit, the system MUST send `{title} + \n\n + {full body} + \n\n + {backlink}` without truncation.
38. The truncation rule applies **per platform** — Bluesky may receive the truncated form while LinkedIn receives the full form for the same Ratel post.

### FR-6: Privacy guard at fan-out time

39. The publishing pipeline MUST re-check a post's visibility **immediately before each platform call**, not only at enqueue time. If the post has been flipped to `Private` or `Team-shared` between enqueue and dispatch, the worker MUST abort that job and mark it `Skipped` (a new state distinct from `Failed`).
40. A post whose visibility is `Private` or `Team-shared` at any point in the syndication path MUST NOT be sent to any external platform. This check MUST be implemented as a hard guard at two layers — at enqueue (FR-9 #50) and at dispatch (this requirement).

### FR-7: Post-detail syndication panel (author-only)

41. The post detail page MUST display a **"Syndication" section** below the post body, **visible only to the post's author**. Non-author signed-in users and non-signed-in visitors MUST NOT see this section.
42. The section MUST show a header summary: **"X of Y succeeded · Z retrying"**.
43. Each syndicated-platform row MUST show: platform logo, name, status pill (Published / Pending / Failed / Skipped), external URL link (when Published), and engagement counts (likes / comments / reposts) when available.
44. A `Failed` row MUST show the human-readable error reason and a **Retry** button. Retry MUST re-enqueue only that platform's job.
45. Engagement counts MUST refresh on a schedule (Phase 1 default: poll every 6 hours per platform) and on manual user refresh. Engagement counts are author-only — they MUST NOT be exposed via any non-author surface.

### FR-8: Public landing page (backlink target)

46. When a syndicated post's backlink is opened by a non-signed-in user, Ratel MUST render a **public-facing post page** that:
    - Shows the full post body, hero image, author name + avatar, publish time.
    - Shows a clear **Subscribe to {author}'s Essence House** CTA in the sidebar / sticky bar.
    - Renders without forcing the visitor to sign up.
    - Includes proper **OG meta tags** (title, description, image) so the original Bluesky / LinkedIn / Threads card preview is rich.
47. The public page MUST include a **referral / source banner** at the top when the visitor arrived with `?utm_source=bluesky` / `linkedin` / `threads` (or via known external-network referrer header) — e.g., "**You're reading this on Ratel** — the canonical source. Continue reading uninterrupted." UTM detection takes priority over the `Referer` header because some networks strip referrers.
48. The public page MUST be **performant on mobile** (Lighthouse mobile performance ≥ 80, LCP < 2.5s on a fast 3G simulation).
49. SEO must be correct: canonical URL points at Ratel, no `noindex`, structured data for `Article`.

### FR-9: Visibility and privacy

50. Only **public** Ratel posts are eligible for cross-posting. **Private** and **team-shared** posts MUST NOT be syndicated and MUST NOT show the cross-post sidebar at compose time. This check is the **first of two privacy guards** (the second is at fan-out dispatch, FR-5 #39–40).
51. If a user changes a post's visibility from public to private after syndication, the syndicated copies REMAIN on the external networks (we cannot retroactively unpublish there). The Ratel-side post detail MUST surface a notice: "Syndicated copies on Bluesky / LinkedIn / Threads remain visible. Delete them on each platform manually."

### FR-10: Failure observability

52. Every syndication job's outcome MUST be logged with: post id, platform, attempt count, retry-stage (initial / 1-min / 10-min / 1-hr), latency, outcome, error category (if any).
53. Logs MUST NOT contain credentials, app passwords, OAuth tokens, or full body content. Body content is logged as length only. Credential fields MUST be redacted at the logger boundary so a future logger swap can't accidentally regress this.

## Acceptance criteria

The feature is shippable when a tester acting as a creator can complete the entire flow end-to-end:

- [ ] AC-1: New user completes signup and is automatically routed to the "Connect your networks" interstitial before reaching home.
- [ ] AC-2: User connects Bluesky via app-password flow; the row flips to "Connected" without leaving the interstitial.
- [ ] AC-3: User connects LinkedIn via OAuth; the OAuth redirect returns the user to the interstitial with the row showing "Connected".
- [ ] AC-4: User clicks "Skip" on the interstitial and lands on home with no platforms connected and no error.
- [ ] AC-4b: After dismissing the interstitial once (Continue or Skip), signing out and back in does NOT re-show the interstitial — the only entry point is now Settings → Connections.
- [ ] AC-5: User opens **Settings → Connections** and sees Bluesky + LinkedIn connected, Threads not connected, Farcaster as "Coming soon".
- [ ] AC-6: Toggling LinkedIn's per-platform auto-post off persists across reload.
- [ ] AC-7: Disconnecting Bluesky requires confirmation; after confirming, the credential is gone and the row shows "Not connected".
- [ ] AC-8: Composing a new public post shows the cross-post sidebar with both connected platforms enabled by default and Threads as a Connect-CTA row.
- [ ] AC-9: Disabling LinkedIn in the sidebar drops the "Reaching N networks" count by 1; the persistent setting is unchanged.
- [ ] AC-10: Composing a body > 300 chars shows a Bluesky truncation warning while the LinkedIn warning stays clean.
- [ ] AC-11: Composing a private post shows NO cross-post sidebar.
- [ ] AC-12: Publishing a public post fans out to both connected platforms; the post detail page shows two `Pending` rows that flip to `Published` with external URLs.
- [ ] AC-13: A simulated LinkedIn `auth_expired` failure marks that platform `Failed` with a "Reconnect" CTA; Bluesky stays `Published`.
- [ ] AC-14: The user receives an in-app notification for the LinkedIn `auth_expired` failure.
- [ ] AC-15: Clicking Retry on the failed LinkedIn row re-enqueues only LinkedIn (not Bluesky).
- [ ] AC-16: The Bluesky-published copy contains a backlink URL pointing to the Ratel post.
- [ ] AC-17: Visiting that backlink URL while signed out renders the public post page with subscribe CTA and no forced sign-up.
- [ ] AC-18: The public landing page rendered from a Bluesky referrer shows the "You're reading this on Ratel" banner.
- [ ] AC-19: Engagement counts (likes / comments) appear on the post-detail syndication panel within one refresh cycle of the external platform receiving them.
- [ ] AC-20: An author switching their post from public → private displays the "syndicated copies remain visible" notice.
- [ ] AC-21: Lighthouse mobile performance for the public landing page is ≥ 80.
- [ ] AC-22: A Ratel post whose body would exceed Bluesky's 300-char limit is sent to Bluesky as `{title}\n\n{first sentence}…\n{backlink with utm_source=bluesky}`; the backlink URL is intact and not truncated.
- [ ] AC-23: Every syndicated copy's backlink URL contains the correct `?utm_source=` parameter (`bluesky`, `linkedin`, or `threads`) for that platform.
- [ ] AC-24: A `network_error` failure on LinkedIn auto-retries at 1 minute, 10 minutes, and 1 hour after the prior failure (3 attempts total) before being marked permanently `Failed`.
- [ ] AC-25: An `auth_expired` failure does NOT auto-retry; it waits for the user to reconnect via the notification CTA.
- [ ] AC-26: A user who attempts to connect Threads without a linked Instagram business/creator account sees the "인스타그램 프로페셔널 계정 전환이 필요합니다" modal and no Threads connection is created.
- [ ] AC-27: Flipping a post's visibility from public to private between enqueue and dispatch causes the in-flight syndication job(s) to enter `Skipped` state without sending to the external platform.
- [ ] AC-28: Server logs for a syndication job contain post id + platform + retry-stage + outcome but contain no credentials, no OAuth tokens, and no body content (length only).
- [ ] AC-29: A non-author signed-in user opening a syndicated post's detail page sees the post body and comments but no syndication panel and no engagement counts.

## Constraints

### Privacy & security
- **Credentials are never logged, never displayed, never exported.** They are KMS-encrypted at rest and decrypted only inside the Lambda that publishes.
- **Private and team-shared posts MUST NOT leak.** No syndication path may consume them, including bug paths (e.g., a draft-saved-as-public-then-flipped-private must be defensively re-checked at fan-out time).
- **Token revocation must be honored on the next request.** If a user revokes the connection on Ratel after a job is enqueued, the worker MUST re-check connection state immediately before publishing and abort if revoked.

### Platform limits
- **Bluesky AT Protocol**: 300 chars, 4 images. App-password flow only (no public OAuth from third parties).
- **LinkedIn API**: 3,000 chars, OAuth 2.0, member-token TTL ~60 days, refreshable.
- **Threads API (Meta Graph)**: 500 chars, requires linked Instagram business/creator account, long-lived token with auto-refresh.
- **Rate limits**: each platform's published rate limit must be respected. We MUST batch / debounce so a creator publishing 10 posts in 1 minute does not hit a per-minute quota.

### Cost
- **No paid platform tiers.** All Phase 1 networks are accessible on free developer tiers.
- **Engagement-count polling MUST be amortized.** Polling every connected user every minute would exhaust quotas; use 6-hour intervals or webhook subscriptions where the platform supports them.

### Reliability
- **Best-effort, eventually-consistent.** A single platform outage MUST NOT block publishing to Ratel itself or to other connected platforms.
- **Idempotent retries.** Re-running a syndication job for the same `(post_id, platform)` MUST NOT create duplicate external posts. Use platform-side dedupe keys where possible; otherwise track our own external-post-id mapping.

### Product
- **Backlink is non-negotiable.** Every syndicated copy carries a backlink even when this costs characters in tight budgets like Bluesky's 300.
- **Authoring stays on Ratel.** No reverse syndication. No "publish from Bluesky to Ratel". The funnel direction is one-way.

## Decisions log

- **D-1 (resolved)**: *Should the post-signup onboarding be a multi-step wizard (Account → Connect → Essence)?* — **No.** The mockup `onboarding-connect-socials.html` originally drew a 3-step stepper, but the existing signup is a single modal (`SignupModal`) and the Essence flow is a separate roadmap (`essence-connector`) that does not share a wizard with cross-posting. We adopted a **single-screen interstitial** (FR-2) shown once, between signup-success and home, with no stepper UI and no coupling to Essence. The mockup's stepper region was removed in the design refresh.

- **D-2 (resolved)**: *Bluesky truncation strategy for the 300-char limit.* — Truncate using the fixed order **`{title}` + `\n\n` + `{first sentence of body}` + `…` + `\n` + `{backlink URL}`**. The backlink (with UTM) is non-truncatable; if the title alone exceeds the budget, body is omitted entirely. Codified as FR-5 #36–38. Same rule applies to all platforms when their per-platform body would exceed the limit; only the limit value differs.

- **D-3 (resolved)**: *Scheduling in Phase 1.* — **Out of scope.** Cross-posting fan-out fires immediately on Ratel publish (FR-5 #29). Scheduled / draft / future-time enqueue is a Phase 2 concern. The Phase 1 implementation should keep the dispatcher's API shape extensible enough that a future scheduler can hand off jobs without refactor, but no scheduler code lands in Phase 1.

- **D-4 (resolved)**: *Phase 1 retry policy.* — **Exponential backoff, up to 3 attempts**: 1st retry at 1 minute, 2nd retry at 10 minutes, 3rd retry at 1 hour after the prior failure. Applies to retryable categories (`rate_limited`, `network_error`, `unknown`). Non-retryable categories (`auth_expired`, `content_rejected`) wait for user action. Codified as FR-5 #34.

- **D-5 (resolved)**: *Engagement-count visibility.* — **Author-only.** Engagement counts (likes / comments / reposts pulled from each external platform) are shown only on the post detail's syndication panel and only to the post's author. No non-author surface exposes them. Codified as FR-7 #41 + #45.

- **D-6 (resolved)**: *Threads connect failure when user lacks an Instagram business/creator account.* — Surface a dedicated modal: **"Threads 연결을 위해 인스타그램 프로페셔널 계정 전환이 필요합니다."** with a link to Meta's account-conversion page. No partial Threads connection is created. Codified as FR-3 #21.

- **D-7 (resolved)**: *Public-landing referrer detection.* — Always inject `?utm_source={platform}` (e.g., `?utm_source=bluesky`) into every syndicated backlink. The UTM is the **primary** signal because some networks strip the `Referer` header. The `Referer` header serves as a fallback. Codified as FR-5 #31 + FR-8 #47.

- **D-8 (resolved · cross-cutting)**: *Credential handling.* — All external-platform credentials (Bluesky app password, LinkedIn OAuth tokens, Threads Meta tokens) are KMS-encrypted at rest and redacted at the logger boundary. Logs MUST NOT contain credentials, OAuth tokens, or full body content. Codified as FR-1 #6 + FR-10 #53 + Constraints (Privacy & security).

- **D-9 (resolved · cross-cutting)**: *Private / team-shared post leak prevention.* — Implement as a **two-layer hard guard**: (1) at compose time the cross-post sidebar is hidden for non-public posts (FR-9 #50), and (2) at fan-out dispatch the worker re-checks the post's current visibility immediately before each platform call and aborts (state `Skipped`) if it is no longer public (FR-6 #39–40). Codified as FR-6 + FR-9.

## Open questions

- ~~OQ-1: *Per-network compose variants in Phase 1.5?*~~ — **Resolved 2026-04-28** (see [docs/superpowers/specs/2026-04-28-cross-posting-design.md](../docs/superpowers/specs/2026-04-28-cross-posting-design.md#resolved-decisions)). Decision: a sidecar `PostSyndicationDirective` carries `enabled_platforms` + `platform_overrides` so Stage 1 can act as a factory; `Feed` stays free of per-platform fields. v1.5 adds `body_override: Option<String>` to `SyndicationJob` — UI/factory change only, no schema migration on `Feed`.

## References

- **Approved design** (Stage 2 artifact): `app/ratel/assets/design/cross-posting/`
  - `onboarding-connect-socials.html` — single-screen post-signup interstitial (D-1)
  - `social-connections.html` — Settings → Connections page
  - `compose-with-crosspost.html` — composer with cross-post sidebar + connect modals
  - `post-detail-syndicated.html` — post detail with syndication panel + retry (author-only)
  - `backlink-landing.html` — public landing page from external referrer
- **Workflow**: `.claude/rules/workflows/roadmap-elaboration.md` (this spec follows that template)
- **Platform docs**:
  - Bluesky AT Protocol: https://docs.bsky.app/
  - LinkedIn Marketing API (UGC posts): https://learn.microsoft.com/en-us/linkedin/marketing/integrations/community-management/shares/
  - Threads API: https://developers.facebook.com/docs/threads/
- **Related Ratel concepts**:
  - Essence House (creator subscription surface) — referenced as the conversion target throughout.
  - Notification system — used for `auth_expired` reconnect CTAs (FR-5 #35).
