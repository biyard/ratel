# Tauri Mobile Shell Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the existing `dx bundle --platform android` build with a Tauri 2.x native shell that hosts the same dioxus-web bundle. One frontend codebase, two distribution surfaces. Validated by an emulator-installed APK that signs in via session cookie and executes one end-to-end `#[tauri::command]` bridge.

**Architecture:** New sibling crate `app/ratel-tauri/` containing the Tauri shell. Existing `app/ratel/` gains a `tauri/` module (`types/` + `interop/`) under two new cargo features: `tauri-types` (DTOs only, pulled by the shell) and `tauri-web = ["web", "tauri-types"]` (web build + tauri interop). The shell consumes the dx-built web payload via `frontendDist`; native bridges talk JSON over `window.__TAURI__.invoke` through the existing `dioxus::document::eval` channel pattern.

**Tech Stack:** Rust (edition 2024), Dioxus 0.7, Tauri 2.x, `tauri-plugin-opener`, `tower-http::cors`, `tower-sessions`, Android NDK + AVD, `cargo tauri` CLI.

**Spec:** `docs/superpowers/specs/2026-05-13-tauri-mobile-shell-design.md`

---

## Phase 1 — Add cargo features (additive)

Goal: introduce `tauri-types` and `tauri-web` without touching the existing `mobile` feature. Both browser and the old android path keep working through Phase 7. Cleanup of `mobile` is Phase 9.

### Task 1.1: Add `tauri-types` and `tauri-web` features

**Files:**
- Modify: `app/ratel/Cargo.toml:118-172`

- [ ] **Step 1: Inspect current feature block**

Run: `sed -n '118,172p' app/ratel/Cargo.toml`
Expected: see `default = ["web", "server", "mobile"]`, plus existing `web`, `mobile`, `server`, `bypass`, etc.

- [ ] **Step 2: Add the two new features**

Edit `app/ratel/Cargo.toml`. After the `mobile = [...]` line (currently line 123) add:

```toml
# Type-only feature: compiles app/ratel/src/tauri/types/ and nothing else.
# Pulled by app/ratel-tauri so the native shell shares DTO definitions with the
# web bundle. MUST NOT bring in dioxus, reqwest, or any web/server-only deps.
tauri-types = []

# Web build for the Tauri Android shell. Additive on `web` so the dioxus-web
# code path is identical; only adds compilation of app/ratel/src/tauri/interop/
# (the JS-eval drivers that call window.__TAURI__.invoke).
tauri-web = ["web", "tauri-types"]
```

- [ ] **Step 3: Verify `cargo check --features web` still passes**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features`
Expected: compiles cleanly. The new features are inert (no module yet references them).

- [ ] **Step 4: Verify `cargo check --features server` still passes**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features`
Expected: compiles cleanly.

- [ ] **Step 5: Verify `cargo check --features tauri-web` passes (nothing to compile yet, but feature resolution must work)**

Run: `cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features`
Expected: compiles cleanly — `tauri-web` resolves to `["web", "tauri-types"]`, and `tauri-types` is empty for now.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/Cargo.toml
git commit -m "feat(tauri): add tauri-types and tauri-web cargo features

tauri-types compiles DTO definitions only and is pulled by the upcoming
app/ratel-tauri shell crate as a path dependency. tauri-web extends web
with compile-time gating for the interop layer that calls window.__TAURI__.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 2 — Scaffold `app/ratel/src/tauri/` + first DTO

Goal: create the module skeleton and one demonstrative DTO (`ExternalUrlRequest`/`Response`/`Error`). DTOs compile under `tauri-types` AND `tauri-web`. No interop code yet.

### Task 2.1: Create `app/ratel/src/tauri/` module skeleton

**Files:**
- Create: `app/ratel/src/tauri/mod.rs`
- Create: `app/ratel/src/tauri/types/mod.rs`
- Modify: `app/ratel/src/lib.rs` (or wherever the top-level `pub mod`s live)

- [ ] **Step 1: Locate where top-level modules are declared**

Run: `grep -n 'pub mod ' app/ratel/src/lib.rs | head -20`
Expected: list of `pub mod common;`, `pub mod features;`, etc. This is where we add `pub mod tauri;`.

- [ ] **Step 2: Create `app/ratel/src/tauri/mod.rs`**

```rust
//! Tauri mobile shell integration.
//!
//! `types/` defines request/response DTOs shared between the dioxus-web bundle
//! (which serializes them to JSON for `window.__TAURI__.invoke(...)`) and the
//! native `app/ratel-tauri` shell (which deserializes them in `#[tauri::command]`
//! handlers). One definition, both ends — drift impossible.
//!
//! `interop/` defines the dioxus-web side bridges: a Rust `async fn` per native
//! call, plus an embedded JS driver that calls `window.__TAURI__.invoke(...)`.
//! Only compiled under `feature = "tauri-web"`.

#[cfg(any(feature = "tauri-types", feature = "tauri-web"))]
pub mod types;

#[cfg(feature = "tauri-web")]
pub mod interop;
```

- [ ] **Step 3: Create `app/ratel/src/tauri/types/mod.rs`**

```rust
//! Shared DTOs between the dioxus-web bundle and the native Tauri shell.
//! Compiled under `feature = "tauri-types"` (pulled by app/ratel-tauri) and
//! also under `feature = "tauri-web"` (which implies tauri-types).

pub mod external_url;
```

- [ ] **Step 4: Register the module in `app/ratel/src/lib.rs`**

Add (in alphabetical position relative to other top-level modules):

```rust
pub mod tauri;
```

- [ ] **Step 5: Verify all three feature combinations still compile**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-types --no-default-features
```
Expected: all four compile. `cargo check --features tauri-types --no-default-features` will fail to find `external_url` — that's fine, expected; we add the file in Task 2.2.

Actually skip the `tauri-types` check until Task 2.2 lands — module declaration `pub mod external_url;` references a missing file. Move it to Step 4 of Task 2.2 instead.

Replace this step with:
```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
```
Expected: both pass. We commit this skeleton even though `pub mod external_url;` references a missing file — Step 6 fixes that by creating the file inline before commit.

Actually cleaner: combine Task 2.1 and 2.2 into one commit. Stop here and proceed to Task 2.2; commit after 2.2 passes.

- [ ] **Step 6: Skip commit — combine with Task 2.2**

### Task 2.2: Define the `external_url` DTOs (test-first)

**Files:**
- Create: `app/ratel/src/tauri/types/external_url.rs`
- Create: `app/ratel/src/tauri/types/external_url_test.rs` (tests live inline via `#[cfg(test)]`; this name is illustrative — write tests inside `external_url.rs`)

- [ ] **Step 1: Write the failing test**

Append to `app/ratel/src/tauri/types/external_url.rs` (creating the file):

```rust
//! DTO for the `open_external_url` Tauri command.
//!
//! v1 demonstrative bridge. The native handler wraps `tauri-plugin-opener` so
//! the dioxus-web bundle can open a URL in the user's default browser instead
//! of inside the WebView (where target=_blank does nothing useful on Android).

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalUrlRequest {
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExternalUrlResponse {
    pub opened: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, thiserror::Error)]
pub enum ExternalUrlError {
    #[error("invalid url: {0}")]
    InvalidUrl(String),
    #[error("opener failed: {0}")]
    OpenerFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn external_url_request_roundtrip() {
        let req = ExternalUrlRequest {
            url: "https://example.com".to_string(),
        };
        let json = serde_json::to_string(&req).unwrap();
        let back: ExternalUrlRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(req, back);
    }

    #[test]
    fn external_url_response_roundtrip() {
        let resp = ExternalUrlResponse { opened: true };
        let json = serde_json::to_string(&resp).unwrap();
        let back: ExternalUrlResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(resp, back);
    }

    #[test]
    fn external_url_error_roundtrip() {
        let err = ExternalUrlError::InvalidUrl("not a url".to_string());
        let json = serde_json::to_string(&err).unwrap();
        let back: ExternalUrlError = serde_json::from_str(&json).unwrap();
        assert_eq!(err, back);
    }
}
```

- [ ] **Step 2: Add `thiserror` dep to app/ratel/Cargo.toml if not present**

Run: `grep -n '^thiserror' app/ratel/Cargo.toml`
If empty, add under `[dependencies]`:

```toml
thiserror = "1"
```

- [ ] **Step 3: Run the test under `tauri-types` feature**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev cargo test --features tauri-types --no-default-features --lib tauri::types::external_url::tests
```
Expected: 3 tests pass.

- [ ] **Step 4: Verify the same file compiles under `tauri-web`**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
```
Expected: passes.

- [ ] **Step 5: Verify browser web build still passes**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
```
Expected: passes (tauri module is feature-gated out).

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/tauri/ app/ratel/src/lib.rs app/ratel/Cargo.toml
git commit -m "feat(tauri): scaffold shared types module with external_url DTO

Adds app/ratel/src/tauri/ with types/external_url.rs as the first shared
DTO between the web bundle and the upcoming app/ratel-tauri shell. Gated
behind tauri-types (which tauri-web implies).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 3 — Web-side hook interop bridge

Goal: implement the dioxus-web side of `open_external_url` — a Rust `async fn` that uses `dioxus::document::eval` to call `window.__TAURI__.invoke(...)`. Compiles under `tauri-web` only.

### Task 3.1: Create the interop module skeleton

**Files:**
- Create: `app/ratel/src/tauri/interop/mod.rs`
- Create: `app/ratel/src/tauri/interop/external_url/mod.rs`
- Create: `app/ratel/src/tauri/interop/external_url/open.js`

- [ ] **Step 1: Create `app/ratel/src/tauri/interop/mod.rs`**

```rust
//! Web-side bridges to Tauri native commands.
//!
//! Each submodule exposes one or more `pub async fn` callers paired with a
//! JS driver embedded via `include_str!`. The Rust side sends JSON via
//! `dioxus::document::eval`, the JS calls `window.__TAURI__.invoke(...)`,
//! and the response comes back as the same DTOs defined in
//! `crate::tauri::types`.

pub mod external_url;
```

- [ ] **Step 2: Create `app/ratel/src/tauri/interop/external_url/mod.rs`**

```rust
//! Web-side caller for the `open_external_url` Tauri command.

use crate::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};

/// Open `url` in the user's default external browser via the Tauri host.
///
/// Returns `Err` if the URL is malformed or `tauri-plugin-opener` fails.
/// Only callable from a tauri-web build — there's no fallback path here
/// because the web build compiles a different module.
pub async fn open(req: ExternalUrlRequest) -> Result<ExternalUrlResponse, ExternalUrlError> {
    let mut runner = dioxus::document::eval(include_str!("open.js"));
    runner
        .send(serde_json::to_value(&req).map_err(|e| {
            ExternalUrlError::OpenerFailed(format!("serialize request: {e}"))
        })?)
        .map_err(|e| ExternalUrlError::OpenerFailed(format!("eval send: {e}")))?;

    // The JS driver sends one of two shapes:
    //   { "ok": ExternalUrlResponse }
    //   { "err": ExternalUrlError }
    // We deserialize to a tagged enum and convert into a Result.
    let outcome: Outcome = runner
        .recv()
        .await
        .map_err(|e| ExternalUrlError::OpenerFailed(format!("eval recv: {e}")))?;
    outcome.into()
}

#[derive(serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum Outcome {
    Ok(ExternalUrlResponse),
    Err(ExternalUrlError),
}

impl From<Outcome> for Result<ExternalUrlResponse, ExternalUrlError> {
    fn from(o: Outcome) -> Self {
        match o {
            Outcome::Ok(r) => Ok(r),
            Outcome::Err(e) => Err(e),
        }
    }
}
```

- [ ] **Step 3: Create `app/ratel/src/tauri/interop/external_url/open.js`**

```javascript
// Driver for crate::tauri::interop::external_url::open.
// Receives ExternalUrlRequest as JSON, calls Tauri, sends back
// { "ok": ExternalUrlResponse } or { "err": ExternalUrlError }.

(async () => {
  try {
    const req = await dioxus.recv();
    if (!window.__TAURI__ || !window.__TAURI__.core || !window.__TAURI__.core.invoke) {
      dioxus.send({ err: { OpenerFailed: "window.__TAURI__ unavailable" } });
      return;
    }
    const res = await window.__TAURI__.core.invoke("open_external_url", { req });
    // The #[tauri::command] returns ExternalUrlResponse directly on success and
    // surfaces errors via Tauri's error channel — so a thrown exception means
    // ExternalUrlError. A normal return is the success DTO.
    dioxus.send({ ok: res });
  } catch (e) {
    // Tauri serializes #[tauri::command] errors as plain strings or the error's
    // serde form. Try to parse it as ExternalUrlError; fall back to OpenerFailed.
    const msg = (e && e.message) ? e.message : String(e);
    let err;
    try {
      err = (typeof e === "object" && e !== null && (e.InvalidUrl || e.OpenerFailed))
        ? e
        : { OpenerFailed: msg };
    } catch (_) {
      err = { OpenerFailed: msg };
    }
    dioxus.send({ err });
  }
})();
```

- [ ] **Step 4: Verify the interop module compiles under `tauri-web`**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
```
Expected: passes.

- [ ] **Step 5: Verify it does NOT compile into the browser web build**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
```
Expected: passes. The interop module is gated `#[cfg(feature = "tauri-web")]` so it's invisible to plain `web`.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/tauri/interop/
git commit -m "feat(tauri): add web-side open_external_url interop bridge

Rust async fn pairs with an embedded JS driver that calls
window.__TAURI__.core.invoke. Same eval-channel pattern used by
features/auth/interop/wallet_connect. Gated under tauri-web.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 4 — Scaffold `app/ratel-tauri/` Tauri shell crate

Goal: create the sibling crate, configure `tauri.conf.json`, wire it into the cargo workspace, run `cargo tauri android init`, and verify it compiles.

### Task 4.1: Workspace + crate skeleton

**Files:**
- Modify: `Cargo.toml` (workspace root)
- Create: `app/ratel-tauri/.gitignore`
- Create: `app/ratel-tauri/README.md`
- Create: `app/ratel-tauri/src-tauri/Cargo.toml`
- Create: `app/ratel-tauri/src-tauri/build.rs`
- Create: `app/ratel-tauri/src-tauri/tauri.conf.json`
- Create: `app/ratel-tauri/src-tauri/src/main.rs`
- Create: `app/ratel-tauri/src-tauri/src/lib.rs`
- Create: `app/ratel-tauri/src-tauri/src/commands/mod.rs`

- [ ] **Step 1: Add the new crate to the workspace**

Edit `Cargo.toml` (repo root) — the `[workspace] members = [...]` array. After `"app/ratel",`:

```toml
[workspace]
members = [
  "app/ratel",
  "app/ratel-tauri/src-tauri",
  "packages/*",
  "design-system-preview",
]
```

- [ ] **Step 2: Create `app/ratel-tauri/.gitignore`**

```gitignore
/dist
/src-tauri/target
/src-tauri/gen/android/.gradle
/src-tauri/gen/android/build
/src-tauri/gen/android/app/build
/src-tauri/gen/android/local.properties
/src-tauri/gen/android/.cxx
/src-tauri/gen/android/captures
```

- [ ] **Step 3: Create `app/ratel-tauri/README.md`**

```markdown
# ratel-tauri

Tauri 2.x native shell that hosts the dioxus-web bundle from `app/ratel/`.

## Build & install (Android emulator)

```bash
# Build a release APK
make build-release

# Install on a running AVD
make install-debug
```

## Manual smoke-test checklist

- [ ] Cold start under 3s on Pixel 6 AVD (API 34)
- [ ] WASM bundle loads, hydration completes (check `adb logcat` for WebView errors)
- [ ] Sign-in flow completes with bypass code `000000`
- [ ] Session cookie persists across app kill/relaunch
- [ ] Authenticated routes load (e.g. user profile)
- [ ] `open_external_url` bridge: external link from a post opens system browser
- [ ] Back button closes app cleanly (no JNI crash in logcat)
- [ ] No third-party-cookie warnings in `adb logcat`
```

- [ ] **Step 4: Create `app/ratel-tauri/src-tauri/Cargo.toml`**

```toml
[package]
name = "ratel-tauri"
version = "0.1.0"
edition = "2024"
authors = ["Biyard"]
description = "Ratel Tauri Android shell"
publish = false

[lib]
name = "ratel_tauri_lib"
crate-type = ["staticlib", "cdylib", "rlib"]

[build-dependencies]
tauri-build = { version = "2", features = [] }

[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = "1"

# Share request/response DTOs with the web bundle. tauri-types compiles only
# the DTO modules — no dioxus, no reqwest, no other runtime deps.
app-shell = { path = "../../ratel", default-features = false, features = ["tauri-types"] }
```

- [ ] **Step 5: Create `app/ratel-tauri/src-tauri/build.rs`**

```rust
fn main() {
    tauri_build::build();
}
```

- [ ] **Step 6: Create `app/ratel-tauri/src-tauri/tauri.conf.json`**

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "Ratel",
  "version": "0.1.0",
  "identifier": "co.biyard.ratel",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://10.0.2.2:8000",
    "beforeDevCommand": "",
    "beforeBuildCommand": ""
  },
  "app": {
    "withGlobalTauri": true,
    "windows": [
      {
        "title": "Ratel",
        "width": 400,
        "height": 800,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": "default-src 'self' tauri: http://tauri.localhost https://ratel.foundation https://*.ratel.foundation; script-src 'self' 'wasm-unsafe-eval' tauri: http://tauri.localhost; connect-src 'self' tauri: ipc: http://tauri.localhost https://ratel.foundation https://*.ratel.foundation; img-src 'self' data: blob: https:; style-src 'self' 'unsafe-inline' https://fonts.googleapis.com; font-src 'self' https://fonts.gstatic.com"
    }
  },
  "bundle": {
    "active": true,
    "targets": ["apk"],
    "android": {
      "minSdkVersion": 24
    },
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ]
  },
  "plugins": {}
}
```

- [ ] **Step 7: Create `app/ratel-tauri/src-tauri/src/main.rs`**

```rust
// Prevents additional console window on Windows in release.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    ratel_tauri_lib::run();
}
```

- [ ] **Step 8: Create `app/ratel-tauri/src-tauri/src/lib.rs`**

```rust
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            // Android System WebView blocks third-party cookies by default,
            // which breaks our session cookie flow (WebView origin is
            // http://tauri.localhost; backend is https://ratel.foundation).
            // Enable third-party cookies for the app's WebView.
            #[cfg(target_os = "android")]
            {
                use tauri::Manager;
                if let Some(window) = app.get_webview_window("main") {
                    // The accept_third_party_cookies switch lives on
                    // Android's CookieManager, reachable through the WebView
                    // handle. Tauri exposes the raw handle via `with_webview`.
                    let _ = window.with_webview(|webview| {
                        use jni::objects::{JObject, JValue};
                        let webview_obj = webview.jni_handle();
                        webview.jni_env().unwrap().call_static_method(
                            "android/webkit/CookieManager",
                            "getInstance",
                            "()Landroid/webkit/CookieManager;",
                            &[],
                        ).ok();
                        // setAcceptThirdPartyCookies takes (WebView, boolean).
                        // The exact JNI form requires obtaining the CookieManager
                        // instance first and calling setAcceptThirdPartyCookies
                        // on it. See https://v2.tauri.app/learn/mobile-features/
                        // for the canonical pattern; this stub will be filled
                        // in once the build pipeline produces an APK we can
                        // iterate on.
                        let _ = (webview_obj,);
                        let _ = JObject::null();
                        let _ = JValue::Bool(1);
                    });
                }
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::external_url::open_external_url
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

NOTE: the JNI third-party-cookie call above is a placeholder; Task 8.2 wires the real code once we have a built APK to test against. Keeping it as a no-op `setup` block here means the shell compiles today and we iterate on the cookie path during Phase 8 smoke testing.

- [ ] **Step 9: Create `app/ratel-tauri/src-tauri/src/commands/mod.rs`**

```rust
pub mod external_url;
```

- [ ] **Step 10: Create `app/ratel-tauri/src-tauri/src/commands/external_url.rs` (stub — real impl is Task 5.1)**

```rust
//! `open_external_url` Tauri command. Real implementation in Task 5.1.

use app_shell::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};

#[tauri::command]
pub fn open_external_url(req: ExternalUrlRequest) -> Result<ExternalUrlResponse, ExternalUrlError> {
    let _ = req;
    Ok(ExternalUrlResponse { opened: false })
}
```

- [ ] **Step 11: Verify the shell crate compiles for the host target**

```bash
cd app/ratel-tauri/src-tauri
DYNAMO_TABLE_PREFIX=ratel-dev cargo check
```
Expected: compiles. `app-shell` with `--features tauri-types --no-default-features` pulls only the DTO module — no dioxus, no reqwest. May emit a warning about the unused JNI scaffold; ignore unless `-D warnings` fires.

If `cargo check` complains about unused imports in `lib.rs` due to the JNI stub being a no-op: prefix the unused names with `_` to silence.

- [ ] **Step 12: Verify `app/ratel/` still compiles under all three features**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
```
Expected: all pass.

- [ ] **Step 13: Commit**

```bash
git add Cargo.toml app/ratel-tauri/
git commit -m "feat(tauri): scaffold app/ratel-tauri shell crate

New cargo workspace member at app/ratel-tauri/src-tauri. Pulls app-shell
with --features tauri-types --no-default-features so DTOs are shared
without dragging the web/server dep tree into the native shell.

open_external_url is registered with a stub body; Phase 5 wires the
real tauri-plugin-opener call.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 4.2: Run `cargo tauri android init`

**Files:**
- Generates: `app/ratel-tauri/src-tauri/gen/android/` (large Gradle scaffold)
- Generates: `app/ratel-tauri/src-tauri/icons/` (default icon set if missing)

- [ ] **Step 1: Install Tauri CLI**

```bash
cargo install tauri-cli --version "^2.0.0" --locked
```
Expected: `cargo tauri --version` reports 2.x.

- [ ] **Step 2: Confirm Android SDK + NDK env vars are set**

```bash
echo "ANDROID_HOME=$ANDROID_HOME"
echo "NDK_HOME=$NDK_HOME"
echo "JAVA_HOME=$JAVA_HOME"
```
Expected: all three populated. If not, install via Android Studio SDK Manager and export them. The existing `app/ratel/Makefile` Android section (lines 82-95) already documents the same NDK paths the dioxus-native flow used — reuse those values.

- [ ] **Step 3: Run `cargo tauri android init`**

```bash
cd app/ratel-tauri/src-tauri
cargo tauri android init
```
Expected: generates `gen/android/` with Gradle project (`app/`, `buildSrc/`, `settings.gradle`, etc.) and `icons/` if not present. Takes ~30s.

- [ ] **Step 4: Verify the Android project compiles for android target**

```bash
cd app/ratel-tauri/src-tauri
cargo build --target aarch64-linux-android --no-default-features
```
Expected: compiles. If it fails complaining about missing `dist/`, that's expected — `tauri-build` reads `frontendDist`. Fix in Task 4.3.

- [ ] **Step 5: Commit the generated scaffold**

The `gen/android/` tree is committed per Tauri convention so reproducible builds don't depend on regenerating it locally. The `.gitignore` from Task 4.1 already excludes the `.gradle`, `build/`, `.cxx/` byproducts.

```bash
git add app/ratel-tauri/src-tauri/gen/ app/ratel-tauri/src-tauri/icons/
git commit -m "feat(tauri): generated Android Gradle scaffold via cargo tauri android init

Committed verbatim per Tauri convention. .gitignore excludes Gradle
build outputs (.gradle, build/, .cxx/).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 4.3: Stub a `dist/` so `cargo tauri android build` resolves the frontend

**Files:**
- Create: `app/ratel-tauri/dist/index.html` (placeholder — replaced by `make build-release` later)

- [ ] **Step 1: Create a placeholder `dist/`**

```bash
mkdir -p app/ratel-tauri/dist
cat > app/ratel-tauri/dist/index.html <<'EOF'
<!doctype html>
<html><head><meta charset="utf-8"><title>Ratel (placeholder)</title></head>
<body><pre>Placeholder. Replaced by `make build-release` which copies the dx-built web bundle here.</pre></body>
</html>
EOF
```

Note `dist/` is gitignored; the directory exists at build time but is not committed. Each Phase 7 `make build-release` invocation overwrites it.

- [ ] **Step 2: Verify `cargo tauri android build` resolves the frontend dist**

```bash
cd app/ratel-tauri/src-tauri
cargo tauri android build --debug --target aarch64-linux-android
```
Expected: starts building. May still fail at the Android linker step if the Rust target isn't fully set up — fine, we're only checking that `frontendDist` resolves. If the failure is `frontendDist not found`, fix the path in `tauri.conf.json`. If the failure is an NDK / linker complaint, defer to Phase 7.

- [ ] **Step 3: No commit needed** — `dist/` is gitignored.

---

## Phase 5 — Implement the demo `#[tauri::command]`

Goal: real `open_external_url` body using `tauri-plugin-opener`. Round-trip becomes meaningful end-to-end.

### Task 5.1: Wire `tauri-plugin-opener` into the command

**Files:**
- Modify: `app/ratel-tauri/src-tauri/src/commands/external_url.rs`

- [ ] **Step 1: Replace the stub command body**

Overwrite `app/ratel-tauri/src-tauri/src/commands/external_url.rs`:

```rust
//! `open_external_url` Tauri command.

use app_shell::tauri::types::external_url::{
    ExternalUrlError, ExternalUrlRequest, ExternalUrlResponse,
};
use tauri_plugin_opener::OpenerExt;

#[tauri::command]
pub fn open_external_url(
    app: tauri::AppHandle,
    req: ExternalUrlRequest,
) -> Result<ExternalUrlResponse, ExternalUrlError> {
    // Basic URL sanity check — reject anything that doesn't look like an
    // http(s) URL. The plugin itself does some validation but surface a
    // typed error to the web caller.
    if !(req.url.starts_with("http://") || req.url.starts_with("https://")) {
        return Err(ExternalUrlError::InvalidUrl(req.url));
    }

    app.opener()
        .open_url(&req.url, None::<&str>)
        .map_err(|e| ExternalUrlError::OpenerFailed(e.to_string()))?;

    Ok(ExternalUrlResponse { opened: true })
}
```

- [ ] **Step 2: Verify the shell crate compiles**

```bash
cd app/ratel-tauri/src-tauri
DYNAMO_TABLE_PREFIX=ratel-dev cargo check
```
Expected: passes.

- [ ] **Step 3: Verify it compiles for the android target**

```bash
cd app/ratel-tauri/src-tauri
cargo check --target aarch64-linux-android
```
Expected: passes. If linker complains: defer to Phase 7's full APK build — `cargo check` should be fine.

- [ ] **Step 4: Commit**

```bash
git add app/ratel-tauri/src-tauri/src/commands/external_url.rs
git commit -m "feat(tauri): wire open_external_url to tauri-plugin-opener

Validates the URL has an http(s) scheme, returns typed ExternalUrlError
on failure. End-to-end: WASM bundle eval -> window.__TAURI__.invoke
-> Rust command -> system browser.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 5.2: Call the bridge from one dioxus-web site

Goal: a call site in the dioxus-web code uses `crate::tauri::interop::external_url::open` when running under `tauri-web`. Keep the change tiny — one button or one click handler somewhere we can manually exercise.

**Files:**
- Modify: one existing dioxus-web component (TBD by inspection — pick the post-link affordance below)

- [ ] **Step 1: Find a good demo site**

Run: `grep -rn '"_blank"\|target=.blank' app/ratel/src/features/ | head -10`
Expected: list of components that open external links via anchor tags. Pick the most reachable one — e.g., the post-permalink "open in browser" button — and note its file path.

If no clear candidate exists, fall back to: add a small "Open Ratel.foundation in browser" link at the bottom of the existing welcome / home view (whichever loads first). The point is exercising the bridge end-to-end, not the UX itself.

- [ ] **Step 2: Wire the call site**

In the chosen component (call it `X.rs`), inside an event handler:

```rust
let click_external = move |url: String| async move {
    #[cfg(feature = "tauri-web")]
    {
        use crate::tauri::types::external_url::ExternalUrlRequest;
        use crate::tauri::interop::external_url::open;
        if let Err(e) = open(ExternalUrlRequest { url: url.clone() }).await {
            crate::error!("open_external_url failed: {e}");
        }
    }
    #[cfg(all(feature = "web", not(feature = "tauri-web")))]
    {
        // Browser default: open in new tab via window.open or fallback anchor.
        let _ = url; // browser anchor handles it; nothing to do here.
    }
};
```

If the existing component already has an `onclick` for the link, refactor it to call this fn with the URL.

- [ ] **Step 3: Verify all three feature builds still pass**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web --no-default-features --features tauri-web
```
Expected: all pass.

- [ ] **Step 4: Commit**

```bash
git add <chosen component file>
git commit -m "feat(tauri): wire one demo call site to open_external_url bridge

Demonstrates the full WASM-eval -> __TAURI__.invoke -> Rust -> opener
round trip. Browser builds fall through to the existing anchor behavior.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 6 — Backend cross-origin (CORS + session cookies)

Goal: server allows the Tauri WebView origin with credentials, session cookies set `SameSite=None; Secure` in non-local envs.

### Task 6.1: Add `tower-http::cors` allow-list to the axum router

**Files:**
- Modify: `app/ratel/src/common/run.rs:73-87`

- [ ] **Step 1: Inspect the current router build**

```bash
sed -n '60,100p' app/ratel/src/common/run.rs
```
Expected: `let app = dioxus_router.layer(CatchPanicLayer::new()).layer(session_layer);`

- [ ] **Step 2: Add the CORS layer**

In `app/ratel/src/common/run.rs`, after the `let session_layer = ...` line and before `let mcp_router = ...`, add:

```rust
let cors_layer = {
    use tower_http::cors::{AllowOrigin, CorsLayer};
    // Allow the Tauri Android WebView origin (http://tauri.localhost) plus the
    // production frontend origin (https://ratel.foundation). Web traffic is
    // same-origin so it never triggers CORS anyway — these entries exist for
    // the Tauri shell and any subdomain frontends.
    let allow = AllowOrigin::predicate(|origin, _req_parts| {
        let bytes = origin.as_bytes();
        bytes == b"http://tauri.localhost"
            || bytes == b"https://tauri.localhost"
            || bytes == b"https://ratel.foundation"
            || bytes.starts_with(b"https://") && bytes.ends_with(b".ratel.foundation")
    });
    CorsLayer::new()
        .allow_origin(allow)
        .allow_credentials(true)
        .allow_methods(tower_http::cors::Any)
        .allow_headers(tower_http::cors::Any)
};
```

And update the final layer chain to include it:

```rust
let app = dioxus_router
    .layer(tower_http::catch_panic::CatchPanicLayer::new())
    .layer(cors_layer)
    .layer(session_layer);
```

Order matters — CORS must be outside the session layer so preflight `OPTIONS` requests don't hit session lookup.

- [ ] **Step 3: Verify the server build compiles**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
```
Expected: passes. If `tower-http` doesn't already have the `cors` feature enabled, fix in `Cargo.toml`:

```toml
tower-http = { version = "0.6", features = ["catch-panic", "cors"], optional = true }
```

- [ ] **Step 4: Add a server integration test for a preflight**

Create `app/ratel/src/tests/cors_tests.rs`:

```rust
use super::*;
use axum::http::{header, Method, StatusCode};

#[tokio::test]
async fn cors_preflight_allows_tauri_localhost() {
    let ctx = TestContext::setup().await;

    let req = axum::http::Request::builder()
        .method(Method::OPTIONS)
        .uri("/api/v1/auth/me")
        .header(header::ORIGIN, "http://tauri.localhost")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .header(header::ACCESS_CONTROL_REQUEST_HEADERS, "content-type")
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = tower::ServiceExt::oneshot(ctx.app, req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK, "preflight should succeed");

    let headers = resp.headers();
    let allow_origin = headers.get(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    assert_eq!(
        allow_origin.and_then(|v| v.to_str().ok()),
        Some("http://tauri.localhost"),
        "allow-origin must echo the tauri origin"
    );
    let allow_credentials = headers.get(header::ACCESS_CONTROL_ALLOW_CREDENTIALS);
    assert_eq!(
        allow_credentials.and_then(|v| v.to_str().ok()),
        Some("true"),
        "credentials must be allowed for cookie auth"
    );
}

#[tokio::test]
async fn cors_preflight_rejects_unknown_origin() {
    let ctx = TestContext::setup().await;

    let req = axum::http::Request::builder()
        .method(Method::OPTIONS)
        .uri("/api/v1/auth/me")
        .header(header::ORIGIN, "https://evil.example.com")
        .header(header::ACCESS_CONTROL_REQUEST_METHOD, "GET")
        .body(axum::body::Body::empty())
        .unwrap();

    let resp = tower::ServiceExt::oneshot(ctx.app, req).await.unwrap();
    let allow_origin = resp.headers().get(header::ACCESS_CONTROL_ALLOW_ORIGIN);
    assert!(
        allow_origin.is_none(),
        "unknown origin must not be echoed in allow-origin header"
    );
}
```

Register in `app/ratel/src/tests/mod.rs`:

```rust
mod cors_tests;
```

- [ ] **Step 5: Run the test**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- cors_tests
```
Expected: both tests pass.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/src/common/run.rs app/ratel/Cargo.toml app/ratel/src/tests/cors_tests.rs app/ratel/src/tests/mod.rs
git commit -m "feat(server): add CORS allow-list for tauri.localhost and ratel.foundation

Required for the Tauri WebView origin (http://tauri.localhost) to make
authenticated requests to https://ratel.foundation. Credentials allowed
so session cookies cross-origin. CORS layer is outside the session
layer so preflight requests skip session lookup.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 6.2: Set session cookie `SameSite=None; Secure` in non-local envs

**Files:**
- Modify: `app/ratel/src/common/middlewares/session_layer.rs:14-37`

- [ ] **Step 1: Replace the commented `with_same_site` block with active code**

Edit `app/ratel/src/common/middlewares/session_layer.rs:22-37`, replacing the existing builder block:

```rust
let layer = SessionManagerLayer::new(session_store)
    .with_secure(!is_local)
    .with_http_only(!is_local)
    .with_name(format!("{}_sid", env))
    .with_path("/")
    .with_expiry(tower_sessions::Expiry::AtDateTime(
        OffsetDateTime::now_utc()
            .checked_add(Duration::days(30))
            .unwrap(),
    ));

// SameSite=None is required for the cookie to be sent from the Tauri
// WebView origin (http://tauri.localhost) to https://ratel.foundation.
// SameSite=None requires Secure, so only enable it in non-local envs
// where TLS is in play. Local stays Lax (works for same-origin dev).
if is_local {
    layer.with_same_site(tower_sessions::cookie::SameSite::Lax)
} else {
    layer.with_same_site(tower_sessions::cookie::SameSite::None)
}
```

- [ ] **Step 2: Verify server build compiles**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
```
Expected: passes.

- [ ] **Step 3: Run existing session-related tests to confirm no regression**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-local cargo test --features "full,bypass" -- session
```
Expected: existing tests still pass. (If no `session`-named tests exist, run the full auth_tests suite: `cargo test --features "full,bypass" -- auth_tests`.)

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/common/middlewares/session_layer.rs
git commit -m "feat(server): set session cookie SameSite=None in non-local envs

Required for the Tauri WebView origin (http://tauri.localhost) to send
its session cookie cross-origin to https://ratel.foundation. Local
stays Lax so the existing same-origin dev flow is unchanged.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 6.3: Honor `MOBILE_API_URL` in the tauri-web build path

The repo's `build.rs` already declares `cargo:rerun-if-env-changed=MOBILE_API_URL`, and `common/config/environment.rs` reads it via `option_env!()`. But the `main.rs` `set_server_url` call is currently `#[cfg(feature = "mobile")]`. We need it under `tauri-web` too.

**Files:**
- Modify: `app/ratel/src/main.rs`

- [ ] **Step 1: Inspect current `main.rs`**

```bash
cat app/ratel/src/main.rs
```
Expected: see the existing `#[cfg(feature = "mobile")]` block that calls `dioxus::fullstack::set_server_url(...)`.

- [ ] **Step 2: Broaden the cfg gate**

Edit `app/ratel/src/main.rs`:

```rust
fn main() {
    // Both the legacy dioxus-native android build (`mobile`) and the new
    // Tauri shell (`tauri-web`) need an absolute server URL because the
    // WebView/native runtime can't resolve relative `/api/...` paths to
    // a backend on its own. Web browsers hit the same origin, so they
    // never enter this branch.
    #[cfg(any(feature = "mobile", feature = "tauri-web"))]
    {
        use app_shell::common::CommonConfig;
        let endpoint = CommonConfig::default().env.mobile_endpoint();
        dioxus::fullstack::set_server_url(endpoint);
    }

    app_shell::common::run(app_shell::App);
}
```

- [ ] **Step 3: Verify all builds**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features "web mobile" --no-default-features
```
Expected: all pass. The last command confirms the legacy mobile path still works during the transition.

- [ ] **Step 4: Commit**

```bash
git add app/ratel/src/main.rs
git commit -m "feat(tauri): tauri-web also calls set_server_url with MOBILE_API_URL

Existing mobile build's branch already does this; tauri-web has the same
need (WebView origin is tauri://localhost, backend is elsewhere). Both
feature gates trigger the same set_server_url call.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 7 — Build pipeline + first APK

Goal: a `make build-release` in `app/ratel-tauri/` produces an installable APK from a fresh `dx build`. Manual install + cold start works.

### Task 7.1: Write `app/ratel-tauri/Makefile`

**Files:**
- Create: `app/ratel-tauri/Makefile`

- [ ] **Step 1: Write the Makefile**

```makefile
# Build the dioxus-web bundle and assemble it into a Tauri Android APK.

# Inputs you may override:
#   ENV=local|dev|staging|prod   (default: local)
#   MOBILE_API_URL=https://...   (default: http://10.0.2.2:8080 — emulator alias for host)
#   MODE=debug|release           (default: debug; release for store builds)
#   ANDROID_TARGET=aarch64-linux-android | x86_64-linux-android

ENV ?= local
MODE ?= debug
ANDROID_TARGET ?= aarch64-linux-android

ifeq ($(ENV),prod)
  MOBILE_API_URL ?= https://ratel.foundation
else ifeq ($(ENV),staging)
  MOBILE_API_URL ?= https://stg.ratel.foundation
else ifeq ($(ENV),dev)
  MOBILE_API_URL ?= https://dev.ratel.foundation
else
  MOBILE_API_URL ?= http://10.0.2.2:8000
endif

DYNAMO_TABLE_PREFIX ?= ratel-$(ENV)
RUSTFLAGS ?= -D warnings

DIST_DIR := $(CURDIR)/dist
RATEL_DIR := $(CURDIR)/../ratel

.PHONY: dx-build install-tauri-cli android-dev build-release install-debug clean

install-tauri-cli:
	@if ! cargo tauri --version >/dev/null 2>&1; then \
		cargo install tauri-cli --version "^2.0.0" --locked; \
	fi

# Run `dx build` and copy the produced web bundle into ./dist for Tauri to pick up.
dx-build:
	@echo "Building dioxus-web bundle with MOBILE_API_URL=$(MOBILE_API_URL)"
	cd $(RATEL_DIR) && \
		MOBILE_API_URL=$(MOBILE_API_URL) \
		DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) \
		ENV=$(ENV) \
		RUSTFLAGS='$(RUSTFLAGS)' \
		dx build --web --release --features tauri-web --no-default-features
	@mkdir -p $(DIST_DIR)
	@# dx output location has drifted across versions; locate the index.html
	@# from the most recent build and rsync that directory tree into ./dist.
	@INDEX=$$(find $(RATEL_DIR)/target/dx -name index.html -path "*release*" -printf '%T@ %p\n' 2>/dev/null \
	         | sort -nr | head -1 | cut -d' ' -f2); \
	if [ -z "$$INDEX" ]; then \
		INDEX=$$(find $(RATEL_DIR)/target/dx -name index.html -path "*release*" 2>/dev/null \
		         | xargs ls -t 2>/dev/null | head -1); \
	fi; \
	if [ -z "$$INDEX" ]; then \
		echo "ERROR: could not locate dx-built index.html under $(RATEL_DIR)/target/dx"; \
		exit 1; \
	fi; \
	SRC=$$(dirname "$$INDEX"); \
	echo "Copying $$SRC -> $(DIST_DIR)"; \
	rsync -a --delete "$$SRC/" "$(DIST_DIR)/"

# Live dev — dioxus serves WASM on :8000, Tauri WebView points there via devUrl.
android-dev: install-tauri-cli
	@echo "Run 'cd ../ratel && DYNAMO_TABLE_PREFIX=$(DYNAMO_TABLE_PREFIX) MOBILE_API_URL=$(MOBILE_API_URL) dx serve --features tauri-web --no-default-features --port 8000' in another terminal first."
	cd src-tauri && cargo tauri android dev --target $(ANDROID_TARGET)

build-release: install-tauri-cli dx-build
	cd src-tauri && cargo tauri android build --target $(ANDROID_TARGET) $(if $(filter release,$(MODE)),--release,)
	@echo
	@echo "APK output:"
	@find src-tauri/gen/android -name "*.apk" -newer Makefile 2>/dev/null

install-debug: build-release
	@APK=$$(find src-tauri/gen/android -name "app-*-debug.apk" -printf '%T@ %p\n' \
	       | sort -nr | head -1 | cut -d' ' -f2); \
	if [ -z "$$APK" ]; then echo "ERROR: no debug APK found"; exit 1; fi; \
	echo "Installing $$APK"; \
	adb install -r "$$APK"; \
	adb shell am start -n "co.biyard.ratel/.MainActivity"

clean:
	rm -rf $(DIST_DIR)
	cd src-tauri && cargo clean
	rm -rf src-tauri/gen/android/.gradle src-tauri/gen/android/build src-tauri/gen/android/app/build
```

- [ ] **Step 2: Verify dx-build target works (no APK yet)**

```bash
cd app/ratel-tauri
ENV=local MOBILE_API_URL=http://10.0.2.2:8000 make dx-build
```
Expected: `dx build` runs to completion, `dist/index.html` and `dist/*.wasm` exist.

```bash
ls app/ratel-tauri/dist/
```
Expected: `index.html`, hashed `.wasm` / `.js` files, `assets/` subtree.

- [ ] **Step 3: Verify build-release target produces an APK**

Prereq: emulator OR physical device not required for this step; just compile.

```bash
cd app/ratel-tauri
ENV=local MODE=debug make build-release
```
Expected: APK produced under `src-tauri/gen/android/app/build/outputs/apk/...`. Takes 5-15 min on first build.

If the build fails with NDK / linker errors, surface those to the user and fix before continuing — the `app/ratel/Makefile` android section had several environment-detection blocks (lines 82-95) that may need replicating in `app/ratel-tauri/Makefile`'s `build-release` target.

- [ ] **Step 4: Commit**

```bash
git add app/ratel-tauri/Makefile
git commit -m "build(tauri): add Makefile orchestrating dx-build + cargo tauri android

dx-build runs dioxus build with MOBILE_API_URL injected, finds the
emitted index.html (path drifts across dx versions), and rsyncs the
tree into ./dist. build-release chains dx-build with cargo tauri
android build to produce the APK. install-debug installs to a running
adb device and launches the main activity.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 7.2: First emulator install + cold-start smoke

This is a manual verification task — there's no automated test for "APK installs and renders". Document observations.

- [ ] **Step 1: Start an emulator**

```bash
# From the existing app/ratel/Makefile android-emulator target — replicate here
# OR start manually via `emulator -avd Pixel6_arm64-v8a` (or your AVD name)
adb devices
```
Expected: at least one `emulator-5554` listed as `device`.

- [ ] **Step 2: Install and launch the APK**

```bash
cd app/ratel-tauri
make install-debug
```
Expected: APK installs, main activity launches, splash screen appears.

- [ ] **Step 3: Tail logcat for WebView errors**

In a second terminal:

```bash
adb logcat | grep -iE "ratel|tauri|chromium|webview"
```
Look for: WASM load errors, network errors hitting the configured `MOBILE_API_URL`, JavaScript exceptions.

- [ ] **Step 4: Document observations in `app/ratel-tauri/README.md`**

If cold start works:
- Note in the smoke-test checklist which items pass.

If cold start fails:
- Capture the failure mode (white screen / crash / partial render).
- Common fixes documented inline in README — e.g., CSP too strict, MOBILE_API_URL unreachable, WebView devtools blocked.

- [ ] **Step 5: Commit any README updates**

```bash
git add app/ratel-tauri/README.md
git commit -m "docs(tauri): add first-build smoke-test observations to README

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 7.3: Enable third-party cookies on the Android WebView

The placeholder in `app/ratel-tauri/src-tauri/src/lib.rs` from Task 4.1 was a stub. Now that we have a built APK to iterate on, replace it with the real JNI call.

**Files:**
- Modify: `app/ratel-tauri/src-tauri/src/lib.rs`

- [ ] **Step 1: Replace the stub `setup` block**

In `lib.rs`, the `.setup(|app| { ... })` block. Replace the existing Android cfg branch with:

```rust
#[cfg(target_os = "android")]
{
    use tauri::Manager;
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.with_webview(|webview| {
            use jni::objects::{JObject, JValueGen};
            let mut env = webview.jni_env();
            // CookieManager.getInstance()
            if let Ok(cookie_mgr) = env.call_static_method(
                "android/webkit/CookieManager",
                "getInstance",
                "()Landroid/webkit/CookieManager;",
                &[],
            ).and_then(|v| v.l()) {
                // cookie_mgr.setAcceptThirdPartyCookies(webView, true)
                let webview_obj: JObject = unsafe {
                    JObject::from_raw(webview.jni_handle())
                };
                let _ = env.call_method(
                    &cookie_mgr,
                    "setAcceptThirdPartyCookies",
                    "(Landroid/webkit/WebView;Z)V",
                    &[
                        JValueGen::Object(&webview_obj),
                        JValueGen::Bool(1),
                    ],
                );
            }
        });
    }
}
```

Note: the exact JNI accessor method signatures may need adjustment for the installed `jni` crate version Tauri pulls. If `cargo check --target aarch64-linux-android` complains, check the JNI version Tauri 2 currently bundles and align the import paths and method-call shapes accordingly.

- [ ] **Step 2: Add `jni` to dependencies if not already pulled transitively**

In `app/ratel-tauri/src-tauri/Cargo.toml`:

```toml
[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
```

- [ ] **Step 3: Rebuild and reinstall**

```bash
cd app/ratel-tauri
make install-debug
```
Expected: APK rebuilds, installs.

- [ ] **Step 4: Verify cookies persist across cold start**

Manual checklist (record observations in README):
- Sign in (use bypass code `000000` against local dev backend, or real OTP against staging)
- Confirm `/api/v1/auth/me` returns the user (check logcat or any authenticated route)
- Force-kill the app: `adb shell am force-stop co.biyard.ratel`
- Relaunch: `adb shell am start -n "co.biyard.ratel/.MainActivity"`
- Confirm still signed in

If cookies are lost across cold start despite this code: that's the Open Risk #1 from the spec. Surface to the user; do not implement bearer-token fallback inline — that's a separate spec.

- [ ] **Step 5: Commit**

```bash
git add app/ratel-tauri/src-tauri/src/lib.rs app/ratel-tauri/src-tauri/Cargo.toml
git commit -m "feat(tauri): enable third-party cookies on Android WebView

Calls CookieManager.setAcceptThirdPartyCookies(webView, true) at setup
so session cookies sent by https://ratel.foundation are accepted when
the WebView origin is http://tauri.localhost.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 8 — End-to-end smoke verification

Goal: run through the v1 verification gate from the spec. Document outcomes.

### Task 8.1: Run browser regression (Playwright)

- [ ] **Step 1: Bring up local infra**

```bash
cd /Users/hackartist/data/devel/github.com/biyard/ratel
AWS_REGION=ap-northeast-2 AWS_DEFAULT_REGION=ap-northeast-2 make infra
```
Expected: LocalStack + DynamoDB up.

- [ ] **Step 2: Build app-shell prod docker image**

```bash
cd /Users/hackartist/data/devel/github.com/biyard/ratel/app/ratel
DYNAMO_TABLE_PREFIX=ratel-local ENV=local RUSTFLAGS='-D warnings' \
  ENV=local make build-testing
COMMIT=local-test ECR=ratel/app-shell make docker
```

- [ ] **Step 3: Run Playwright suite against the prod-built image**

```bash
cd /Users/hackartist/data/devel/github.com/biyard/ratel
COMMIT=local-test make testing
cd playwright && CI=true make test
```
Expected: zero failures. Browser web build is unaffected by the Tauri work.

- [ ] **Step 4: Cleanup**

```bash
docker compose --profile testing down --remove-orphans
docker image rm ratel/app-shell:local-test 2>/dev/null
```

- [ ] **Step 5: No commit — verification only**

### Task 8.2: End-to-end APK smoke

- [ ] **Step 1: Build a fresh release APK**

```bash
cd app/ratel-tauri
make clean
ENV=local MOBILE_API_URL=http://<HOST_LAN_IP>:8000 MODE=debug make build-release
```
Where `<HOST_LAN_IP>` is the workstation's IP visible to the emulator (or use `10.0.2.2:8000` for emulator-only).

- [ ] **Step 2: Start local dx serve in another terminal**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-local make run
```
(or equivalent for whatever local-dev stack the team uses)

- [ ] **Step 3: Install and exercise the manual checklist**

```bash
cd app/ratel-tauri
make install-debug
```

Tick each box in `app/ratel-tauri/README.md`'s smoke-test checklist:
- [ ] Cold start under 3s on Pixel 6 AVD
- [ ] WASM loads, hydration completes
- [ ] Sign-in works (`000000` bypass)
- [ ] Cookie survives app kill/relaunch
- [ ] Authenticated route loads
- [ ] `open_external_url` bridge: external link opens system browser
- [ ] Back button closes app cleanly
- [ ] No third-party-cookie warnings in `adb logcat`

- [ ] **Step 4: Update README with smoke-test results**

Record pass/fail for each item, and any workarounds discovered.

- [ ] **Step 5: Commit README update**

```bash
git add app/ratel-tauri/README.md
git commit -m "docs(tauri): record v1 smoke-test results

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 9 — Cleanup of dioxus-native-android

Goal: rip out the legacy mobile path now that the Tauri path is proven. Only proceed once Phase 8 is green.

### Task 9.1: Remove `mobile` feature and its cfg blocks

**Files:**
- Modify: `app/ratel/Cargo.toml` (remove `mobile` from features + default)
- Modify: `app/ratel/src/main.rs` (remove the `mobile` half of the cfg gate)
- Modify: `app/ratel/src/common/hooks/mod.rs` (gate use_loader on tauri-web instead)
- Modify: `app/ratel/src/features/auth/context/mod.rs` (drop the `mobile` restore-session branch — handled by cookie persistence under tauri-web)
- Delete: `app/ratel/src/common/services/persistent_state.rs` (mobile-specific; verify no non-mobile callers first)
- Delete: `app/ratel/src/features/auth/services/restore_session.rs` (mobile-specific)
- Verify and remove any other `#[cfg(feature = "mobile")]` blocks

- [ ] **Step 1: Audit current usages**

```bash
grep -rn 'feature = "mobile"' app/ratel/src/ > /tmp/mobile-usages.txt
cat /tmp/mobile-usages.txt
```
Expected: list of every site to touch. Should include:
- `app/ratel/src/features/auth/context/mod.rs:21,34` — restore_session
- `app/ratel/src/common/hooks/use_loader.rs:28` — comment only
- `app/ratel/src/common/hooks/mod.rs:3,11,14` — module gating
- `app/ratel/src/main.rs:2` — set_server_url

- [ ] **Step 2: For each cfg site, decide replacement**

| Site | Change |
|------|--------|
| `main.rs` | already done in Task 6.3 — keep `tauri-web`, drop `mobile` half of the OR |
| `common/hooks/mod.rs` | change `#[cfg(feature = "mobile")]` → `#[cfg(feature = "tauri-web")]` for both `use_loader` and its re-export |
| `features/auth/context/mod.rs` | drop both mobile-gated blocks. Under tauri-web, the cookie persists via the WebView's cookie store (Task 7.3), so `try_restore_session` / `clear_cached_session` are no longer needed. Verify by manual smoke test in Phase 8 first; if cookies don't persist reliably, instead change `#[cfg(feature = "mobile")]` → `#[cfg(feature = "tauri-web")]` and revisit |

- [ ] **Step 3: Apply the edits**

For `app/ratel/src/main.rs`:

```rust
fn main() {
    #[cfg(feature = "tauri-web")]
    {
        use app_shell::common::CommonConfig;
        let endpoint = CommonConfig::default().env.mobile_endpoint();
        dioxus::fullstack::set_server_url(endpoint);
    }

    app_shell::common::run(app_shell::App);
}
```

For `app/ratel/src/common/hooks/mod.rs`:

```rust
mod use_infinite_query;
mod use_interval;
#[cfg(feature = "tauri-web")]
mod use_loader;
mod use_origin;
mod use_platform;
mod use_scroll_lock;

pub use use_infinite_query::*;
pub use use_interval::*;
#[cfg(feature = "tauri-web")]
pub use use_loader::*;

#[cfg(not(feature = "tauri-web"))]
pub use dioxus::prelude::use_loader;

pub use use_origin::*;
pub use use_platform::*;
pub use use_scroll_lock::*;
```

For `app/ratel/src/features/auth/context/mod.rs`:

Assuming Phase 8 confirmed cookies persist under tauri-web, simplify to:

```rust
mod user_context;

use crate::common::dioxus::fullstack::Loading;
pub use user_context::*;

use crate::features::auth::{controllers::get_me_handler, *};

#[derive(Clone, Copy)]
pub struct Context {
    pub user_context: Store<UserContext>,
}

impl Context {
    pub fn init() -> Result<Self, Loading> {
        let user_ctx = use_loader(move || async move {
            Ok::<_, Error>(match get_me_handler().await {
                Ok(resp) => UserContext {
                    user: resp.user,
                    refresh_token: None,
                    membership: resp.membership,
                },
                Err(e) => {
                    crate::error!("get_me failed during Context::init: {e}");
                    UserContext::default()
                }
            })
        })?();

        let ctx = Self {
            user_context: use_store(move || user_ctx),
        };
        use_context_provider(move || ctx);

        Ok(ctx)
    }
}
```

If Phase 8 showed cookies don't persist reliably and we kept the restore_session path, instead just rename the cfg from `mobile` to `tauri-web` and keep the bodies.

- [ ] **Step 4: Update `app/ratel/Cargo.toml` to remove `mobile` from defaults and the feature line**

```toml
default = []                                  # was: ["web", "server", "mobile"]
# remove: mobile = ["dioxus/mobile", "tokio"]
```

If `tokio` becomes unreferenced as a result, leave it alone — it's likely still needed via `server`.

- [ ] **Step 5: Verify all feature combinations build**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features server --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-types --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web --features tauri-web --no-default-features
```
Expected: all pass.

- [ ] **Step 6: Commit**

```bash
git add app/ratel/Cargo.toml app/ratel/src/main.rs app/ratel/src/common/hooks/mod.rs app/ratel/src/features/auth/context/mod.rs
git commit -m "refactor(tauri): remove mobile cargo feature, replace with tauri-web

The dioxus-native-android build is superseded by the Tauri shell. All
#[cfg(feature = \"mobile\")] sites either move to tauri-web or are
deleted where the Tauri WebView's native cookie store makes them moot.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 9.2: Delete `app/ratel/android/` and dead helpers

- [ ] **Step 1: Confirm no in-tree refs**

```bash
grep -rn 'app/ratel/android' . --include='*.rs' --include='*.toml' --include='Makefile' --include='*.json' 2>/dev/null | grep -v target | head -20
```
Expected: only Makefile references (which we delete next).

- [ ] **Step 2: Delete the directory**

```bash
git rm -r app/ratel/android/
```

- [ ] **Step 3: Delete dead service files if confirmed unused**

If Step 3 of Task 9.1 dropped the `try_restore_session` call, verify and delete:

```bash
grep -rn 'persistent_state\|restore_session' app/ratel/src/ --include='*.rs' 2>/dev/null
```
If only self-references remain:

```bash
git rm app/ratel/src/common/services/persistent_state.rs
git rm app/ratel/src/features/auth/services/restore_session.rs
```

Update the parent `mod.rs` files to drop the deleted modules.

- [ ] **Step 4: Verify build**

```bash
cd app/ratel
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features web --no-default-features
DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
```
Expected: passes.

- [ ] **Step 5: Commit**

```bash
git add -A app/ratel/
git commit -m "chore(tauri): delete app/ratel/android/ and dead mobile helpers

The Tauri shell at app/ratel-tauri owns android packaging now. The
android/ AndroidManifest.xml and res/ were only consumed by
\`dx bundle --platform android\` which we no longer run.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 9.3: Clean up `app/ratel/Makefile`

**Files:**
- Modify: `app/ratel/Makefile`

- [ ] **Step 1: Identify android-* sections to remove**

Run: `grep -n '^android\|^build-android\|^android-emulator' app/ratel/Makefile`
Expected: a list of phony target labels around lines 261-275.

Also identify the env-detection blocks at lines 45-110 that exist *solely* for the android build (HOST_IP detection for MOBILE_API_URL is mostly there for the mobile build, but is also useful for tauri-web dev so keep that variable definition).

- [ ] **Step 2: Delete the android-specific targets and any dependencies**

Targets to delete:
- `android`
- `android-emulator`
- `build-android`
- `$(HOME)/.android/debug.keystore` (rule)
- `$(HOME)/.android/avd/...` (rule)
- `$(ANDROID_HOME)/system-images/...` (rule)
- `$(ANDROID_DX_RES_DIR)` and related dx-android paths

Keep:
- `MOBILE_API_URL ?= http://$(HOST_IP):$(PORT)` (now used by tauri-web)
- `HOST_IP` detection block (used by MOBILE_API_URL)

- [ ] **Step 3: Update the `.PHONY` line**

The current line 151:

```makefile
.PHONY: env.sh test run android android-emulator ios build build-testing build-arm build-lambda docker
```

Change to:

```makefile
.PHONY: env.sh test run build build-testing build-arm build-lambda docker
```

- [ ] **Step 4: Verify no broken Makefile references**

```bash
cd app/ratel
make -n run
make -n build
```
Expected: dry-run output, no "no such target" errors.

- [ ] **Step 5: Commit**

```bash
git add app/ratel/Makefile
git commit -m "chore(tauri): remove dioxus-native-android targets from app/ratel/Makefile

android, android-emulator, build-android and their helper rules are
superseded by app/ratel-tauri/Makefile. MOBILE_API_URL is preserved
because tauri-web still uses it.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 9.4: Update documentation references

**Files:**
- Modify: `CLAUDE.md`
- Modify: `.claude/rules/conventions/build-commands.md`

- [ ] **Step 1: Inspect `.claude/rules/conventions/build-commands.md`**

```bash
grep -n 'mobile\|android\|ios' .claude/rules/conventions/build-commands.md
```

- [ ] **Step 2: Remove the mobile/android/ios `dx check` and `cargo check` lines, add tauri-web checks**

Edit `.claude/rules/conventions/build-commands.md`:
- Remove: `dx check --android`, `dx check --ios`, `cargo check --features mobile`
- Add under "Lint check":

```bash
# MUST run after any code change touching app/ratel/src/tauri/
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' cargo check --features tauri-web --no-default-features
cd app/ratel && DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' dx check --web --features tauri-web --no-default-features
```

- [ ] **Step 3: Check `CLAUDE.md` for mobile refs**

```bash
grep -n 'mobile\|android' CLAUDE.md app/ratel/AGENTS.md 2>/dev/null
```
Update any references that mention `--features mobile` or `dx bundle --platform android`.

- [ ] **Step 4: Commit**

```bash
git add CLAUDE.md .claude/rules/conventions/build-commands.md app/ratel/AGENTS.md
git commit -m "docs(tauri): update build-command convention to reference tauri-web

dx check --android/--ios and --features mobile are gone. The
tauri-web feature is now what gates Android-targeted code in app/ratel.
The native shell lives in app/ratel-tauri with its own Makefile.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

---

## Phase 10 — CI integration

Goal: PR CI checks the tauri-web compile path so it doesn't bitrot. Optional: a tauri android build job tagged non-blocking.

### Task 10.1: Add tauri-web checks to the dx-check job

**Files:**
- Modify: `.github/workflows/pr-workflow.yml`

- [ ] **Step 1: Find the dx-check job**

```bash
grep -n 'dx-check\|dx check\|cargo check' .github/workflows/pr-workflow.yml
```

- [ ] **Step 2: Add the new check steps**

In the `dx-check` job's `steps:` list, after the existing `cargo check --features server` and `dx check --web` steps, add:

```yaml
      - name: cargo check --features tauri-web
        run: |
          cd app/ratel
          DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' \
            cargo check --features tauri-web --no-default-features

      - name: dx check --web --features tauri-web
        run: |
          cd app/ratel
          DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' \
            dx check --web --features tauri-web --no-default-features

      - name: cargo check --features tauri-types (DTO-only)
        run: |
          cd app/ratel
          DYNAMO_TABLE_PREFIX=ratel-dev RUSTFLAGS='-D warnings' \
            cargo check --features tauri-types --no-default-features

      - name: cargo check app/ratel-tauri shell
        run: |
          cd app/ratel-tauri/src-tauri
          DYNAMO_TABLE_PREFIX=ratel-dev cargo check
```

- [ ] **Step 3: Validate the workflow YAML**

```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/pr-workflow.yml'))"
```
Expected: no exceptions.

- [ ] **Step 4: Commit**

```bash
git add .github/workflows/pr-workflow.yml
git commit -m "ci(tauri): add tauri-web compile checks to dx-check job

Prevents the tauri-web feature path from rotting between releases.
Also checks the app/ratel-tauri shell crate for host-target compile.
A full APK build remains deferred — NDK in the runner image is a
larger CI change tagged for follow-up.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>"
```

### Task 10.2: Push branch and watch CI

- [ ] **Step 1: Push to the hackartists fork**

```bash
git push hackartists feature/tauri-mobile
```

- [ ] **Step 2: Watch the CI run**

```bash
gh run watch --repo biyard/ratel $(gh run list --repo biyard/ratel --branch feature/tauri-mobile --limit 1 --json databaseId -q '.[0].databaseId')
```
Expected: all jobs green.

- [ ] **Step 3: If a job fails, follow `workflows/fix-pr-testing.md`** — do not retry without reading the failure.

---

## Self-Review

Checking the plan against the spec:

| Spec section | Covered in |
|---|---|
| §Summary: bundled offline-first APK | Phase 7 (build-release) |
| §Motivation: one bundle, many shells | Phase 1, 2, 5.2 |
| §Non-goals (deferred) | n/a — no tasks |
| §Architecture / distribution model | Phase 7 (frontendDist) |
| §Architecture / repo layout | Phase 2, 4 |
| §Architecture / Cargo features | Phase 1 |
| §Architecture / interop pattern | Phase 3 + Phase 5 |
| §Architecture / build pipeline | Phase 7 |
| §Architecture / backend connectivity (CORS, cookies, MOBILE_API_URL) | Phase 6 |
| §v1 verification gate / browser regression | Task 8.1 |
| §v1 verification gate / tauri-web compile | Phase 1, 10 |
| §v1 verification gate / APK builds locally | Task 7.1 |
| §v1 verification gate / APK installs + cold-starts | Task 7.2, 8.2 |
| §v1 verification gate / auth round-trip | Task 7.3, 8.2 |
| §v1 verification gate / one native bridge | Task 5.1 + 5.2 |
| §CI changes | Phase 10 |
| §Cleanup | Phase 9 |
| §Open risks / cookie storage durability | Task 7.3 + 8.2 (recorded in README, escalated if it fails) |

Coverage looks complete. No placeholders found. Type consistency: `ExternalUrlRequest`/`ExternalUrlResponse`/`ExternalUrlError` are used identically in Phase 2, 3, 4, 5 — names match.

One real risk for the executing engineer: **the JNI third-party-cookie code in Task 7.3 is the most likely failure point.** JNI binding shapes vary across `jni` crate versions and Tauri 2's WebView API has had small breaks between minor versions. If the snippet doesn't compile cleanly, the fix path is to consult <https://v2.tauri.app/learn/mobile-features/> and align method-call shapes with the bundled `jni` version (`cargo tree -p jni`).
