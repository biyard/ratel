# Passwordless Email-Code Auth — Design Spec

**Author / Date**: (pairing) · 2026-06-16
**Status**: Approved (design) — pending implementation plan

## Summary

Replace email **password** auth with a **passwordless email verification-code** flow, and **merge the separate signup and login** UIs into one flow. Social login (Google OAuth, wallet/SIWE) and phone/Telegram auth are unchanged. **No DB migration** — `User` is a schemaless DynamoDB entity, so new users simply omit `password` and existing users keep theirs (unused).

## Goal

A user enters their email, receives a code, enters the code, and is either logged in (existing account) or routed into a short profile step (new account) — no password anywhere.

## Non-goals

- No change to social login (OAuth/wallet) or phone/Telegram auth.
- No DB migration, no table/GSI changes.
- No password reset (passwords are gone from the UX).
- No change to session mechanism (tower-sessions, `{env}_sid`, `SESSION_KEY_USER_ID`).

## User flow (unified entry)

1. **Email** → "Continue" → `POST /api/auth/verification/send-verification-code` (Email).
2. **Code** → "Continue" → `POST /api/auth/login` with `Email { email, code }`:
   - code invalid/expired/exceeded → show inline error, stay on code step.
   - **account exists** → server sets session, returns user → logged in, close.
   - **account does not exist** (code valid, no user) → server returns `AuthError::UserNotFound` → frontend advances to step 3.
3. **New user profile**: display name, username, ToS agreement → `POST /api/auth/signup` with `Email { email, code, ... }` (no password) → user created (`password: None`) + session → navigate to `Route::OnboardingConnectionsPage {}` (the existing "connect" onboarding, unchanged).
- Social login buttons (Google / wallet) remain on the entry modal, unchanged.
- **Removed from UI**: password input/step, "forgot password", the separate signup entry point.

## Backend changes (reuse existing endpoints — Decision 2)

### `send_code` — `controllers/send_code.rs`
- In `send_email_code_handler`, **remove the "email already registered → `Error::Duplicate`" check**. Sending a code now works for both new and existing emails (it serves login *and* signup). Keep the existing attempt-count / expiry / block-time limits (`MAX_ATTEMPT_COUNT=5`, `EXPIRATION_TIME=1800`, `ATTEMPT_BLOCK_TIME=300`).

### `login` — `controllers/login.rs`
- Change `LoginRequest::Email` from `{ email, password, device_id }` to **`{ email, code, device_id }`**.
- New email-login logic:
  1. Verify the code (reuse the verify path: `EmailVerification::find_by_email`, bypass `"000000"`, expiry, attempt-count, value match). On failure return the matching error (`InvalidVerificationCode` / `ExpiredVerification` / `ExceededAttemptEmailVerification`).
  2. `User::find_by_email(email)`:
     - `Some(user)` → set session (`SESSION_KEY_USER_ID = user.pk`), create refresh token if `device_id`, return `LoginResponse { user, refresh_token }`.
     - `None` → return `AuthError::UserNotFound` (signal "valid code, no account → go signup"). Do **not** consume the code, so the subsequent signup call can re-verify it.
- Remove the password-hash + `find_by_email_and_password` path.

### `signup` — `controllers/signup.rs`
- Change `SignupType::Email` from `{ email, password, code }` to **`{ email, code }`**.
- Logic: verify code (same), ensure email not already registered (`Error::Duplicate` if it is), create `User::new(..., password: None)`, then existing post-create steps unchanged (referral code, session insert, optional refresh token). Return `SignupResponse`.

### Removed / deprecated (Decision 1 — full password removal)
- `controllers/reset_password.rs` and the password-reset send-code path.
- `views/forgot_password/` UI.
- Password login path + `hash_password` usage in auth (the util may be deleted if unused elsewhere; verify before removing).

### Untouched
- `User.password: Option<String>` field and the `gsi1` (`EMAIL#PASSWORD`) index **stay defined** (no table change). New users write no `password`; existing users keep theirs (ignored).
- Phone/Telegram/OAuth/wallet `LoginRequest`/`SignupType` variants and their handlers.

## Data model / no-migration

- `User` is a `DynamoEntity` (schemaless). New items omit `password`; existing items keep it. The `gsi1` email+password index remains but is no longer queried. **Zero migration.**
- Existing accounts (created with a password) log in via email+code: looked up by `find_by_email` (gsi3), password ignored. No lockout.

## Frontend changes

- Collapse `login_modal` + `sign_up_modal` into a single email-code flow (entry = the current login modal; reuse the signup modal's profile/consent step for new users).
- Steps: email → code → (existing: done) / (new: display_name + username + ToS) → signup → `OnboardingConnectionsPage`.
- Remove password fields/step, forgot-password link/route, and the separate "Sign up" entry.
- Keep Google/wallet buttons.
- Error handling: distinguish code errors (stay, show message) from `UserNotFound` (advance to profile step).
- Mobile (Tauri) calls already route through native `api_request` — no special handling needed.

## Test plan

**Server (`src/tests/auth_tests.rs` or similar):**
- send-code succeeds for both new and existing email.
- login `Email{email, code}`: existing user → 200 + session; non-existent → `UserNotFound`; wrong code → `InvalidVerificationCode`; expired → `ExpiredVerification`.
- signup `Email{email, code}` (no password): creates user with `password == None` + session.
- Regression: an existing user that has a stored password logs in via email+code (no password supplied) → success.

**E2E (Playwright):**
- Update web auth specs that fill password (now removed).
- Update `playwright/tests/tauri/create-space.spec.js` signup flow: the password step is gone; flow is email → code → profile → finished.

## Risks / notes

- Code is reusable within its 30-min window (current behavior). The new-user path relies on this (login verify → signup verify reuse the same code). Acceptable; matches existing semantics.
- All e2e/smoke tests that type a password must be updated in the same change, or CI breaks.
- `bypass` code `"000000"` continues to work for tests.
