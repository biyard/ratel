use super::*;

// Browser build: Firebase JS SDK loaded via window.ratel.invoke into
// `signInWithPopup` against gstatic.com. The popup flow doesn't work
// reliably inside Tauri's WebView (Google returns disallowed_useragent),
// so the tauri-web build below replaces both functions with a native
// bridge through tauri-plugin-google-auth.
#[cfg(all(feature = "web", not(feature = "tauri-web")))]
define_invoke_js!(
    init_firebase,
    "init_firebase",
    crate::common::FirebaseConfig
);
#[cfg(all(feature = "web", not(feature = "tauri-web")))]
define_invoke_js!(sign_in, "signIn", res: super::UserInfo);

// Tauri-web build: route through native Google Sign In on Android via
// `tauri-plugin-google-auth`. The plugin's `signIn` command uses Credential
// Manager + Google AuthorizationClient and returns tokens directly from
// Google. No Firebase JS init is needed because we never call into
// `window.ratel.signIn` in this build.
#[cfg(feature = "tauri-web")]
pub async fn init_firebase(_: &crate::common::FirebaseConfig) -> crate::common::Result<()> {
    Ok(())
}

define_invoke_tauri!(sign_in, "google_sign_in", res: crate::tauri::TokenResponse);
