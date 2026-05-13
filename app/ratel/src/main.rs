fn main() {
    // Both the legacy dioxus-native android build (`mobile`) and the new
    // Tauri shell (`tauri-web`) need an absolute server URL because the
    // WebView/native runtime cannot resolve relative `/api/...` paths to
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
