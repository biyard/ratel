fn main() {
    // The Tauri WebView serves the bundle from http://tauri.localhost, so
    // dioxus-fullstack's default "fetch from page origin" would route every
    // server function call back to itself. Pin the base URL to the actual
    // backend (baked at compile time via MOBILE_API_URL).
    #[cfg(feature = "tauri-web")]
    {
        use app_shell::common::CommonConfig;
        let endpoint = CommonConfig::default().env.mobile_endpoint();
        dioxus::fullstack::set_server_url(endpoint);
    }

    #[cfg(not(feature = "skip-server"))]
    app_shell::common::run(app_shell::App);
}
