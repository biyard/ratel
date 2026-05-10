fn main() {
    #[cfg(feature = "mobile")]
    {
        use app_shell::common::CommonConfig;

        // Override the dioxus-fullstack reqwest client BEFORE any request is
        // issued. On Android the default client (HTTP/1.1 + keep-alive) stalls
        // on the second sequential request to the same host: response arrives
        // at the kernel but never wakes the tokio task. Force HTTP/1.1 only
        // (no HTTP/2 ALPN), zero idle pool, hard 15s timeout so any hang is
        // surfaced as an error rather than silent.
        eprintln!("[ratel-mobile] installing custom reqwest client");
        let client = dioxus::fullstack::reqwest::Client::builder()
            .http1_only()
            .pool_max_idle_per_host(0)
            .pool_idle_timeout(std::time::Duration::from_millis(1))
            .tcp_nodelay(true)
            .connect_timeout(std::time::Duration::from_secs(10))
            .timeout(std::time::Duration::from_secs(15))
            .read_timeout(std::time::Duration::from_secs(10))
            .build()
            .expect("failed to build mobile reqwest client");
        match dioxus::fullstack::GLOBAL_REQUEST_CLIENT.set(client) {
            Ok(()) => eprintln!("[ratel-mobile] custom reqwest client installed"),
            Err(_) => eprintln!("[ratel-mobile] custom reqwest client install FAILED — already set"),
        }

        let endpoint = CommonConfig::default().env.mobile_endpoint();
        dioxus::fullstack::set_server_url(endpoint);

        // Force `Accept: application/json` on every server-function call from
        // the mobile binary. Without this, dioxus-server's PRG (Post-Redirect-Get)
        // middleware in `dioxus-server-0.7.6/src/serverfn.rs:120-128` sees a
        // request with `Accept: text/html` + a `Referer` header and rewrites
        // the successful 200 response into a 302 pointing at Referer, turning
        // every server function call into an infinite redirect loop on mobile.
        // The PRG path is intended for HTML form posts; mobile reqwest must
        // opt out by advertising JSON only.
        let mut headers = dioxus::fullstack::HeaderMap::new();
        headers.insert(
            dioxus::fullstack::http::header::ACCEPT,
            dioxus::fullstack::HeaderValue::from_static("application/json"),
        );
        dioxus::fullstack::set_request_headers(headers);
    }

    app_shell::common::run(app_shell::App);
}
