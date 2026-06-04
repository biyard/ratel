# Launchpad Point Integration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make ratel act as the external point provider for Launchpad's company-point flow — receiving Launchpad's HMAC-signed `lookup`/`deduct` callbacks and delegating them to the Biyard console via the existing `BiyardService`.

**Architecture:** A new `features/launchpad_partner/` module exposes three unauthenticated, HMAC-verified Axum routes (`/launchpad/health|points/lookup|points/deduct`). `lookup` → `BiyardService.get_user_balance`; `deduct` → idempotency-guarded `BiyardService.exchange_points` + balance re-query. A small `LaunchpadDeduction` DynamoDB row makes `deduct` idempotent. A view component renders the per-user "convert on Launchpad" link carrying an AES-256-GCM token. Launchpad and the console are unchanged.

**Tech Stack:** Rust (edition 2024), Dioxus fullstack, Axum, DynamoDB (`DynamoEntity` derive), `aes-gcm`, `hmac`, `sha2`, `hex`, `base64`.

**Spec:** `docs/superpowers/specs/2026-06-04-launchpad-point-integration-design.md`

**Working dir:** `/Users/leechanhui/Projects/ratel-copy/ratel`, branch `feat/launchpad-point-integration`.

**Build/test commands (run from repo root):**
- Server compile: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features server`
- Tests: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- <test_name>` (requires `make infra` / LocalStack DynamoDB up)
- Web compile: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features web`

---

## File Structure

| File | Responsibility |
|---|---|
| `app/ratel/src/common/types/entity_type.rs` (modify) | Add `LaunchpadDeduction(String)` sk variant |
| `app/ratel/src/features/mod.rs` (modify) | `pub mod launchpad_partner;` |
| `app/ratel/src/features/launchpad_partner/mod.rs` (create) | Module wiring + re-exports |
| `app/ratel/src/features/launchpad_partner/config.rs` (create) | `LaunchpadPartnerConfig` (env reads) |
| `app/ratel/src/features/launchpad_partner/crypto.rs` (create) | `encrypt_user_token` + `verify_signature` |
| `app/ratel/src/features/launchpad_partner/error.rs` (create) | `PartnerError` + status mapping |
| `app/ratel/src/features/launchpad_partner/types.rs` (create) | callback request/response DTOs |
| `app/ratel/src/features/launchpad_partner/models/mod.rs` (create) | model re-export |
| `app/ratel/src/features/launchpad_partner/models/deduction.rs` (create) | `LaunchpadDeduction` DynamoEntity |
| `app/ratel/src/features/launchpad_partner/controllers.rs` (create) | health/lookup/deduct inner handlers |
| `app/ratel/src/features/launchpad_partner/server.rs` (create) | Axum router + outer handlers |
| `app/ratel/src/features/launchpad_partner/views/mod.rs` (create) | view re-export |
| `app/ratel/src/features/launchpad_partner/views/connect_button.rs` (create) | entry button component |
| `app/ratel/src/features/launchpad_partner/i18n.rs` (create) | EN/KO strings |
| `app/ratel/src/common/run.rs` (modify) | merge partner router |
| `app/ratel/src/tests/setup.rs` (modify) | merge partner router into test app |
| `app/ratel/src/tests/mod.rs` (modify) | register `launchpad_partner_tests` |
| `app/ratel/src/tests/launchpad_partner_tests.rs` (create) | integration tests |
| `RUN-launchpad-demo.md` (create, repo root) | manual E2E config + scenario |

---

## Task 1: EntityType variant + module skeleton

**Files:**
- Modify: `app/ratel/src/common/types/entity_type.rs`
- Create: `app/ratel/src/features/launchpad_partner/mod.rs`
- Modify: `app/ratel/src/features/mod.rs`

- [ ] **Step 1: Add the `LaunchpadDeduction` sk variant**

In `app/ratel/src/common/types/entity_type.rs`, find the `pub enum EntityType { ... }` block (it derives `DynamoEnum, SubPartition`). Add a variant alongside the existing ones (place it after `Session,` or any existing simple variant — variant order is irrelevant). The `DynamoEnum` derive produces the `LAUNCHPAD_DEDUCTION#<id>` prefix from the variant name automatically:

```rust
    LaunchpadDeduction(String),
```

- [ ] **Step 2: Create the module file**

Create `app/ratel/src/features/launchpad_partner/mod.rs`:

```rust
//! Launchpad company-point integration.
//!
//! Ratel acts as the external point provider for Launchpad's per-service
//! token economy. Launchpad calls our HMAC-signed callbacks; we delegate
//! balance/deduct to the Biyard console via `BiyardService`. We own no
//! point state except an idempotency ledger (`LaunchpadDeduction`).

pub mod config;
pub mod crypto;
pub mod error;
pub mod types;
pub mod views;

#[cfg(feature = "server")]
pub mod controllers;
#[cfg(feature = "server")]
pub mod models;
#[cfg(feature = "server")]
pub mod server;

mod i18n;
```

- [ ] **Step 3: Register the module**

In `app/ratel/src/features/mod.rs`, add alongside the other `pub mod` lines:

```rust
pub mod launchpad_partner;
```

- [ ] **Step 4: Create placeholder submodule files so the tree compiles incrementally**

Create empty-but-valid stubs (filled in later tasks). Create `app/ratel/src/features/launchpad_partner/views/mod.rs`:

```rust
pub mod connect_button;
pub use connect_button::*;
```

Create `app/ratel/src/features/launchpad_partner/models/mod.rs`:

```rust
#![cfg(feature = "server")]

pub mod deduction;
pub use deduction::*;
```

> The remaining referenced files (`config.rs`, `crypto.rs`, `error.rs`, `types.rs`, `controllers.rs`, `server.rs`, `i18n.rs`, `views/connect_button.rs`, `models/deduction.rs`) are created in the tasks below. Do not compile until Task 2+ create them; this task ends without a build.

- [ ] **Step 5: Commit**

```bash
cd /Users/leechanhui/Projects/ratel-copy/ratel
git add app/ratel/src/common/types/entity_type.rs app/ratel/src/features/mod.rs app/ratel/src/features/launchpad_partner/mod.rs app/ratel/src/features/launchpad_partner/views/mod.rs app/ratel/src/features/launchpad_partner/models/mod.rs
git commit -m "feat(launchpad_partner): module skeleton + LaunchpadDeduction sk variant"
```

---

## Task 2: Config module

**Files:**
- Create: `app/ratel/src/features/launchpad_partner/config.rs`

- [ ] **Step 1: Write the failing test**

Create `app/ratel/src/features/launchpad_partner/config.rs` with the test first:

```rust
//! Env-backed config for the Launchpad partner integration.
//! Mirrors `common/config/server/biyard.rs`: compile-time `option_env!`
//! for secrets/ids, runtime `std::env::var` for the URL.

#[derive(Debug, Clone)]
pub struct LaunchpadPartnerConfig {
    /// Base URL of the Launchpad app the connect button points at.
    pub base_url: String,
    /// Launchpad project id this ratel instance is registered as.
    pub project_id: &'static str,
    /// Shared secret: AES key material for the user token AND HMAC key
    /// for verifying Launchpad callbacks. Must match Launchpad's project
    /// company_secret_key.
    pub shared_secret: &'static str,
    /// Symbol returned to Launchpad in point lookups (cosmetic).
    pub point_symbol: &'static str,
}

impl Default for LaunchpadPartnerConfig {
    fn default() -> Self {
        Self {
            base_url: std::env::var("LAUNCHPAD_BASE_URL")
                .unwrap_or_else(|_| "http://localhost:8080".to_string()),
            project_id: option_env!("LAUNCHPAD_PROJECT_ID").unwrap_or("launchpad-demo"),
            shared_secret: option_env!("LAUNCHPAD_PARTNER_SECRET")
                .unwrap_or("dev-demo-shared-secret-change-me"),
            point_symbol: option_env!("LAUNCHPAD_POINT_SYMBOL").unwrap_or("P"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_present() {
        let cfg = LaunchpadPartnerConfig::default();
        assert!(!cfg.base_url.is_empty());
        assert!(!cfg.project_id.is_empty());
        assert!(!cfg.shared_secret.is_empty());
        assert_eq!(cfg.point_symbol, "P");
    }
}
```

- [ ] **Step 2: Run the test to verify it passes**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- launchpad_partner::config::tests::defaults_are_present`
Expected: PASS (this is a config-only task; the test is a smoke test of `Default`).

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/launchpad_partner/config.rs
git commit -m "feat(launchpad_partner): env-backed config"
```

---

## Task 3: Crypto (token encrypt + signature verify)

**Files:**
- Create: `app/ratel/src/features/launchpad_partner/crypto.rs`

- [ ] **Step 1: Write the failing tests**

Create `app/ratel/src/features/launchpad_partner/crypto.rs`:

```rust
//! Crypto for the Launchpad handoff, byte-compatible with Launchpad's
//! `demo_preview/server.rs` and `demo/brand-demo/server.js`.
//!
//! - `encrypt_user_token`: AES-256-GCM(key = SHA256(secret)), output
//!   `base64url(nonce[12] ‖ ciphertext ‖ tag)`. Launchpad decrypts it.
//! - `verify_signature`: HMAC-SHA256(secret, "{timestamp}.{raw_body}")
//!   compared against the hex `x-launchpad-signature` header.

use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

/// Encrypt a company user key (ratel user uuid) into the `lp_user` token.
pub fn encrypt_user_token(secret: &str, user_id: &str) -> Result<String, String> {
    let key = Sha256::digest(secret.as_bytes());
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(|e| e.to_string())?;
    // 12-byte nonce. Production would use a CSPRNG; for the demo a uuid v4
    // tail is sufficient entropy and avoids a getrandom feature gate.
    let uuid = uuid::Uuid::new_v4();
    let nonce_bytes = &uuid.as_bytes()[..12];
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(nonce_bytes), user_id.as_bytes())
        .map_err(|e| e.to_string())?;
    let mut blob = Vec::with_capacity(12 + ciphertext.len());
    blob.extend_from_slice(nonce_bytes);
    blob.extend_from_slice(&ciphertext);
    Ok(URL_SAFE_NO_PAD.encode(blob))
}

/// Verify a Launchpad callback signature over `"{timestamp}.{raw_body}"`.
pub fn verify_signature(secret: &str, timestamp: &str, signature_hex: &str, raw_body: &str) -> bool {
    if timestamp.is_empty() || signature_hex.is_empty() {
        return false;
    }
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(timestamp.as_bytes());
    mac.update(b".");
    mac.update(raw_body.as_bytes());
    let Ok(sig) = hex::decode(signature_hex) else {
        return false;
    };
    mac.verify_slice(&sig).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sign(secret: &str, timestamp: &str, body: &str) -> String {
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(timestamp.as_bytes());
        mac.update(b".");
        mac.update(body.as_bytes());
        hex::encode(mac.finalize().into_bytes())
    }

    // Replicates Launchpad's decrypt to prove the token round-trips.
    fn decrypt(secret: &str, token: &str) -> String {
        let blob = URL_SAFE_NO_PAD.decode(token).unwrap();
        let (nonce, ct) = blob.split_at(12);
        let key = Sha256::digest(secret.as_bytes());
        let cipher = Aes256Gcm::new_from_slice(&key).unwrap();
        let pt = cipher.decrypt(Nonce::from_slice(nonce), ct).unwrap();
        String::from_utf8(pt).unwrap()
    }

    #[test]
    fn token_round_trips_for_launchpad() {
        let secret = "lps_test_secret";
        let token = encrypt_user_token(secret, "user-abc-123").unwrap();
        assert_eq!(decrypt(secret, &token), "user-abc-123");
    }

    #[test]
    fn valid_signature_verifies() {
        let secret = "lps_test_secret";
        let ts = "1717459200000";
        let body = r#"{"project_id":"p","company_user_key":"u"}"#;
        let sig = sign(secret, ts, body);
        assert!(verify_signature(secret, ts, &sig, body));
    }

    #[test]
    fn tampered_body_fails() {
        let secret = "lps_test_secret";
        let ts = "1717459200000";
        let sig = sign(secret, ts, "original");
        assert!(!verify_signature(secret, ts, &sig, "tampered"));
    }

    #[test]
    fn missing_parts_fail() {
        assert!(!verify_signature("s", "", "ab", "body"));
        assert!(!verify_signature("s", "ts", "", "body"));
        assert!(!verify_signature("s", "ts", "nothex!!", "body"));
    }
}
```

- [ ] **Step 2: Run the tests to verify they pass**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- launchpad_partner::crypto::tests`
Expected: 4 tests PASS. If `uuid` is not in scope, confirm it is a workspace dep (it is — used across ratel) and refer to it as `uuid::Uuid`.

- [ ] **Step 3: Commit**

```bash
git add app/ratel/src/features/launchpad_partner/crypto.rs
git commit -m "feat(launchpad_partner): AES token + HMAC signature verify (launchpad-compatible)"
```

---

## Task 4: `LaunchpadDeduction` idempotency model

**Files:**
- Create: `app/ratel/src/features/launchpad_partner/models/deduction.rs`
- Test: `app/ratel/src/tests/launchpad_partner_tests.rs` (created here, extended later)
- Modify: `app/ratel/src/tests/mod.rs`

- [ ] **Step 1: Write the model**

Create `app/ratel/src/features/launchpad_partner/models/deduction.rs`:

```rust
//! Idempotency ledger for Launchpad point deductions. The Biyard console
//! `exchange_points` has no idempotency key, so a retried Launchpad
//! `deduct` would double-spend without this guard.

#![cfg(feature = "server")]

use crate::common::*;

#[derive(Debug, Default, Clone, Serialize, Deserialize, DynamoEntity)]
pub struct LaunchpadDeduction {
    pub pk: Partition,
    pub sk: EntityType,

    pub idempotency_key: String,
    pub company_user_key: String,
    pub point_amount: i64,
    pub brand_tx_id: String,
    pub remaining_points: i64,
    pub created_at: i64,
}

#[cfg(feature = "server")]
impl LaunchpadDeduction {
    pub fn new(
        company_user_key: &str,
        idempotency_key: &str,
        point_amount: i64,
        brand_tx_id: &str,
        remaining_points: i64,
    ) -> Self {
        Self {
            pk: Partition::User(company_user_key.to_string()),
            sk: EntityType::LaunchpadDeduction(idempotency_key.to_string()),
            idempotency_key: idempotency_key.to_string(),
            company_user_key: company_user_key.to_string(),
            point_amount,
            brand_tx_id: brand_tx_id.to_string(),
            remaining_points,
            created_at: crate::common::utils::time::get_now_timestamp_millis(),
        }
    }
}
```

> Verify `get_now_timestamp_millis` is the correct helper — it is used in `common/models/reward/pending_reward.rs`. If a different name is in use there, mirror that exact call.

- [ ] **Step 2: Write the failing test**

Create `app/ratel/src/tests/launchpad_partner_tests.rs`:

```rust
use super::*;
use crate::common::types::{EntityType, Partition};
use crate::features::launchpad_partner::models::LaunchpadDeduction;

#[tokio::test]
async fn deduction_row_round_trips() {
    let ctx = TestContext::setup().await;
    let user = format!("u-{}", uuid::Uuid::new_v4());
    let key = format!("lp_{}", uuid::Uuid::new_v4());

    let row = LaunchpadDeduction::new(&user, &key, 500, "tx_demo", 740);
    row.create(&ctx.ddb).await.expect("create");

    let fetched = LaunchpadDeduction::get(
        &ctx.ddb,
        Partition::User(user.clone()),
        Some(EntityType::LaunchpadDeduction(key.clone())),
    )
    .await
    .expect("get")
    .expect("row present");

    assert_eq!(fetched.point_amount, 500);
    assert_eq!(fetched.brand_tx_id, "tx_demo");
    assert_eq!(fetched.remaining_points, 740);
}
```

- [ ] **Step 3: Register the test module**

In `app/ratel/src/tests/mod.rs`, add alongside the other `mod *_tests;` lines:

```rust
mod launchpad_partner_tests;
```

- [ ] **Step 4: Run the test**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- deduction_row_round_trips`
Expected: PASS (requires LocalStack DynamoDB up via `make infra`). If `create`/`get` signatures differ, cross-check against `DemoPointBalance` usage in the launchpad repo (`get(cli, pk, Some(sk))`) — the `DynamoEntity` derive is the same macro family.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/launchpad_partner/models/deduction.rs app/ratel/src/tests/launchpad_partner_tests.rs app/ratel/src/tests/mod.rs
git commit -m "feat(launchpad_partner): LaunchpadDeduction idempotency model + test"
```

---

## Task 5: Error type + callback DTOs + handlers

**Files:**
- Create: `app/ratel/src/features/launchpad_partner/error.rs`
- Create: `app/ratel/src/features/launchpad_partner/types.rs`
- Create: `app/ratel/src/features/launchpad_partner/controllers.rs`

- [ ] **Step 1: Create the error type**

Create `app/ratel/src/features/launchpad_partner/error.rs`:

```rust
//! Errors for Launchpad callbacks, mapped to HTTP status codes mirroring
//! the reference brand-demo.

#[derive(Debug, Clone, thiserror::Error)]
pub enum PartnerError {
    #[error("invalid signature")]
    InvalidSignature,
    #[error("project mismatch")]
    ProjectMismatch,
    #[error("unknown user")]
    UnknownUser,
    #[error("invalid point amount")]
    InvalidAmount,
    #[error("insufficient points")]
    Insufficient,
    #[error("server error")]
    Server,
}

#[cfg(feature = "server")]
impl PartnerError {
    pub fn status(&self) -> u16 {
        match self {
            PartnerError::InvalidSignature => 401,
            PartnerError::ProjectMismatch => 403,
            PartnerError::UnknownUser => 404,
            PartnerError::InvalidAmount => 400,
            PartnerError::Insufficient => 409,
            PartnerError::Server => 500,
        }
    }
}
```

- [ ] **Step 2: Create the callback DTOs**

Create `app/ratel/src/features/launchpad_partner/types.rs`:

```rust
//! Wire DTOs for Launchpad point callbacks. Field names match Launchpad's
//! `demo_preview/server.rs` exactly — do not rename.

use crate::common::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct LookupBody {
    pub project_id: String,
    pub company_user_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct LookupResponse {
    pub available_points: i64,
    pub point_symbol: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DeductBody {
    pub project_id: String,
    pub company_user_key: String,
    pub point_amount: i64,
    pub idempotency_key: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct DeductResponse {
    pub brand_tx_id: String,
    pub deducted_points: i64,
    pub remaining_points: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct HealthBody {
    pub project_id: String,
    pub check: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub ok: bool,
    pub project_id: String,
    pub service: String,
}
```

> If `Serialize`/`Deserialize` are not re-exported from `crate::common::*`, import them from `serde` directly (`use serde::{Deserialize, Serialize};`). Check a sibling DTO file (e.g. `services/biyard/mod.rs`) for the project's convention.

- [ ] **Step 3: Create the inner handlers**

Create `app/ratel/src/features/launchpad_partner/controllers.rs`:

```rust
//! Inner callback handlers. Each takes already-verified inputs and
//! delegates point reads/writes to the Biyard console via `BiyardService`.

#![cfg(feature = "server")]

use crate::common::types::Partition;
use crate::common::utils::time::current_month;
use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
use crate::features::launchpad_partner::error::PartnerError;
use crate::features::launchpad_partner::models::LaunchpadDeduction;
use crate::features::launchpad_partner::types::{
    DeductBody, DeductResponse, HealthResponse, LookupResponse,
};

/// Read the user's current-month console balance.
pub async fn lookup(company_user_key: &str) -> Result<LookupResponse, PartnerError> {
    let cfg = LaunchpadPartnerConfig::default();
    let common = crate::common::CommonConfig::default();
    let biyard = common.biyard();
    let pk = Partition::User(company_user_key.to_string());

    let balance = biyard
        .get_user_balance(pk, current_month())
        .await
        .map_err(|e| {
            crate::error!("launchpad lookup: biyard balance failed: {e}");
            PartnerError::UnknownUser
        })?;

    Ok(LookupResponse {
        available_points: balance.balance,
        point_symbol: cfg.point_symbol.to_string(),
    })
}

/// Idempotently deduct points by issuing a console Exchange transaction.
pub async fn deduct(req: &DeductBody) -> Result<DeductResponse, PartnerError> {
    if req.point_amount <= 0 {
        return Err(PartnerError::InvalidAmount);
    }
    let common = crate::common::CommonConfig::default();
    let cli = common.dynamodb();
    let biyard = common.biyard();
    let pk = Partition::User(req.company_user_key.clone());

    // Idempotency: replay a stored result instead of double-spending.
    if let Ok(Some(existing)) = LaunchpadDeduction::get(
        cli,
        pk.clone(),
        Some(crate::common::types::EntityType::LaunchpadDeduction(
            req.idempotency_key.clone(),
        )),
    )
    .await
    {
        return Ok(DeductResponse {
            brand_tx_id: existing.brand_tx_id,
            deducted_points: existing.point_amount,
            remaining_points: existing.remaining_points,
        });
    }

    let tx = biyard
        .exchange_points(pk.clone(), req.point_amount, current_month())
        .await
        .map_err(|e| {
            crate::error!("launchpad deduct: biyard exchange failed: {e}");
            PartnerError::Insufficient
        })?;

    // Exchange returns no remaining balance; re-query for it.
    let remaining = biyard
        .get_user_balance(pk, current_month())
        .await
        .map(|b| b.balance)
        .unwrap_or(0);

    let row = LaunchpadDeduction::new(
        &req.company_user_key,
        &req.idempotency_key,
        req.point_amount,
        &tx.transaction_id,
        remaining,
    );
    if let Err(e) = row.create(cli).await {
        crate::error!("launchpad deduct: idempotency row write failed: {e}");
    }

    Ok(DeductResponse {
        brand_tx_id: tx.transaction_id,
        deducted_points: req.point_amount,
        remaining_points: remaining,
    })
}

/// Health check — no console call; just confirms config + signature.
pub fn health() -> HealthResponse {
    let cfg = LaunchpadPartnerConfig::default();
    HealthResponse {
        ok: true,
        project_id: cfg.project_id.to_string(),
        service: "ratel".to_string(),
    }
}
```

> Cross-check method/field names against `app/ratel/src/common/services/biyard/mod.rs`: `get_user_balance(Partition, String) -> UserPointBalanceResponse{ balance }`, `exchange_points(Partition, i64, String) -> TransactPointResponse{ transaction_id }`. If `crate::error!` macro is unavailable in scope, use `tracing::error!`.

- [ ] **Step 4: Compile-check the server target**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features server`
Expected: compiles clean (server.rs/router not wired yet, but these files must type-check). If `crate::common::CommonConfig::default()` is the wrong path, mirror `features/social/pages/reward/user/controllers/get_rewards.rs` which does `let cfg = crate::common::CommonConfig::default(); let biyard = cfg.biyard();`.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/src/features/launchpad_partner/error.rs app/ratel/src/features/launchpad_partner/types.rs app/ratel/src/features/launchpad_partner/controllers.rs
git commit -m "feat(launchpad_partner): callback DTOs, errors, and console-delegating handlers"
```

---

## Task 6: Axum router + wiring + integration tests

**Files:**
- Create: `app/ratel/src/features/launchpad_partner/server.rs`
- Modify: `app/ratel/src/common/run.rs`
- Modify: `app/ratel/src/tests/setup.rs`
- Test: `app/ratel/src/tests/launchpad_partner_tests.rs` (extend)

- [ ] **Step 1: Create the router + outer handlers**

Create `app/ratel/src/features/launchpad_partner/server.rs`:

```rust
//! Axum router for Launchpad callbacks. Routes are unauthenticated
//! (no session) and instead verified by HMAC signature, matching
//! Launchpad's `demo_preview/server.rs` contract.

#![cfg(feature = "server")]

use crate::common::axum::{
    body::Bytes,
    extract::Json,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
use crate::features::launchpad_partner::controllers;
use crate::features::launchpad_partner::crypto::verify_signature;
use crate::features::launchpad_partner::error::PartnerError;
use crate::features::launchpad_partner::types::{DeductBody, HealthBody, LookupBody};
use serde::Serialize;

pub fn router() -> Router {
    Router::new()
        .route("/launchpad/health", post(health))
        .route("/launchpad/points/lookup", post(lookup))
        .route("/launchpad/points/deduct", post(deduct))
}

#[derive(Serialize)]
struct ErrorBody {
    error: String,
}

fn err_response(e: PartnerError) -> Response {
    let status = StatusCode::from_u16(e.status()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    (status, Json(ErrorBody { error: e.to_string() })).into_response()
}

/// Verify HMAC + project id; returns the raw body parsed as T.
fn verify_and_parse<T: serde::de::DeserializeOwned>(
    headers: &HeaderMap,
    body: &Bytes,
    project_id_in_body: impl Fn(&T) -> String,
) -> Result<T, PartnerError> {
    let cfg = LaunchpadPartnerConfig::default();
    let raw = std::str::from_utf8(body).map_err(|_| PartnerError::Server)?;
    let ts = header(headers, "x-launchpad-timestamp");
    let sig = header(headers, "x-launchpad-signature");
    if !verify_signature(cfg.shared_secret, &ts, &sig, raw) {
        return Err(PartnerError::InvalidSignature);
    }
    let parsed: T = serde_json::from_str(raw).map_err(|_| PartnerError::Server)?;
    // Project id may arrive in a header or the body; require body match.
    let header_pid = header(headers, "x-launchpad-project-id");
    let body_pid = project_id_in_body(&parsed);
    let effective = if header_pid.is_empty() { body_pid } else { header_pid };
    if effective != cfg.project_id {
        return Err(PartnerError::ProjectMismatch);
    }
    Ok(parsed)
}

fn header(headers: &HeaderMap, name: &str) -> String {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

async fn health(headers: HeaderMap, body: Bytes) -> Response {
    match verify_and_parse::<HealthBody>(&headers, &body, |b| b.project_id.clone()) {
        Ok(_) => (StatusCode::OK, Json(controllers::health())).into_response(),
        Err(e) => err_response(e),
    }
}

async fn lookup(headers: HeaderMap, body: Bytes) -> Response {
    let parsed = match verify_and_parse::<LookupBody>(&headers, &body, |b| b.project_id.clone()) {
        Ok(p) => p,
        Err(e) => return err_response(e),
    };
    match controllers::lookup(&parsed.company_user_key).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => err_response(e),
    }
}

async fn deduct(headers: HeaderMap, body: Bytes) -> Response {
    let parsed = match verify_and_parse::<DeductBody>(&headers, &body, |b| b.project_id.clone()) {
        Ok(p) => p,
        Err(e) => return err_response(e),
    };
    match controllers::deduct(&parsed).await {
        Ok(resp) => (StatusCode::OK, Json(resp)).into_response(),
        Err(e) => err_response(e),
    }
}
```

> Confirm `crate::common::axum` re-exports `body::Bytes`, `extract::Json`, `routing::post`. The membership router imports `crate::common::axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router}`; add `body::Bytes` and `http::HeaderMap` from the same re-export (or `use axum::body::Bytes;` if `crate::common::axum` doesn't surface it).

- [ ] **Step 2: Wire into production router**

In `app/ratel/src/common/run.rs`, find the router-merge block (around lines 120-128) and add the partner router:

```rust
    let launchpad_partner_router = crate::features::launchpad_partner::server::router();
    let dioxus_router = dioxus::server::router(app)
        .merge(mcp_router)
        .merge(membership_router)
        .merge(arcade_router)
        .merge(cross_posting_router)
        .merge(launchpad_partner_router);
```

- [ ] **Step 3: Wire into the test app router**

In `app/ratel/src/tests/setup.rs`, find the `dioxus_router` build inside `TestContext::setup()` and add the merge:

```rust
        let dioxus_router = dioxus::server::router(App)
            .merge(mcp_router)
            .merge(arcade_router)
            .merge(crate::features::launchpad_partner::server::router());
```

- [ ] **Step 4: Write the failing integration tests**

Append to `app/ratel/src/tests/launchpad_partner_tests.rs`:

```rust
use crate::axum::body::Body;
use crate::axum::http::Request;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use tower::ServiceExt;

const TEST_PROJECT_ID: &str = "launchpad-demo";
const TEST_SECRET: &str = "dev-demo-shared-secret-change-me";

fn sign(ts: &str, body: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(TEST_SECRET.as_bytes()).unwrap();
    mac.update(ts.as_bytes());
    mac.update(b".");
    mac.update(body.as_bytes());
    hex::encode(mac.finalize().into_bytes())
}

async fn post_callback(
    ctx: &TestContext,
    path: &str,
    body: &str,
    signed: bool,
    project_id: &str,
) -> u16 {
    let ts = "1717459200000";
    let mut builder = Request::builder()
        .method("POST")
        .uri(path)
        .header("content-type", "application/json")
        .header("x-launchpad-timestamp", ts)
        .header("x-launchpad-project-id", project_id);
    let sig = if signed { sign(ts, body) } else { "deadbeef".to_string() };
    builder = builder.header("x-launchpad-signature", sig);
    let req = builder.body(Body::from(body.to_string())).unwrap();
    let resp = ctx.app.clone().oneshot(req).await.unwrap();
    resp.status().as_u16()
}

#[tokio::test]
async fn health_ok_with_valid_signature() {
    let ctx = TestContext::setup().await;
    let body = format!(
        r#"{{"project_id":"{TEST_PROJECT_ID}","check":"launchpad_company_point_health"}}"#
    );
    let status = post_callback(&ctx, "/launchpad/health", &body, true, TEST_PROJECT_ID).await;
    assert_eq!(status, 200, "valid health should be 200");
}

#[tokio::test]
async fn callback_rejects_bad_signature() {
    let ctx = TestContext::setup().await;
    let body = format!(
        r#"{{"project_id":"{TEST_PROJECT_ID}","check":"launchpad_company_point_health"}}"#
    );
    let status = post_callback(&ctx, "/launchpad/health", &body, false, TEST_PROJECT_ID).await;
    assert_eq!(status, 401, "bad signature should be 401");
}

#[tokio::test]
async fn callback_rejects_project_mismatch() {
    let ctx = TestContext::setup().await;
    let body = r#"{"project_id":"wrong-project","check":"launchpad_company_point_health"}"#;
    let status = post_callback(&ctx, "/launchpad/health", body, true, "wrong-project").await;
    assert_eq!(status, 403, "project mismatch should be 403");
}

#[tokio::test]
async fn deduct_is_idempotent_without_console() {
    // Pre-seed the idempotency row; the second deduct must return the stored
    // result WITHOUT calling the console (which is unreachable in tests).
    let ctx = TestContext::setup().await;
    let user = format!("u-{}", uuid::Uuid::new_v4());
    let key = format!("lp_{}", uuid::Uuid::new_v4());
    LaunchpadDeduction::new(&user, &key, 500, "tx_seed", 740)
        .create(&ctx.ddb)
        .await
        .expect("seed");

    let body = format!(
        r#"{{"project_id":"{TEST_PROJECT_ID}","company_user_key":"{user}","point_amount":500,"idempotency_key":"{key}"}}"#
    );
    let status = post_callback(&ctx, "/launchpad/points/deduct", &body, true, TEST_PROJECT_ID).await;
    assert_eq!(status, 200, "idempotent replay should be 200 without console");
}
```

> The compile-time `option_env!` defaults in `config.rs` are `launchpad-demo` / `dev-demo-shared-secret-change-me`, so the test constants match without env injection. If CI sets `LAUNCHPAD_PROJECT_ID`/`LAUNCHPAD_PARTNER_SECRET` at compile time, update the test constants to match. `tower` and `hmac`/`sha2`/`hex` are already workspace deps used by other tests.

- [ ] **Step 5: Run the tests**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- launchpad_partner_tests`
Expected: `health_ok_with_valid_signature`, `callback_rejects_bad_signature`, `callback_rejects_project_mismatch`, `deduct_is_idempotent_without_console`, `deduction_row_round_trips` all PASS.

- [ ] **Step 6: Compile-check server target**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features server`
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add app/ratel/src/features/launchpad_partner/server.rs app/ratel/src/common/run.rs app/ratel/src/tests/setup.rs app/ratel/src/tests/launchpad_partner_tests.rs
git commit -m "feat(launchpad_partner): HMAC-verified callback router + integration tests"
```

---

## Task 7: Entry button view + i18n

**Files:**
- Create: `app/ratel/src/features/launchpad_partner/i18n.rs`
- Create: `app/ratel/src/features/launchpad_partner/views/connect_button.rs`

- [ ] **Step 1: Create i18n strings**

Create `app/ratel/src/features/launchpad_partner/i18n.rs`:

```rust
use dioxus_translate::translate;

translate! {
    LaunchpadPartnerTranslate;
    convert_cta: { en: "Convert on Launchpad", ko: "Launchpad에서 전환하기" },
}
```

> Confirm ratel's i18n macro path. Other ratel features use `dioxus_translate::translate;` — match a sibling `i18n.rs` (e.g. `features/membership/i18n.rs`) for the exact import and `translate!` syntax/locale hook.

- [ ] **Step 2: Create the connect button component**

Create `app/ratel/src/features/launchpad_partner/views/connect_button.rs`:

```rust
//! Per-user "Convert on Launchpad" entry point. Builds the encrypted
//! handoff URL for the signed-in ratel user and renders it as a link.

use crate::common::*;
use crate::features::launchpad_partner::config::LaunchpadPartnerConfig;
use crate::features::launchpad_partner::i18n::LaunchpadPartnerTranslate;

#[component]
pub fn LaunchpadConnectButton(user_id: String) -> Element {
    let url = build_entry_url(&user_id);
    let tr = LaunchpadPartnerTranslate::new(use_locale());

    rsx! {
        a {
            class: "launchpad-connect-btn",
            href: "{url}",
            "{tr.convert_cta}"
        }
    }
}

/// `{base}/connect?project_id={pid}&lp_user={token}`. Token is built only
/// on the server (encryption needs the shared secret); on the web target
/// the link falls back to the bare connect URL (the server-rendered href
/// already carries the token after SSR).
fn build_entry_url(user_id: &str) -> String {
    let cfg = LaunchpadPartnerConfig::default();
    let base = cfg.base_url.trim_end_matches('/');

    #[cfg(feature = "server")]
    {
        match crate::features::launchpad_partner::crypto::encrypt_user_token(
            cfg.shared_secret,
            user_id,
        ) {
            Ok(token) => format!(
                "{base}/connect?project_id={}&lp_user={token}",
                cfg.project_id
            ),
            Err(_) => format!("{base}/connect?project_id={}", cfg.project_id),
        }
    }
    #[cfg(not(feature = "server"))]
    {
        let _ = user_id;
        format!("{base}/connect?project_id={}", cfg.project_id)
    }
}
```

> `use_locale` + `#[component]` + `rsx!` come through `crate::common::*` in ratel components — verify against a sibling view. If the token must be present in the web bundle's href, expose a tiny server fn that returns the token and call it from a `use_loader`; for the demo, SSR-rendered href is sufficient. Add the `.launchpad-connect-btn` style per ratel's styling convention (Tailwind classes inline are also fine).

- [ ] **Step 3: Compile-check both targets**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features web`
Then: `cargo check --features server`
Expected: both clean.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/features/launchpad_partner/i18n.rs app/ratel/src/features/launchpad_partner/views/connect_button.rs
git commit -m "feat(launchpad_partner): connect-button view + i18n"
```

---

## Task 8: Mount the button + manual E2E doc

**Files:**
- Modify: a ratel page to render `LaunchpadConnectButton` (decide concrete page below)
- Create: `RUN-launchpad-demo.md` (repo root)

- [ ] **Step 1: Mount the button on a visible page**

Render `LaunchpadConnectButton { user_id: <signed-in user uuid> }` on a page the demo user lands on after login. Recommended: the user's reward/points view (`app/ratel/src/features/social/pages/reward/user/`), since that already shows the console balance — the conversion CTA belongs next to the balance. Obtain the signed-in user's uuid from the existing auth context/user (the inner string of `Partition::User`). Import the component:

```rust
use crate::features::launchpad_partner::views::LaunchpadConnectButton;
// ...inside the rsx! where the balance is shown:
LaunchpadConnectButton { user_id: user_uuid.clone() }
```

> Pick the exact insertion point by reading the chosen page's `rsx!`. Keep it minimal — one CTA next to the balance.

- [ ] **Step 2: Compile-check**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features web && cargo check --features server`
Expected: clean.

- [ ] **Step 3: Write the manual E2E runbook**

Create `RUN-launchpad-demo.md` at the repo root:

```markdown
# Ratel ⇄ Launchpad point integration — demo runbook

## 1. Launchpad (admin setup, no code change)
1. Run launchpad locally on :8080 (`make run` in the launchpad repo).
2. Admin → create a project; complete company-point setup:
   - Company URL: `http://localhost:8000`
   - point paths: defaults (`/launchpad/points/lookup`, `/launchpad/points/deduct`)
   - point symbol: `P`
3. Copy the generated shared secret (`lps_...`) and the project id.
4. Create and OPEN a point conversion round.
5. Ensure the launchpad demo user has a `wallet_address` (wallet sign-in).

## 2. Ratel (this repo)
Build/run with the launchpad values injected at compile time:

```bash
cd app/ratel
LAUNCHPAD_PROJECT_ID="<project_id>" \
LAUNCHPAD_PARTNER_SECRET="<lps_secret>" \
LAUNCHPAD_BASE_URL="http://localhost:8080" \
DYNAMO_TABLE_PREFIX=launchpad-local \
  dx serve --port 8000 --web
```

Ratel must also have its `BIYARD_*` env set so the demo user has console points.

## 3. Verify the callback wiring
From launchpad admin, run the company-point health check → expects 200 from
`http://localhost:8000/launchpad/health`.

## 4. Demo flow
1. Log into ratel; open the points page → balance shown (from console).
2. Click "Launchpad에서 전환하기" → lands on launchpad `/connect` (be logged
   into launchpad in the same browser).
3. `/connect` shows the live balance (launchpad → ratel → console).
4. Convert N points → console balance drops by N; launchpad records the
   off-chain conversion.
5. Refresh ratel → reduced balance. Retry the same convert → no double-deduct.
```

- [ ] **Step 4: Final full verification**

Run:
```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features server
DYNAMO_TABLE_PREFIX=launchpad-local RUSTFLAGS='-D warnings' cargo check --features web
DYNAMO_TABLE_PREFIX=launchpad-local cargo test --features "full,bypass" -- launchpad_partner
```
Expected: checks clean; all `launchpad_partner` tests pass.

- [ ] **Step 5: Commit**

```bash
git add -A
git commit -m "feat(launchpad_partner): mount connect button + demo runbook"
```

---

## Self-Review Notes

- **Spec coverage:** config (T2), crypto/token+HMAC (T3), idempotency model (T4), callback contract health/lookup/deduct (T5–T6), console delegation via `get_user_balance`/`exchange_points` (T5), entry button (T7), launchpad config + scenario (T8 runbook). All spec sections mapped.
- **Console happy-path (lookup, real deduct) is verified MANUALLY** in T8, not by automated test — the console (`api.biyard.co`) is an external dependency with no in-repo mock. Automated tests cover signature (401/403), health (200), and idempotent-replay (200 without console). This is the explicit, allowed "manual verification" carve-out.
- **Type consistency:** `LaunchpadDeduction::new(company_user_key, idempotency_key, point_amount, brand_tx_id, remaining_points)` used identically in T4 model, T5 deduct handler, and T6 seed test. `EntityType::LaunchpadDeduction(String)` used in T1, T4, T5. Biyard methods named per `services/biyard/mod.rs`.
- **Known risks** (monthly balance, Exchange tx_type semantics) are documented in the spec and accepted; no task changes console behavior.
