# AI Post Draft (opinion-gathering template)

**Status**: Ready for development (Stage 3) — design approved at `app/ratel/assets/design/ai-post-draft/`
**Slug**: `ai-post-draft`
**Primary use case**: A paid Ratel creator drafting an opinion-gathering post supplies a topic, the background motivating the post, and the kind of feedback they want — and Ratel's AI assistant returns a structured five-section draft they can edit and publish.

## Problem

Drafting a well-structured opinion-gathering post on Ratel today starts from a blank canvas. Creators must invent the structure (background, purpose, scope, the actual questions they're asking the community, participation logistics) every time. Two failure modes follow:

- **Cognitive load wastes the first ten minutes** of every draft on form rather than content. Creators know what they want to ask but have to manually shape it into something readers can act on.
- **Posts ship without enough context.** Skipping the "why this matters" framing is the most common reason an opinion-gathering thread gets shallow replies — readers don't know what's at stake or what kind of input is useful.

Ratel also lacks a clearly differentiated capability that justifies a paid membership in the eyes of new sign-ups. A high-quality AI drafting assistant is the kind of feature paying users expect and free users actively notice missing.

## Goal

A paid creator opens a new post, clicks an **AI 로 작성 ✨** button, picks the opinion-gathering template, fills a short form (topic + background + what feedback they want + optional participation notes + output language), and receives a complete five-section draft prefilled in the editor. They edit freely and publish on their own terms.

## Non-goals

- **No additional templates in this phase.** Only the opinion-gathering template ships. Statements, policy comparisons, event announcements, and other templates are deferred to a separate roadmap item.
- **No multi-turn conversational drafting.** The flow is a single form submission, not a chat. A conversational variant is a future enhancement, not part of this phase.
- **No partial rewrites or AI editing of existing content.** The AI generates a whole draft once; it does not rewrite paragraphs, summarize, or translate selections.
- **No regeneration of the same post.** Each post may consume at most one successful AI draft. If the result is unsatisfactory, the creator edits manually rather than re-running AI.
- **No AI features for free users.** Free-tier users see the AI entry point but cannot use it; clicking it leads to an upsell. There is no metered free trial in this phase.
- **No image, media, or attachment generation.** The AI returns text only.
- **No usage analytics dashboard.** Per-user usage statistics, cost dashboards, and admin reporting are out of scope.
- **No background or asynchronous generation.** The draft is generated synchronously while the creator waits in the modal; there is no "we'll email you when it's done" mode.

## User stories

### Paid creator drafting an opinion post

- As a paid creator, I want to **see an AI assist button** on every empty draft so I know I can lean on it whenever I'm not sure how to start.
- As a paid creator, I want to **pick a template** that matches what I'm writing so the AI generates content shaped for that purpose (currently opinion-gathering only).
- As a paid creator, I want the **input form to be short** — three or four short questions, not a long survey — so I can get to a draft in under a minute.
- As a paid creator, I want to **choose the output language** (Korean or English) directly in the form, defaulting to my current UI language but overridable for that single generation.
- As a paid creator, I want a **clear warning before AI generation overwrites** any content I've already typed in the editor.
- As a paid creator, I want to **see a loading state** while the AI is working and be able to **cancel** if it's taking too long.
- As a paid creator, I want the **generated draft to land directly in the editor** as the title plus a body with the five expected sections, so I can immediately start editing rather than copy-paste.
- As a paid creator, after a successful generation I want the **AI button to disappear from that post** so I'm not tempted to re-run it and so the per-post one-shot limit is visually obvious.
- As a paid creator, if the **AI call fails** I want a clear error message and the option to retry with the same inputs, without the one-shot limit being consumed.

### Free-tier user discovering the feature

- As a free user, I want to **see that the AI assist feature exists** so I can decide whether it's worth upgrading.
- As a free user clicking the AI button, I want a **clear upsell modal** that explains the benefit and offers a one-click path to the membership page, rather than a generic "upgrade required" error.

### Workspace owner / platform operator

- As the platform operator, I want **the paid-only restriction enforced on the server** so a malicious client cannot bypass the membership check and consume AI calls.
- As the platform operator, I want the **per-post one-shot limit enforced on the server** so retry abuse cannot inflate cost.

## Functional requirements

1. The post editor SHALL display an **"AI 로 작성 ✨" entry point** on every post draft regardless of the user's membership tier, with visual treatment that signals it as a premium capability.
2. When a **free-tier user** clicks the AI entry point, the system SHALL open an upsell modal that explains the feature, names the required tier ("Pro or higher" — i.e., any tier other than Free), and provides a single-click path to the membership upgrade page. No AI request is sent.
3. When a **paid user** clicks the AI entry point, the system SHALL open the AI draft modal showing a template picker. The opinion-gathering template SHALL be the only selectable option in this phase.
4. After the user selects the opinion-gathering template, the system SHALL display an input form with the following fields:
   - **Topic** (single line) — required
   - **Background / motivation** (multi-line, short paragraph) — required
   - **Feedback you want from the community** (multi-line, free form) — required
   - **Participation notes** (multi-line) — optional
   - **Output language** (dropdown: Korean / English) — defaults to the user's current locale
5. The **"Generate draft" button** SHALL remain disabled until all three required fields contain non-empty text. The optional field and the language dropdown do not gate the button.
6. If the editor already contains a non-empty title or body when the user submits the form, the system SHALL show a **confirmation dialog** ("Existing content will be replaced") before sending the AI request. Cancelling the dialog returns the user to the form with their inputs intact.
7. While the AI request is in flight, the modal SHALL show a **loading indicator** and a **cancel button**. Cancelling aborts the request without consuming the per-post one-shot allowance.
8. On **successful generation**, the system SHALL:
   - Replace the editor's title and body with the AI's output.
   - Mark the post as having consumed its one-shot AI draft allowance.
   - Close the modal.
   - Return the user to the normal editor view with focus in the editor.
9. The **generated body** SHALL contain exactly five sections in this order, each as a heading followed by paragraph text:
   1. Background (추진배경 / Background)
   2. Purpose (추진목적 / Purpose)
   3. Content (추진내용 / Content)
   4. Topics for input (의견수렴 사항 / Topics for Input)
   5. How to participate (참여 안내 / How to Participate)
10. **Section headings SHALL match the selected output language** (Korean labels for Korean output, English labels for English output).
11. On **AI failure** (network error, model error, parsing failure), the system SHALL:
    - Keep the modal open with the user's inputs preserved.
    - Display a human-readable error message.
    - Offer a **"Try again"** button that re-sends the same request.
    - NOT mark the post as having consumed its allowance.
12. Once a post's one-shot allowance has been consumed, the system SHALL hide the AI entry point on that post permanently (for that post; other posts are unaffected).
13. The server SHALL **re-validate membership tier** on every AI draft request. A client-side bypass MUST result in a server-side rejection.
14. The server SHALL **re-validate the one-shot allowance** on every AI draft request. A client-side bypass attempt against an already-consumed post MUST result in a server-side rejection.
15. The user's form inputs SHALL be used solely to construct the AI prompt for the current request and SHALL NOT be persisted in long-term storage. (They may live in client memory or transient request state for retry purposes only.)

## Acceptance criteria

- [x] **AC-1** — The AI entry point is visible on the post editor for every authenticated user.
- [x] **AC-2** — A free-tier user clicking the AI entry point sees an upsell modal that names the required tier ("Pro or higher") and provides a working link to the membership page; no AI request is sent.
- [x] **AC-3** — A paid user clicking the AI entry point sees a template picker with one selectable template ("Opinion gathering").
- [x] **AC-4** — Selecting the opinion-gathering template reveals the input form with topic, background, feedback-you-want, participation-notes (optional), and output-language fields.
- [x] **AC-5** — The output-language dropdown defaults to the user's current UI locale.
- [x] **AC-6** — The "Generate draft" button is disabled until topic, background, and feedback-you-want are all non-empty; it activates as soon as all three contain text.
- [ ] **AC-7** — If the editor has existing non-empty content, submitting the form first shows a "content will be replaced" confirmation dialog, and cancelling that dialog preserves both the form inputs and the existing editor content. *(deferred — the editor's autosave makes the overwrite a recoverable action; revisit if user feedback flags it.)*
- [x] **AC-8** — During AI generation, a loading indicator and a cancel button are visible; cancelling aborts the request and leaves the post's one-shot allowance intact.
- [x] **AC-9** — On success, the editor's title and body are replaced with the AI output, the modal closes, and the AI entry point disappears from that post.
- [x] **AC-10** — The generated body contains exactly five sections in the specified order, each rendered as a heading followed by paragraph content.
- [x] **AC-11** — When the output language is Korean, all five section headings are in Korean; when English, all five are in English.
- [x] **AC-12** — On AI failure, the modal stays open with inputs preserved, an error message is shown, and a "Try again" button re-sends the request without consuming the allowance.
- [x] **AC-13** — After a successful AI generation on a post, reopening that post's editor shows no AI entry point.
- [x] **AC-14** — Server-side: a free-tier user calling the AI draft endpoint directly (bypassing the client) receives a rejection identifying paid-only access.
- [x] **AC-15** — Server-side: a paid user calling the AI draft endpoint for a post that has already consumed its allowance receives a rejection identifying the one-shot limit.

## Constraints

- **Cost ceiling per post**: One successful AI call per post, capped by the one-shot rule. Expected cost per call falls in the $0.01–$0.05 range based on current Claude Sonnet pricing for ~1.5K output tokens; the per-post cap keeps spend predictable without per-user metering.
- **Latency**: Median end-to-end response is expected to fall under 8 seconds. The UI must surface a loading state and a cancel option so the user is never left wondering whether the request is still alive.
- **Privacy of user inputs**: Form inputs are used only to construct the AI prompt. They are not stored beyond the lifetime of the request (other than what's needed for retry inside the open modal). The generated draft is, of course, stored as part of the post the user is composing.
- **Server-side enforcement**: Both the paid-tier check and the one-shot per-post check must be enforced on the server. Client-side checks are advisory only.
- **Hallucination guardrails**: The AI prompt must instruct the model to use only the information the user provided and to insert a neutral placeholder when a section has no input, rather than inventing facts, names, statistics, or quotes.
- **Existing platform infrastructure**: The Ratel app already integrates with AWS Bedrock and uses Claude Sonnet for the AI moderator feature. The AI drafting feature must reuse that infrastructure rather than introducing a new provider.

## Open questions

_None at handoff to Stage 2._

## References

- Existing AI moderator feature using AWS Bedrock + Claude Sonnet — `app/ratel/src/features/ai_moderator/`
- Membership tier check (`is_paid()`) and hook — `app/ratel/src/features/auth/hooks/use_user_membership.rs`
- Post editor where the AI entry point will be added — `app/ratel/src/features/posts/views/post_edit/`
