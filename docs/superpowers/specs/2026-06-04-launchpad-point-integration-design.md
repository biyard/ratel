# Ratel ⇄ Launchpad Point Integration — Design

**Slug**: `launchpad-point-integration`
**Status**: Draft (awaiting user review)
**Date**: 2026-06-04
**Repos**: `ratel` (this repo — all code lands here) · `launchpad` (config only, no code change)

## Problem

Launchpad runs a per-service token economy: an operating service awards
**points**, users convert points into a per-service **S Token**. Launchpad
already ships the full company-point integration protocol (a `/connect`
hand-off page, `connect`/`lookup`/`convert` server functions, and HMAC-signed
callbacks) plus a reference external service (`demo/brand-demo`, "Daybreak
Coffee", a Node/Express app). We want to demonstrate that integration with a
**real Biyard service — ratel — as the external point provider**, end-to-end in
a browser.

The catch: **ratel does not own points.** `User.points: i64` is a vestigial
field (always `0`, never written). The source of truth for points is the
external **Biyard console** (`https://api.biyard.co`), which ratel reads/writes
through `BiyardService` (`app/ratel/src/common/services/biyard/mod.rs`).

## Goal

A browser-driven E2E demo: a logged-in ratel user clicks "Launchpad에서
전환하기", lands on launchpad, sees their **real console point balance**, and
converts a chosen amount to a launchpad token — with the points **actually
deducted on the console** and the conversion recorded in launchpad
(off-chain). Ratel is the integration adapter; the console stays the point
source of truth.

## Non-goals

- **No on-chain token mint.** Conversion stops at launchpad's off-chain
  `PointConversion` record (off-chain by explicit decision). No wallet-connect,
  contract deploy, or Kaia testnet transaction.
- **No launchpad code changes.** Launchpad's `/connect` page, company-point
  handlers, and callback protocol are reused as-is. Launchpad is configured via
  the admin setup UI only.
- **No console (Biyard) code changes.** Ratel uses the console only through the
  existing `BiyardService` client. We do not have the console repo here.
- **No SSO.** The user logs into ratel and launchpad separately. Account linking
  happens transparently on arrival at `/connect`.
- **No replacement of the console's own point→token features.** We are layering
  launchpad's token economy on top of console points, not migrating the console.

## Architecture

```
[browser]
  ratel login → "Launchpad에서 전환하기" button (per-user token embedded) → click
        │
        ▼
[launchpad :8080]  GET /connect?project_id=..&lp_user=<token>     (already built)
  connect / lookup / convert server fns ──HMAC callbacks──┐
                                                          ▼
[ratel :8000]  features/launchpad_partner/  (NEW — this spec)
  POST /launchpad/points/lookup  ─▶ BiyardService.get_user_balance() ┐
  POST /launchpad/points/deduct  ─▶ BiyardService.exchange_points()  ├▶ [console api.biyard.co]
  POST /launchpad/health                                             ┘   (point source of truth)
```

Ratel is a **translation adapter** between launchpad's callback contract and the
console's point API. It owns no point state except an idempotency ledger.

## New ratel module: `app/ratel/src/features/launchpad_partner/`

| File | Responsibility |
|---|---|
| `config.rs` | `LAUNCHPAD_BASE_URL`, `LAUNCHPAD_PROJECT_ID`, `LAUNCHPAD_PARTNER_SECRET` via `option_env!` (mirror `common/config/server/biyard.rs`) |
| `crypto.rs` | `encrypt_user_token(secret, user_id)` (AES-256-GCM, key = `SHA256(secret)`, `base64url(nonce‖ciphertext‖tag)`) · `verify_signature(secret, headers, raw_body)` (HMAC-SHA256 over `"{timestamp}.{raw_body}"`) — byte-identical to launchpad `demo_preview/server.rs` + `demo/brand-demo/server.js` |
| `controllers.rs` | `lookup` / `deduct` / `health` inner handlers; map `company_user_key` → `Partition::User(key)` → `BiyardService` |
| `models/deduction.rs` | `LaunchpadDeduction` idempotency record, derives `DynamoEntity` |
| `server.rs` | Axum router (template: `features/membership/server/mod.rs`); merged in `common/run.rs` |
| `views/connect_button.rs` (+ `i18n.rs`) | Renders the launchpad entry button with the logged-in user's encrypted token |

`features/mod.rs` gains `pub mod launchpad_partner;`. `run.rs` gains one
`.merge(crate::features::launchpad_partner::server::router())`.

## Callback contract (launchpad → ratel)

Common to all three: HMAC verify + `x-launchpad-project-id` header must equal
`LAUNCHPAD_PROJECT_ID` (else 403). Headers: `x-launchpad-timestamp` (epoch ms),
`x-launchpad-signature` (hex HMAC-SHA256 of `"{timestamp}.{raw_body}"`).

| Endpoint | Request body | → console call | Response |
|---|---|---|---|
| `POST /launchpad/health` | `{project_id, check}` | — | `{ok: true, project_id, service}` |
| `POST /launchpad/points/lookup` | `{project_id, company_user_key}` | `get_user_balance(User(key), current_month())` | `{available_points: balance, point_symbol}` |
| `POST /launchpad/points/deduct` | `{project_id, company_user_key, point_amount, idempotency_key}` | idempotency check → `exchange_points(User(key), point_amount, current_month())` → `get_user_balance` for remaining | `{brand_tx_id: transaction_id, deducted_points, remaining_points}` |

Errors mirror the reference: invalid signature → 401, unknown user → 404,
invalid amount → 400, insufficient/console-rejected → 409. Internal detail is
logged via `crate::error!`, never returned to the caller.

### Identity

`convert_user_id` accepts `Partition::User(id) → id`. So the token carries the
ratel user **uuid** (the inner string of `Partition::User`). On a callback,
ratel reconstructs `Partition::User(company_user_key)` before calling the
console. The launchpad-side link (`ProjectExternalUserLink`) maps the
signed-in launchpad user ↔ this ratel uuid.

### Idempotency (`LaunchpadDeduction`)

The console's `exchange_points` has no idempotency key, so a retried launchpad
`deduct` would double-spend. Ratel guards this:

- pk = `USER#<ratel_uuid>`, sk = `LAUNCHPAD_DEDUCTION#<idempotency_key>`
- fields: `idempotency_key`, `company_user_key`, `point_amount`, `brand_tx_id`,
  `remaining_points`, `created_at`
- On `deduct`: if a row for `idempotency_key` exists, return its stored result
  without calling the console again; otherwise call `exchange_points`, then
  persist the row, then return.

## Ratel entry point (browser)

A button (component `connect_button.rs`) shown to the logged-in ratel user that
links to `{LAUNCHPAD_BASE_URL}/connect?project_id={LAUNCHPAD_PROJECT_ID}&lp_user={encrypt_user_token(secret, user_uuid)}`.
Optionally append `&round_id=<open round>` after querying launchpad's public
`GET /api/onchain/projects/{project_id}/rounds` (the brand-demo does this; may
be skipped for a minimal demo). Placement: a demo-appropriate ratel page
(decided in the plan).

## Launchpad side — configuration only (no code)

1. Admin creates a project, sets up company-point with
   `callback_base_url = http://localhost:8000` and default paths
   (`/launchpad/points/{lookup,deduct}`), point symbol, min/unit.
2. Admin obtains the shared secret (`lps_...`) and the `project_id`.
3. Admin creates and **opens** a point conversion round.
4. Inject the secret into ratel as `LAUNCHPAD_PARTNER_SECRET` and the id as
   `LAUNCHPAD_PROJECT_ID`.

## Demo scenario (narrative)

**Prerequisites**: ratel + launchpad running locally; ratel connected to the
console (`BIYARD_*`) with the demo user holding console points; launchpad project
+ open round + secret prepared; launchpad demo user has a `wallet_address`
(required by `convert`).

1. "A ratel user earned points through activity." — ratel screen shows the
   balance pulled from the console (e.g. 1,240 PT).
2. "We move these points into launchpad's token economy." — click "Launchpad에서
   전환하기" → jump to launchpad `/connect`.
3. "Launchpad asks ratel for the live balance, and ratel delegates to the
   console." — `/connect` shows 1,240 PT (callback chain launchpad→ratel→console).
4. "Convert 500 points." — execute → ratel issues an Exchange transaction to the
   console → **console points actually drop to 740** → launchpad records the
   off-chain conversion.
5. "Both sides reconcile." — ratel refresh shows 740; launchpad shows the
   conversion entry. (Bonus: retrying the same request does not double-deduct on
   the console — idempotency.)

## Known simplifications / risks

1. **Monthly balance.** Console points are per-`month`; `lookup` reports only
   `current_month()`'s balance. Acceptable for the demo.
2. **`deduct` uses tx_type `"Exchange"`.** The console client exposes only
   `award_points` (`Award`) and `exchange_points` (`Exchange`, "Point-to-Token
   Exchange") for spending. So a launchpad conversion is recorded as an
   `Exchange` on the console — semantically overlapping with the console's own
   token exchange. A dedicated `Spend`/`Burn` tx_type would be cleaner but the
   console repo is unavailable to confirm. **Confirmed acceptable for this demo.**
3. **`remaining_points`.** `exchange_points` returns no remaining balance, so
   ratel re-queries `get_user_balance` after the exchange.
4. **Two logins.** Ratel and launchpad logins are separate (no SSO); linking is
   transparent on arrival at `/connect`.
5. **Wallet required.** Launchpad `convert` requires `user.wallet_address`; the
   demo launchpad user must have one set.

## Test plan

- Ratel integration tests for `launchpad_partner`:
  - signature: valid → 200; missing/incorrect signature → 401; project mismatch → 403
  - `lookup`: returns console balance (console call mocked/stubbed)
  - `deduct`: calls exchange, returns deducted/remaining
  - `deduct` idempotency: same `idempotency_key` twice → one console call, same result
- Launchpad: no new tests (existing flow unchanged).
- Manual: the full browser scenario above.

## References

- Reference external service: `launchpad/demo/brand-demo/server.js` (the contract,
  in Node) + `launchpad/demo/brand-demo/README.md`
- Launchpad callback mock + signature: `launchpad/app/src/demo_preview/server.rs`
- Launchpad handlers: `launchpad/app/src/features/community/company_points/company_points.rs`
- Launchpad `/connect` page: `launchpad/app/src/pages/connect/page.rs`
- Ratel console client: `app/ratel/src/common/services/biyard/mod.rs`
- Ratel config pattern: `app/ratel/src/common/config/server/biyard.rs`
- Ratel router merge: `app/ratel/src/common/run.rs`; webhook router template:
  `app/ratel/src/features/membership/server/mod.rs`
