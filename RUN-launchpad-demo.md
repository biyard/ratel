# Ratel ⇄ Launchpad point integration — demo runbook

Ratel acts as the external point provider for Launchpad's company-point flow.
Ratel does **not** store points — it delegates balance/deduct to the Biyard
console (`BiyardService`). This runbook drives the browser E2E: a ratel user's
console points are looked up on Launchpad and converted to a token (off-chain).

## 1. Launchpad (admin setup — no code change)
1. Run launchpad locally on `:8080` (`make run` in the launchpad repo).
2. Admin → create a project; complete company-point setup:
   - Company URL: `http://localhost:8000`
   - point paths: defaults (`/launchpad/points/lookup`, `/launchpad/points/deduct`)
   - point symbol: `P`
3. Copy the generated shared secret (`lps_...`) and the project id.
4. Create and **OPEN** a point conversion round.
5. Ensure the launchpad demo user has a `wallet_address` (wallet sign-in) —
   Launchpad's `convert` requires it.

## 2. Ratel (this repo)
Build/run with the launchpad values injected. `LAUNCHPAD_PROJECT_ID`,
`LAUNCHPAD_PARTNER_SECRET`, and `LAUNCHPAD_POINT_SYMBOL` are read at **compile
time** (`option_env!`); `LAUNCHPAD_BASE_URL` is read at runtime.

```bash
cd app/ratel
LAUNCHPAD_PROJECT_ID="<project_id>" \
LAUNCHPAD_PARTNER_SECRET="<lps_secret>" \
LAUNCHPAD_BASE_URL="http://localhost:8080" \
DYNAMO_TABLE_PREFIX=launchpad-local \
  dx serve --port 8000 --web
```

Ratel must also have its `BIYARD_*` env set (`BIYARD_API_URL`, `BIYARD_API_KEY`,
`BIYARD_PROJECT_ID`) so the demo user actually has console points.

## 3. Verify the callback wiring
From Launchpad admin, run the company-point health check → expects `200` from
`http://localhost:8000/launchpad/health` (HMAC-signed; ratel verifies the
`x-launchpad-signature` over `"{timestamp}.{body}"`).

## 4. Demo flow
1. Log into ratel; open the **rewards/points page** → the points summary card
   shows the console balance and a **"Launchpad에서 전환하기"** button.
2. Click it → lands on launchpad `/connect?project_id=..&lp_user=<token>` (be
   logged into launchpad in the same browser; linking is automatic on arrival).
3. `/connect` shows the live balance — callback chain
   `launchpad → ratel /launchpad/points/lookup → console get_user_balance`.
4. Convert N points → `launchpad convert → ratel /launchpad/points/deduct →
   console exchange_points`. Console points drop by N; launchpad records the
   off-chain `PointConversion`.
5. Refresh ratel → reduced balance. Retrying the same convert does **not**
   double-deduct (ratel's `LaunchpadDeduction` idempotency row replays the
   stored result).

## Tests
```bash
cd app/ratel
# pure unit tests (no infra): config + crypto round-trip + HMAC verify
DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "server,bypass" --lib -- \
  launchpad_partner::crypto launchpad_partner::config

# DDB-backed integration tests (require LocalStack up via docker compose):
#   deduction_row_round_trips, health_ok_with_valid_signature,
#   callback_rejects_bad_signature, callback_rejects_project_mismatch,
#   deduct_is_idempotent_without_console
DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- launchpad_partner_tests
```

## Known simplifications (see the design spec)
- Console points are per-`month`; lookup reports `current_month()`'s balance.
- `deduct` uses the console `Exchange` tx_type (the only spend path the client
  exposes) — recorded as an Exchange on the console.
- `exchange_points` returns no remaining balance, so ratel re-queries it.
- Ratel and launchpad logins are separate (no SSO); linking is transparent on
  arrival at `/connect`.
