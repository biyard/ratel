use super::*;

// Browser build: Firebase JS SDK loaded via window.ratel.invoke into
// `signInWithPopup` against gstatic.com. The popup flow doesn't work
// reliably inside Tauri's WebView (Google returns disallowed_useragent),
// so the tauri-web build below replaces both functions with a native
// bridge through tauri-plugin-google-auth.
// Web/SSR build (dev, prod, local dx serve): Firebase JS SDK via
// `window.ratel.invoke -> signInWithPopup`. The branch is selected on
// `fullstack` rather than `not(tauri-web)` because dx silently turns on
// the `tauri-web` feature alongside `web` in some build paths, which
// used to flip this whole module into the Android-native path on the
// regular web build and route `sign_in` to `crate::tauri::sign_in`
// (the `tauri-plugin-google-auth` bridge) — that bridge is absent in
// the browser, so the wasm-bindgen import threw `undefined.then(...)`
// and tripped a `BorrowMutError` panic in dioxus' reactive context.
// Keying on `fullstack` reliably separates the two cases:
//   - regular web/SSR build → has `fullstack` → web functions below
//   - Tauri Android shell   → no `fullstack` (Cargo.toml note line 122)
//                              → native sign_in re-export further down.
#[cfg(feature = "fullstack")]
define_invoke_js!(
    init_firebase,
    "init_firebase",
    crate::common::FirebaseConfig
);
#[cfg(feature = "fullstack")]
define_invoke_js!(sign_in, "signIn", res: super::UserInfo);

// Tauri-web build (Android shell only, never web): route through native
// Google Sign In via `tauri-plugin-google-auth`. The plugin's `signIn`
// command uses Credential Manager + Google AuthorizationClient and
// returns tokens directly from Google. No Firebase JS init is needed
// because we never call into `window.ratel.signIn` in this build.
//
// Guard is `tauri-web AND NOT fullstack` so this branch is *only*
// selected by the Tauri Android build (`make build-tauri` →
// `--features tauri-web --fullstack false`). On a regular web build
// where dx silently enables `tauri-web` alongside `web,fullstack`,
// `fullstack` is on and this branch is skipped — the `fullstack` arm
// above provides the real Firebase implementation instead.
#[cfg(all(feature = "tauri-web", not(feature = "fullstack")))]
pub async fn init_firebase(_: &crate::common::FirebaseConfig) -> crate::common::Result<()> {
    Ok(())
}

#[cfg(all(feature = "tauri-web", not(feature = "fullstack")))]
pub use crate::tauri::sign_in;
