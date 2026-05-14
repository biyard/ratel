use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let manifest_path = Path::new(&manifest_dir);
    let js_dir = manifest_path.join("js");
    let assets_dir = manifest_path.join("assets");

    let status = Command::new("npm")
        .args(["install"])
        .current_dir(&js_dir)
        .status()
        .expect("failed to run npm install");
    assert!(status.success(), "npm install for ratel/common failed");

    let status = Command::new("npm")
        .args(["run", "build"])
        .env("ASSETS_DIR", &assets_dir)
        .current_dir(&js_dir)
        .status()
        .expect("failed to run npm build");
    assert!(status.success(), "npm build for ratel/common failed");

    println!("cargo:rerun-if-changed={}", "build.rs");
    println!("cargo:rerun-if-changed={}", js_dir.join("src").display());

    // `option_env!()` in `src/common/config/environment.rs` reads these at
    // compile time. Without `rerun-if-env-changed`, cargo caches stale values
    // into the binary even when the env vars change between builds — which
    // silently breaks `make android` when `MOBILE_API_URL` is overridden.
    println!("cargo:rerun-if-env-changed=MOBILE_API_URL");
    println!("cargo:rerun-if-env-changed=ENV");

    // For tauri-web builds, parse google-services.json (the canonical Tauri
    // config) and expose the web OAuth client_id as a compile-time env var.
    // `crate::tauri::interop::google_sign_in` reads it via `option_env!()`.
    // No manual env var setting needed — google-services.json is the single
    // source of truth.
    if std::env::var("CARGO_FEATURE_TAURI_WEB").is_ok() {
        emit_google_oauth_client_id(manifest_path);
    }
}

fn emit_google_oauth_client_id(manifest_path: &Path) {
    let gs_path: PathBuf = manifest_path
        .join("..")
        .join("ratel-tauri")
        .join("src-tauri")
        .join("google-services.json");
    println!("cargo:rerun-if-changed={}", gs_path.display());

    let raw = match std::fs::read_to_string(&gs_path) {
        Ok(s) => s,
        Err(e) => panic!(
            "feature \"tauri-web\" requires {} ({})",
            gs_path.display(),
            e
        ),
    };
    let json: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("invalid JSON in {}: {}", gs_path.display(), e));

    // google-services.json layout: client[0].oauth_client[] — the entry with
    // client_type == 3 is the Web OAuth client_id. That's what the Android
    // Credential Manager / GoogleIdOption.setServerClientId(...) expects.
    let oauth_client_id = json["client"]
        .as_array()
        .and_then(|clients| clients.first())
        .and_then(|c| c["oauth_client"].as_array())
        .and_then(|oauth_clients| {
            oauth_clients
                .iter()
                .find(|c| c["client_type"].as_i64() == Some(3))
        })
        .and_then(|c| c["client_id"].as_str())
        .unwrap_or_else(|| {
            panic!(
                "{} has no oauth_client entry with client_type=3 (web OAuth client); \
                 re-download from Firebase Console with the web app registered",
                gs_path.display()
            )
        });

    println!("cargo:rustc-env=GOOGLE_OAUTH_CLIENT_ID={oauth_client_id}");
}
