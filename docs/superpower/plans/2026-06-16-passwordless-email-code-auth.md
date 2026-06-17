# Passwordless Email-Code Auth Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.
>
> **Commit/push policy for THIS repo:** the user has asked that commits/pushes happen only with explicit approval. The `Commit` steps below are the intended boundaries — pause and ask before running them unless the user has said otherwise.

**Goal:** Replace email password auth with a passwordless email verification-code flow, merging the separate signup/login UIs into one flow, with no DB migration.

**Architecture:** Reuse existing `login`/`signup` endpoints (Decision: reuse, not new endpoint). `login(Email)` becomes `{email, code}` — verifies the code, then `find_by_email`: existing user → session; valid code but no user → `AuthError::UserNotFound`. Frontend treats `UserNotFound` as "go to profile step" and calls `signup(Email{email, code, …})` which creates a user with `password: None`. A shared `verify_email_code()` helper (extracted from `verify_code.rs`) is used by both so verification logic (expiry, attempt count, `bypass` "000000") is DRY. `User` is a schemaless DynamoEntity, so omitting `password` needs no migration; existing users keep theirs (ignored).

**Tech Stack:** Rust (edition 2024), Dioxus 0.7 fullstack, Axum, DynamoDB (DynamoEntity), tower-sessions. Tests: `cargo test --features "full,bypass"`, Playwright.

**Spec:** [docs/superpower/2026-06-16-passwordless-email-code-auth.md](../2026-06-16-passwordless-email-code-auth.md)

---

## File Structure

**Backend (modify):**
- `app/ratel/src/features/auth/controllers/verify_code.rs` — extract `pub async fn verify_email_code(cli, email, code) -> Result<()>`; `verify_email_code_handler` calls it.
- `app/ratel/src/features/auth/controllers/send_code.rs` — drop the "email already registered" rejection in `send_email_code_handler`; remove `send_password_reset_code_handler` + `send_password_reset_email_code_handler`.
- `app/ratel/src/features/auth/controllers/login.rs` — `LoginRequest::Email { email, code, device_id }`; `login_with_email` verifies code + `find_by_email`.
- `app/ratel/src/features/auth/controllers/signup.rs` — `SignupType::Email { email, code }`; `signup_with_email` (no password) using shared verify.
- `app/ratel/src/features/auth/controllers/reset_password.rs` — delete.
- `app/ratel/src/features/auth/controllers/mod.rs` — drop `reset_password` + the removed route registrations.
- `app/ratel/src/app.rs` / route enum — remove the forgot-password route if registered.

**Backend (keep, do NOT change):** `common/models/auth/user.rs` (`password: Option<String>` + `gsi1` stay — no migration), phone/oauth/wallet/telegram paths.

**Frontend (modify):**
- `app/ratel/src/features/auth/components/login_modal/mod.rs` — becomes the unified email-code entry.
- `app/ratel/src/features/auth/components/sign_up_modal/mod.rs` — profile/consent step reused for new users; remove password/email-only-signup entry, or fold into login_modal.
- `app/ratel/src/features/auth/views/forgot_password/` — delete (route + view).
- `app/ratel/src/features/auth/i18n.rs` (+ modal i18n) — remove password strings.

**Tests (modify):**
- `app/ratel/src/tests/auth_tests.rs` (or the existing auth test file) — server tests.
- `playwright/tests/web/*auth*.spec.js` — remove password steps.
- `playwright/tests/tauri/create-space.spec.js` — signup flow no longer types a password.

---

## Task 1: Extract shared `verify_email_code` helper

**Files:**
- Modify: `app/ratel/src/features/auth/controllers/verify_code.rs`

- [ ] **Step 1: Add the shared helper and make the handler delegate to it.** Replace the body of `verify_email_code_handler` (lines 40-86) so the verification logic lives in a reusable `pub` fn:

```rust
/// Shared email-code verification used by verify-code, login, and signup.
/// Returns Ok(()) when the code is valid; increments attempt_count and errors
/// otherwise. Honors the `bypass` test code "000000". Does NOT consume the
/// code (so login→signup can reuse the same code within its window).
#[cfg(feature = "server")]
pub async fn verify_email_code(
    cli: &aws_sdk_dynamodb::Client,
    email: &str,
    code: &str,
) -> Result<()> {
    use crate::features::auth::constants::MAX_ATTEMPT_COUNT;

    let now = crate::common::utils::time::get_now_timestamp();
    let (verification_list, _) = EmailVerification::find_by_email(
        cli,
        email,
        EmailVerificationQueryOption::builder().limit(1),
    )
    .await?;

    if verification_list.is_empty() {
        return Err(Error::NotFoundVerificationCode);
    }

    #[cfg(feature = "bypass")]
    if code.eq("000000") {
        return Ok(());
    }

    let email_verification = verification_list[0].clone();

    if email_verification.attempt_count >= MAX_ATTEMPT_COUNT {
        return Err(Error::ExceededAttemptEmailVerification);
    }
    if email_verification.expired_at < now {
        return Err(Error::ExpiredVerification);
    }
    if email_verification.value != code {
        EmailVerification::updater(email_verification.pk, email_verification.sk)
            .increase_attempt_count(1)
            .execute(cli)
            .await?;
        return Err(Error::InvalidVerificationCode);
    }

    Ok(())
}

#[cfg(feature = "server")]
async fn verify_email_code_handler(email: String, code: String) -> Result<VerifyCodeResponse> {
    verify_email_code(&crate::features::auth::config::get().dynamodb(), &email, &code).await?;
    Ok(VerifyCodeResponse { success: true })
}
```

- [ ] **Step 2: Verify build (server).**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: compiles clean (no unused warnings).

- [ ] **Step 3: Commit** (ask first per repo policy).

```bash
git add app/ratel/src/features/auth/controllers/verify_code.rs
git commit -m "refactor(auth): extract shared verify_email_code helper"
```

---

## Task 2: Allow send-code for existing emails

**Files:**
- Modify: `app/ratel/src/features/auth/controllers/send_code.rs`

- [ ] **Step 1: Drop the duplicate-email rejection.** Replace `send_email_code_handler` (lines 119-134) with:

```rust
#[cfg(feature = "server")]
pub async fn send_email_code_handler(email: String) -> Result<SendCodeResponse> {
    // No existence check: this code serves BOTH login (existing account) and
    // signup (new account) in the unified passwordless flow.
    send_email_code(email).await
}
```

- [ ] **Step 2: Remove the password-reset send path.** Delete `send_password_reset_code_handler` (lines 36-41), `SendPasswordResetCodeRequest` (lines 22-26), and `send_password_reset_email_code_handler` (lines 43-59). (`send_email_code` stays — it's the shared sender.)

- [ ] **Step 3: Verify build.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server`
Expected: compiles clean. If `send_password_reset_code_handler` was registered in a router/`mod.rs`, the build error will point to it — remove that registration too (handled in Task 5).

- [ ] **Step 4: Commit** (ask first).

```bash
git add app/ratel/src/features/auth/controllers/send_code.rs
git commit -m "feat(auth): send verification code for existing emails too; drop password-reset send"
```

---

## Task 3: `login(Email)` becomes email+code

**Files:**
- Modify: `app/ratel/src/features/auth/controllers/login.rs`
- Test: `app/ratel/src/tests/auth_tests.rs`

- [ ] **Step 1: Write failing tests.** Add to the auth test file (uses `TestContext`; `bypass` code is "000000"):

```rust
#[tokio::test]
async fn test_email_code_login_existing_user() {
    let ctx = TestContext::setup().await;
    // create_email_user is a helper that signs up an email user; if it doesn't
    // exist, create one via the signup endpoint with code "000000".
    let email = format!("login-{}@test.com", uuid::Uuid::now_v7());
    // send code
    let (s, _, _) = crate::test_post! { app: ctx.app.clone(),
        path: "/api/auth/verification/send-verification-code",
        body: { "email": email } };
    assert_eq!(s, 200);
    // signup (new user, no password)
    let username = format!("u{}", chrono::Utc::now().timestamp_micros());
    let (s, _, _) = crate::test_post! { app: ctx.app.clone(), path: "/api/auth/signup",
        body: { "email": email, "code": "000000", "display_name": "T",
                "username": username, "profile_url": "", "description": "",
                "term_agreed": true, "informed_agreed": false } };
    assert_eq!(s, 200);
    // login with email + code
    let (s, _, body) = crate::test_post! { app: ctx.app.clone(), path: "/api/auth/login",
        body: { "email": email, "code": "000000" } };
    assert_eq!(s, 200, "email-code login: {:?}", body);
}

#[tokio::test]
async fn test_email_code_login_unknown_user_returns_not_found() {
    let ctx = TestContext::setup().await;
    let email = format!("nouser-{}@test.com", uuid::Uuid::now_v7());
    let (_, _, _) = crate::test_post! { app: ctx.app.clone(),
        path: "/api/auth/verification/send-verification-code", body: { "email": email } };
    let (s, _, _) = crate::test_post! { app: ctx.app.clone(), path: "/api/auth/login",
        body: { "email": email, "code": "000000" } };
    assert_ne!(s, 200, "unknown user must not log in (frontend branches to signup)");
}
```

- [ ] **Step 2: Run tests, verify they fail.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_email_code_login`
Expected: FAIL (login still expects `password`; request shape mismatch / deserialize error).

- [ ] **Step 3: Change `LoginRequest::Email` and the handler.** In `login.rs`, change the `Email` variant (lines 20-25) to:

```rust
    Email {
        email: String,
        code: String,
        #[serde(default)]
        device_id: Option<String>,
    },
```

Update the match arm (lines 70-74) to pass `code`:

```rust
        LoginRequest::Email { email, code, device_id: _ } => login_with_email(cli, email, code).await?,
```

Replace `login_with_email` (lines 232-260) with:

```rust
#[cfg(feature = "server")]
pub async fn login_with_email(
    cli: &aws_sdk_dynamodb::Client,
    email: String,
    code: String,
) -> Result<User> {
    crate::features::auth::controllers::verify_code::verify_email_code(cli, &email, &code).await?;

    let (users, _) =
        User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    // Valid code but no account → caller (frontend) treats this as "go signup".
    users.into_iter().next().ok_or(AuthError::UserNotFound.into())
}
```

(Adjust the `verify_email_code` path to however controllers are re-exported; if `controllers` are `pub use`d under `auth::*`, `verify_email_code(...)` may be importable directly.)

- [ ] **Step 4: Run tests, verify they pass.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_email_code_login`
Expected: PASS.

- [ ] **Step 5: Commit** (ask first).

```bash
git add app/ratel/src/features/auth/controllers/login.rs app/ratel/src/tests/auth_tests.rs
git commit -m "feat(auth): passwordless email-code login"
```

---

## Task 4: `signup(Email)` drops password

**Files:**
- Modify: `app/ratel/src/features/auth/controllers/signup.rs`
- Test: `app/ratel/src/tests/auth_tests.rs`

- [ ] **Step 1: Write failing test.**

```rust
#[tokio::test]
async fn test_email_code_signup_no_password_creates_user() {
    let ctx = TestContext::setup().await;
    let email = format!("signup-{}@test.com", uuid::Uuid::now_v7());
    let username = format!("u{}", chrono::Utc::now().timestamp_micros());
    let (_, _, _) = crate::test_post! { app: ctx.app.clone(),
        path: "/api/auth/verification/send-verification-code", body: { "email": email } };
    let (s, _, body) = crate::test_post! { app: ctx.app.clone(), path: "/api/auth/signup",
        body: { "email": email, "code": "000000", "display_name": "T",
                "username": username, "profile_url": "", "description": "",
                "term_agreed": true, "informed_agreed": false } };
    assert_eq!(s, 200, "passwordless signup: {:?}", body);
    // No `password` field is sent; user is created and session established.
}
```

- [ ] **Step 2: Run test, verify it fails.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_email_code_signup_no_password`
Expected: FAIL (current `SignupType::Email` requires `password`).

- [ ] **Step 3: Change `SignupType::Email` and the email signup fn.** In `signup.rs`, change the `Email` variant (lines 43-47) to:

```rust
    Email {
        email: String,
        code: String,
    },
```

Update the match arm (lines 91-95):

```rust
        SignupType::Email { email, code } => signup_with_email(cli, req.clone(), email, code).await?,
```

Rename/replace `signup_with_email_password` (lines 163-224) with `signup_with_email` (drop password, drop the `hash_password` import use, reuse the shared verifier):

```rust
#[cfg(feature = "server")]
async fn signup_with_email(
    cli: &aws_sdk_dynamodb::Client,
    SignupRequest { display_name, username, profile_url, term_agreed, informed_agreed, .. }: SignupRequest,
    email: String,
    code: String,
) -> Result<User> {
    crate::features::auth::controllers::verify_code::verify_email_code(cli, &email, &code).await?;

    let (users, _) = User::find_by_email(cli, &email, UserQueryOption::builder().limit(1)).await?;
    if !users.is_empty() {
        return Err(Error::Duplicate(format!("Email already registered: {}", email)));
    }

    ensure_username_available(cli, &username).await?;

    let user = User::new(
        display_name, email, profile_url, term_agreed, informed_agreed,
        UserType::Individual, username,
        None, // passwordless
    );
    user.create(cli).await?;
    Ok(user)
}
```

Remove the now-unused `password::hash_password` import (line 10).

- [ ] **Step 4: Run test, verify it passes.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- test_email_code_signup_no_password`
Expected: PASS.

- [ ] **Step 5: Commit** (ask first).

```bash
git add app/ratel/src/features/auth/controllers/signup.rs app/ratel/src/tests/auth_tests.rs
git commit -m "feat(auth): passwordless email-code signup"
```

---

## Task 5: Remove password endpoints + reset/forgot

**Files:**
- Delete: `app/ratel/src/features/auth/controllers/reset_password.rs`
- Modify: `app/ratel/src/features/auth/controllers/mod.rs` (drop `mod reset_password;` + re-exports + any router registration of `reset_password_handler` / `send_password_reset_code_handler`)
- Delete: `app/ratel/src/features/auth/views/forgot_password/` (and its `mod` declaration)
- Modify: the `Route` enum / router to drop the forgot-password route

- [ ] **Step 1: Delete reset_password controller + module wiring.** `rm app/ratel/src/features/auth/controllers/reset_password.rs`; remove its `mod`/`pub use` in `controllers/mod.rs`; remove any `.route("/api/auth/reset", ...)` / `send-password-reset-code` registration in the router (search `reset_password_handler`, `send_password_reset_code_handler`).

- [ ] **Step 2: Delete forgot-password view + route.** `rm -rf app/ratel/src/features/auth/views/forgot_password`; remove its `mod` declaration; remove the `ForgotPassword*` route variant + any `Link`/`nav` to it (search `forgot_password`, `ForgotPassword`, `Forgot password`).

- [ ] **Step 3: Verify build (all targets).**

Run:
```
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
```
Expected: both clean. Fix any dangling references the compiler flags.

- [ ] **Step 4: Commit** (ask first).

```bash
git add -A app/ratel/src/features/auth
git commit -m "chore(auth): remove password reset + forgot-password flow"
```

---

## Task 6: Unify the auth modal into the email-code flow

**Files:**
- Modify: `app/ratel/src/features/auth/components/login_modal/mod.rs` (primary entry)
- Modify: `app/ratel/src/features/auth/components/sign_up_modal/mod.rs` (reuse profile/consent step; remove password + email-signup entry)

- [ ] **Step 1: Read both modal files fully** before editing (they are large RSX). Identify: the step/signal state, where `send_code_handler` / `verify_code_handler` / `login_handler` / `signup_handler` are called, the password input/step, and the social buttons.

- [ ] **Step 2: Implement the unified flow** in `login_modal`. State machine (signals): `step: Email | Code | Profile`. Behavior:
  - **Email step:** input email → "Continue" calls `send_code_handler(SendCodeRequest::Email { email })` → on Ok, go to Code step.
  - **Code step:** input code → "Continue" calls `login_handler(LoginRequest::Email { email, code, device_id })`:
    - `Ok(resp)` → store user in context, `popup.close()`, run `on_success` (existing user logged in).
    - `Err(e)` where `e` is `UserNotFound` → go to Profile step (new user; code already verified).
    - other `Err` (invalid/expired code) → show inline error, stay on Code step.
  - **Profile step:** display_name, username, ToS (+ optional newsletter) → "Finished" calls `signup_handler(SignupRequest { signup_type: Email { email, code }, display_name, username, profile_url: default, description: "", term_agreed, informed_agreed, phone_number: None, device_id })` → on Ok, navigate `nav.replace(Route::OnboardingConnectionsPage {})` (mirror current signup success at sign_up_modal lines ~411-420).
  - Keep Google / wallet buttons calling their existing handlers unchanged.
  - Remove: password input + password step, "Forgot password" link, the separate "Sign up" entry (signup is now the new-user branch). Distinguishing `UserNotFound`: match on the typed error/`Display`; prefer matching the `AuthError::UserNotFound` variant if the client error type preserves it, else compare the translated message — confirm how `common::Error` surfaces on the client and branch accordingly.

- [ ] **Step 3: Lint + format** every changed `.rs`:

```
rustywind --custom-regex 'class: "(.*)"' --write app/ratel/src/features/auth/components/login_modal/mod.rs
dx fmt -f app/ratel/src/features/auth/components/login_modal/mod.rs
```
(repeat for sign_up_modal/mod.rs)

- [ ] **Step 4: Verify build.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web`
Expected: clean.

- [ ] **Step 5: Commit** (ask first).

```bash
git add app/ratel/src/features/auth/components
git commit -m "feat(auth): unified passwordless email-code login/signup modal"
```

---

## Task 7: i18n cleanup

**Files:**
- Modify: auth modal `i18n.rs` files

- [ ] **Step 1: Remove password-related translation keys** (password, confirm password, forgot password, password rules) and add any new strings used by the unified flow (e.g., a single "Continue" / code-step copy). Ensure every user-facing string in the new flow comes from `translate!` (EN + KO). Grep the changed modal RSX for raw Korean/English literals.

- [ ] **Step 2: Verify build.** `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --features web` → clean.

- [ ] **Step 3: Commit** (ask first). `git commit -m "chore(auth): i18n for passwordless flow"`

---

## Task 8: Full build verification

- [ ] **Step 1: All targets compile.**

```
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features mobile
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web
```
Expected: all clean.

- [ ] **Step 2: Run auth server tests.**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- auth`
Expected: PASS (new email-code tests + existing auth tests).

---

## Task 9: Update web Playwright auth specs

**Files:**
- Modify: `playwright/tests/web/*` specs that sign up / log in with a password

- [ ] **Step 1: Find specs that type a password.** `grep -rn "password\|Enter your password\|Re-enter your password" playwright/tests/web`.

- [ ] **Step 2: Rewrite those flows** to: email → "Send"/"Continue" → fill code `000000` → "Verify"/"Continue" → (new user) fill display name + username + accept ToS → "Finished". Remove password fills and the password-confirm step. Use the existing shared helpers (`goto`, `click`, `fill`).

- [ ] **Step 3: Run.** `cd playwright && npx playwright test tests/web/<changed>.spec.js` (against a `--features bypass` backend). Expected: PASS.

- [ ] **Step 4: Commit** (ask first). `git commit -m "test(auth): web e2e passwordless flow"`

---

## Task 10: Update Tauri smoke test signup flow

**Files:**
- Modify: `playwright/tests/tauri/create-space.spec.js` (signup section ~lines 280-360)

- [ ] **Step 1: Remove the password steps** from the signup section: delete the two `fillSelector('input[placeholder="Enter your password"]' …)` and `'Re-enter your password'` calls. Flow becomes: email → Send → code `000000` → Verify → display name + username → accept ToS → "Finished Sign-up". (The API calls were already switched to `api_request` invoke in a prior change — leave those.)

- [ ] **Step 2: Verify locally if possible** (Android emulator + bypass image) or rely on CI `playwright-tests`/the tauri smoke job. Expected: signup → team → post → space passes.

- [ ] **Step 3: Commit** (ask first). `git commit -m "test(tauri): passwordless signup in smoke test"`

---

## Self-Review (completed during planning)

- **Spec coverage:** send-code-for-existing (T2), login email+code (T3), signup no-password (T4), full password removal (T5), unified modal + OnboardingConnectionsPage (T6), i18n (T7), no-migration (User.password/gsi1 untouched — stated in T4/file-structure), tests (T3/T4/T8), e2e updates (T9/T10). Phone/Telegram/OAuth/wallet untouched (not in any task). ✓
- **No placeholders:** backend steps contain real code; frontend steps specify exact flow + calls and require reading the large RSX first (noted). ✓
- **Type consistency:** `verify_email_code(cli, &str, &str) -> Result<()>` defined in T1, called in T3/T4. `LoginRequest::Email { email, code, device_id }` (T3) and `SignupType::Email { email, code }` (T4) match the frontend calls in T6. ✓

---

## Open implementation notes

- **`UserNotFound` branching on the client:** confirm whether `common::Error` deserializes to a typed variant on the client or only a message. If only a message, branch on the translated `AuthError::UserNotFound` text (or add a small typed discriminator). Resolve in T6 Step 2.
- **Code reuse window:** login's `verify_email_code` must NOT consume the code (it doesn't — verify only increments on mismatch), so the subsequent signup call re-verifies the same code. Confirmed in T1.
- **`hash_password` util:** after T3/T4 it may be unused; remove it only if no other caller (grep before deleting).
