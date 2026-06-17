pub(crate) use tauri_common::*;
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_google_auth::init())
        .setup(|_app| {
            // iOS: stop WKWebView's scrollView from auto-applying the safe-area
            // contentInset. WKWebView already hands JS a viewport that excludes
            // the safe area (innerHeight 759 on iPhone 15), and our CSS then
            // subtracts env(safe-area-inset) again → the page body ends up
            // double-inset (666 instead of 759), leaving a dead band at the
            // bottom. Setting contentInsetAdjustmentBehavior = .never (raw 2)
            // makes the webview report the full viewport so the CSS inset is
            // applied exactly once — matching Android.
            #[cfg(target_os = "ios")]
            {
                use tauri::Manager;
                if let Some(window) = _app.get_webview_window("main") {
                    let _ = window.with_webview(|webview| unsafe {
                        use objc2::msg_send;
                        let wk = webview.inner() as *mut objc2::runtime::AnyObject;
                        let scroll: *mut objc2::runtime::AnyObject =
                            msg_send![wk, scrollView];
                        let _: () =
                            msg_send![scroll, setContentInsetAdjustmentBehavior: 2isize];
                        // Enable edge-swipe back/forward navigation. PortOne
                        // identity verification navigates the WHOLE WebView to
                        // the PG (Inicis/KCB) page and only returns via
                        // `redirectUrl` on success — on *cancel* it does NOT
                        // redirect back, so the WebView is stranded on the PG
                        // page. Android escapes via the system back button; iOS
                        // has none, and WKWebView ships this gesture OFF by
                        // default. Turning it on lets the user swipe left→right
                        // to return to the app (the verification page is a
                        // pushed history entry).
                        let _: () = msg_send![wk, setAllowsBackForwardNavigationGestures: true];
                        // NOTE: do NOT disable the scrollView here. Pages like
                        // the post-signup onboarding flow scroll the webview's
                        // own scrollView (their content exceeds the viewport),
                        // so `setScrollEnabled: false` would trap users on a
                        // clipped screen. It also did NOT stop the keyboard-
                        // avoidance shift it was meant to (iOS ignores
                        // scrollEnabled for that) — the topbar/modal jump is
                        // handled in CSS via `--vv-offset-top` compensation
                        // (see app.rs + use_scroll_lock + popup/mod.rs).
                    });
                }
            }
            // Third-party cookie support for Android WebView is wired in
            // Task 7.3 once we have an APK to iterate on. For now this is
            // a no-op so the shell crate compiles cleanly on the host.
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::api_request::api_request,
            commands::external_url::open_external_url,
            commands::google_sign_in::google_sign_in,
            commands::s3_upload::s3_put_object,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
