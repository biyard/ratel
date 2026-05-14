use std::path::{Path, PathBuf};

fn main() {
    tauri_build::build();
    emit_google_oauth_client_id();
}

/// Parse `google-services.json` (sibling of this build.rs) and expose the
/// web OAuth client_id as a compile-time env var. `commands::google_sign_in`
/// reads it via `env!()` and forwards to `tauri-plugin-google-auth`. The
/// file is the single source of truth — the web bundle does not see it.
fn emit_google_oauth_client_id() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let gs_path: PathBuf = Path::new(&manifest_dir).join("google-services.json");
    println!("cargo:rerun-if-changed={}", gs_path.display());

    let raw = match std::fs::read_to_string(&gs_path) {
        Ok(s) => s,
        Err(e) => panic!(
            "ratel-tauri requires {} (the canonical Tauri config; gitignored): {}",
            gs_path.display(),
            e
        ),
    };
    let json: serde_json::Value = serde_json::from_str(&raw)
        .unwrap_or_else(|e| panic!("invalid JSON in {}: {}", gs_path.display(), e));

    // google-services.json layout: client[0].oauth_client[] — the entry with
    // client_type == 3 is the Web Application OAuth client_id. That's what
    // Android Credential Manager / GoogleIdOption.setServerClientId(...)
    // requires, NOT the Android-app client (type 1).
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
