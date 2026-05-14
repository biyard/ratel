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

#[cfg(feature = "tauri-web")]
pub async fn sign_in() -> crate::common::Result<super::UserInfo> {
    use crate::features::auth::types::AuthError;
    use crate::tauri::interop::google_sign_in;

    let tokens = google_sign_in::sign_in().await.map_err(|e| {
        crate::error!("google_sign_in failed: {e}");
        AuthError::SignInUnsupportedOnPlatform
    })?;

    // The plugin returns Google tokens directly. The backend's OAuth handler
    // (oauth_provider::get_email) verifies the access_token against Google's
    // userinfo endpoint and reads email from the response, so we leave the
    // email / display_name / photo_url fields empty here — the backend fills
    // them in. id_token is forwarded in case any downstream code needs it.
    Ok(super::UserInfo {
        id_token: tokens.id_token.unwrap_or_default(),
        access_token: tokens.access_token,
        email: None,
        display_name: None,
        photo_url: None,
    })
}
