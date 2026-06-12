fn main() {
    // `option_env!("GOOGLE_OAUTH_CLIENT_ID")` (commands/google_sign_in.rs) is
    // read at compile time. Without this, cargo caches the previously-baked
    // value (e.g. an empty string) and re-running the build with a new
    // GOOGLE_OAUTH_CLIENT_ID silently keeps the old one — which breaks native
    // Google sign-in with "No credentials available".
    println!("cargo:rerun-if-env-changed=GOOGLE_OAUTH_CLIENT_ID");

    tauri_build::build();
}
